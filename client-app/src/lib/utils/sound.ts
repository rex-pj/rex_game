/**
 * Sound Manager — qHortus Game Audio
 *
 * Uses Web Audio API (AudioBuffer) instead of HTMLAudioElement for:
 *   - Zero latency: plays directly from decoded buffer, no seek needed
 *   - No sound overlap: each play creates an independent BufferSourceNode
 *   - Simultaneous sounds: correct + levelComplete can play at the same time
 *
 * iOS Safari unlock strategy:
 *   1. Fetch raw ArrayBuffers immediately on init (no AudioContext needed)
 *   2. On first user gesture: create AudioContext, play a silent 1-sample buffer
 *      (required "unlock" step for iOS Safari), then decode all sounds
 *   3. playSound() works normally after unlock
 *
 *   Key design: _unlockPromise is a singleton — concurrent gesture events
 *   (touchstart + pointerdown on the same tap) share the same Promise,
 *   eliminating race conditions.
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
	flip: '/sounds/flashcard_flip.wav',
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

// Singleton Promise — prevents concurrent _unlock() executions (touchstart + pointerdown race)
let _unlockPromise: Promise<void> | null = null;

// Decoded AudioBuffers ready to play
const _buffers: Partial<Record<SoundName, AudioBuffer>> = {};
// Raw ArrayBuffers fetched before AudioContext exists
const _rawBuffers: Partial<Record<SoundName, ArrayBuffer>> = {};
// In-flight fetch promises — prevents duplicate fetches
const _fetchPromises: Partial<Record<SoundName, Promise<void>>> = {};

function _createContext(): AudioContext {
	return new (window.AudioContext ||
		(window as unknown as { webkitAudioContext: typeof AudioContext }).webkitAudioContext)();
}

function _fetchRaw(name: SoundName): Promise<void> {
	// Return existing promise if already fetching or fetched
	if (_fetchPromises[name]) return _fetchPromises[name]!;
	_fetchPromises[name] = (async () => {
		try {
			const res = await fetch(SOUND_FILES[name]);
			if (res.ok) {
				_rawBuffers[name] = await res.arrayBuffer();
			}
		} catch {
			// Network error or file missing — fail silently
		}
	})();
	return _fetchPromises[name]!;
}

function _decode(ctx: AudioContext, raw: ArrayBuffer): Promise<AudioBuffer> {
	// Use callback form for Safari < 14.1 compatibility.
	// Promise form of decodeAudioData returns undefined on older Safari,
	// so `await ctx.decodeAudioData(buf)` silently stores undefined in _buffers.
	return new Promise((resolve, reject) => ctx.decodeAudioData(raw.slice(0), resolve, reject));
}

async function _decodeAll(): Promise<void> {
	if (!_ctx) return;
	const ctx = _ctx;
	await Promise.all(
		(Object.keys(SOUND_FILES) as SoundName[]).map(async (name) => {
			const raw = _rawBuffers[name];
			if (!raw || _buffers[name]) return;
			try {
				_buffers[name] = await _decode(ctx, raw);
			} catch {
				// Unsupported format or corrupt file — fail silently
			}
		})
	);
}

async function _doUnlock(): Promise<void> {
	// _ctx is created early in initSound(), so it always exists here
	if (!_ctx) return;

	// resume() and source.start() must both be synchronous — no await before them.
	// iOS Safari only recognises the gesture in the synchronous call stack.
	if (_ctx.state === 'suspended' || _ctx.state === 'interrupted' as AudioContextState) {
		_ctx.resume(); // fire-and-forget — do NOT await here
	}

	// Play a silent 1-sample buffer — required unlock mechanism for older iOS Safari
	const silent = _ctx.createBuffer(1, 1, 22050);
	const source = _ctx.createBufferSource();
	source.buffer = silent;
	source.connect(_ctx.destination);
	// noteOn(0) fallback for very old Safari that predates source.start()
	if (typeof source.start === 'function') {
		source.start(0);
	} else {
		(source as unknown as { noteOn: (t: number) => void }).noteOn(0);
	}

	// Async work — gesture context no longer required from this point
	try {
		// Wait for all in-flight fetches to complete before decoding
		await Promise.all((Object.keys(SOUND_FILES) as SoundName[]).map(_fetchRaw));
		await _decodeAll();
	} catch {
		// Unlock failed — audio will not work on this device
	}
}

/** Returns the singleton unlock Promise. Safe to call concurrently. */
function _unlock(): Promise<void> {
	if (!_unlockPromise) {
		_unlockPromise = _doUnlock();
	}
	return _unlockPromise;
}

/** Load mute preference from localStorage and start prefetching audio files. Call in onMount. */
export function initSound(): boolean {
	if (!browser) return false;
	_muted = localStorage.getItem(STORAGE_KEY) === 'true';

	// Create AudioContext early (not lazily in the gesture handler).
	// iOS Safari is more reliable when the context already exists before the gesture fires.
	if (!_ctx) {
		_ctx = _createContext();
	}

	// Fetch raw audio data immediately — no AudioContext needed for this step
	(Object.keys(SOUND_FILES) as SoundName[]).forEach(_fetchRaw);

	// Use 'click' and 'touchend' instead of 'touchstart'/'pointerdown'.
	// 'touchstart' can fire during a scroll gesture and may not count as a valid
	// audio gesture on iOS Safari. 'click' and 'touchend' are more reliable.
	window.addEventListener('click', _unlock, { once: true });
	window.addEventListener('touchend', _unlock, { once: true, passive: true });

	return _muted;
}

function _playBuffer(ctx: AudioContext, buffer: AudioBuffer, volume: number): void {
	// Each play = independent source node → sounds never interrupt each other
	const source = ctx.createBufferSource();
	const gain = ctx.createGain();
	source.buffer = buffer;
	gain.gain.value = volume;
	source.connect(gain);
	gain.connect(ctx.destination);
	source.start(0);
}

/** Play a sound instantly from decoded buffer. Silently ignored if muted or not yet unlocked. */
export function playSound(name: SoundName): void {
	if (_muted || !browser || !_ctx) return;
	const buffer = _buffers[name];
	if (!buffer) return;

	const ctx = _ctx;
	const volume = SOUND_VOLUMES[name];

	// Check !== 'running' to also handle iOS Safari's 'interrupted' state
	// (triggered by phone call, Siri, tab losing focus) which resume() alone cannot fix
	if (ctx.state !== 'running') {
		ctx.resume().then(() => _playBuffer(ctx, buffer, volume));
	} else {
		_playBuffer(ctx, buffer, volume);
	}
}

/** Toggle mute, persisted to localStorage. Returns new muted state. */
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
