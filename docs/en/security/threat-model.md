# Threat Model (STRIDE)

| Threat Category | Potential Attack Vector | Mandatory Mitigation |
|---|---|---|
| **Spoofing** | Forged calls or fake payloads | Strict typing and cryptographic signature verification. |
| **Tampering** | Malicious modification of code or goals | Integrity checks via SHA-256 digests and branch protection rules. |
| **Repudiation** | Actions performed without audit trails | Immutable event logs, verifiable test evidence, and signed commits. |
| **Information Disclosure** | Accidental exposure of credentials or paths | Automated pre-commit scanning, sanitization, and secret redaction in logs. |
| **Denial of Service** | Unbounded memory consumption or hang loops | Bounded context limits (LPC), non-blocking I/O, explicit timeouts, and bounded rope capacities. |
| **Elevation of Privilege** | Subagent or subprocess container escapes | Strict sandbox execution policies, fail-closed boundaries, and explicit tool whitelists. |
