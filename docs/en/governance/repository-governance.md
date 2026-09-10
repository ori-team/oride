# Repository Governance & Workflow Rules

1. **Branches**:
   - `main`: Protected trunk branch. Direct pushes should be avoided in collaborative setups; changes integrate via Pull Requests.
   - Branch naming conventions: `feat/*`, `fix/*`, `chore/*`, `docs/*`, `refactor/*`.

2. **Commits**:
   - Conventional Commits standard is mandatory: `type(scope): imperative description`.
   - Examples: `feat(modal): add vim navigation motions`, `docs: add english documentation suite`.

3. **Pull Requests & Merge Gating**:
   - Merge strategy: `squash` merge with branch deletion upon successful integration.
   - All CI quality gates (`cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test --workspace`) must pass 100%.
