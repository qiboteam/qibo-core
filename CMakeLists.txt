cmake_minimum_required(VERSION 3.22)
project(cpp_with_rust)

set(CMAKE_CXX_STANDARD 17)

set(CARGO_MANIFEST ${CMAKE_SOURCE_DIR}/Cargo.toml)
set(CARGO_TARGET_DIR ${CMAKE_SOURCE_DIR}/target)

set(BLOBSTORE_SOURCE_FILE ${CMAKE_SOURCE_DIR}/src/lib.rs)
set(BLOBSTORE_BRIDGE_CPP ${CARGO_TARGET_DIR}/cxxbridge/qibo-core/src/lib.rs.cc)
set(BLOBSTORE_LIB ${CARGO_TARGET_DIR}/debug/${CMAKE_STATIC_LIBRARY_PREFIX}qibo_core${CMAKE_STATIC_LIBRARY_SUFFIX})

# Add a custom command that builds the rust crate and generates C++ bridge code
add_custom_command(
        OUTPUT ${BLOBSTORE_BRIDGE_CPP} ${BLOBSTORE_LIB}
        COMMAND cargo build --manifest-path ${CARGO_MANIFEST}
        DEPENDS ${BLOBSTORE_SOURCE_FILE}
        USES_TERMINAL
        COMMENT "Running cargo..."
)

add_executable(cpp_with_rust src/main.cc ${BLOBSTORE_BRIDGE_CPP})

target_include_directories(
        cpp_with_rust
        PRIVATE
        ${CARGO_TARGET_DIR}/cxxbridge/qibo-core/src/
)

target_link_libraries(cpp_with_rust ${BLOBSTORE_LIB})

set_target_properties(
        cpp_with_rust
        PROPERTIES ADDITIONAL_CLEAN_FILES ${CARGO_TARGET_DIR}
)

