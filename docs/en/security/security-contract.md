# Strict Security Contract

This document establishes the **inviolable security requirements** that must be fulfilled by every contributor (human or AI agent) before any code is merged:

1. **Zero Tolerance for Secrets**:
   - No private keys, API tokens, cloud credentials, or certificates may be committed, even temporarily.
   - Any secret leak triggers immediate commit rejection and mandatory credential rotation.

2. **Isolation & Least Privilege**:
   - Subagents and helper processes operate strictly with tools declared in their harness profiles.
   - Subprocess invocations must escape arguments and avoid unescaped shell interpolation.

3. **Boundary Validation**:
   - All input received across boundaries (files, environment variables, LSP JSON-RPC, external plugins) is treated as untrusted.
   - Input sanitization and schema verification occur at the boundary layer before reaching domain logic.

4. **Supply Chain Integrity**:
   - External dependencies must be audited against known vulnerability databases (CVEs).
   - Immediate rejection of packages with critical advisories or conflicting licenses.
