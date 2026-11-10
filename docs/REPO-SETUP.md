# Repository setup

This is a runbook for the repository **owner**. The current GitHub user has
**WRITE**, not **ADMIN**, so a worker or contributor cannot apply these
settings. Do not run repository-administration commands from a worker session;
the owner applies them in the GitHub UI (or with an admin-scoped token).

## About

Set the repository description to:

> Unofficial terminal-first CLI for SIMASTER UGM: scriptable, JSON-first, agent-friendly.

Set topics:

```text
rust
cli
ugm
simaster
student
terminal
json
agent
oauth
cas
academic-data
command-line
```

Use the same description and homepage in `Cargo.toml` and the README.

## Features

| Setting | Value | Reason |
|---|---|---|
| Issues | On | Bug reports, feature requests, endpoint reports, auth issues. |
| Discussions | On | Questions and usage help; see `SUPPORT.md`. |
| Wiki | Off | Documentation lives in the repository and is reviewed like code. |
| Projects | Optional | Use only if the maintainers need a board; not a source of truth. |
| Sponsorships | Optional | Not required for the project. |
| Pages | Off until M6 | The docs site is planned for M6. |
| Releases | On | Required for release-plz and signed tags. |

## Labels

Create these labels exactly. The names are used by the issue forms and the
triage policy.

| Label | Description | Color |
|---|---|---|
| `type:bug` | Something is broken or behaves differently from the contract. | `d73a4a` |
| `type:feature` | A new capability or a change in behavior. | `a2eeef` |
| `type:docs` | Documentation, examples, or help text. | `0075ca` |
| `area:auth` | Authentication, sessions, tokens, or credential storage. | `5319e7` |
| `area:api` | Endpoint adapters, API map, fixtures, or upstream drift. | `1d76db` |
| `area:cli` | Command grammar, flags, output, completions. | `0e8a16` |
| `area:security` | Security-sensitive change; requires the security review path. | `b60205` |
| `good first issue` | Scoped, documented, and suitable for a first contribution. | `7057ff` |
| `help wanted` | Maintainers want a contributor to take it. | `008672` |
| `needs-triage` | Not yet assessed by a maintainer. | `fbca04` |
| `needs-info` | Waiting for a reproduction, version, or sanitized evidence. | `d876e3` |
| `blocked:upstream` | Waiting on UGM or an external dependency. | `e99695` |
| `breaking-change` | Requires an RFC and two approvals. | `d93f0b` |

Do not create labels that duplicate these without an RFC. Keep the set small
enough to be usable.

## Milestones

| Milestone | Description | State after this wave |
|---|---|---|
| M0 — Documentation | Docs, governance, templates, ADRs. | Close when merged. |
| M1 — Skeleton and CI | Rust crate, doctor, completions, tests, CI. | Close when CI is green on GitHub. |
| M2 — Auth and identity | Web/CAS login, storage, status, logout, whoami. | Open. |
| M3 — Read commands | Profile, schedule, grades, attendance, fixtures, canary. | Open. |
| M4 — Agent surface | JSONL, `--no-input`, read-only API, MCP server. | Open. |
| M5 — Write operations | Opt-in writes, dry-run, confirmation, audit. | Open. |
| M6 — Release and partnership | release-plz, packages, docs site, UGM engagement. | Open. |

## Branch protection

Protect `main` with a ruleset or branch protection rule:

- Require a pull request before merging.
- Require **1 approval** by default.
- Require **2 approvals** for changes under `src/auth/`, `docs/AUTH.md`,
  `SECURITY.md`, `.github/workflows/`, and the API adapters. Enforce this in
  review and with a path-based ruleset if the plan supports it; CODEOWNERS adds
  the required reviewers.
- Dismiss stale approvals when new commits are pushed.
- Require review from Code Owners.
- Require the CI status checks: `fmt`, `clippy`, the three `test` matrix
  entries, `msrv (1.85)`, `deny`, and `docs`.
- Require branches to be up to date before merging.
- Require linear history.
- Do not allow force pushes.
- Do not allow deletions.
- Do not allow bypass, including for administrators. The owner may temporarily
  lift protection for a documented security incident and restore it
  immediately.
- No direct pushes to `main` after the bootstrap. The initial commit may be
  pushed by the owner to establish the branch.

## Merge settings

- Allow **squash merging only**. Disable merge commits and rebase merging so the
  history stays linear and the changelog maps to squashed PRs.
- Automatically delete the head branch after merge.
- Use the PR title as the squash commit subject; the PR template supplies the
  body.
- Enable auto-merge only if required checks are enforced; do not let an author
  bypass review.

## Security features

Enable in repository settings:

- Dependabot alerts.
- Dependabot security updates.
- Secret scanning.
- Push protection for secret scanning.
- Private vulnerability reporting / GitHub Security Advisories.
- Default workflow permissions: **read repository contents**. A workflow must
  declare any additional permission it needs.
- Require approval for first-time contributors' workflows if available.
- CodeQL after M1. If Rust analysis is not available for the repository plan,
  record the limitation and rely on clippy, `cargo-deny`, `cargo-audit`, and
  manual review; do not claim CodeQL coverage that does not exist.

Do not enable a feature that sends repository data to a third party without
documenting it in `SECURITY.md`.

## CODEOWNERS and teams

The current `CODEOWNERS` uses individual accounts:

```text
* @Andhiki @vityasyyy
```

When an organization and teams exist, replace the individuals with teams, for
example:

```text
* @<org>/maintainers
/src/auth/ @<org>/auth-reviewers
/src/api/ @<org>/api-reviewers
/.github/workflows/ @<org>/maintainers
```

Do not guess an organization name. Create the team first, then update the file
in a reviewed PR. The auth and API paths require both maintainers until a team
has at least two reviewers.

## Environment and secrets

The project needs no repository secrets for normal CI. If a scheduled canary is
added, use a dedicated maintainer account and a repository secret with the
narrowest scope, or run the canary locally. Never store a real student's
credentials in repository secrets. Document every secret in `SECURITY.md`.

## Organization migration checklist

The repository stays at `Andhiki/simaster` until an organization exists. The
migration is **blocked** on creating that organization and naming at least two
administrators.

When the organization exists:

1. Create the organization with two-factor authentication required.
2. Create the teams: maintainers, auth reviewers, API reviewers.
3. Transfer the repository. GitHub redirects the old URL, but update every
   canonical link.
4. Recreate branch protection and rulesets; they do not transfer with the
   repository.
5. Re-enable Dependabot, secret scanning, push protection, and private
   vulnerability reporting.
6. Move or re-create repository secrets. Do not copy personal tokens.
7. Update `CODEOWNERS`, README badges, `Cargo.toml` repository/homepage, issue
   form links, and package metadata.
8. Verify CI on the new owner, including the MSRV and dependency jobs.
9. Verify releases, tags, and the security advisory link.
10. Announce the move and keep the old URL redirect in place.
11. Archive the old access list and revoke stale deploy keys or tokens.

## Post-setup verification

- Open a test PR and confirm the required checks and approvals are enforced.
- Confirm a direct push to `main` is rejected.
- Confirm a secret in a test branch is blocked by push protection.
- Confirm Dependabot can open a PR for a minor update.
- Confirm the security advisory link resolves.
- Confirm the issue forms render with the expected labels.
- Record the results in the repository's governance issue or changelog.
