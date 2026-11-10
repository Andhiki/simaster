# Governance

Decisions are lightweight and recorded; the project is small and the surface is sensitive.

## Mission and scope

Make SIMASTER UGM usable from a terminal, a script, and an AI agent through a
small, auditable, read-only-by-default client that respects student privacy,
UGM policy, and undocumented upstream limits.

- A Rust CLI for students to read their own SIMASTER data.
- A stable JSON contract and documented exit codes for automation.
- Official UGM ID OAuth 2.0/OIDC with browser PKCE, and web session with CAS
  fallback while registration is pending.
- vNext as a read-only data adapter, never as an authentication path.

## Roles

**Contributor** opens issues and PRs and follows the Code of Conduct. **Reviewer** reviews an area after sustained high-quality review and may request changes, but does not merge. **Maintainer** merges, triages, sets direction, votes on RFCs, and enforces the Code of Conduct; the role follows nomination and lazy consensus. **Security maintainer** owns advisories, `SECURITY.md`, and private reports, and can block a release. **Release manager** runs release-plz, signs tags, publishes, and verifies installs. At least one security maintainer and one release manager must exist before a release.

## Decision making

**Lazy consensus.** A proposal is an issue or pull request with the problem,
options, recommendation, and impact. The window is 7 days, or 72 hours for a
time-sensitive security fix with a written reason. No technical objection means
acceptance; an objection starts discussion, then a vote if consensus fails.

**Vote.** A simple majority of votes cast decides, with at least two votes and a
quorum of two thirds of active maintainers. Ties fail. Reasons are recorded. A
security maintainer may block a release for an unresolved security issue.

**RFCs.** Required for a breaking CLI, JSON envelope, config, or exit-code change; a new authentication surface or credential-storage change; a new write command or safeguard; a repository, license, or governance change; or a new endpoint family. Open a pull request under `docs/rfcs/` when it exists, otherwise an issue with the `type:feature` or `breaking-change` label. It stays open at least 7 days and links the change.

## Maintainer lifecycle

**Nomination.** A maintainer nominates a contributor or reviewer with their
contributions, area of ownership, and conflicts. After 7 days without objection,
the nominee accepts the role and the security and privacy expectations.

**Removal.** Grounds are six months of inactivity, repeated Code of Conduct
violations, a security or privacy breach, or an unmanageable conflict of
interest. Removal needs a two-thirds majority of active maintainers and a
written reason; the person may appeal.

**Emeritus.** A maintainer who leaves in good standing is listed as emeritus,
without voting or merge rights, and may return by accepting current governance.

## Repository ownership

- **Home:** `pendopo-dev/simaster`; the previous `Andhiki/simaster` URL redirects.
- **CODEOWNERS:** `@Andhiki` and `@vityasyyy` own the repository and the auth, API, and workflow paths. Update the file when a team exists.
- **Settings and migration:** the owner runbook, including access, teams, and organization migration, is tracked in the [repository settings issue](https://github.com/pendopo-dev/simaster/issues/49).

## Security and conduct

The security maintainer owns the private advisory, coordinates the fix and disclosure timeline, and is the reporter's contact; see [`SECURITY.md`](SECURITY.md). Maintainers enforce the [Code of Conduct](CODE_OF_CONDUCT.md); a maintainer with a conflict recuses themselves. A maintainer must disclose any relationship that could bias a decision, such as UGM employment, a vendor relationship, a competing project, or a personal relationship with a contributor. The person may discuss but must not vote or merge when the conflict is material.

## Amendment

Changes require an RFC, a 7-day comment window, and a simple majority of active
maintainers. The security maintainer may apply an emergency security change and
seek ratification within 7 days.
