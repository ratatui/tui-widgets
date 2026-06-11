set shell := ["bash", "-uc"]

default:
    @just --list

fmt:
    # rustfmt.toml uses unstable rustfmt options for comment wrapping and import grouping.
    cargo +nightly fmt --all

fmt-check:
    # Keep check mode on the same toolchain as fmt so CI and local formatting agree.
    cargo +nightly fmt --all -- --check

clippy toolchain="stable":
    cargo +{{toolchain}} clippy --all-targets --all-features --workspace -- -D warnings

clippy-stable:
    just clippy stable

clippy-beta:
    just clippy beta

clippy-all: clippy-stable clippy-beta

test:
    cargo test --all-features --workspace

semver-checks:
    cargo semver-checks --workspace

test-scrollbar:
    cargo test -p tui-scrollbar --all-features
