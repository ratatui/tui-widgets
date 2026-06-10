set shell := ["bash", "-uc"]

default:
    @just --list

fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all -- --check

clippy toolchain="stable":
    cargo +{{toolchain}} clippy --all-targets --all-features --workspace -- -D warnings

clippy-stable:
    just clippy stable

clippy-beta:
    just clippy beta

clippy-all: clippy-stable clippy-beta

test:
    cargo test --all-features --workspace

test-scrollbar:
    cargo test -p tui-scrollbar --all-features
