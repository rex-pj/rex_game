/**
 * Sound Manager — qHortus Game Audio
 *
 * Usage:
 *   import { playSound, toggleMute, initSound, isMuted } from '$lib/utils/sound';
 *
 *   onMount(() => initSound());          // load mute preference
 *   playSound('correct');                // play a sound
 *   const muted = toggleMute();          // toggle & return new state
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
const _cache: Partial<Record<SoundName, HTMLAudioElement>> = {};

function _getAudio(name: SoundName): HTMLAudioElement | null {
	if (!browser) return null;
	if (!_cache[name]) {
		const audio = new Audio(SOUND_FILES[name]);
		audio.volume = SOUND_VOLUMES[name];
		audio.preload = 'auto';
		_cache[name] = audio;
	}
	return _cache[name]!;
}

/** Preload all sounds to avoid first-play latency */
export function preloadSounds(): void {
	if (!browser) return;
	(Object.keys(SOUND_FILES) as SoundName[]).forEach(_getAudio);
}

/** Load mute preference from localStorage. Call in onMount. */
export function initSound(): boolean {
	if (!browser) return false;
	_muted = localStorage.getItem(STORAGE_KEY) === 'true';
	preloadSounds();
	return _muted;
}

/** Play a sound. Silently ignored if muted or in SSR. */
export function playSound(name: SoundName): void {
	if (_muted || !browser) return;
	const audio = _getAudio(name);
	if (!audio) return;
	// Reset so rapid repeats work (e.g. clicking fast in speed match)
	audio.currentTime = 0;
	audio.play().catch(() => {
		// Autoplay policy — browser blocks play before user interaction.
		// Fail silently; sound will work after first user gesture.
	});
}

/** Toggle mute, persist to localStorage. Returns new muted state. */
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
