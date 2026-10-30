# Task 42: Smartphone Hardware Configurator (Struct Update Syntax)

## 🎯 What is Your Task?
Write a Rust program to configure smartphone hardware models using the Struct Update Syntax (`..`):
1. Define a struct `DeviceSpec` with `#[derive(Debug)]`:
   - `model: String`
   - `storage_gb: u32`
   - `ram_gb: u32`
   - `has_5g: bool`
2. Create `phone_base`:
   - Model: `"Pixel Standard"`
   - Storage: `128`
   - RAM: `8`
   - 5G: `true`
3. Create `phone_pro` using the struct update syntax:
   - Provide custom model: `"Pixel Pro"`
   - Provide custom storage: `256`
   - Copy remaining fields (`ram_gb` and `has_5g`) from `phone_base` using `..phone_base`!
4. Print both specifications.

---

## 📋 Expected Output on Your Screen
```text
=== PHONE HARDWARE CONFIGURATOR ===
Base Model:
DeviceSpec { model: "Pixel Standard", storage_gb: 128, ram_gb: 8, has_5g: true }

Pro Model (Upgraded Storage & Model via ..):
DeviceSpec { model: "Pixel Pro", storage_gb: 256, ram_gb: 8, has_5g: true }
===================================
```

---

## 🛠️ How to Do This (Step-by-Step):

1. Open [`src/main.rs`](file:///d:/rust-learning/tasks/05_structs_and_enums/task_42_struct_instances/src/main.rs).
2. Define the struct with `#[derive(Debug)]`.
3. Instantiate `phone_base`.
4. Instantiate `phone_pro` specifying `model` and `storage_gb`, ending with `..phone_base`.
5. Test:
   ```bash
   cd tasks/05_structs_and_enums/task_42_struct_instances
   cargo run
   ```

---

## 🔒 Solution & Explanations
* Working Code: [`solution/solution.rs`](file:///d:/rust-learning/tasks/05_structs_and_enums/task_42_struct_instances/solution/solution.rs)
* Line-by-Line Breakdown: [`solution/explain.md`](file:///d:/rust-learning/tasks/05_structs_and_enums/task_42_struct_instances/solution/explain.md)

