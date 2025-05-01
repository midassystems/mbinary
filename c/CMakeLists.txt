cmake_minimum_required(VERSION 3.14)
project(mbinary VERSION 1.0 LANGUAGES CXX)

# Use modern C++ standards
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED True)
set(CMAKE_OSX_DEPLOYMENT_TARGET "14.2")


enable_testing()
find_package(GTest CONFIG REQUIRED)

# Path to the cbindgen-generated header
set(CBINDGEN_HEADER_DIR "${CMAKE_SOURCE_DIR}/../target")

# Rust binary
set(RUST_PROJECT_DIR "${CMAKE_SOURCE_DIR}/..")
set(RUST_LIB_DIR "${RUST_PROJECT_DIR}/target/debug") 
set(RUST_LIB "${RUST_LIB_DIR}/libmbinary.a")

# Test executable
add_executable(MbinaryTests 
    tests/test_decode_c.cpp 
    tests/test_encode_c.cpp
    tests/test_records.cpp
)

# Include the cbindgen header directory
target_include_directories(MbinaryTests PRIVATE 
    "${CBINDGEN_HEADER_DIR}" # Include the directory containing mbinary.h
    "${RUST_LIB_DIR}"
)

# Link the Rust library
target_link_libraries(MbinaryTests PRIVATE "${RUST_LIB}")

# Link Google Test to the tests
target_link_libraries(MbinaryTests PRIVATE 
    GTest::gtest 
    GTest::gtest_main 
)

# Add tests to CTest
add_test(NAME MbinaryTests COMMAND MbinaryTests)

# Link macOS frameworks (CoreFoundation and Security)
if (APPLE)
    target_link_libraries(MbinaryTests PRIVATE 
        "-framework CoreFoundation"
        "-framework Security"
        "-framework SystemConfiguration"
    )
endif()
