# flexfetch Governance

## Model: BDFL with a core contributor team

flexfetch is a small, focused project. It is governed by a single project
maintainer (BDFL) who holds final decision authority on the roadmap and on
merge decisions, supported by a small core contributor team.

## Roles

### Maintainer (BDFL)

- Owns the project vision and the ROADMAP.
- Final say on roadmap priorities, feature gates, and dependency additions.
- Approves releases and tag pushes.
- May delegate decision authority per-area.

### Core contributors

- Long-term, trusted contributors with write access.
- Review PRs, triage issues, and maintain CI/package files.
- Propose roadmap changes for the maintainer's approval.

### Contributors

- Anyone submitting issues or PRs.
- No commit access required; contributions are welcomed via the standard
  PR flow (see `CONTRIBUTING.md`).

## Decision process

1. **Small changes** (bug fixes, docs, tests): reviewed and merged by any core
   contributor with write access.
2. **Feature work**: must reference a ROADMAP task or be justified against the
   roadmap. If it adds a dependency or a feature gate, expect scrutiny on the
   binary-size diet.
3. **Roadmap changes**: proposed as a ROADMAP.md diff in a PR; the maintainer
   approves or rejects with a reason recorded in the **Rejected / decisions**
   section (so the rationale is preserved).

## Releases

- Maintainer creates tags; CI builds/signs/uploads (see `SECURITY.md`).
- SemVer: breaking CLI/config changes → minor bump (pre-1.0: 0.x), features →
  patch/minor, fixes → patch.

## Conflicts

Disagreements that can't be resolved by discussion are escalated to the
maintainer, whose decision is final. The maintainer is expected to consult the
core team and the roadmap before deciding.

## Changes to this document

Amendments require a PR approved by the maintainer, recorded here for
transparency.
