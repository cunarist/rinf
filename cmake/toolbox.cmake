SET(toolbox_cmake_root "${CMAKE_CURRENT_LIST_DIR}/..")

function(apply_toolbox target manifest_dir lib_name)

    set(TOOLBOX_LIB_NAME "${lib_name}")
    set(TOOLBOX_LIB_FULL_NAME "${CMAKE_SHARED_MODULE_PREFIX}${TOOLBOX_LIB_NAME}${CMAKE_SHARED_MODULE_SUFFIX}")
    if (CMAKE_CONFIGURATION_TYPES)
        set(TOOLBOX_TARGET_DIR "${CMAKE_CURRENT_BINARY_DIR}/$<CONFIG>")
        set(OUTPUT_LIB "${CMAKE_CURRENT_BINARY_DIR}/$<CONFIG>/${TOOLBOX_LIB_FULL_NAME}")
    else()
        set(TOOLBOX_TARGET_DIR "${CMAKE_CURRENT_BINARY_DIR}")
        set(OUTPUT_LIB "${CMAKE_CURRENT_BINARY_DIR}/${TOOLBOX_LIB_FULL_NAME}")
    endif()
    set(TOOLBOX_TEMP_DIR "${CMAKE_CURRENT_BINARY_DIR}/toolbox_build")

    if (FLUTTER_TARGET_PLATFORM)
        set(TOOLBOX_TARGET_PLATFORM "${FLUTTER_TARGET_PLATFORM}")
    else()
        set(TOOLBOX_TARGET_PLATFORM "windows-x64")
    endif()

    set(TOOLBOX_ENV
        "TOOLBOX_CMAKE=${CMAKE_COMMAND}"
        "TOOLBOX_CONFIGURATION=$<CONFIG>"
        "TOOLBOX_MANIFEST_DIR=${CMAKE_CURRENT_SOURCE_DIR}/${manifest_dir}"
        "TOOLBOX_LIB_NAME=${TOOLBOX_LIB_NAME}"
        "TOOLBOX_BUILD_DIR=${TOOLBOX_TEMP_DIR}"
        "TOOLBOX_TARGET_DIR=${TOOLBOX_TARGET_DIR}"
        "TOOLBOX_TARGET_PLATFORM=${TOOLBOX_TARGET_PLATFORM}"
        "TOOLBOX_TOOL_TEMP_DIR=${TOOLBOX_TEMP_DIR}/tool"
    )

    if (WIN32)
        set(SCRIPT_EXTENSION ".cmd")
        set(IMPORT_LIB_EXTENSION ".lib")
    else()
        set(SCRIPT_EXTENSION ".sh")
        set(IMPORT_LIB_EXTENSION "")
    endif()

    # Using generators in custom command is only supported in CMake 3.20+
    if (CMAKE_CONFIGURATION_TYPES AND ${CMAKE_VERSION} VERSION_LESS "3.20.0")
        foreach(CONFIG IN LISTS CMAKE_CONFIGURATION_TYPES)
            add_custom_command(
                OUTPUT
                "${CMAKE_CURRENT_BINARY_DIR}/${CONFIG}/${TOOLBOX_LIB_FULL_NAME}"
                "${CMAKE_CURRENT_BINARY_DIR}/_phony_"
                COMMAND ${CMAKE_COMMAND} -E env ${TOOLBOX_ENV}
                "${toolbox_cmake_root}/run_rust_tool${SCRIPT_EXTENSION}" build_cmake
                VERBATIM
            )
        endforeach()
    else()
        add_custom_command(
            OUTPUT
            ${OUTPUT_LIB}
            "${CMAKE_CURRENT_BINARY_DIR}/_phony_"
            COMMAND ${CMAKE_COMMAND} -E env ${TOOLBOX_ENV}
            "${toolbox_cmake_root}/run_rust_tool${SCRIPT_EXTENSION}" build_cmake
            VERBATIM
        )
    endif()

    set_source_files_properties("${CMAKE_CURRENT_BINARY_DIR}/_phony_" PROPERTIES SYMBOLIC TRUE)

    add_custom_target("${target}_toolbox" DEPENDS ${OUTPUT_LIB})

    add_dependencies("${target}" "${target}_toolbox")

    target_link_libraries("${target}" PRIVATE "${OUTPUT_LIB}${IMPORT_LIB_EXTENSION}")

    # Allow adding the output library to plugin bundled libraries
    set("${target}_toolbox_lib" ${OUTPUT_LIB} PARENT_SCOPE)

endfunction()