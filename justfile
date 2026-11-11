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

# Lint the multi-file OpenAPI documents (issue #51). Requires Node on PATH;
# the CLI is fetched on demand, nothing is installed into the repo.
api-lint:
    npx --yes @redocly/cli@2.77.0 lint docs/openapi/simaster-web/openapi.yaml
