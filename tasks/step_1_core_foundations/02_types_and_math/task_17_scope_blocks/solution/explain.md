# Task 17: Solution Breakdown & Scope Boundaries

## 1. Line-by-Line Breakdown
* Entering `{` creates a lexical scope; it does not necessarily allocate a new stack frame.
* `dungeon_chest` and the inner `gold` (600) are usable only inside the inner block.
* After the inner block ends, its local names cannot be used from the outer block. No heap allocation is involved in these integer values.
* The final gold message prints the outer `gold`, which remained untouched at `100`. This proves how Rust guarantees data isolation inside scopes!

