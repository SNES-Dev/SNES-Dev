cmake_minimum_required(VERSION 3.15)
project(snesdev_rust)

if(DEFINED SNESDEV_INTERNAL_BUILD_RUSTAPI)
import(ExternalProject)
ExternalProject_add(rust-api)
endif()