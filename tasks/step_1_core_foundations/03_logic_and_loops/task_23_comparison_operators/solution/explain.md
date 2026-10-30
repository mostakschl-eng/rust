# Task 23: Solution Breakdown & Equality Comparisons

## 1. Line-by-Line Breakdown
* `entered_pin == correct_pin`: Compares two `&str` text slices byte-by-byte. If all characters match identically, it returns `true`.
* `attempt_count <= max_attempts`: Checks whether the integer `1` is less than or equal to `3`.
* In Rust, comparing strings with `==` works naturally out of the box (unlike Java, where `==` compares memory pointers instead of string contents).


`within_limit` answers whether this attempt number is at most the maximum; `not_at_limit` answers a different question using `!=`. For example, attempt 4 is not equal to 3 but is outside the limit. These flags are displayed independently in this comparison exercise. A real access check would need to enforce both identity and allowed attempts.
