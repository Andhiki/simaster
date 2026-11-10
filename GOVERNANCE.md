# Governance

This document explains how the project makes decisions, who may make them, and
how the maintainer group changes over time. It is intentionally lightweight:
the project is small, the surface is sensitive, and the goal is to keep the
process predictable rather than bureaucratic.

## Mission

Make SIMASTER UGM usable from a terminal, a script, and an AI agent through a
small, auditable, read-only-by-default client that respects student privacy,
UGM policy, and the limits of undocumented upstream interfaces.

## Scope

In scope:

- A Rust CLI for students to read their own SIMASTER data.
- A stable JSON contract and documented exit codes for automation.
- Authentication through the web session and CAS fallback, with an optional
  experimental vNext OAuth provider.
- Documentation of endpoint evidence, stability, and provenance.
- A future read-only API surface and a separate MCP server.

## Non-goals

- A general-purpose SIMASTER administration tool.
- Credential sharing, multi-user token pooling, or proxy accounts.
- Bypassing authentication, authorization, CAPTCHA, or MFA controls.
- Copying, shipping, or documenting the official mobile app's embedded client
  secret as a usable credential.
- Write operations by default. Writes are a later, opt-in milestone.
- An official UGM product. The project is unofficial and makes no affiliation
  claim.

## Roles

| Role | Responsibilities | How to become one |
|---|---|---|
| **Contributor** | Opens issues and PRs, writes tests and docs, follows the Code of Conduct. | First accepted contribution. |
| **Reviewer** | Reviews PRs in an area; may request changes but does not merge. | Nominated by a maintainer after sustained, high-quality review. |
| **Maintainer** | Merges PRs, triages issues, sets direction, votes on RFCs, enforces the Code of Conduct. | Nomination plus lazy consensus (see below). |
| **Security maintainer** | Tracks advisories, coordinates private reports, owns `SECURITY.md`, can block a release for an unresolved security issue. | A maintainer with security experience, confirmed by the maintainer group. |
| **Release manager** | Runs release-plz, signs tags, publishes releases, verifies post-release installs. | A maintainer, rotating by semester or by agreement. |

A person may hold more than one role. At least one security maintainer and one
release manager must exist before a release is published.

### Current assignments

| Role | Current holder |
|---|---|
| Maintainer | `@Andhiki`, `@vityasyyy` |
| Security maintainer | Vacant until a maintainer with security experience is confirmed. Until then, the maintainers in `CODEOWNERS` receive reports and escalate to the group. |
| Release manager | Vacant until the M6 release preparation. The maintainer group appoints one before the first release. |

"Vacant" is a real state, not a placeholder: the role exists, the responsibility
is assigned to the maintainer group as a fallback, and the gap is resolved
before a release.

## Decision making

### Lazy consensus

Most decisions use **lazy consensus**:

1. A proposal is posted as an issue or pull request with the problem, the
   options, the recommended option, and the impact.
2. The comment window is **7 days** for normal changes. It may be shortened to
   72 hours for a time-sensitive security fix, with a written reason.
3. If no maintainer objects with a technical reason during the window, the
   proposal is accepted.
4. An objection starts a discussion. The proposer revises or withdraws. If
   consensus still fails, the maintainers vote.

### Maintainer vote

If consensus fails, the active maintainers vote. A simple majority of votes
cast decides, with at least two votes and a quorum of two thirds of active
maintainers. Ties fail. Votes and their reasons are recorded in the issue or
RFC. A security maintainer may block a release for an unresolved security
issue; the block is recorded and reviewed at the next maintainer meeting.

### RFCs

An RFC is required for:

- a breaking change to the CLI grammar, JSON envelope, config format, or exit
  codes;
- a new authentication surface or a change to credential storage;
- a new write command, or a change to write safeguards;
- a repository, license, or governance change;
- a new endpoint family that is not already covered by the API map.

