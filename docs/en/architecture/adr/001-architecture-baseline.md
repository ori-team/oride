# ADR 001: Architectural Baseline and Clean Architecture Principles

## Status
Accepted

## Context
The project demands high maintainability, strict isolation of core editing rules from terminal frameworks and external dependencies, and support for automated unit and integration tests without heavy infrastructure dependencies.

## Decision
We adopt Clean Architecture (Ports and Adapters). All code dependencies must point towards the central domain. Interactions with external infrastructure must occur exclusively through port interfaces.

## Consequences
- **Positive**: Complete in-memory testability; easy driver substitution; independence from UI frameworks.
- **Negative**: Introduction of intermediate data mapping layers (DTOs and entities).
