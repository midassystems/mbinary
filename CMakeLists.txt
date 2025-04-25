cmake_minimum_required(VERSION 3.14)
project(mbinary VERSION 1.0 LANGUAGES CXX)

# Use modern C++ standards
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# Call the bash script to build and deploy
execute_process(
    COMMAND bash ${CMAKE_CURRENT_SOURCE_DIR}/scripts/build_c.sh
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    RESULT_VARIABLE result
)

# Check for success or failure
if(NOT result EQUAL 0)
    message(FATAL_ERROR "Error occurred during the build and deployment process!")
else()
    message(STATUS "Build and deployment successful!")
endif()


add_library(mbinary STATIC IMPORTED GLOBAL)

set_target_properties(mbinary PROPERTIES
    INTERFACE_INCLUDE_DIRECTORIES "${CMAKE_CURRENT_SOURCE_DIR}/include"
#     IMPORTED_LOCATION "${CMAKE_CURRENT_SOURCE_DIR}/../mbinary-build/libmbinary.a"
)

