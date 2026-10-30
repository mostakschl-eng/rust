# Task 11: Solution Breakdown & Integer Sizing

## 1. Line-by-Line Breakdown
* `let freezer_temp: i8 = -18;`: Stored in a single 8-bit byte. Since temperature can drop below zero, a signed integer such as `i8` is appropriate.
* `let battery_percentage: u8 = 95;`: Battery ranges between 0 and 100. It can never be negative, so an unsigned byte `u8` (which holds 0 to 255) is the most memory-efficient choice possible!
* `let active_players: u32 = 1_250_000;`: Exceeds the 65,535 limit of `u16`, so `u32` (up to 4.2 billion) is the standard selection.
* `let world_population: i64 = 8_000_000_000;`: Exceeds 4.2 billion, so it requires a 64-bit container (`i64` or `u64`).

---

## 2. Memory Efficiency
Choosing small integer types when possible saves memory and cache space. Choose a type for the valid range first; total memory use also depends on data layout. Other languages also support fixed-width numbers.

