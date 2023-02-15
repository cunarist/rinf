add_subdirectory(../connectors/corrosion ${CMAKE_CURRENT_BINARY_DIR}/corrosion)

set(BUNDLED_CRATES)
set(INSTALL_BUNDLE_LIB_DIR
  "${CMAKE_INSTALL_PREFIX}/lib"
) # Copied from default Flutter template

cmake_policy(SET CMP0079 NEW)
corrosion_import_crate(MANIFEST_PATH
  # Using an environment variable from Flutter
  ${runner_SOURCE_DIR}/../native/hub/Cargo.toml
)
target_link_libraries(${BINARY_NAME} PUBLIC hub)
list(APPEND BUNDLED_CRATES $<TARGET_FILE:hub-shared>)

foreach(bundled_crate ${BUNDLED_CRATES})
  install(FILES ${bundled_crate}
    DESTINATION ${INSTALL_BUNDLE_LIB_DIR} COMPONENT Runtime
  )
endforeach(bundled_crate)