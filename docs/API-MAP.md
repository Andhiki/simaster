# API map

This document records the upstream surfaces the CLI may use, the evidence for
each claim, and the stability policy. It is an interoperability map, not an
exploitation guide.

> **Evidence quality:** unless a row says `live-tested: yes` with a date, the
> behavior is reconstructed from prior-art code or static analysis of the
> official mobile app. It is `unverified`. Static analysis describes client
> code, not server behavior.

## Status legend

| Status | Meaning |
|---|---|
| **available** | Implemented and covered by tests in this repository. No API surface is available in M1. |
| **planned** | Accepted for a future milestone; not implemented. |
| **experimental** | Implemented or specified behind an explicit opt-in; may change without notice. |
| **unverified** | Documented from non-live evidence. No server contract is assumed. |
| **blocked** | Not implemented and not planned in the current scope, with a stated reason. |

## Surfaces

| Surface | Auth | Data | Stability | Scope |
|---|---|---|---|---|
| Web session (`simaster.ugm.ac.id`) | Username/password form + session cookie. | Portal data; profile, schedule, grades, attendance. | `unverified` | Read-only. Planned M2/M3. |
| vNext gateway (`api.simaster.ugm.ac.id`) | OAuth Bearer + `UGMFWSERVICE: 1`. | Student data by service label. | `unverified`, `experimental` | Read-only subset. Planned M3. |
| vNext OAuth (`oauth.simaster.ugm.ac.id`) | Authorization code; access and refresh tokens. | Token issuance. | `unverified`, `experimental` | Opt-in provider; user-supplied client. |
| Staff gateway (`api.simaster.ugm.ac.id/v1/staff/...`) | Bearer + `UGMFWSERVICE: 1` (one path omits it). | Staff profiles and attendance scans. | `blocked; unverified` | Out of scope for a student CLI. |
| Legacy presensi (`simaster.ugm.ac.id/presensivnext/...`) | Client-derived Basic credential; presensi Bearer. | Attendance write. | `blocked; unverified` | Intentionally not implemented; client-derived credentials and write risk. |

## Endpoint registry

The `owner` column lists the people accountable for evidence and review, not
the only people allowed to change the row.

