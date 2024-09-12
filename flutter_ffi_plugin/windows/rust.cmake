cmake_policy(SET CMP0079 NEW)
apply_standard_settings(${BINARY_NAME})

set_target_properties(
  ${BINARY_NAME} PROPERTIES
  CXX_VISIBILITY_PRESET hidden
  BUILD_RPATH_USE_ORIGIN ON
)

target_compile_definitions(${BINARY_NAME} PRIVATE FLUTTER_PLUGIN_IMPL)
target_link_libraries(${BINARY_NAME} PRIVATE flutter)
