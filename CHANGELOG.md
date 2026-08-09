# Changelog

All notable changes to aur-gate are documented here. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-08-08

Initial public release of the Rust implementation. aur-gate is a deterministic
pre-install gate for Arch Linux AUR updates, created during the May–June 2026
AUR supply-chain attack to stop a malicious candidate before yay/paru reaches
makepkg or pacman. The retired Bash implementation is frozen under
`archive/bash/`.

### Added

- **Deterministic gate** — captures one immutable candidate SHA, diffs
  `accepted..candidate`, and classifies hard findings (block), boring metadata
  (pass), or review-required signals.
- **Trust anchor** — `accept` advances state only when pacman's root-owned
  local database freshly confirms the staged version, pkgbase identity, build
  time, and install time. A blocked, unavailable, moved, stale, or uninstalled
  candidate can never advance the anchor.
- **Cross-shell wrapper** — generated Bash/Zsh wrapper intercepts yay/paru and
  holds one lock across the complete gate → helper → accept transaction; strips
  lock/staging capabilities before untrusted build code runs.
- **Makepkg seam guard** — requires the helper checkout to equal the staged
  SHA, checks committed regular-file surfaces, rejects every untracked or dirty
  file, and forces a fresh private build (closes index-flag, commit-graph, and
  staged-SHA races).
- **Missing-cache path** — clone fresh, reconstruct the installed-version
  baseline when retained history permits, run the same diff pipeline, and always
  require whole-candidate review; without a baseline, whole-candidate review
  remains mandatory.
- **Isolated Git boundary** — all Git calls go through `git::safe_git`:
  HTTP(S) only, isolated config and environment, explicit origin validation,
  replace refs and grafts disabled, repo-local include/http config rejected.
- **Advisory LLM verifier** — optional `llm` integration that can clear only
  deterministic boring-edge findings; it never overrides hard, review, or
  audit-unavailable outcomes.
- **Commands** — `gate`, `check`, `audit`, `scan`, `explain`, `accept`,
  `rules`, `wrapper`, `selftest`; exit codes `0` clean, `1` blocked, `2`
  review, `3` usage/configuration error.
- **Documentation** — design ledger, threat model, and a durable public
  finding catalog under `docs/findings/`.

### Security

- Fail-closed at every external boundary; a blocked update never advances
  accepted state.
- Wrapper rejects alternate helper trust contexts, custom makepkg
  programs/mflags, roots, configs, and AUR endpoints; remaining trust-context
  skip-list entries are rejected on dispatch.
- Maintainer identity changes, added `SKIP`, arbitrary/dynamic `install=`, and
  non-boring removals never auto-clear.
- State directories are current-user-owned real directories with mode `0700`.

[0.1.0]: https://github.com/bermudi/aur-gate/releases/tag/v0.1.0
