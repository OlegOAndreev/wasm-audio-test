cmake_minimum_required(VERSION 3.16)
project(wasm-cpp-audio LANGUAGES C CXX)

# Set C++ standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

if(NOT DEFINED EMSCRIPTEN_MALLOC)
    set(EMSCRIPTEN_MALLOC dlmalloc)
endif()

if(CMAKE_BUILD_TYPE STREQUAL "Debug" AND NOT EMSCRIPTEN_MALLOC STREQUAL "mimalloc")
    add_compile_options(-fsanitize=address)
    add_link_options(-fsanitize=address)
endif()

if(EMSCRIPTEN)
    add_executable(wasm-cpp-audio
        src/main.cpp
    )

    target_compile_options(wasm-cpp-audio PRIVATE
        -Wall
        -fstack-protector
        -ftrivial-auto-var-init=zero
        -D_LIBCPP_HARDENING_MODE=_LIBCPP_HARDENING_MODE_FAST
    )

    target_include_directories(wasm-cpp-audio PRIVATE
        vendor/miniaudio
        vendor/sokol
    )

    target_link_options(wasm-cpp-audio PRIVATE
        -lembind
        -sSTRICT
        -sRETAIN_COMPILER_SETTINGS
        -sMALLOC=${EMSCRIPTEN_MALLOC}
        -sALLOW_MEMORY_GROWTH
        -sMODULARIZE
        -sENVIRONMENT=[web,webview,worker]
        -sEXPORT_ES6
        -sEXPORT_NAME='createWasmCppModule'
        -sEXPORTED_FUNCTIONS=[_malloc,_free]
        -sEXPORTED_RUNTIME_METHODS=[HEAPU32,HEAPF32]
        --emit-tsd wasm-cpp-audio.d.ts
        --emit-symbol-map
    )
else()
    message(FATAL_ERROR "This project must be built with Emscripten. Please use 'emcmake cmake' instead of 'cmake'.")
endif()
