cmake_minimum_required(VERSION 3.16)
project(system-programming-3)

set(CMAKE_CXX_STANDARD 17)

add_executable(system-programming-3 src/main.cpp src/check.hpp src/common.hpp src/find_value.hpp src/square_matrix.hpp)

find_package(Threads REQUIRED)
link_libraries(Threads::Threads)