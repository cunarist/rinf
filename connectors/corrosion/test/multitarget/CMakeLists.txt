corrosion_tests_add_test(multitarget "bin1;bin2;bin3")

set_tests_properties("multitarget_run_bin1" PROPERTIES PASS_REGULAR_EXPRESSION
        "Hello, world!\r?\nHello, bin1! I'm Cpp!"
        )

set_tests_properties("multitarget_run_bin2" PROPERTIES PASS_REGULAR_EXPRESSION
        "Hello, world!\r?\nHello, bin2! I'm Cpp!"
        )

set_tests_properties("multitarget_run_bin3" PROPERTIES PASS_REGULAR_EXPRESSION
        "Hello, world!\r?\nHello, bin3! I'm Cpp!"
        )
