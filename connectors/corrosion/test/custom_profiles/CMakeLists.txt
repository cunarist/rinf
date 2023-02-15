corrosion_tests_add_test(custom_profiles "custom-profile-exe")

set_tests_properties("custom_profiles_run_custom-profile-exe" PROPERTIES PASS_REGULAR_EXPRESSION
        "^Hello, Cpp! I'm Rust!\r?\n$"
        )
if(Rust_VERSION VERSION_LESS 1.57.0)
        set_tests_properties(custom_profiles_build custom_profiles_run_custom-profile-exe PROPERTIES
                DISABLED TRUE
        )
endif()
