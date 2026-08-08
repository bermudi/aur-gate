# gh#36 — Systematic wrapper skip-list / dispatch reject-list drift

**Source:** GitHub issue [#36](https://github.com/bermudi/aur-gate/issues/36)
(H-followup to #30)
**Status:** fixed
**Severity:** high
**Lines:** `assets/wrapper.sh:64-74` (classifier skip list), `assets/wrapper.sh:124-175` (dispatch reject list)

## Summary

The wrapper's classifier skip list contained 25 entries whose values are not
package targets. Only four of them (`--hookdir`, `--cachedir`, `--gpgdir`,
`--logfile`) had been added to the dispatch reject list as part of #30. The
remaining 21 were being passed through to yay/paru as `"$@"`, where they could
alter the helper's review step, build/clone context, or the privileged `pacman
-U` install step without any trust decision.

## Attack scenarios

### Review/editor tools

1. User runs `yay -Syu --editor /tmp/evil --editmenu`.
2. Wrapper classifier treats `/tmp/evil` as a non-target value (skip list) and
   dispatch does not reject it.
3. yay re-enables the diff/edit menu and invokes `/tmp/evil` on the PKGBUILD.
4. Arbitrary code runs as the user, outside the auditor, and may modify the
   package before it is built.

Paru has the same shape via `--review`, `--savechanges`, `--fm`, and `--bat`.
Even when the wrapper pins `--skipreview` / `--nosavechanges`, a user-supplied
`--review` (or `=true` / `=false` override) after the pinned flags could
re-enable the review step if the helper uses last-wins semantics.

### Build / clone context

`--builddir` (yay) and `--clonedir` (paru) redirect where the helper downloads
and runs PKGBUILDs. While the makepkg guard materializes the audited tree from
an immutable SHA into a private directory, allowing the caller to choose the
helper's checkout surface is an unnecessary trust-context override and defeats
"fail closed".

### Pacman privileged install behavior

`--overwrite`, `--assume-installed`, `--ask`, `--ignore`, and `--ignoregroup`
change the behavior of `pacman -U` / `pacman -S` as root: overwriting system
files, bypassing dependency checks, auto-answering prompts, or silently
dropping packages from the audited manifest. The manifest-to-installed binding
catches some of these, but they are still context-changing options that belong
in the denylist.

### Low-risk cosmetic options

`--color`, `--print-format`, `--sortby`, `--searchby`, `--requestsplitn`,
`--completioninterval`, `--limit`, `--develsuffixes`, and the four
`--answer*` helpers are display / auto-answer options that do not alter trust
context. They remain in the classifier skip list and pass through.

## Fix

For each of the 25 skip-list entries, chose one of:

1. **Reject in dispatch** (high and medium risk, plus review/display tools).
2. **Document as safe passthrough** (low-risk cosmetic options) and add a
   contract assertion that the option remains in the skip list.

Rejected options (both bare and `--opt=value` forms) now abort dispatch with
the existing `custom helper/build trust context is unsupported` message before
any helper runs:

- yay review/editor: `--editor`, `--editorflags`, `--editmenu`, `--diffmenu`
- paru review/display: `--review`, `--savechanges`, `--bat`, `--batflags`,
  `--fm`, `--fmflags`
- helper build/clone context: `--builddir`, `--clonedir`
- pacman install behavior: `--overwrite`, `--assume-installed`, `--ask`,
  `--ignore`, `--ignoregroup`

The wrapper's own pinned review-suppression values
(`--diffmenu=false`/`--editmenu=false` for yay, `--skipreview`/`--nosavechanges`
for paru) remain allowed, so a user shell alias that duplicates them is not
broken. Any other form of these flags (bare or `=value`) is rejected.

The wrapper contract unit test in `src/wrapper.rs` now asserts both the
reject-list and safe-passthrough fragments, and a new integration test
`wrapper_dispatch_rejects_review_build_pacman_context_options` exercises the
new rejects under bash and zsh for both helpers.

## Verification

- `bash -n assets/wrapper.sh` and `zsh -n assets/wrapper.sh` — clean.
- `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings` — clean.
- `cargo test --all-targets --no-fail-fast` — all tests pass, including
  `wrapper_dispatch_rejects_review_build_pacman_context_options`.
- `cargo run --quiet -- selftest` — passes.
- `bash -n assets/wrapper.sh` and `zsh -n assets/wrapper.sh` — clean.

## Lesson

A classifier skip list and a dispatch reject list that are maintained
independently will drift. Every skip-list entry that carries a non-package
operand must also be dispositioned against the dispatch reject list: either it
is a trust-context override and must be rejected, or it is purely
cosmetic/display and must be covered by a durable contract test. "Skipped
during audit" is not the same as "safe to pass through to the helper". Regular
systematic reconciliation of the two lists is cheaper than per-CVE catch-up.
