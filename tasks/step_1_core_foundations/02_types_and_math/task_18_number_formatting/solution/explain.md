# Task 18: Solution Breakdown & Alignment

## 1. Line-by-Line Breakdown
* `{:<16}` allocates 16 character spaces and flushes the text against the left edge.
* `{:>7.2}` right-aligns the number within 7 characters and formats it to 2 decimal places.
* Together with `$` prefix, this aligns the decimal points for these values (fields expand for longer values), `.` line up vertically, creating a clean terminal table!


## Precision note

This is a small arithmetic/formatting exercise. Binary floats approximate many decimal amounts; display formatting does not make the stored number exact. Real money calculations need a defined rounding policy and an appropriate representation, such as integer minor units.
