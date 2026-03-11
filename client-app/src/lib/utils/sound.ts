/**
 * Sound Manager — qHortus Game Audio
 *
 * Dùng Web Audio API (AudioBuffer) thay vì HTMLAudioElement để:
 *   - Zero latency: play từ decoded buffer, không cần seek
 *   - Không lẫn âm: mỗi lần play là 1 BufferSourceNode độc lập
 *   - Đồng thời: correct + levelComplete có thể play cùng lúc
 *
 * Usage:
 *   import { playSound, toggleMute, initSound, isMuted } from '$lib/utils/sound';
 *
 *   onMount(() => initSound());
 *   playSound('correct');
 *   const muted = toggleMute();
 */

import { browser } from '$app/environment';

export type SoundName = 'flip' | 'correct' | 'wrong' | 'levelComplete' | 'achievement';

const SOUND_FILES: Record<SoundName, string> = {
	flip: '/sounds/flashcard_flip.ogg',
	correct: '/sounds/correct.wav',
	wrong: '/sounds/wrong.wav',
	levelComplete: '/sounds/levelcompletesplash.wav',
	achievement: '/sounds/achievement.wav'
};

// Volume tuning per sound — perceptual loudness normalization
const SOUND_VOLUMES: Record<SoundName, number> = {
	flip: 0.35,
	correct: 0.55,
	wrong: 0.45,
	levelComplete: 0.75,
	achievement: 0.85
};

const STORAGE_KEY = 'qhortus_sound_muted';

let _muted = false;
let _ctx: AudioContext | null = null;
const _buffers: Partial<Record<SoundName, AudioBuffer>> = {};

function _getContext(): AudioContext | null {
	if (!browser) return null;
	if (!_ctx) {
		_ctx = new (window.AudioContext || (window as unknown as { webkitAudioContext: typeof AudioContext }).webkitAudioContext)();
	}
	// Resume nếu bị suspend do autoplay policy của browser
	if (_ctx.state === 'suspended') {
		_ctx.resume();
	}
	return _ctx;
}

async function _loadBuffer(name: SoundName): Promise<void> {
	const ctx = _getContext();
	if (!ctx || _buffers[name]) return;
	try {
		const res = await fetch(SOUND_FILES[name]);
		const arrayBuf = await res.arrayBuffer();
		_buffers[name] = await ctx.decodeAudioData(arrayBuf);
	} catch {
		// Fail silently — file không tồn tại hoặc format không support
	}
}

/** Preload & decode tất cả sound files. Gọi sớm để zero-latency khi play. */
export function preloadSounds(): void {
	if (!browser) return;
	(Object.keys(SOUND_FILES) as SoundName[]).forEach(_loadBuffer);
}

/** Load mute preference từ localStorage. Gọi trong onMount. */
export function initSound(): boolean {
	if (!browser) return false;
	_muted = localStorage.getItem(STORAGE_KEY) === 'true';
	preloadSounds();
	return _muted;
}

/** Play sound tức thì từ decoded buffer. Silently ignored nếu muted hoặc chưa load xong. */
export function playSound(name: SoundName): void {
	if (_muted || !browser) return;
	const ctx = _getContext();
	if (!ctx) return;
	const buffer = _buffers[name];
	if (!buffer) return;

	// Mỗi lần play = 1 source node riêng → không bao giờ cắt nhau
	const source = ctx.createBufferSource();
	const gain = ctx.createGain();
	source.buffer = buffer;
	gain.gain.value = SOUND_VOLUMES[name];
	source.connect(gain);
	gain.connect(ctx.destination);
	source.start(0);
}

/** Toggle mute, persist sang localStorage. Trả về trạng thái mới. */
export function toggleMute(): boolean {
	_muted = !_muted;
	if (browser) {
		localStorage.setItem(STORAGE_KEY, String(_muted));
	}
	return _muted;
}

export function isMuted(): boolean {
	return _muted;
}
