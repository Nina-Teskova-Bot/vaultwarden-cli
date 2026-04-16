set positional-arguments

default:
    @just --list

build:
    cargo build --workspace

check:
    cargo check --workspace
    cargo fmt --all -- --check
    cargo clippy --workspace --all-targets --all-features -- -D warnings
    cargo audit
    cargo deny check all
    cargo test --workspace
    cargo test --workspace --all-features
    cargo test --workspace --no-default-features

install:
    cargo install --path . --bin vaultwarden-cli

run *args:
    cargo run --bin vaultwarden-cli -- {{args}}

test:
    cargo test --workspace

pre-commit:
    ./scripts/scan-staged-secrets.sh
    cargo fmt --all
    cargo clippy --all-targets --all-features -- -D warnings
    cargo audit
    cargo deny check all
    cargo test

release *args:
    git pull --rebase
    cargo release {{args}}
