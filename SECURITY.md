# Security Policy

aur-gate is a security boundary for AUR updates: it reviews candidate trees,
holds one trust lock across the gate → helper → accept transaction, and rejects
untrusted build input at the makepkg seam. If you find a way past that boundary,
please report it privately.

## Supported versions

The `main` branch is the only supported line. Patches land on `main` and are
released as new tags; there is no long-term-support branch.

## Reporting a vulnerability

Use GitHub's private security advisory flow — the "Report a vulnerability"
button on the repository page:

https://github.com/bermudi/aur-gate/security/advisories/new

Please include:

- the aur-gate revision (`git rev-parse HEAD`) and the wrapper you sourced, if
  the bug is in the wrapper path
- the package, PKGBUILD, or helper command that triggered it, or exact steps to
  reproduce
- what you expected and what actually happened
- a minimal reproducer, if practical

## What happens next

1. The report is acknowledged within two business days.
2. We work with you on a fix and agree on a coordinated disclosure timeline.
3. Once fixed, the durable lesson is recorded in
   [`docs/findings/`](docs/findings/README.md), the public finding catalog.

Do not disclose the issue publicly until a fix is available or we agree
otherwise.

## Scope

In scope: the Rust implementation, [`assets/wrapper.sh`](assets/wrapper.sh), and
the shipped package. Out of scope: the AUR itself, yay/paru, pacman, and Arch
infrastructure — but if aur-gate failed to block something those tools did, that
failure is very much in scope.

See [`docs/threat-model.md`](docs/threat-model.md) for the attacker model and
defensive principles.
