cmake_minimum_required(VERSION 3.22)
project(ffi VERSION 0.1.0 LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 11)

add_library(sample STATIC 
        cpp/data.h
        cpp/sample.cpp
        cpp/sample.h
        )


