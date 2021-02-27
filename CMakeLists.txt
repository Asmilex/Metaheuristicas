cmake_minimum_required(VERSION 2.8.9)

project(Metaheuristicas)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)

include_directories(include)

file(GLOB SOURCES "src/*.cpp")

add_executable(MH ${SOURCES})