# UX Polish & Neurodiversity Accessibility Plan

**Status:** Delivered across Tier S, Tier A, and Tier B MVPs (**0.2.0**).  
**Core Usability Principle:** Discoverability > Memorization; Single clear focus; Esc always exits.

---

## Product Decisions

| Item | Decision | Status |
|---|---|---|
| **Mouse Support** | Opt-in via `mouse = true` or View menu | Delivered |
| **Top Menu Bar** | File, Edit, View, Go, Git, Help with mnemonic navigation | Delivered |
| **SCM Panel** | Stage/unstage, commit, ahead/behind, diff inspector | Delivered |
| **Keystroke Macros** | Explicitly removed to preserve anti-bloat principles | Removed |
| **External HTML Preview** | Out of scope | Excluded |
| **Interactive Terminal** | Integrated PTY with seamless shell control | Delivered |

---

## Tier S Deliveries (High Usability)

| ID | Feature | Implementation Details |
|---|---|---|
| **U0** | Usable Terminal | PTY interactive shell; `Ctrl+C/D` passed to shell; clear focus states |
| **U1** | High Contrast Context Banner | Visual banner indicating active focus (`EDITOR \| TREE \| TERM \| SCM`) |
| **U1b** | Clean Status Line | Stable status information (file · line/col · branch); ephemeral messages |
| **U2** | Menu Bar | Full dropdown menu bar with right-aligned shortcut indicators |
| **U2b** | Which-Key Guide | Dynamic helper screen listing available chord bindings (`Alt+/`) |
| **U2c** | Discoverable Palette | Direct access via `Ctrl+Shift+P` and `F1` |
| **U4** | Compact Search Modal | Centered, non-intrusive mini-modal for buffer find and replace |

---

## Tier A Deliveries (Workflow Polish)

| ID | Feature | Implementation Details |
|---|---|---|
| **U5** | Welcome Overlay | Initial helper card showing essential shortcuts |
| **U6** | Dedicated SCM Panel | Right-aligned panel for dirty file tracking (`Ctrl+Shift+G`) |
| **U7a** | Buffer Picker | Fuzzy buffer selector across open tabs (`Ctrl+Shift+O`) |
| **U7b** | Jump List | Recent cursor navigation history (`Ctrl+Alt+O` / `Ctrl+Alt+I`) |
| **U7c** | Status Line Git Blame | Active line author and commit hash display |
| **U7d** | Match Counters | Find match tally indicator (`Match N of M`) |
| **U7e** | Read-Only Git Diff | View staged/unstaged changes via `F2` |