An RFC is a pull request under `docs/rfcs/` when that directory exists. Until
then, open an issue with the `type:feature` or `breaking-change` label and the
same content. An RFC stays open for at least 7 days, has a named decision, and
links the implemented change.

## Maintainer lifecycle

### Nomination

Any maintainer may nominate a contributor or reviewer. The nomination states the
person's contributions, the area they would own, and any conflict of interest.
After a 7-day lazy-consensus window with no objection, the nominee becomes a
maintainer. The nominee must accept the role and confirm the security and
privacy expectations.

### Removal

A maintainer may be removed for:

- sustained inactivity (no review, merge, or governance participation for six
  months);
- repeated Code of Conduct violations;
- a security or privacy breach;
- a conflict of interest that cannot be managed.

Removal requires a maintainer vote with a two-thirds majority of active
maintainers and a written reason shared with the person before the vote when
practical. A removed maintainer may appeal to the remaining maintainers.

### Emeritus

A maintainer who steps away in good standing becomes an **emeritus maintainer**.
Emeritus status keeps the person listed, removes voting and merge rights, and
allows a return by confirming availability and accepting the current
governance. Emeritus status is appropriate for graduation, exchange, or a
semester with no capacity.

### Semester handover

Because this is a university project, the maintainer group performs a handover
at the end of each academic semester:

1. Confirm at least two active maintainers and one security maintainer.
2. Transfer repository administration to the incoming owner group.
3. Review open security advisories and unreleased changes.
4. Update `CODEOWNERS`, the release manager, and the repository settings
   runbook.
5. Record the handover in a governance issue or the changelog.
6. Archive credentials that are no longer needed; never pass personal tokens to
   a successor.

## Repository ownership

- **Current home:** `Andhiki/simaster`.
- **Target home:** a dedicated organization account. The migration is
  **blocked** until an organization exists and at least two administrators are
  available. The organization name will be chosen by an RFC; this document does
  not assume one.
- **Administrators:** at least two people with admin access. The current
  maintainer group does not meet this requirement yet; see
  [`docs/REPO-SETUP.md`](docs/REPO-SETUP.md).
- **CODEOWNERS:** global owners are `@Andhiki` and `@vityasyyy`; auth, API, and
  workflow paths require both. Update the file when a team exists.
- **Repository settings:** the current GitHub user has **WRITE**, not
  **ADMIN**. Repository settings are a runbook for the owner, not commands a
  contributor runs. See [`docs/REPO-SETUP.md`](docs/REPO-SETUP.md).

## Releases

The release manager prepares releases with release-plz. A release requires:

- green CI on `main`;
- approval from the release manager and one other maintainer;
- two approvals for a release that includes authentication, security, or
  breaking changes;
- a signed tag and a changelog entry;
- post-release verification of the published artifact.

The security maintainer may block a release for an unresolved security issue.
The maintainer group documents the reason and the resolution.

## Security response

The security maintainer owns the private advisory, coordinates the fix and the
disclosure timeline, and is the point of contact for reporters. See
[`SECURITY.md`](SECURITY.md) for response targets and scope.

## Code of Conduct enforcement

Maintainers enforce the [Code of Conduct](CODE_OF_CONDUCT.md). A maintainer
with a conflict of interest must recuse themselves from the discussion and the
decision. Enforcement actions are documented privately; a summary is shared
with the maintainer group. Retaliation is itself a violation.

## Conflict of interest

A maintainer must disclose any relationship that could bias a decision, such as
employment by UGM, a vendor relationship, a competing project, or a personal
relationship with a contributor. Disclosure is recorded in the relevant issue.
The person may participate in discussion but must not vote or merge when the
conflict is material.

## Amendment

Changes to this document require an RFC, a 7-day comment window, and approval by
a simple majority of active maintainers. The change is recorded in
[`CHANGELOG.md`](CHANGELOG.md) under `Changed`. Emergency changes for a security
incident may be applied by the security maintainer and ratified within 7 days.
