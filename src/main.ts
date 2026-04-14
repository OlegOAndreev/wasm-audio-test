import { type MainModule as WasmCppModule } from '../wasm-cpp/build/wasm-cpp-audio';
import MainModuleFactory from '../wasm-cpp/build/wasm-cpp-audio';
import initWasmRustModule, {
    CPALState,
    init_cpal,
    init_tinyaudio,
    TinyaudioState,
} from '../wasm-rust/build/wasm_rust_audio';
import { getById, setupWindowOnError } from './common-utils';

let wasmCppModule: WasmCppModule;

const miniaudioStartButton = getById<HTMLButtonElement>('miniaudio-start-btn');
const miniaudioStopButton = getById<HTMLButtonElement>('miniaudio-stop-btn');
const sokolStartButton = getById<HTMLButtonElement>('sokol-start-btn');
const sokolStopButton = getById<HTMLButtonElement>('sokol-stop-btn');
const tinyaudioStartButton = getById<HTMLButtonElement>('tinyaudio-start-btn');
const tinyaudioStopButton = getById<HTMLButtonElement>('tinyaudio-stop-btn');
const cpalStartButton = getById<HTMLButtonElement>('cpal-start-btn');
const cpalStopButton = getById<HTMLButtonElement>('cpal-stop-btn');

function disableStartButtons() {
    miniaudioStartButton.disabled = true;
    sokolStartButton.disabled = true;
    tinyaudioStartButton.disabled = true;
    cpalStartButton.disabled = true;
}

function enableStartButtons() {
    miniaudioStartButton.disabled = false;
    sokolStartButton.disabled = false;
    tinyaudioStartButton.disabled = false;
    cpalStartButton.disabled = false;
}

async function testMiniaudio() {
    if (!wasmCppModule.start_miniaudio()) {
        throw new Error('Failed to initialize miniaudio');
    }
    miniaudioStartButton.style.display = 'none';
    miniaudioStopButton.style.display = 'block';
    disableStartButtons();
}

function stopMiniaudio() {
    wasmCppModule.stop_miniaudio();
    miniaudioStartButton.style.display = 'block';
    miniaudioStopButton.style.display = 'none';
    enableStartButtons();
}

async function testSokol() {
    if (!wasmCppModule.start_sokol_audio()) {
        throw new Error('Failed to initialize sokol audio');
    }
    sokolStartButton.style.display = 'none';
    sokolStopButton.style.display = 'block';
    disableStartButtons();
}

function stopSokol() {
    wasmCppModule.stop_sokol_audio();
    sokolStartButton.style.display = 'block';
    sokolStopButton.style.display = 'none';
    enableStartButtons();
}

let tinyaudioState: TinyaudioState | null = null;

async function testTinyaudio() {
    tinyaudioState = init_tinyaudio();
    tinyaudioStartButton.style.display = 'none';
    tinyaudioStopButton.style.display = 'block';
    disableStartButtons();
}

function stopTinyaudio() {
    tinyaudioState?.free();
    tinyaudioState = null;
    tinyaudioStartButton.style.display = 'block';
    tinyaudioStopButton.style.display = 'none';
    enableStartButtons();
}

let cpalState: CPALState | null = null;

async function testCPAL() {
    cpalState = init_cpal();
    cpalStartButton.style.display = 'none';
    cpalStopButton.style.display = 'block';
    disableStartButtons();
}

function stopCPAL() {
    cpalState?.free();
    cpalState = null;
    cpalStartButton.style.display = 'block';
    cpalStopButton.style.display = 'none';
    enableStartButtons();
}

async function init() {
    // Try initializing wasm before setting onerror as it breaks webviews for some reason.
    try {
        wasmCppModule = await MainModuleFactory();
    } catch (error) {
        alert(`WASM init failed: ${error}`);
        throw error;
    }
    await initWasmRustModule();

    setupWindowOnError();

    miniaudioStartButton.addEventListener('click', testMiniaudio);
    miniaudioStopButton.addEventListener('click', stopMiniaudio);
    sokolStartButton.addEventListener('click', testSokol);
    sokolStopButton.addEventListener('click', stopSokol);
    tinyaudioStartButton.addEventListener('click', testTinyaudio);
    tinyaudioStopButton.addEventListener('click', stopTinyaudio);
    cpalStartButton.addEventListener('click', testCPAL);
    cpalStopButton.addEventListener('click', stopCPAL);
}

init();
