# Lesson 40: Key-Value Stores: HashMaps (`HashMap<K, V>`)

In Arrays and Vectors, we accessed items using **numeric indices**: `0, 1, 2, ...`  
What if you want to look up an item by its **name** instead of a number?

For example:
* Looking up a contact's phone number by typing their **name** (`"Rahim"` -> `"01711-XXXXXX"`).
* Looking up the price of a fruit by its **label** (`"Apple"` -> `$2.50`).
* Looking up a country's capital city (`"Bangladesh"` -> `"Dhaka"`).

In Rust, this is done using a **`HashMap<K, V>`**! 📖

---

## 1. The Real-Life Analogy: The Phonebook or Dictionary 📔

Think of an old-fashioned physical telephone directory:
* You don't look up a person by saying "give me person number 472".
* You search for the **Key**: `"Smith, John"`.
* Next to that key, you find the **Value**: `"555-0199"`.

In Rust:
> A **`HashMap<Key, Value>`** stores pairs of data.  
> You provide a **Key** to quickly look up, insert, update, or delete its associated **Value**!

---

## 2. Bringing HashMap into Scope: `use std::collections::HashMap;`

Because not every simple Rust program needs a HashMap, it is not loaded into the default prelude.  
You must import it at the very top of your file:
```rust
use std::collections::HashMap;
```

---

## 3. Your First HashMap in Action

Let's build a fruit price lookup system:

```rust
use std::collections::HashMap;

fn main() {
    // 1. Create a new mutable HashMap:
    let mut fruit_prices = HashMap::new();

    // 2. Insert (Key, Value) pairs:
    fruit_prices.insert("Apple", 2.50);
    fruit_prices.insert("Banana", 1.20);
    fruit_prices.insert("Mango", 4.00);

    println!("Total fruit types: {}", fruit_prices.len());

    // 3. Look up a price using .get(&key):
    let query = "Mango";
    match fruit_prices.get(query) {
        Some(price) => println!("The price of {} is ${:.2}", query, price),
        None => println!("Sorry, we do not sell {}!", query),
    }

    // 4. Look up a fruit that does NOT exist:
    let missing = "Dragonfruit";
    match fruit_prices.get(missing) {
        Some(price) => println!("The price of {} is ${:.2}", missing, price),
        None => println!("Sorry, we do not sell {}! (Safely handled)", missing),
    }
}
```

### 📋 Expected Terminal Output:
```text
Total fruit types: 3
The price of Mango is $4.00
Sorry, we do not sell Dragonfruit! (Safely handled)
```

---

## 4. Updating Values in a HashMap

### Case A: Overwriting an Existing Value
If you call `.insert()` with a key that already exists, it simply overwrites the old value:
```rust
fruit_prices.insert("Apple", 3.00); // Apple price updated from 2.50 to 3.00!
```

### Case B: Only Insert If Key Doesn't Exist: `.entry().or_insert()`
What if you want to add an item **only if it's not already in the map**?  
Rust provides the powerful **Entry API**:

```rust
// Only insert Orange if it's missing; otherwise leave the existing price untouched:
fruit_prices.entry("Orange").or_insert(1.75);
```

---

## 5. Iterating Through a HashMap

You can loop through all key-value pairs using a `for` loop with tuple destructuring:

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Alice", 92);
    scores.insert("Bob", 85);
    scores.insert("Charlie", 97);

    println!("=== SCOREBOARD ===");
    for (player, score) in &scores {
        println!("{}: {} pts", player, score);
    }
}
```

> [!NOTE]
> HashMaps do **not** keep items in any guaranteed order! When you iterate, items may print in a random order. For a chosen order, explicitly sort collected entries before printing; an array or vector preserves the order you put into it.

---

## 6. Common HashMap Methods

| Method | Description | Example |
|---|---|---|
| `map.insert(k, v)` | Inserts or updates pair | `map.insert("ID", 101);` |
| `map.get(&k)` | Returns `Option<&V>` | `map.get(&"ID")` |
| `map.contains_key(&k)` | Returns `true` if key exists | `map.contains_key(&"ID")` |
| `map.remove(&k)` | Removes key and returns its value | `map.remove(&"ID");` |
| `map.len()` | Number of key-value pairs | `map.len()` |
| `map.entry(k).or_insert(v)` | Inserts default only if key is missing | `map.entry("A").or_insert(0);` |

---

## 7. Common Beginner Traps ⚠️

### Trap: Forgetting `&` in `.get()`
The `.get()` method expects a reference to the key:
```rust
let price = fruit_prices.get("Apple"); // "Apple" already has type &str
// If key is a String:
let key = String::from("Apple");
let price = fruit_prices.get(key.as_str()); // borrow String text as &str
```

---

## 8. Quick Recap Checklist
- [x] Import HashMap with `use std::collections::HashMap;`.
- [x] Store data in key-value pairs using `map.insert(key, value)`.
- [x] Retrieve values safely using `map.get(&key)`, which returns `Some(&value)` or `None`.
- [x] Use `.entry(key).or_insert(default)` to insert only when missing.
- [x] HashMaps do not preserve insertion order.

---

## Borrowed text keys

This example stores `&str` keys. A string literal such as `"Apple"` is already a reference, so `.get("Apple")` needs no extra `&`. For a numeric-key map you usually write `.get(&number)`. Inserting an existing key replaces its old value; it does not create a duplicate key.

## Symbol reading guide

| Symbol or syntax | Meaning |
|---|---|
| `HashMap<K, V>` | Map from keys to values |
| `.get(key)` | Optional borrowed value |
| `.entry(key).or_insert(v)` | Insert only when missing |
| `::` | Separate path components |

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/step_1_core_foundations/04_collections/task_40_hashmaps/TASK.md`](../../../tasks/step_1_core_foundations/04_collections/task_40_hashmaps/TASK.md).
* You've completed Level 4! Next up: [Level 5: Structs, Enums & Pattern Matching](../05_structs_and_enums/41_named_structs.md).

## Continue Your Study Tracker

After finishing this level's numbered tasks, build [checkpoint 4](../../../tasks/step_1_core_foundations/04_collections/project_04_session_collection/TASK.md). Bring forward your own previous checkpoint; keep its working version separate.
