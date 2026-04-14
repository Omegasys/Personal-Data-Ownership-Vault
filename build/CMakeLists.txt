cmake_minimum_required(VERSION 3.16)
project(data_vault NONE)

# This is a Rust project wrapper (for integration with other systems)

add_custom_target(build_vault ALL
    COMMAND cargo build --manifest-path ${CMAKE_SOURCE_DIR}/Cargo.toml
)

add_custom_target(run_vault
    COMMAND cargo run --manifest-path ${CMAKE_SOURCE_DIR}/Cargo.toml
)
