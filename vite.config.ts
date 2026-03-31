import { sveltekit } from '@sveltejs/kit/vite';
import tailwindcss from '@tailwindcss/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [tailwindcss(), sveltekit()],

	// Tauri expects a fixed port during development
	server: {
		host: '0.0.0.0',
		port: 1420,
		strictPort: true,
		proxy: {
			// In browser/headless development, forward frontend /api calls to the Rust headless server.
			'/api': {
				target: 'http://127.0.0.1:4173',
				changeOrigin: true
			}
		}
	},

	// Env prefix for Tauri
	envPrefix: ['VITE_', 'TAURI_']
});
