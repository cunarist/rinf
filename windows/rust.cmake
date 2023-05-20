# We include Corrosion inline here, but ideally in a project with
# many dependencies we would need to install Corrosion on the system.
# See instructions on https://github.com/AndrewGaspar/corrosion#cmake-install
# Once done, uncomment this line:
# find_package(Corrosion REQUIRED)

include(FetchContent)
FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/AndrewGaspar/corrosion.git
    GIT_TAG origin/stable/v0.3
)
FetchContent_MakeAvailable(Corrosion)

corrosion_import_crate(MANIFEST_PATH ../native/hub/Cargo.toml)
target_link_libraries(${BINARY_NAME} PRIVATE hub)
list(APPEND PLUGIN_BUNDLED_LIBRARIES $<TARGET_FILE:hub-shared>)
