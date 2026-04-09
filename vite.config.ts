import { sveltekit } from '@sveltejs/kit/vite';
import tailwindcss from '@tailwindcss/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [tailwindcss(), sveltekit()],

	// Keep a fixed dev port for local frontend/backend integration.
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

	// Only expose Vite-prefixed environment variables to the frontend bundle.
	envPrefix: ['VITE_']
});
