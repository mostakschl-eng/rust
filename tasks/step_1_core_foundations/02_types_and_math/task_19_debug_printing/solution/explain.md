# Task 19: Solution Breakdown & Debug Diagnostics

## 1. Line-by-Line Breakdown
* Normal Display `{}` fails on tuples because a tuple is a compound type that doesn't have an agreed-upon human format.
* `{:?}` invokes the `Debug` trait, which formats the data with parentheses `("Flight 707", 32.7767, ...)` and quotes around text.
* `{:#?}` spreads each element across its own line with 4-space indentation.
* `dbg!(airspeed_knots)` prints stderr output showing a diagnostic such as `[YOUR_FILE:LINE:COLUMN] airspeed_knots = 505`, letting you inspect variables in the middle of calculations without writing format strings!

