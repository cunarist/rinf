# The Flutter tooling requires that developers have CMake 3.10 or later
# installed. You should not increase this version, as doing so will cause
# the plugin to fail to compile for some customers of the plugin.
cmake_minimum_required(VERSION 3.10)

# Project-level configuration.
set(PROJECT_NAME "rust_in_flutter")
project(${PROJECT_NAME} LANGUAGES CXX)

# Invoke the build for native code shared with the other target platforms.
# This can be changed to accommodate different builds.
add_subdirectory("${CMAKE_CURRENT_SOURCE_DIR}/../src" "${CMAKE_CURRENT_BINARY_DIR}/shared")

# List of absolute paths to libraries that should be bundled with the plugin.
# This list could contain prebuilt libraries, or libraries created by an
# external build triggered from this build file.
set(rust_in_flutter_bundled_libraries
  # Defined in ../src/CMakeLists.txt.
  # This can be changed to accommodate different builds.
  $<TARGET_FILE:rust_in_flutter>
  PARENT_SCOPE
)

apply_standard_settings(${BINARY_NAME})

set_target_properties(${BINARY_NAME} PROPERTIES
  CXX_VISIBILITY_PRESET hidden
  BUILD_RPATH_USE_ORIGIN ON
)

target_compile_definitions(${BINARY_NAME} PRIVATE FLUTTER_PLUGIN_IMPL)

cmake_policy(SET CMP0079 NEW)

target_link_libraries(${BINARY_NAME} PRIVATE flutter PkgConfig::GTK)

include("../cargokit/cmake/cargokit.cmake")

get_filename_component(TARGET_PATH "${CMAKE_SOURCE_DIR}" DIRECTORY)
get_filename_component(TARGET_NAME "${TARGET_PATH}" NAME)

apply_cargokit(${BINARY_NAME} ../${TARGET_NAME}/native/hub hub "")

target_link_libraries(${BINARY_NAME} PUBLIC hub)

set(
  PLUGIN_BUNDLED_LIBRARIES
  ${PLUGIN_BUNDLED_LIBRARIES} $<TARGET_FILE:rust_in_flutter>
  PARENT_SCOPE
)