| Surface | Method | Path | Auth | Request | Response | Provenance | Stability | Live-tested | Owner |
|---|---|---|---|---|---|---|---|---|---|
| Web session | `POST` | `https://simaster.ugm.ac.id/services/simaster/service_login` | Username/password form; cookie jar. | Form fields `aId`, `username`, `password`. | JSON with `isLogin`, `namaLengkap`; session cookie. | Prior-art `simaster.py`. | `unverified` | No | `@Andhiki @vityasyyy` |
| CAS | `GET` | `https://sso.ugm.ac.id/cas/login?service=...` | CAS session; `lt` token. | Query `service`; login form later posts `username`, `password`, `lt`, `_eventId`, `submit`. | HTML login form and `302` with a ticket. | Prior-art `simaster.py`, `oauth.py`. | `unverified` | No | `@Andhiki @vityasyyy` |
| Web session | `GET` | `https://simaster.ugm.ac.id/ugmfw/signin_simaster/signin_proses` | Cookie jar. | No body documented. | Redirect to CAS. | Prior-art `simaster.py`. | `unverified` | No | `@Andhiki @vityasyyy` |
| Web session | `POST` | `https://simaster.ugm.ac.id/ugmfw/signin_simaster/captchasound_verification` | Cookie jar; `simasterUGM_token`. | Form with `simasterUGM_token`, `captcha`. | Redirect or HTML indicating the result. | Prior-art `simaster.py`. | `unverified` | No | `@Andhiki @vityasyyy` |
| vNext OAuth | `GET` | `https://oauth.simaster.ugm.ac.id/oauth/authorize` | Browser session; authorization code. | Query `response_type=code`, `client_id`, `redirect_uri`, `scope`; this CLI adds `state` and PKCE. | Redirect to the registered redirect with `code` or `error`. | Static analysis. The official app has no `state`/PKCE. | `unverified`, `experimental` | No | `@Andhiki @vityasyyy` |
| vNext OAuth | `POST` | `https://oauth.simaster.ugm.ac.id/oauth/token` | Client authentication + authorization code. | Form `grant_type`, `code`, `redirect_uri`, client credentials, `code_verifier`. | Token JSON: access token, refresh token, expiry, token type. | Static analysis. | `unverified`, `experimental` | No | `@Andhiki @vityasyyy` |
| vNext gateway | `POST` | `https://api.simaster.ugm.ac.id/v1/student/micro/services` | Bearer + `UGMFWSERVICE: 1`. | Multipart `json1` (raw JSON) + `json` (base64url); body `{micro, service, query, param}`. | JSON; some callers read `res` and treat 400 as a branch. | Static analysis. | `unverified` | No | `@Andhiki @vityasyyy` |
| vNext gateway | `POST` | `https://api.simaster.ugm.ac.id/v1/staff/micro/services` | Bearer + `UGMFWSERVICE: 1`. | Same envelope; staff profile label. | JSON with `res`. | Static analysis. | `blocked; unverified` | No | `@Andhiki @vityasyyy` |
| Account lookup | `POST` | `https://api.simaster.ugm.ac.id/staff` | Bearer + `UGMFWSERVICE: 1`. | Same envelope; username lookup label. | JSON with `res`, `is_active`, `user_tipe_nomor`; callers expect 400. | Static analysis; route has no recovered UI caller. | `blocked; unverified` | No | `@Andhiki @vityasyyy` |
| Staff attendance | `POST` | `https://api.simaster.ugm.ac.id/v1/staff/micro/doscanent` | Bearer; no `UGMFWSERVICE` observed. | Multipart with QR, GPS, and identity fields. | Success body is discarded by the client. | Static analysis. | `blocked; unverified` | No | `@Andhiki @vityasyyy` |
| Legacy presensi | `POST` | `https://simaster.ugm.ac.id/presensivnext/api/signin` | Basic client credential; client-derived password. | Form `username`, `password`. | JSON with a presensi token. | Static analysis. | `blocked; unverified` | No | `@Andhiki @vityasyyy` |
| Legacy presensi | `POST` | `https://simaster.ugm.ac.id/presensivnext/api/send_peserta` | Presensi Bearer. | Multipart attendance fields. | Attendance result. | Static analysis. | `blocked; unverified` | No | `@Andhiki @vityasyyy` |

The binary also contains OAuth **development** endpoints (`oauth.dev.ugm.ac.id`)
and a Postman callback. They are not targets, are not documented as usable
here, and must not be tested. The project only uses documented production
hosts with the user's own account.

### Excluded endpoints

The staff gateway, `/staff` lookup, `doscanent`, and both legacy presensi
endpoints are intentionally excluded:

- they are staff or write surfaces, not read-only student surfaces;
- the legacy presensi flow derives a credential from a client credential and a
  username, which this project will not reproduce;
- the attendance scan endpoint carries QR, GPS, and identity fields and is a
  write operation;
- the `/staff` lookup has no recovered UI caller and the evidence does not
  establish that it is safe or intended for students.

Exclusion is a product decision, not a claim that the endpoints are broken.

## vNext gateway envelope

Static analysis shows a two-field multipart body with a JSON header:

- `Authorization: Bearer <token>`
- `UGMFWSERVICE: 1`
- `Content-Type: application/json;charset=UTF-8` (observed in the client)
- `Charset: utf-8`
- `accept: application/json`
- `json1`: the raw JSON body
- `json`: a base64url variant of the same JSON

The base64url variant is described as standard Base64 with `+` replaced by
`.`, `/` replaced by `_`, and `=` replaced by `-`. This is unusual and
`unverified`; capture the final wire format before depending on it.

The logical body is:

```json
{
  "micro": "student",
  "service": "<service-label>",
  "query": [],
  "param": []
}
```

`query` and `param` are operation-specific. Their exact shapes are
`unverified`; do not reuse a shape across labels without a fixture and a
contract test. Some recovered callers compare the HTTP status with `400` and
then read a `res` field. Do not generalize that behavior: it is a static
observation that needs a live capture.

