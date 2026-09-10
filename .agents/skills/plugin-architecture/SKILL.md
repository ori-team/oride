# Plugin Architecture

## Purpose
Build sandboxed extension points with versioned contracts and explicit permissions.

## Use when
Task or Goal requires plugin architecture capabilities.

## Do not use when
Not relevant to active Goal or out of project scope.

## Required context
- Active Goal and Task definition
- Relevant source modules and architecture docs

## Procedure
1. Load minimal relevant context.
2. Prefer declarative/sandboxed extension points first; version contracts and require explicit permissions.
3. Execute required verification gates.

## Decision rules
- Favor smallest sufficient changeset.
- Adhere strictly to project architecture.

## Evidence required
- Automated test runs, build outputs, or verification logs.

## Output contract
- Clean, tested implementation or findings report.

## Stop conditions
- Sufficient evidence collected or budget exhausted.

## Escalation rules
- Escalate to lead/human if locked Goal or security boundary is impacted.
