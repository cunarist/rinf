cmake_policy(SET CMP0079 NEW)

apply_standard_settings(${BINARY_NAME})

set_target_properties(${BINARY_NAME} PROPERTIES
  CXX_VISIBILITY_PRESET hidden)

target_compile_definitions(${BINARY_NAME} PRIVATE FLUTTER_PLUGIN_IMPL)

target_link_libraries(${BINARY_NAME} PRIVATE flutter flutter_wrapper_plugin)

include("../cargokit/cmake/cargokit.cmake")

get_filename_component(TARGET_PATH "${CMAKE_SOURCE_DIR}" DIRECTORY)
get_filename_component(TARGET_NAME "${TARGET_PATH}" NAME)

apply_cargokit(${BINARY_NAME} ../${TARGET_NAME}/native/hub hub hub_init)

target_link_libraries(${BINARY_NAME} PUBLIC hub)

set(
  PLUGIN_BUNDLED_LIBRARIES
  ${PLUGIN_BUNDLED_LIBRARIES} $<TARGET_FILE:rust_in_flutter>
  PARENT_SCOPE
)
