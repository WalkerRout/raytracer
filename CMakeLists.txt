cmake_minimum_required(VERSION 3.5.1)

# project name
project(raytracer)

# creates the variable EXEC and sets it to hello
set(EXEC raytracer)

# set the C++20 standard
set(CMAKE_CXX_STANDARD 20)

# I../include
# L../lib
include_directories(include)
include_directories(src)

add_subdirectory(src)
add_subdirectory(test)
add_subdirectory(lib/googletest)
link_directories(lib)

#file(GLOB_RECURSE SOURCES LIST_DIRECTORIES true *.cpp)

#add_executable(${EXEC} ${SOURCES})