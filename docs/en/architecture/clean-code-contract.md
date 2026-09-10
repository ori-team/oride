# Architecture & Clean Code Contract

This document defines the **mandatory software engineering contract** for all implementations across this project.

## 1. Clean Code Principles

1. **Single Responsibility Principle (SRP)**: Every module, class, or crate has one unambiguous reason to change.
2. **High Cohesion & Loose Coupling**: Modules must be self-contained and interact exclusively through abstract interfaces or typed contracts.
3. **Domain-Expressive Naming**: Variables, functions, and types must reflect the ubiquitous domain language. Generic names like `manager`, `helper`, `utils`, or `data` are forbidden.
4. **Focused Functions**: Functions must perform a single logical action and ideally fit on a single viewing screen.
5. **Explicit Errors**: Silent exception suppression (`catch-all`, ignored errors) is strictly prohibited. Every error must be handled, wrapped, or propagated with context.
6. **Zero Speculative Abstraction (YAGNI)**: Implement abstractions only when two or more concrete, proven use cases exist.

## 2. Dependency Direction (Clean Architecture)

- Dependencies point strictly **inward**, towards essential core domain rules.
- External mechanisms (filesystems, subprocesses, CLI, terminal UI libraries) are infrastructure details encapsulated by adapters.
- The core of the application remains agnostic of presentation layers and external protocols.

## 3. Modularity & Decoupling

- No package or crate may import its own consumer.
- Dependency cycles are strictly forbidden and validated in CI pipelines.
- Every folder in the repository must be self-explanatory and contain a structured `README.md`.
