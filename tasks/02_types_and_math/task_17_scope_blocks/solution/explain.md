# Task 17: Solution Breakdown & Scope Boundaries

## 1. Line-by-Line Breakdown
* When execution enters `{`, a new local stack frame is created.
* `dungeon_chest` and the inner `gold` (600) exist only between lines 10 and 13.
* When the closing `}` is reached on line 14, the inner `gold` is deallocated.
* Line 16 prints the outer `gold`, which remained untouched at `100`. This proves how Rust guarantees data isolation inside scopes!
