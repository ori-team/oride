# Contributing to Oride

**English** · [Português](CONTRIBUTING.pt-BR.md)

Thank you for your interest in contributing to **Oride**! Oride is a lightweight, modular, and extensible terminal code editor and mini-IDE written in Rust.

We welcome contributions of all kinds: bug fixes, performance optimizations, syntax support, documentation improvements, theme additions, and architecture enhancements.

---

## 🧭 Core Architectural Invariants

Before writing code, please review our fundamental invariants to ensure your pull request aligns with the project philosophy:

1. **Core Without UI:** `oride-core`, `oride-config`, and `oride-keymap` must remain 100% headless and testable without a physical TTY. Never import `ratatui` or `crossterm` into core crates. UI rendering logic belongs exclusively in `oride-ui` or `oride-app`.
2. **Actions as Data:** Keystrokes, chords, and command palette entries dispatch named, typed actions (`Action` enum) rather than mutating state directly in input handlers.
3. **Fail-Closed & Resilient:** Subsystems like LSP servers, PTY shells, and Git subprocesses must fail gracefully with descriptive status line warnings — never panic or crash the editor.
4. **LSP via PATH:** Language servers are spawned dynamically via executable binaries in `$PATH` (`rust-analyzer`, `clangd`, etc.), keeping the editor decoupled from compiler binaries.
5. **Anti-Bloat & Contained TUI:** All editor features operate inside the terminal and the Oride process. Avoid uncontained external dependencies, browser renderers, or heavy macro bloat.
6. **One Concept per PR:** Keep pull requests focused on a single change or feature slice, following the design phases in [`docs/en/design.md`](docs/en/design.md).

---

## 🛠️ Development Setup & Prerequisites

### Requirements
- **Rust Toolchain:** Stable 1.80+ (`rustup default stable`)
- **Cargo Components:** `rustfmt` and `clippy` (`rustup component add rustfmt clippy`)
- **Git**

### Building and Testing Locally

```bash
# Clone the repository
git clone https://github.com/ori-team/oride.git
cd oride

# Compile in debug mode
cargo build

# Run headless test demo
cargo run -p oride -- --demo

# Run the full workspace test suite
cargo test --workspace

# Launch the editor in current workspace
cargo run -p oride -- .
```

---

## 🔄 Development Workflow

### 1. Branch Naming
Create a dedicated branch from `main`:
- `feat/feature-name` — New features and enhancements
- `fix/bug-description` — Defect and bug fixes
- `docs/topic-name` — Documentation and guide updates
- `refactor/scope-name` — Code refactoring without behavior changes
- `chore/task-name` — Tooling, CI, and dependency updates

### 2. Test-Driven Development (TDD)
- When fixing a bug, first write a failing automated test reproducing the defect, then implement the fix to make it pass.
- When adding a feature, provide unit tests asserting contract compliance and edge cases.
- Tests must be deterministic, isolated, and fast.

### 3. Conventional Commits
All commits must follow the [Conventional Commits](https://www.conventionalcommits.org/) format:

```text
type(scope): imperative description

[optional body]

[optional footer]
```

**Examples:**
- `feat(modal): add vim navigation motions`
- `fix(lsp): parse multiline utf-16 text edits correctly`
- `docs(guides): add theme creation tutorial`
- `refactor(search): optimize ripgrep glob filter conversion`

---

## 🧪 Mandatory Quality Gates

Before opening a pull request, your code must pass all three validation gates locally without errors or warnings:

```bash
# 1. Strict formatting verification
cargo fmt --all -- --check

# 2. Strict linter verification (zero warnings allowed)
cargo clippy --all-targets -- -D warnings

# 3. Comprehensive test suite across all 15 workspace crates
cargo test --workspace
```

---

## 📚 Living Documentation & Bilingual Parity

Oride follows the **Living Canonical Documentation** discipline:

1. **`CHANGELOG.md`:** For any user-facing or technical change, add an entry under `## Unreleased` (`### Added`, `### Changed`, `### Fixed`, or `### Removed`).
2. **User Guides:** User-facing guides are organized under `docs/guides/`:
   - Portuguese: [`docs/guides/pt/`](docs/guides/pt/)
   - English: [`docs/guides/en/`](docs/guides/en/)
   If you modify a user guide or configuration parameter, keep both language versions in sync.
3. **Internal Specifications:** Architecture, testing, and security documents are maintained in [`docs/`](docs/) and mirrored in [`docs/en/`](docs/en/).

---

## 🚀 Submitting a Pull Request

1. Push your branch to your GitHub fork or origin:
   ```bash
   git push origin feat/your-feature
   ```
2. Open a Pull Request against the `main` branch.
3. Provide a clear description of the problem solved, design decisions made, and test coverage added.
4. Ensure all continuous integration (CI) workflows pass.
5. Address any code review feedback. Pull requests are merged using the `squash` strategy.

Thank you for helping make **Oride** better!
