add_subdirectory(../connectors/corrosion ${CMAKE_CURRENT_BINARY_DIR}/corrosion)

cmake_policy(SET CMP0079 NEW)
corrosion_import_crate(
  MANIFEST_PATH
  ${CMAKE_SOURCE_DIR}/../native/hub/Cargo.toml
)
target_link_libraries(${BINARY_NAME} PUBLIC hub)

set(
  PLUGIN_BUNDLED_LIBRARIES
  ${PLUGIN_BUNDLED_LIBRARIES} $<TARGET_FILE:hub-shared>
  PARENT_SCOPE
)