The client has two networking stacks. The V1 stack uses a validated access
token and sends `UGMFWSERVICE: 1`; the V2 stack reads the stored token and
omits `UGMFWSERVICE`. The order in which interceptors overwrite the
`Authorization` header is `unverified`. This CLI will use one adapter per
surface and record the final request in a fixture test.

## Service labels

The vNext gateway addresses operations by a label in the request body, not by a
separate URL. The inventory below comes from static analysis. All 29 labels are
`unverified`; presence in the binary is not proof that a label is callable,
authorized, or stable.

| # | Micro | Label | Area | Endpoint |
|---:|---|---|---|---|
| 1 | student | `get_agenda_kehadiran_rekap_student` | Attendance recap | `/v1/student/micro/services` |
| 2 | student | `get_agenda_kehadiran_rinci_student` | Attendance detail | `/v1/student/micro/services` |
| 3 | student | `get_banner_student` | Banner | `/v1/student/micro/services` |
| 4 | student | `get_berita_list_student` | News | `/v1/student/micro/services` |
| 5 | student | `get_jadwal_anda_agenda_student` | Schedule agenda | `/v1/student/micro/services` |
| 6 | student | `get_jadwal_anda_date_student` | Schedule by date | `/v1/student/micro/services` |
| 7 | student | `get_jadwal_anda_now_student` | Current schedule | `/v1/student/micro/services` |
| 8 | student | `get_jadwal_anda_studi_student` | Study schedule | `/v1/student/micro/services` |
| 9 | student | `get_konfirmasi_qr_student` | QR confirmation | `/v1/student/micro/services` |
| 10 | student | `get_riwayat_keaktifan_mhs` | Activity history | `/v1/student/micro/services` |
| 11 | student | `get_riwayat_pembayaran_mhs` | Payment history | `/v1/student/micro/services` |
| 12 | student | `get_riwayat_permohonan_mhs` | Request history | `/v1/student/micro/services` |
| 13 | student | `get_semester_student` | Semester or period | `/v1/student/micro/services` |
| 14 | staff | `get_simaster_atribut_by_username` | Account lookup | `/staff` |
| 15 | staff | `get_staff_profil_by_staff_id` | Staff profile | `/v1/staff/micro/services` |
| 16 | student | `get_student_beasiswa_progres_ajuan_by_niu` | Scholarship progress | `/v1/student/micro/services` |
| 17 | student | `get_student_beasiswa_tawar_by_tahun` | Scholarship offers | `/v1/student/micro/services` |
| 18 | student | `get_student_jadwal_ujian_by_niu` | Exam schedule | `/v1/student/micro/services` |
| 19 | student | `get_student_krs_by_niu_semester` | KRS and study results | `/v1/student/micro/services` |
| 20 | student | `get_student_oss_detail_by_id` | OSS detail | `/v1/student/micro/services` |
| 21 | student | `get_student_oss_list_by_niu` | OSS list | `/v1/student/micro/services` |
| 22 | student | `get_student_pengumuman_by_niu` | Announcements | `/v1/student/micro/services` |
| 23 | student | `get_student_profil_by_niu` | Student profile | `/v1/student/micro/services` |
| 24 | student | `get_student_sks__semester_by_niu` | SKS per semester | `/v1/student/micro/services` |
| 25 | student | `get_studi_jadwal_periode` | Period schedule | `/v1/student/micro/services` |
| 26 | student | `get_studi_kehadiran_rekap` | Attendance recap | `/v1/student/micro/services` |
| 27 | student | `get_studi_kehadiran_rinci` | Attendance detail | `/v1/student/micro/services` |
| 28 | student | `get_studi_krs` | KRS | `/v1/student/micro/services` |
| 29 | student | `get_studi_max_sks` | SKS limit | `/v1/student/micro/services` |

Labels 14 and 15 are staff surfaces and are `blocked`. The rest are candidates
for read-only student features.

### Profile endpoint

