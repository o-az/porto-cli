set shell := ["fish", "-c"]
set dotenv-load := true
set positional-arguments := true

onboard *args='':
    cargo run --manifest-path Cargo.toml -- onboard {{ args }}

build *args='':
    cargo build {{ args }}

fmt:
    taplo format
    cargo fmt --all
    just --fmt --unstable

lint:
    taplo lint
    cargo check --all-features
    cargo clippy --all-targets --all-features

check: fmt
    cargo check --all-features --release
    cargo clippy --all-targets --all-features --release

publish:
    cargo publish
