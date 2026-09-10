# Observability & Telemetry

## Core Observability Pillars
1. **Structured Logs**: Status and debug logs emitted with UTC timestamps, severity levels, and operation contexts.
2. **Diagnostics Engine (`:health`)**: Built-in interactive diagnostics modal checking external CLI tool presence, LSP responsiveness, and terminal rendering capabilities.
3. **Fail-Closed Reporting**: Non-fatal operational warnings (e.g. LSP spawn failures or dirty Git conflicts) are reported non-destructively in the status line.
