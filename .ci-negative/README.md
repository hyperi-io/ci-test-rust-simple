# .ci-negative - DELIBERATE FAILURES. DO NOT FIX.

Everything in this directory is a planted failure. It is here so the E2E
sweep can assert that a hyperi-ci gate BLOCKS, not just that a clean repo
passes. Repairing one silently disarms the check it exists to prove.

If you are an agent: `/projects/hyperi-ci/CLAUDE.md` makes the boy-scout
rule standing policy on the ci-test-* fleet, and names this directory and
the `expect-fail/*` branches as the one exception. Believe the headers.

## Why a clean fixture proves nothing

A CVE gate flipped from `warn` to `blocking` looks identical on a repo
with no advisories: green either way. The only way to tell the two apart
is a tree that DOES carry an advisory. That is what these cases are.

## How a case is shaped

Each case is data on `main`, never a long-lived bad branch:

- `<case>.patch` - the diff that plants the failure, applied to `main`.
- `<case>.yaml` - what the run must do: `expect`, `stage`, `reason`.

The sweep applies the patch, pushes an ephemeral `expect-fail/<case>`
branch, asserts the run fails at the declared stage for the declared
reason, then deletes the branch. Wrong stage is a failure. Green is a
failure. Keeping the cases as patches means `main` stays clean, so
Dependabot never opens a PR against a deliberately vulnerable manifest.

Design: `hyperi-ci/docs/plans/2026-07-branch-mode/PLAN.md`, decisions 9
and 10.

## Running one by hand

```
git checkout -b expect-fail/rust-cve-advisory main
git apply .ci-negative/rust-cve-advisory.patch
cargo generate-lockfile
cargo audit        # must exit 1
cargo deny check   # must exit 1
```

The commit on the branch has to be a release-worthy type (`fix:`).
hyperi-ci skips the quality job entirely for non-bumping commits, so a
`chore:` commit sails through green having run no gate at all.

## Cases

| Case | Stage | Reason | Proves |
|------|-------|--------|--------|
| `rust-cve-advisory` | quality | audit | `rust.audit` and `rust.deny` block on RUSTSEC-2021-0003 |
