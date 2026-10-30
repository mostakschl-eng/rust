# Solution Explanation: Task 31 (Tuples)

Let's break down how tuples store multiple data types into a single package!

---

## 🔍 The Solution Code

```rust
fn main() {
    println!("=== AIR CARGO SHIPMENT LABEL ===");

    let parcel = (10482, "Tokyo", 4.75, false);

    println!("Tracking ID:  #{}", parcel.0);
    println!("Destination:  {}", parcel.1);
    println!("Weight:       {} kg", parcel.2);
    println!("Delivered?    {}", parcel.3);
    println!("Raw Parcel:   {:?}", parcel);

    println!("================================");
}
```

---

## 🧩 Line-by-Line Breakdown

### 1. Creating the Tuple:
```rust
let parcel = (10482, "Tokyo", 4.75, false);
```
* Notice the parentheses `( )`.
* Rust infers the type of `parcel` as `(i32, &str, f64, bool)`.
* It bundles an integer, string slice, floating-point number, and boolean into a single variable on the stack.

### 2. Accessing Fields by Index:
```rust
parcel.0 // 10482 (First item)
parcel.1 // "Tokyo" (Second item)
parcel.2 // 4.75 (Third item)
parcel.3 // false (Fourth item)
```
* Like array indices, tuple indices always start at `0`.
* We use a period/dot `.` directly before the index number.

### 3. Debug Printing:
```rust
println!("Raw Parcel:   {:?}", parcel);
```
* Because tuples don't implement the standard `Display` trait `{}` by default, we use `{:?}` (Debug) to view the entire tuple at once.

