cmake_minimum_required (VERSION 3.10)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

set(CMAKE_C_COMPILER clang)
set(CMAKE_C_STANDARD 11)

project(stb LANGUAGES C)

add_library(stb STATIC
    stb.c
)

target_include_directories(stb PUBLIC
    stb
)