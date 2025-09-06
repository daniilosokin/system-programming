cmake_minimum_required(VERSION 3.16)
project(system-programming-4)

set(CMAKE_CXX_STANDARD 17)

include_directories(src/util)

add_executable(system-programming-4-server src/server/main.cpp)
add_executable(system-programming-4-client src/client/main.cpp)

install(TARGETS system-programming-4-client system-programming-4-server RUNTIME DESTINATION bin)