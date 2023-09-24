execute_process(
    COMMAND cmd /c dart run rust_in_flutter message 
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}/../
    RESULT_VARIABLE result  
    OUTPUT_VARIABLE output  
    ERROR_VARIABLE error_output 
)
if(result EQUAL 0)
    message("Generate protobuf messages successfully:${output}")
else()
    message(FATAL_ERROR "Generate protobuf messages failed with error code ${result}. Error output:${error_output}")
endif()