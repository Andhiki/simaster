## Constraints

- Official UGM ID OAuth 2.0/OIDC documentation is the primary auth evidence. Prior-art mobile-app analysis is secondary evidence for the vNext data adapter.
- Read-only by default. Writes require a separate milestone and an RFC.
- No embedded mobile client secret and no multi-user token reuse.
- No live authenticated SIMASTER calls in PR CI. A public `/health` check and mocked discovery/token/user endpoints are the test boundary.
- Every endpoint claim is marked `documented by UGM` or `unverified`, and `live-tested: no` unless an authorized live test is recorded.
- A command is not "done" until its help, JSON contract, exit codes, tests, and docs agree.

## Non-goals

- A web UI, mobile app, or browser extension.
- A general SIMASTER administration tool for staff.
- Credential sharing, token pooling, or proxy accounts.
- The mobile app's embedded client secret or custom-scheme redirect as a usable credential.
- A password grant, CAPTCHA-solving service, MFA bypass, rate-limit evasion, or WAF evasion.
- Scraping other people's data, enumerating accounts, or using `/user/username/{username}`.
- Write operations before M5, or any write that is not explicitly opt-in.
- A backend-for-frontend in v0. Revisit only through an RFC if DTI requires a confidential client and no user-supplied client is workable.
- A claim of UGM affiliation or endorsement.
- A guaranteed upstream compatibility promise for an undocumented API.

## Code structure

- No first-party crate may be above 1000 lines.
- No re-exports.
- No `#[path = "..."]`.
- No `include!`.

## Documentation

- Repository docs describe current state only. No plans, milestones, or delivery history.
- Task state lives in GitHub Issues/milestones; scratchpads live in gitignored `docs/plans/`.
- Human docs are indexed from `README.md`; agent instructions are `AGENTS.md` and `docs/agents/`.

## Agent skills

### Issue tracker

Issues live in the repo's GitHub Issues, driven with the `gh` CLI. See `docs/agents/issue-tracker.md`.

### Triage labels

Five canonical triage roles map 1:1 to label strings (`needs-triage`, `needs-info`, `ready-for-agent`, `ready-for-human`, `wontfix`). See `docs/agents/triage-labels.md`.

### Domain docs

Single-context: `GLOSSARY.md` at the repo root, ADRs in `docs/adr/`. See `docs/agents/domain.md`.
