<script lang="ts">
	import { onMount } from 'svelte';

	let canvas: HTMLCanvasElement;

	// Brand-aligned palette + festive extras
	const COLORS = [
		'#f59e0b', // gold
		'#fbbf24', // gold light
		'#10b981', // green
		'#3b82f6', // blue
		'#ef4444', // red
		'#8b5cf6', // purple
		'#f97316', // orange
		'#ec4899', // pink
		'#06b6d4', // cyan
		'#ffffff' //  white
	];

	type Shape = 'circle' | 'rect' | 'star';

	interface Particle {
		x: number;
		y: number;
		vx: number;
		vy: number;
		color: string;
		size: number;
		rotation: number;
		rotationSpeed: number;
		opacity: number;
		shape: Shape;
	}

	function rand(min: number, max: number) {
		return min + Math.random() * (max - min);
	}

	function randomShape(): Shape {
		const r = Math.random();
		if (r < 0.4) return 'rect';
		if (r < 0.7) return 'circle';
		return 'star';
	}

	function burst(cx: number, cy: number, count: number): Particle[] {
		return Array.from({ length: count }, () => {
			const angle = Math.random() * Math.PI * 2;
			const speed = rand(4, 12);
			return {
				x: cx,
				y: cy,
				vx: Math.cos(angle) * speed,
				vy: Math.sin(angle) * speed - rand(3, 8),
				color: COLORS[Math.floor(Math.random() * COLORS.length)],
				size: rand(5, 12),
				rotation: Math.random() * Math.PI * 2,
				rotationSpeed: rand(-0.25, 0.25),
				opacity: 1,
				shape: randomShape()
			};
		});
	}

	function drawStar(ctx: CanvasRenderingContext2D, size: number) {
		ctx.beginPath();
		for (let i = 0; i < 10; i++) {
			const angle = (i * Math.PI) / 5 - Math.PI / 2;
			const r = i % 2 === 0 ? size : size * 0.45;
			if (i === 0) ctx.moveTo(Math.cos(angle) * r, Math.sin(angle) * r);
			else ctx.lineTo(Math.cos(angle) * r, Math.sin(angle) * r);
		}
		ctx.closePath();
		ctx.fill();
	}

	onMount(() => {
		const ctx = canvas.getContext('2d')!;

		const resize = () => {
			canvas.width = window.innerWidth;
			canvas.height = window.innerHeight;
		};
		resize();
		window.addEventListener('resize', resize);

		const W = canvas.width;
		const H = canvas.height;

		let particles: Particle[] = [];
		let animId: number;

		// Immediate bursts at 3 positions
		particles.push(...burst(W * 0.2, H * 0.35, 55));
		particles.push(...burst(W * 0.5, H * 0.25, 70));
		particles.push(...burst(W * 0.8, H * 0.35, 55));

		// Delayed second wave
		const t1 = setTimeout(() => {
			particles.push(...burst(W * 0.35, H * 0.2, 45));
			particles.push(...burst(W * 0.65, H * 0.2, 45));
		}, 450);

		// Third wave
		const t2 = setTimeout(() => {
			particles.push(...burst(W * 0.15, H * 0.45, 35));
			particles.push(...burst(W * 0.85, H * 0.45, 35));
			particles.push(...burst(W * 0.5, H * 0.15, 50));
		}, 900);

		function animate() {
			ctx.clearRect(0, 0, canvas.width, canvas.height);

			particles = particles.filter((p) => p.opacity > 0.02);

			for (const p of particles) {
				p.x += p.vx;
				p.y += p.vy;
				p.vy += 0.25; // gravity
				p.vx *= 0.98; // air resistance
				p.rotation += p.rotationSpeed;
				p.opacity -= 0.011;

				ctx.save();
				ctx.globalAlpha = Math.max(0, p.opacity);
				ctx.fillStyle = p.color;
				ctx.translate(p.x, p.y);
				ctx.rotate(p.rotation);

				if (p.shape === 'circle') {
					ctx.beginPath();
					ctx.arc(0, 0, p.size / 2, 0, Math.PI * 2);
					ctx.fill();
				} else if (p.shape === 'rect') {
					ctx.fillRect(-p.size / 2, -p.size / 4, p.size, p.size / 2);
				} else {
					drawStar(ctx, p.size / 2);
				}

				ctx.restore();
			}

			if (particles.length > 0) {
				animId = requestAnimationFrame(animate);
			}
		}

		animId = requestAnimationFrame(animate);

		return () => {
			cancelAnimationFrame(animId);
			clearTimeout(t1);
			clearTimeout(t2);
			window.removeEventListener('resize', resize);
		};
	});
</script>

<canvas bind:this={canvas}></canvas>

<style>
	canvas {
		position: fixed;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		pointer-events: none;
		z-index: 1000;
	}
</style>
