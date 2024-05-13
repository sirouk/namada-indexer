build:
    cargo build

check:
    cargo check

fmt:
    cargo +nightly fmt

clippy:
    cargo clippy

clippy-fix:
    cargo clippy --fix --allow-dirty --allow-staged