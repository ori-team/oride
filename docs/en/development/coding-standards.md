# Coding & Engineering Standards

1. **Formatting & Style**:
   - All code must pass official language formatters (`cargo fmt`, `prettier`, etc.) without exception.
   - Line lengths should be maintained within 100 characters where reasonable.

2. **Types & Errors**:
   - Strict typing across all public function and method signatures.
   - Error returns must be explicit and provide contextual information regarding the failed operation.

3. **In-Code Documentation**:
   - Comments explain the *why*, never merely restating the *what*.
   - All public functions, structs, traits, and interfaces must include descriptive docstrings.
