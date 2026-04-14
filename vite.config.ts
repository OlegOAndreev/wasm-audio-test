import type { UserConfig } from 'vite';

export default {
    // For github pages
    base: '/wasm-audio-test/',

    worker: {
        format: 'es',
    },

    build: {
        chunkSizeWarningLimit: 1500,
    },

    plugins: [],
} satisfies UserConfig;
