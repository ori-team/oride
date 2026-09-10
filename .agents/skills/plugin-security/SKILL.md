# Plugin & Extension Security

## Purpose
Enforce capability-based plugin isolation, memory safety boundaries, and manifest permission checks.

## Use when
- Active Goal or Task explicitly requires plugin & extension security operations.
- Operating in mode(s): `implementation`, `review`.

## Do not use when
- Task is out of scope or unrelated to plugin & extension security.
- Bounded token budget or capability policy denies required operations.

## Required context
- Security policy
- Target codebase
- Threat model

## Procedure
1. Establish trust boundaries and identify all untrusted input vectors relevant to plugin & extension security.
2. Audit source code and configuration against standard security baselines.
3. Enforce principle of least privilege and strict input sanitization at system boundaries.
4. Verify absence of vulnerabilities using automated and manual security test cases.
5. Generate structured security gate evidence and audit report.

## Decision rules
- Never trust untrusted input; validate strictly at the external boundary.
- Fail securely: system failures must not default to open access.
- Treat secrets and credentials as confidential; never log or persist in plain text.

## Evidence required
- security-scan
- test

## Output contract
- Security audit report
- Vulnerability remediation patches
- Security gate evidence

## Stop conditions
- Task acceptance criteria satisfied with evidence
- Token budget exhausted
- Blocked on external dependency

## Escalation rules
- Escalate to lead architect or human if locked Goal criteria cannot be met.
- Escalate immediately upon discovering unexpected security vulnerabilities or data loss risks.