The student profile label is `get_student_profil_by_niu`. The exact `param`
shape is **unverified**. The suffix suggests a NIU, but static evidence does not
establish whether the server accepts a NIU in `param`, derives it from the
token, or expects a different array nesting. The CLI will not take an arbitrary
NIU from the user for this call; it will use the authenticated identity or a
documented, read-only identifier path. A live test with the user's own account
is required before the adapter is marked `experimental`.

## OAuth scopes and redirect

The official app requests twelve production scopes. Four grant write access:

| Read-only | Write |
|---|---|
| `user.read` | `user.read-write-update` |
| `userDetail.read` | `mygate.read-write` |
| `student` | `parent.read-write-update` |
| `student.read` | `student.read-write-update` |
| `student.niu.read` | |
| `vehicle.read` | |
| `parent.read` | |
| `transgama.read` | |

This CLI requests only the read-only subset. The write scopes are never
requested by default; a future write feature needs an RFC and `--allow-write`.

The official app uses the custom scheme
`id.ac.ugm.student.vnext.simaster://oauth2` as its redirect. That scheme belongs
to the official app and is **not usable by this CLI**. The CLI uses a loopback
redirect with `state` and PKCE when a registered client is available. Do not
register or intercept the official app's scheme.

The production OAuth endpoints are `https://oauth.simaster.ugm.ac.id/oauth/authorize`
and `https://oauth.simaster.ugm.ac.id/oauth/token`. The official app's flow has
no PKCE and no `state`; this project does not copy that weakness. See
[`AUTH.md`](AUTH.md).

## Stability policy

1. **New endpoint:** starts `unverified`. Documentation, a sanitized fixture,
   and a contract test are required before code is merged.
2. **Experimental:** a maintainer with an authorized account performs a live,
   read-only test with their own data. The result is recorded with a date and
   the response is reduced to a sanitized fixture. The endpoint becomes
   `experimental`.
3. **Stable:** after a canary window with no shape changes, the endpoint may be
   marked `stable`. A stable endpoint still fails with `UPSTREAM_CHANGED` when
   the response shape moves.
4. **Canary:** a nightly, opt-in job uses a maintainer's own account, never a
   shared account, and performs read-only calls. It is not part of PR CI.
5. **Feature flags:** experimental surfaces are gated behind an explicit flag or
   command status. A broken surface can be disabled without a release.
6. **Drift:** a missing field, a changed status, or an unexpected HTML page
   produces `UPSTREAM_CHANGED` (exit 5). The CLI does not guess, silently retry
   with different parameters, or fall back to scraping.
7. **Removal:** a removed endpoint is documented in the changelog and the
   command is disabled with a clear error. There is no silent compatibility
   shim for an undocumented upstream.

## How to add or verify an endpoint

1. Collect evidence from an authorized source: public documentation, static
   analysis, or a live test with your own account.
2. Record the surface, method, path, auth, request shape, response shape,
   provenance, date, and live-tested status.
3. Sanitize the evidence: remove names, NIU numbers, cookies, tokens, device
   identifiers, GPS, and any value that could identify a person. Replace them
   with documented placeholders.
4. Add a fixture under `tests/fixtures/` and a `wiremock` contract test. The
   test must not contact production.
5. Update this file and the command documentation.
6. Request review. Auth, write, and security-relevant endpoints need two
   approvals.
7. Mark the endpoint `unverified` until the live test is complete. Do not
   promote it based on a single happy path; record at least one error shape.

Never include the official app's embedded client secret, a real client ID, a
real token, a real cookie, or real student data in an issue, fixture, test, or
documentation change.

## Responsible use

This map exists to build an interoperable client for a student's own data. It
is not a penetration-testing guide. Do not use it to:

- access another person's records;
- enumerate accounts or identifiers;
- test write endpoints against production;
- bypass CAPTCHA, MFA, rate limits, or authorization;
- run load, fuzz, or denial-of-service tests against UGM;
- share or pool sessions and tokens.

If you find an upstream vulnerability, report it to UGM through its own
channels. Do not publish exploit details or another person's data. The project's
coordinated disclosure policy is in [`SECURITY.md`](../SECURITY.md).
