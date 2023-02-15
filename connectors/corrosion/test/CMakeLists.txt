# This option is currently used to prevent recursion
option(CORROSION_TESTS "Enable Corrosion tests" ON)
mark_as_advanced(CORROSION_TESTS)
if(NOT CORROSION_TESTS)
    return()
endif()

set(test_install_path "${CMAKE_CURRENT_BINARY_DIR}/test-install-corrosion")

set(test_header_contents
        "option(CORROSION_TESTS_FIND_CORROSION \"Use Corrosion as a subdirectory\" OFF)"
        "if (CORROSION_TESTS_FIND_CORROSION)"
        "    set(CMAKE_PREFIX_PATH \"${test_install_path}\" CACHE INTERNAL \"\" FORCE)"
        "    find_package(Corrosion REQUIRED PATHS \"${test_install_path}\" NO_CMAKE_SYSTEM_PATH)"
        "else()"
        "    add_subdirectory(\"${CMAKE_CURRENT_SOURCE_DIR}/..\" corrosion)"
        "endif()"
)

string(REPLACE ";" "\n" test_header_contents "${test_header_contents}")

file(WRITE test_header.cmake "${test_header_contents}")

option(CORROSION_TESTS_INSTALL_CORROSION
        "Install Corrosion to a test directory and let tests use the installed Corrosion"
        OFF)
if(CORROSION_TESTS_INSTALL_CORROSION)
    add_test(NAME "install_corrosion_configure"
        COMMAND
            ${CMAKE_COMMAND}
            -S "${CMAKE_CURRENT_SOURCE_DIR}/.."
            -B "${CMAKE_CURRENT_BINARY_DIR}/build-corrosion"
            -DCORROSION_VERBOSE_OUTPUT=ON
            -DCORROSION_TESTS=OFF
            -DCMAKE_BUILD_TYPE=Release
            "-DCMAKE_INSTALL_PREFIX=${test_install_path}"
    )
    add_test(NAME "install_corrosion_build"
            COMMAND
            ${CMAKE_COMMAND} --build "${CMAKE_CURRENT_BINARY_DIR}/build-corrosion" --config Release
            )
    add_test(NAME "install_corrosion_install"
            COMMAND
            ${CMAKE_COMMAND} --install "${CMAKE_CURRENT_BINARY_DIR}/build-corrosion" --config Release
        )
    set_tests_properties("install_corrosion_configure" PROPERTIES FIXTURES_SETUP "fixture_corrosion_configure")
    set_tests_properties("install_corrosion_build"     PROPERTIES FIXTURES_SETUP "fixture_corrosion_build")
    set_tests_properties("install_corrosion_build"     PROPERTIES FIXTURES_REQUIRED "fixture_corrosion_configure")
    set_tests_properties("install_corrosion_install"   PROPERTIES FIXTURES_REQUIRED "install_corrosion_build")
    set_tests_properties("install_corrosion_install"   PROPERTIES FIXTURES_SETUP "fixture_corrosion_install")
endif()

function(corrosion_tests_add_test test_name bin_names)
# In the future we could add multiple tests here for different configurations (generator, build mode, rust version ...)
# which would allow us to simplify the github job matrix

    if(CMAKE_C_COMPILER)
        set(TEST_C_COMPILER "C_COMPILER" "${CMAKE_C_COMPILER}")
    endif()
    if(CMAKE_CXX_COMPILER)
        set(TEST_CXX_COMPILER "CXX_COMPILER" "${CMAKE_CXX_COMPILER}")
    endif()
    if(CMAKE_GENERATOR_PLATFORM)
        set(TEST_GENERATOR_PLATFORM "GENERATOR_PLATFORM" "${CMAKE_GENERATOR_PLATFORM}")
    endif()

    add_test(NAME "${test_name}_build"
            COMMAND
            ${CMAKE_COMMAND}
            -P "${CMAKE_SOURCE_DIR}/test/ConfigureAndBuild.cmake"
            SOURCE_DIR "${CMAKE_CURRENT_LIST_DIR}/${test_name}"
            BINARY_DIR "${CMAKE_CURRENT_BINARY_DIR}/build"
            GENERATOR "${CMAKE_GENERATOR}"
            RUST_TOOLCHAIN "${Rust_TOOLCHAIN}"
            CARGO_TARGET "${Rust_CARGO_TARGET}"
            SYSTEM_NAME "${CMAKE_SYSTEM_NAME}"
            "${TEST_C_COMPILER}"
            "${TEST_CXX_COMPILER}"
            "${TEST_GENERATOR_PLATFORM}"

            COMMAND_EXPAND_LISTS
            )
    set_tests_properties("${test_name}_build" PROPERTIES FIXTURES_SETUP "build_fixture_${test_name}")
    if(CORROSION_TESTS_INSTALL_CORROSION)
        set_tests_properties("${test_name}_build" PROPERTIES FIXTURES_REQUIRED "fixture_corrosion_install")
    endif()
    foreach(bin ${bin_names})
        if(WIN32)
            set(bin_filename "${bin}.exe")
        else()
            set(bin_filename "${bin}")
        endif()
        add_test(NAME "${test_name}_run_${bin}" COMMAND "${CMAKE_CURRENT_BINARY_DIR}/build/${bin_filename}")
        set_tests_properties("${test_name}_run_${bin}" PROPERTIES FIXTURES_REQUIRED "build_fixture_${test_name}")
        # CMAKE_CROSSCOMPILING is not set when cross-compiling with VS (via -A flag).
        # Todo: We could run x86 binaries on x64 hosts.
        if(CMAKE_CROSSCOMPILING OR CMAKE_VS_PLATFORM_NAME)
            # Todo: In the future we could potentially run some tests with qemu.
            set_tests_properties("${test_name}_run_${bin}" PROPERTIES DISABLED TRUE)
        endif()
    endforeach()
endfunction()

# Please keep this in alphabetical order.
add_subdirectory(cargo_flags)
add_subdirectory(cpp2rust)
if(Rust_VERSION VERSION_GREATER_EQUAL "1.64.0" AND CMAKE_VERSION VERSION_GREATER_EQUAL "3.19.0")
    # Flag `--crate-type` is only supported since Rust 1.64.0
    add_subdirectory(crate_type)
endif()
add_subdirectory(custom_profiles)
add_subdirectory(envvar)
add_subdirectory(external_corrosion_generator)
add_subdirectory(features)
add_subdirectory(find_rust)
add_subdirectory(gensource)
add_subdirectory(hostbuild)
add_subdirectory(multitarget)
add_subdirectory(nostd)
add_subdirectory("output directory")
add_subdirectory(parse_target_triple)
add_subdirectory(rust2cpp)
add_subdirectory(rustflags)
add_subdirectory(workspace)
