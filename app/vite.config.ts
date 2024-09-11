import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";

export default defineConfig({
	plugins: [ sveltekit() ],
	clearScreen: false,
	server: {
		port: 5000,
		strictPort: true 
	},
	// to access the Tauri environment variables set by the CLI with information about the current target
	envPrefix: [
		"VITE_",
		"TAURI_PLATFORM",
		"TAURI_ARCH",
		"TAURI_FAMILY",
		"TAURI_PLATFORM_VERSION",
		"TAURI_PLATFORM_TYPE",
		"TAURI_DEBUG"
	],
	build: {
		// Tauri uses Chromium on Windows and WebKit on macOS and Linux
		target: [ "es2021", "chrome100", "safari13" ],
		minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
		sourcemap: !!process.env.TAURI_DEBUG
	}
});
