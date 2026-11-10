default: check

fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all -- --check

clippy:
    cargo clippy --all-targets --all-features -- -D warnings

test:
    cargo test --all-features

doc:
    RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features

check: fmt-check clippy test doc

run *args:
    cargo run -- {{args}}

deny:
    cargo deny check

audit:
    cargo audit
