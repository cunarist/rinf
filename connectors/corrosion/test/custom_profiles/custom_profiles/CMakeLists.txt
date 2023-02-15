cmake_minimum_required(VERSION 3.15)
project(test_project VERSION 0.1.0)
include(../../test_header.cmake)

set(_release_profile $<IF:$<CONFIG:Release>,release-without-dbg,custom-without-dbg>)
set(custom_profile $<IF:$<CONFIG:Debug>,dev-without-dbg,${_release_profile}>)

corrosion_import_crate(MANIFEST_PATH rust/Cargo.toml PROFILE ${custom_profile})

add_executable(custom-profile-exe main.cpp)
target_link_libraries(custom-profile-exe PUBLIC custom-profiles-lib)
