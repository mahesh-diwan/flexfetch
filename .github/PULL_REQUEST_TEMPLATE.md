## What

<!-- 1-line description of the change -->

## How

<!-- Technical approach: which files/modules changed, feature gates touched -->

## Why

<!-- e.g. "Closes #123", "Follow-up to #456" -->

## Verification

<!-- How did you test? Paste the exact commands + a snippet of the output -->

- [ ] `cargo test --workspace` passes
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` passes
- [ ] `cargo fmt --all -- --check` passes
- [ ] Feature-off path builds: `cargo build -p flexfetch-cli --no-default-features`
- [ ] If a new opt-in feature was added, it builds + clippies with `--features <name>`
- [ ] `CHANGELOG.md` updated

## Notes for reviewers

<!-- Anything unusual: behavior changes, trade-offs, follow-up work -->
