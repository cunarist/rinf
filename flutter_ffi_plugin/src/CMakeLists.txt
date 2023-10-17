# The Flutter tooling requires that developers have CMake 3.10 or later
# installed. You should not increase this version, as doing so will cause
# the plugin to fail to compile for some customers of the plugin.
cmake_minimum_required(VERSION 3.10)

project(rinf_library VERSION 0.0.1 LANGUAGES C)

add_library(rinf SHARED
  "rinf.c"
)

set_target_properties(rinf PROPERTIES
  PUBLIC_HEADER rinf.h
  OUTPUT_NAME "rinf"
)

target_compile_definitions(rinf PUBLIC DART_SHARED_LIB)
