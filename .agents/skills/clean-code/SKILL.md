---
name: clean-code
description: Specific naming conventions, cohesion metrics, coupling detection, function responsibility, duplication patterns, dead code, side effects, mutation, API ergonomics
---
# Clean Code Engineering

## 1. Naming Conventions
Naming is the primary means of communicating intent. Variables should be nouns describing content. Functions should be verbs describing action. Avoid abbreviations. Booleans should indicate true/false context (is_valid, has_children). Names must be searchable and pronounceable.

## 2. Cohesion Metrics
Cohesion measures how strongly related the responsibilities of a module are. Aim for high cohesion: a class should have one primary responsibility. Use LCOM (Lack of Cohesion of Methods) metrics to identify classes to split. All methods must operate on the class's state.

## 3. Coupling Detection
Coupling is the degree of interdependence between modules. Minimize coupling via dependency injection. Prefer interfaces over concrete implementations. Detect coupling by analyzing import statements, cross-module calls, and checking for tight temporal constraints between distinct services.

## 4. Function Responsibility
A function should do one thing, do it well, and do it only. Limit function size to fit on a single screen. Extract sub-tasks into separate, clearly named functions. Avoid mixing different levels of abstraction. The Single Responsibility Principle applies at the function level.

## 5. Duplication Patterns
Identify and eliminate duplicated code using the Rule of Three. Extract common logic into helper functions or base classes. Use Strategy or Template Method patterns to abstract varying behaviors. Be careful not to prematurely abstract coincidental duplication that isn't true behavioral duplication.

## 6. Dead Code Elimination
Regularly audit and remove code that is no longer executed. Use coverage tools and static analysis to identify untouched code paths. Remove deprecated features once their grace period expires. Do not comment out code; rely on version control history.

## 7. Side Effects Management
Minimize and isolate side effects to improve predictability. Favor pure functions that rely only on inputs and return outputs. Explicitly document functions performing I/O. Group side-effecting code at the boundaries of the application (e.g., Hexagonal Architecture).

## 8. State Mutation
Control how and when state changes occur. Prefer immutable data structures where possible. Restrict mutability to small, clearly defined scopes. Use explicit synchronization mechanisms if mutation is necessary in concurrent environments. Avoid global mutable state entirely.

## 9. API Ergonomics
Design APIs that are easy to use correctly and hard to use incorrectly. Provide sensible defaults. Use strong typing and domain types rather than primitive obsession. Ensure consistent naming and behavior across the entire API surface. Follow the Principle of Least Astonishment.

## 10. Code Smells Identification
Train yourself to spot common code smells: Long Method, Large Class, Primitive Obsession, Feature Envy, Data Clumps, Switch Statements, and Temporary Field. Addressing these early prevents systemic rot and keeps the codebase malleable over its lifetime.

