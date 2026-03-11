/**
 * Sound Manager — qHortus Game Audio
 *
 * Uses HTMLAudioElement pool instead of Web Audio API.
 *
 * Why not Web Audio API:
 *   iOS Safari's AudioContext unlock requirements are inconsistent across
 *   versions and cannot be reliably worked around in a SvelteKit SPA context.
 *   HTMLAudioElement works on all platforms with a simple user-gesture requirement.
 *
 * Pool pattern solves the two original HTMLAudioElement problems:
 *   - Latency: each element is pre-created and preloaded — no seek needed
 *   - Overlap: each pool slot is independent — sounds never cut each other off
 *
 * Throttle per sound prevents spam when tapping rapidly (e.g. flip sound).
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

// Minimum ms between plays of the same sound.
// Prevents audio spam when tapping rapidly — 0 means always play.
const SOUND_THROTTLE_MS: Record<SoundName, number> = {
	flip: 120, // max ~8 flips/sec — enough for fast tapping
	correct: 0,
	wrong: 0,
	levelComplete: 500,
	achievement: 500
};

const STORAGE_KEY = 'qhortus_sound_muted';

// Number of concurrent plays allowed per sound.
// 3 is enough for fast tapping without wasting memory.
const POOL_SIZE = 3;

let _muted = false;
let _initialized = false;

const _pools: Partial<Record<SoundName, HTMLAudioElement[]>> = {};

// Tracks which elements are mid-play to avoid race condition on rapid taps
const _busy = new WeakSet<HTMLAudioElement>();

// Timestamps for throttle — last play time per sound
const _lastPlayed: Partial<Record<SoundName, number>> = {};

/** Load mute preference from localStorage and pre-create audio element pools. Call in onMount. */
export function initSound(): boolean {
	if (!browser) return false;
	_muted = localStorage.getItem(STORAGE_KEY) === 'true';

	// Guard against re-initialization on SPA navigation (onMount fires on every page visit).
	// Pools are module-level singletons — creating them again leaks the old Audio elements.
	if (_initialized) return _muted;
	_initialized = true;

	for (const name of Object.keys(SOUND_FILES) as SoundName[]) {
		_pools[name] = Array.from({ length: POOL_SIZE }, () => {
			const audio = new Audio(SOUND_FILES[name]);
			audio.preload = 'auto';
			audio.volume = SOUND_VOLUMES[name];
			audio.load(); // best-effort — works on desktop/Android, iOS may ignore before gesture
			return audio;
		});
	}

	// iOS Safari ignores preload and load() before a user gesture.
	// Warm-up trick: on first gesture, play() then immediately pause() each element.
	// This forces iOS to buffer the audio data without actually playing anything audible.
	const warmUp = () => {
		for (const name of Object.keys(_pools) as SoundName[]) {
			_pools[name]?.forEach((a) => {
				a.volume = 0; // silence before play — no audible blip
				a.play()
					.then(() => {
						a.pause();
						a.currentTime = 0;
						a.volume = _muted ? 0 : SOUND_VOLUMES[name]; // restore
					})
					.catch(() => {});
			});
		}
	};
	document.addEventListener('touchstart', warmUp, { once: true, passive: true });
	document.addEventListener('click', warmUp, { once: true });

	return _muted;
}

/**
 * Play a sound from the pool. Throttled per sound to prevent audio spam.
 * Finds a free slot, marks it busy, plays, then releases it when done.
 */
export function playSound(name: SoundName): void {
	if (_muted || !browser) return;

	// Throttle — skip if called too soon after the last play of this sound
	const throttle = SOUND_THROTTLE_MS[name];
	if (throttle > 0) {
		const last = _lastPlayed[name] ?? 0;
		if (Date.now() - last < throttle) return;
	}
	_lastPlayed[name] = Date.now();

	const pool = _pools[name];
	if (!pool) return;

	// Prefer a slot that is free and not mid-play (race condition guard)
	let audio = pool.find((a) => !_busy.has(a) && (a.ended || a.paused));

	// All free slots are busy — reuse the one furthest along (least disruptive)
	if (!audio) {
		audio = pool.find((a) => !_busy.has(a));
	}

	// Absolute fallback: all slots in use, take the furthest along
	if (!audio) {
		audio = pool.reduce((prev, curr) => (curr.currentTime > prev.currentTime ? curr : prev));
	}

	_busy.add(audio);
	audio.currentTime = 0;
	audio
		.play()
		.catch(() => {
			// Autoplay blocked — requires a user gesture. Silently ignored.
		})
		.finally(() => {
			_busy.delete(audio!);
		});
}

/** Toggle mute, persisted to localStorage. Returns new muted state. */
export function toggleMute(): boolean {
	_muted = !_muted;
	if (browser) {
		localStorage.setItem(STORAGE_KEY, String(_muted));
	}

	// Sync all pool elements: pause + reset currently playing sounds when muting
	for (const name of Object.keys(_pools) as SoundName[]) {
		_pools[name]?.forEach((a) => {
			if (_muted) {
				a.pause();
				a.currentTime = 0;
			}
			a.volume = _muted ? 0 : SOUND_VOLUMES[name];
		});
	}

	return _muted;
}

export function isMuted(): boolean {
	return _muted;
}
