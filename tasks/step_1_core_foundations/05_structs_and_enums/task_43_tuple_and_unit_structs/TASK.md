# Task 43: Graphic Canvas & Security Token (Tuple & Unit Structs)

## 🎯 What is Your Task?
Write a Rust program to implement strong type-safe graphics primitives and a security marker:
1. Define a tuple struct `Color(u8, u8, u8)`.
2. Define a tuple struct `Point3D(i32, i32, i32)`.
3. Define a unit-like struct `SecurityKey`.
4. In `main()`:
   - Create `sky_blue = Color(135, 206, 235)`.
   - Create `cursor = Point3D(100, 250, -50)`.
   - Create `token = SecurityKey`.
   - Access and print RGB channels using `.0`, `.1`, `.2`.
   - Unpack/destructure `cursor` using `let Point3D(x, y, z) = cursor;` and print coordinates.
   - Print token validation message.

---

## 📋 Expected Output on Your Screen
```text
=== GRAPHICS CANVAS & TOKEN SYSTEM ===
Sky Blue Color:
  Red:   135
  Green: 206
  Blue:  235

3D Cursor Position:
  Coordinates: X=100, Y=250, Z=-50

Security Token:
  SecurityKey validated! (0-byte marker)
======================================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/05_structs_and_enums/task_43_tuple_and_unit_structs/src/main.rs).
2. Declare tuple structs with `(Type, Type, ...);` ending with `;`.
3. Declare `struct SecurityKey;`.
4. Test:
   ```bash
   cd tasks/05_structs_and_enums/task_43_tuple_and_unit_structs
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/05_structs_and_enums/task_43_tuple_and_unit_structs/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/05_structs_and_enums/task_43_tuple_and_unit_structs/solution/explain.md)

