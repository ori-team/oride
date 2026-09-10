---
name: lang-c
description: ISO C (C11/C17/C23) bounded buffers, explicit error checking, zero unbounded string APIs, cleanup attributes, and ASan/Valgrind memory verification.
---

# ISO C Bounded Systems & Safe Memory Contract

## 1. Core Principles & Standards
- Target **ISO C11, C17, or C23**.
- **Bounded Buffers**: Every buffer passed to a function MUST be accompanied by its explicit capacity/size parameter (`size_t cap`). Unbounded memory writes are non-negotiable failures.
- **Banned APIs**: `gets`, `strcpy`, `strcat`, `sprintf`, `vsprintf` are strictly prohibited. Use `snprintf`, `strlcpy`, or explicit bounds checking.
- **Explicit Error Returns**: All fallible functions must return an integer status code (`0` on success, negative on error) or a typed result struct. Never ignore return values.

## 2. Memory Lifecycle Discipline
- Every allocation via `malloc` / `calloc` must be immediately checked against `NULL`.
- Free all allocated pointers and set them to `NULL` (`ptr = NULL;`) to prevent Use-After-Free.
- Utilize `__attribute__((cleanup))` where compiler support allows for deterministic scope-based cleanup.

## 3. Verification Loop
1. **Compile**: `gcc -std=c17 -Wall -Wextra -Wpedantic -Wconversion -Werror -D_FORTIFY_SOURCE=2`
2. **Sanitize**: Compile and run tests with `-fsanitize=address,undefined`.
3. **Valgrind**: Run test binaries under Valgrind (`valgrind --leak-check=full --error-exitcode=1`).
