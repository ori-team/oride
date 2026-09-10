# Clean Code Principles

## 1. Naming Conventions
- **Intention-Revealing Names**: The name of a variable, function, or class, should answer all the big questions. It should tell you why it exists, what it does, and how it is used.
- **Avoid Disinformation**: Avoid leaving false clues that obscure the meaning of code.
- **Make Meaningful Distinctions**: Number-series naming (`a1`, `a2`, `.. aN`) is the opposite of intentional naming.

## 2. Functions
- **Small**: The first rule of functions is that they should be small. The second rule of functions is that they should be smaller than that.
- **Do One Thing**: Functions should do one thing. They should do it well. They should do it only.
- **One Level of Abstraction per Function**: We need to make sure that the statements within our function are all at the same level of abstraction.

## 3. Comments
- Comments do not make up for bad code.
- Explain yourself in code.
- Good comments: Legal comments, Informative comments, Explanation of intent, Clarification, Warning of consequences, TODO comments.
- Bad comments: Mumbling, Redundant comments, Misleading comments, Mandated comments, Journal comments, Noise comments.

## 4. Formatting
- **Vertical Formatting**: Small files are usually easier to understand than large files. Concepts that are closely related should be kept vertically close to each other.
- **Horizontal Formatting**: Lines should not be too long (typically < 120 characters).

## 5. Objects and Data Structures
- **Data Abstraction**: Hide implementation details.
- **Data/Object Anti-Symmetry**: Objects hide their data behind abstractions and expose functions that operate on that data. Data structure expose their data and have no meaningful functions.
- **The Law of Demeter**: A module should not know about the innards of the objects it manipulates.
