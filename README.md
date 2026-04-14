# WASM Audio Test

This is a very basic test of a couple of C++ (sokol_audio, miniaudio) and Rust (CPAL, tinyaudio) audio libraries. Plays
a few sine tones.

## Development

### Preparing environment
Install node.js and run to install all the required libraries and scripts.
```bash
npm install
./wasm/install-build-deps.sh
```

### Building
```bash
npm run build
```

Builds in the `dist/` directory.

### Running dev server
```
npx vite
```

Starts the local dev server with URL http://localhost:5173/wasm-audio-test/
