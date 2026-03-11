/**
 * Sound Manager — qHortus Game Audio
 *
 * Uses Web Audio API (AudioBuffer) instead of HTMLAudioElement for:
 *   - Zero latency: plays directly from decoded buffer, no seek needed
 *   - No sound overlap: each play creates an independent BufferSourceNode
 *   - Simultaneous sounds: correct + levelComplete can play at the same time
 *
 * iOS Safari unlock strategy:
 *   1. Create AudioContext in initSound() (onMount) — early, not lazily
 *   2. Register gesture listeners on document with capture:true for all input events
 *   3. On first gesture: synchronously call resume() + source.start() (no await before either)
 *   4. Handle visibilitychange (tab hidden/shown) and onstatechange (iOS interrupted)
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

// Gesture events to listen for — registered with capture:true so they fire
// before any component handler, giving audio the earliest possible unlock.
const UNLOCK_EVENTS = ['touchstart', 'touchend', 'click', 'keydown'] as const;

let _muted = false;
let _ctx: AudioContext | null = null;

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
	return new Promise((resolve, reject) => {
		try {
			// slice(0) creates a copy — decodeAudioData detaches (transfers) the original
			const promise = ctx.decodeAudioData(raw.slice(0), resolve, reject);
			// Some Safari versions both call the callback AND return a Promise.
			// Catching the Promise prevents an Uncaught rejection in those versions.
			if (promise) promise.catch(reject);
		} catch (err) {
			reject(err);
		}
	});
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

function _unlockHandler(): void {
	if (!_ctx) return;

	// If already running from a previous event in this tap sequence, clean up and exit
	if (_ctx.state === 'running') {
		UNLOCK_EVENTS.forEach((e) => document.removeEventListener(e, _unlockHandler, true));
		return;
	}

	// resume() and source.start() MUST be synchronous — before any await.
	// iOS Safari only recognises the user gesture within the synchronous call stack.
	_ctx.resume().catch(() => {});

	// Silent 1-sample buffer — required unlock for older iOS Safari (< iOS 13)
	const silent = _ctx.createBuffer(1, 1, 22050);
	const source = _ctx.createBufferSource();
	source.buffer = silent;
	source.connect(_ctx.destination);
	if (typeof source.start === 'function') {
		source.start(0);
	} else {
		// noteOn() was the old API name in very early WebKit
		(source as unknown as { noteOn: (t: number) => void }).noteOn(0);
	}

	// Do NOT remove listeners immediately here.
	// iOS Safari often rejects resume() from 'touchstart' (may be a scroll gesture).
	// We leave 'touchend' and 'click' listeners intact as fallbacks.
	// Only clean up after confirming the context is truly running.
	setTimeout(() => {
		if (_ctx?.state === 'running') {
			UNLOCK_EVENTS.forEach((e) => document.removeEventListener(e, _unlockHandler, true));
		}
	}, 100);
}

function _resumeIfNeeded(): void {
	if (_ctx && _ctx.state !== 'running') {
		_ctx.resume().catch(() => {});
	}
}

/** Load mute preference from localStorage and start prefetching audio files. Call in onMount. */
export function initSound(): boolean {
	if (!browser) return false;
	_muted = localStorage.getItem(STORAGE_KEY) === 'true';

	// Create AudioContext early — iOS Safari is more reliable when the context
	// already exists before the gesture fires rather than being created inside it
	if (!_ctx) {
		_ctx = _createContext();

		// Handle iOS 'interrupted' state (phone call, Siri, app backgrounded).
		// When interrupted, resume() hangs indefinitely — reset unlock so next
		// user interaction re-triggers the full unlock sequence.
		_ctx.onstatechange = () => {
			if (_ctx?.state === ('interrupted' as AudioContextState)) {
				// Re-register gesture listeners so user can re-unlock by tapping
				UNLOCK_EVENTS.forEach((e) =>
					document.addEventListener(e, _unlockHandler, { capture: true })
				);
			}
		};
	}

	// Fetch and decode sounds immediately in the background.
	// AudioContext can decode even while suspended — no need to wait for a gesture.
	// This ensures _buffers are ready before the first playSound() call.
	Promise.all((Object.keys(SOUND_FILES) as SoundName[]).map(_fetchRaw))
		.then(() => _decodeAll())
		.catch(() => {});

	// Register without { once: true } — _unlockHandler manages its own removal
	// only after confirming ctx.state === 'running' (via setTimeout check).
	// This prevents premature removal if iOS rejects touchstart as a scroll gesture.
	UNLOCK_EVENTS.forEach((e) =>
		document.addEventListener(e, _unlockHandler, { capture: true })
	);

	// Resume when user returns to the tab after backgrounding the app
	document.addEventListener('visibilitychange', () => {
		if (!document.hidden) {
			_resumeIfNeeded();
		}
	});

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

	// !== 'running' catches both 'suspended' and iOS 'interrupted'
	if (ctx.state !== 'running') {
		ctx.resume().then(() => _playBuffer(ctx, buffer, volume)).catch(() => {});
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
