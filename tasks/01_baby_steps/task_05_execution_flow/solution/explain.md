# Task 05: Solution Breakdown & Call Stack Concepts

## 1. Line-by-Line Breakdown
1. `main()` runs first and prints `"Mission Control: Initiating Launch Protocol"`.
2. `main()` calls `launch()`.
3. Inside `launch()`, it prints the pre-flight checks banner.
4. Next, `launch()` calls `check_fuel()`. Rust pauses `launch()`, enters `check_fuel()`, prints the fuel message, and returns.
5. Then, `launch()` calls `check_engines()`. Rust runs it and returns.
6. Finally, `launch()` prints the BLASTOFF message and returns to `main()`.
7. `main()` reaches its closing `}` and the program cleanly exits.

---

## 2. Key Takeaway
This is called the **Call Stack**:
```text
main()
  └── calls launch()
        ├── calls check_fuel() (runs & returns)
        └── calls check_engines() (runs & returns)
```
Each function does its focused job, making your entire application modular and bug-free!
