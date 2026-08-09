# Contributing

## The short version

- The Rust implementation at the repository root is canonical. The Bash
  implementation under [`archive/bash/`](archive/bash/README.md) is frozen
  history — it must not receive fixes.
- Read [`docs/design-ledger.md`](docs/design-ledger.md) and
  [`docs/threat-model.md`](docs/threat-model.md) before touching the trust path.
- Every change must keep the full verification suite green.

## Development setup

A current Rust toolchain plus `/usr/bin/git`, `/usr/bin/curl`, and util-linux
`flock` are required. The integration suites also need `/bin/zsh`.

```sh
cargo build --release
cargo test --all-targets
```

## Verification

Run the entire suite before submitting. CI
([`.github/workflows/ci.yml`](.github/workflows/ci.yml)) runs exactly this on
every push and pull request:

```sh
cargo fmt --check
cargo test --all-targets --no-fail-fast
cargo clippy --all-targets -- -D warnings
cargo run --quiet -- selftest
bash -n assets/wrapper.sh
zsh -n assets/wrapper.sh
```

`--no-fail-fast` ensures a lib-unit flake cannot mask the `harness = false`
integration suites that carry the wrapper/transaction proof tests.

For a controlled live missing-cache boundary, use disposable state/caches and
`check ventoy-bin`; the expected result is review (`2`), never clean.

## Trust invariants

- `accepted/<pkgbase>` advances only to the exact immutable SHA audited, built
  through the wrapper guard, and freshly confirmed by pacman's root-owned DB.
- Capture the candidate SHA once; never re-resolve a mutable branch after audit.
- Every candidate tree leaf is a regular committed blob. The makepkg seam
  repeats the check and rejects every untracked file.
- Added `SKIP`, arbitrary/dynamic `install=`, non-boring removals, and
  maintainer identity changes never auto-clear.
- All Rust Git calls go through `git::safe_git`; HTTP(S) only, isolated config
  and environment, explicit origin validation.
- LLM output is advisory. It may only auto-clear deterministic `BoringEdge`;
  it never overrides hard, review, or audit-unavailable outcomes.
- State directories are current-user-owned real directories with mode `0700`.

## Conventions

- Fail closed at every external boundary; do not swallow exceptions.
- Validate package names and external input before path/URL use.
- Do not weaken transport, state, or pacman evidence checks for fixtures.
- Do not add name blacklists; detect structural behavior.
- A blocked update must never advance accepted state.
- Type-safety: no `any`-style escapes or hidden global test overrides.

## Security findings

Durable lessons belong in [`docs/findings/`](docs/findings/README.md) once a
finding is closed. Open work is tracked in GitHub issues.

If you believe you have found an exploitable vulnerability, do **not** open a
public issue. Follow [`SECURITY.md`](SECURITY.md) and report via GitHub's
private advisory flow.
