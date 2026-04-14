#include <emscripten.h>
#include <emscripten/bind.h>

#include <cmath>
#include <cstring>

// Miniaudio configuration: disable decoding and encoding to reduce size
#define MA_NO_DECODING
#define MA_NO_ENCODING
#define MINIAUDIO_IMPLEMENTATION
#include "../vendor/miniaudio/miniaudio.h"

#define SOKOL_AUDIO_IMPL
#include "../vendor/sokol/sokol_audio.h"

constexpr float PI = 3.14159265358979323846f;

const int REQUESTED_SAMPLE_RATE = 48000;
const int NUM_CHANNELS = 2;

const float AMPLITUDE = 0.2;
const float FREQ_1 = 392.0;
const float FREQ_2 = 490.0;
const float FREQ_3 = 588.0;

struct MiniaudioState {
    ma_device device;
    bool initialized = false;

    float phase1 = 0.0;
    float phase2 = 0.0;
    float phase3 = 0.0;
};

struct SokolAudioState {
    bool initialized = false;
    int sample_rate = 0;

    float phase1 = 0.0;
    float phase2 = 0.0;
    float phase3 = 0.0;
};

static MiniaudioState g_miniaudio_state;
static SokolAudioState g_sokol_state;

// Miniaudio functions
void miniaudio_data_callback(ma_device* pDevice, void* pOutput, const void* pInput, ma_uint32 frameCount) {
    (void)pInput;
    float* output = (float*)pOutput;

    for (ma_uint32 i = 0; i < frameCount; i++) {
        float sample1 = sinf(g_miniaudio_state.phase1) * AMPLITUDE;
        float sample2 = sinf(g_miniaudio_state.phase2) * AMPLITUDE;
        float sample3 = sinf(g_miniaudio_state.phase3) * AMPLITUDE;
        float mixed_sample = sample1 + sample2 + sample3;
        for (int ch = 0; ch < NUM_CHANNELS; ch++) {
            output[i * NUM_CHANNELS + ch] = mixed_sample;
        }

        g_miniaudio_state.phase1 += 2.0f * PI * FREQ_1 / g_miniaudio_state.device.sampleRate;
        g_miniaudio_state.phase2 += 2.0f * PI * FREQ_2 / g_miniaudio_state.device.sampleRate;
        g_miniaudio_state.phase3 += 2.0f * PI * FREQ_3 / g_miniaudio_state.device.sampleRate;
        if (g_miniaudio_state.phase1 >= 2.0f * PI) {
            g_miniaudio_state.phase1 -= 2.0f * PI;
        }
        if (g_miniaudio_state.phase2 >= 2.0f * PI) {
            g_miniaudio_state.phase2 -= 2.0f * PI;
        }
        if (g_miniaudio_state.phase3 >= 2.0f * PI) {
            g_miniaudio_state.phase3 -= 2.0f * PI;
        }
    }
}

bool start_miniaudio() {
    if (g_miniaudio_state.initialized) {
        ma_device_uninit(&g_miniaudio_state.device);
        g_miniaudio_state.initialized = false;
    }

    ma_device_config device_config = ma_device_config_init(ma_device_type_playback);
    device_config.playback.format = ma_format_f32;
    device_config.playback.channels = NUM_CHANNELS;
    device_config.sampleRate = REQUESTED_SAMPLE_RATE;
    device_config.dataCallback = miniaudio_data_callback;
    device_config.pUserData = nullptr;
    ma_result result = ma_device_init(NULL, &device_config, &g_miniaudio_state.device);
    if (result != MA_SUCCESS) {
        return false;
    }
    printf("Got sample rate for miniaudio: %d\n", g_miniaudio_state.device.sampleRate);

    result = ma_device_start(&g_miniaudio_state.device);
    g_miniaudio_state.initialized = result == MA_SUCCESS;
    return g_miniaudio_state.initialized;
}

void stop_miniaudio() {
    if (g_miniaudio_state.initialized) {
        ma_device_uninit(&g_miniaudio_state.device);
        g_miniaudio_state.initialized = false;
    }
}

// Sokol audio functions
void sokol_audio_callback(float* buffer, int num_frames, int num_channels) {
    for (int i = 0; i < num_frames; i++) {
        float sample1 = sinf(g_sokol_state.phase1) * AMPLITUDE;
        float sample2 = sinf(g_sokol_state.phase2) * AMPLITUDE;
        float sample3 = sinf(g_sokol_state.phase3) * AMPLITUDE;
        float mixed_sample = sample1 + sample2 + sample3;
        for (int ch = 0; ch < num_channels; ch++) {
            buffer[i * num_channels + ch] = mixed_sample;
        }

        g_sokol_state.phase1 += 2.0f * PI * FREQ_1 / g_sokol_state.sample_rate;
        g_sokol_state.phase2 += 2.0f * PI * FREQ_2 / g_sokol_state.sample_rate;
        g_sokol_state.phase3 += 2.0f * PI * FREQ_3 / g_sokol_state.sample_rate;
        if (g_sokol_state.phase1 >= 2.0f * PI) {
            g_sokol_state.phase1 -= 2.0f * PI;
        }
        if (g_sokol_state.phase2 >= 2.0f * PI) {
            g_sokol_state.phase2 -= 2.0f * PI;
        }
        if (g_sokol_state.phase3 >= 2.0f * PI) {
            g_sokol_state.phase3 -= 2.0f * PI;
        }
    }
}

bool start_sokol_audio() {
    if (g_sokol_state.initialized) {
        saudio_shutdown();
        g_sokol_state.initialized = false;
    }

    saudio_desc desc = {};
    desc.sample_rate = REQUESTED_SAMPLE_RATE;
    desc.num_channels = NUM_CHANNELS;
    desc.stream_cb = sokol_audio_callback;
    saudio_setup(&desc);

    g_sokol_state.initialized = saudio_isvalid();
    g_sokol_state.sample_rate = saudio_sample_rate();
    printf("Got sample rate for sokol: %d\n", g_sokol_state.sample_rate);

    return g_sokol_state.initialized;
}

void stop_sokol_audio() {
    if (g_sokol_state.initialized) {
        saudio_shutdown();
        g_sokol_state.initialized = false;
    }
}

EMSCRIPTEN_BINDINGS(memory_view_example) {
    emscripten::function("start_miniaudio", &start_miniaudio);
    emscripten::function("stop_miniaudio", &stop_miniaudio);

    emscripten::function("start_sokol_audio", &start_sokol_audio);
    emscripten::function("stop_sokol_audio", &stop_sokol_audio);
}
