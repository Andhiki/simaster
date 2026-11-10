## TL;DR

- What changed, why, and the user-visible impact (at most three bullets).

## Related issue

Closes #<issue-number>

## Motivation

Describe the problem, the constraint, and the chosen approach. Link the design
discussion or RFC when one exists.

## Changes

- List the behavior, files, and docs that changed.
- Note anything intentionally left out.

## Testing

- [ ] `cargo fmt --all -- --check`
- [ ] `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] `cargo test --all-features`
- [ ] `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features`
- [ ] `cargo +1.85.0 check --all-features` when the MSRV is relevant
- [ ] `cargo deny check` when dependencies changed

Commands and results:

```text
<command>
<result>
```

## CLI output or screenshots

Paste sanitized `--help`, JSON, or terminal output for any user-visible change.
Remove credentials, tokens, cookies, names, NIU numbers, and personal data.

## Checklist

- [ ] The change has tests, or the PR explains why a test is not applicable.
- [ ] Tests and the behavior change are in the same commit.
- [ ] Documentation and the changelog are updated when behavior changes.
- [ ] No passwords, cookies, tokens, client secrets, or authorization codes.
- [ ] No real student names, NIU numbers, grades, schedules, or attendance data.
- [ ] The official mobile app's client ID and embedded secret are not included.
- [ ] Endpoint changes update `docs/API-MAP.md` with provenance and stability.
- [ ] A breaking change is labelled `breaking-change` and has an RFC.
- [ ] Required CI checks are green.
- [ ] One approval is requested; auth, security, release, and breaking changes
      request two.

## Reviewer notes

Where should the reviewer start? What is uncertain, and what evidence would
change your mind?
