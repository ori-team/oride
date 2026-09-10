# Canonical Project Documentation (`docs/en/`)

🌐 **Languages:** **English** · [Português (`docs/`)](../README.md)

---

## What is this directory?
The `docs/en/` directory is the canonical English documentation suite for engineering specifications, architecture, product scope, development guidelines, operations, and governance of the **Oride** project.

## Purpose
It implements the **Living Canonical Documentation** principle: the repository is self-contained and all essential technical knowledge lives directly alongside the code in standardized Markdown files.

## User & Developer Quick Links
- 📖 [**User Guide**](../guides/en/user-guide.md) — Comprehensive guide on installation, editing paradigms (Standard & Vim Modal), window splits, keybindings, task runner, and diagnostics.
- ⚙️ [**Configuration Reference**](../guides/en/config.md) — Exhaustive guide for `config.toml` options and LSP configuration.
- 🎨 [**Theme Development Guide**](../guides/en/themes.md) — Declarative TOML theme specification and Live Preview.
- 🗂️ [**User Guides Portal**](../guides/README.md) — Central portal for English (`guides/en/`) and Portuguese (`guides/pt/`) manuals.
- 🏗️ [**Architecture & Design**](design.md) — Modular Rust design, crate boundaries, and technical stack decisions.
- 🔌 [**Plugin API Specification**](plugin-api.md) — External plugin protocols, stdin/stdout JSON, and extension mechanisms.

## Central Intent Router
See [`ATLAS.md`](ATLAS.md) as the intent-guided entrypoint for developers, users, operators, and AI agents.

## Subdirectory Taxonomy
- `architecture/`: System topology, boundaries, Clean Code contracts, and Architecture Decision Records (ADRs).
- `product/`: Product vision, target personas, value proposition, and bounded scope.
- `development/`: Coding standards, style guides, and exhaustive testing strategy (TDD/BDD).
- `operations/`: Release procedures, runbooks, deployments, and observability.
- `security/`: Threat modeling (STRIDE), security policies, and vulnerability contracts.
- `governance/`: Branching policies, pull request workflows, and compliance rules.
- `planning/`: Roadmaps, alpha release checklists, and UX polish plans.
