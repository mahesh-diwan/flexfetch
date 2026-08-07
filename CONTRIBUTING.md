# Contributing to flexfetch

Thanks for your interest! flexfetch is a small, fast, opinionated system-info tool.
Before you open a PR, read this — it will save everyone (including you) a review
round-trip.

## Ground rules

1. **Check existing issues first.** Search open issues and discussions before
   proposing new features. If your idea was previously rejected, don't
   re-propose it without new justification.
2. **The diet is a feature.** Every advanced capability lives behind a
   compile-time feature gate so `--no-default-features` stays small (minimal
   binary ~1.5 MB). New dependencies are scrutinized hard — prefer pure std,
   then pure Rust, then a gated dep. See `flexfetch-core/Cargo.toml` for the
   house style (e.g. `image` pinned to `default-features = false, features =
["png"]`).
3. **Keep the gates honest.** If you add a feature:
   - add it to `flexfetch-core` and/or `flexfetch-cli` `[features]`,
   - verify the feature-off build still compiles + clippies,
   - verify `--version` feature-list cfg-gates match,
   - if it's opt-in, don't put it in `default`.
4. **Zero-spawn collectors.** Default-path collectors must not spawn processes
   on Linux (Phase 4.1). Prefer `/proc`, `/sys`, and libc calls; CLI fallbacks
   are for macOS or last-resort only.

## Development workflow

```sh
cargo build                         # full (default features)
cargo build -p flexfetch-cli --no-default-features   # minimal gate
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all
```

The CI gate is exactly that last trio, plus a feature-off job. Make it pass
locally before pushing.

## DCO sign-off

This project requires **Developer Certificate of Origin** sign-off on every
commit, enforced by a CI check. Sign off with:

```sh
git commit -s        # appends "Signed-off-by: You <you@example.com>"
```

By signing you certify that you have the right to contribute the code under
the project's MIT license (see <https://developercertificate.org>).

## Submitting a PR

- Use the PR template. Fill in the verification checklist — it's how we trust
  that non-trivial changes are safe.
- One logical change per PR. Prefer small PRs.
- If the PR closes an issue, reference it (`Closes #123`).
- Update `CHANGELOG.md` for feature work.

## Code review

Be patient and specific. Reviewers will push back on:

- new dependencies without a diet justification,
- un-gated features that should be opt-in,
- `Command::new` in a Linux default-path collector,
- dead code in feature-off builds (clippy `-D warnings` catches these).
