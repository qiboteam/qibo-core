cmake_minimum_required(VERSION 3.22)
project(qibo-core)

set(CMAKE_CXX_STANDARD 17)

set(CARGO_MANIFEST ${CMAKE_SOURCE_DIR}/Cargo.toml)
set(CARGO_TARGET_DIR ${CMAKE_SOURCE_DIR}/../target)

set(QIBO_CORE_SOURCE_FILE ${CMAKE_SOURCE_DIR}/src/lib.rs)
set(QIBO_CORE_BRIDGE_CPP ${CARGO_TARGET_DIR}/cxxbridge/qibo-core-cxx/src/lib.rs.cc)
set(QIBO_CORE_LIB ${CARGO_TARGET_DIR}/debug/${CMAKE_STATIC_LIBRARY_PREFIX}qibo_core${CMAKE_STATIC_LIBRARY_SUFFIX})

message(${QIBO_CORE_LIB})

# Add a custom command that builds the rust crate and generates C++ bridge code
add_custom_command(
        OUTPUT ${QIBO_CORE_BRIDGE_CPP} ${QIBO_CORE_LIB}
        COMMAND cargo build --manifest-path ${CARGO_MANIFEST}
        DEPENDS ${QIBO_CORE_SOURCE_FILE}
        USES_TERMINAL
        COMMENT "Running cargo..."
)


# Circuit example
add_executable(circuit examples/main.cc ${QIBO_CORE_BRIDGE_CPP})

target_include_directories(
        circuit
        PRIVATE
        ${CARGO_TARGET_DIR}/cxxbridge/qibo-core-cxx/src/
)

target_link_libraries(circuit ${QIBO_CORE_LIB})

set_target_properties(
        circuit
        PROPERTIES ADDITIONAL_CLEAN_FILES ${CARGO_TARGET_DIR}
)
