# Lesson 57: The Golden Borrowing Rule: Many Readers OR One Writer

In Lesson 55 and 56, we saw two types of borrowing:
1. **Shared References (`&T`)**: Read-only, safe to share with everyone.
2. **Mutable References (`&mut T`)**: Exclusive, allows writing, limited to one.

Now, what happens if you try to combine both at the exact same time?  
Can you have **one writer AND one reader** looking at the same memory simultaneously?

The answer is **NO**!  
Today we learn the sacred **Golden Borrowing Rule** of Rust! ⚖️

---

## 1. The Golden Borrowing Rule

> ### 👑 At any given time, you can have:
> ### EITHER:
> ### • Any number of immutable references (`&T`) (Many Readers)
> ### OR:
> ### • Exactly ONE mutable reference (`&mut T`) (One Writer)
> ### BUT NEVER BOTH AT THE SAME TIME!

---

## 2. The Real-Life Analogy: The Restaurant Blackboard Menu 📋

Imagine a restaurant with a giant chalk blackboard listing today's specials:
* **Scenario A (Many Readers):**
  * 20 customers can stand in line and read the blackboard simultaneously.
  * No problem! Everyone sees the same text.
* **Scenario B (One Writer):**
  * The chef walks up with an eraser and chalk to erase "Salmon" and write "Steak".
  * Can customers try to read the word "Salmon" **while the chef is actively wiping the chalk off the board**?
  * No! If someone tries to read half-erased chalk, they will read corrupted garbage or order something that no longer exists!

Therefore:
While the chef is holding the eraser (`&mut`), **nobody else is allowed to read (`&`)**.  
Once the chef finishes writing and steps away, customers can resume reading!

---

## 3. The Classic Compiler Error in Code

Look at what happens when you try to mix a reader and a writer:

```rust
fn main() {
    let mut data = String::from("Rust");

    let reader1 = &data;     // Immutable borrow starts here
    let reader2 = &data;     // Another immutable borrow: totally fine!

    let writer = &mut data;  // ❌ Mutable borrow requested while readers are still active!

    println!("Readers: {}, {}", reader1, reader2);
    println!("Writer:  {}", writer);
}
```

If you compile this, Rust halts with an exact explanation:
```text
error[E0502]: cannot borrow `data` as mutable because it is also borrowed as immutable
  --> src/main.rs:7:18
   |
4  |     let reader1 = &data;
   |                   ----- immutable borrow occurs here
...
7  |     let writer = &mut data;
   |                  ^^^^^^^^^ mutable borrow occurs here
8  |
9  |     println!("Readers: {}, {}", reader1, reader2);
   |                                 ------- immutable borrow later used here
```

Look how helpful the compiler is: it explains that `writer` cannot touch `data` because `reader1` is still expecting `data` not to change underneath its feet!

---

## 4. How to Fix It: Sequential Lifecycles!

The restriction is only about having them **at the same time**.  
If the readers finish their job first, you can happily create a mutable reference afterwards:

```rust
fn main() {
    let mut data = String::from("Rust");

    // 1. Readers do their job:
    let r1 = &data;
    let r2 = &data;
    println!("Readers see: {}, {}", r1, r2);
    // r1 and r2 are never used after this line! Their borrow is finished!

    // 2. Now the writer takes over safely:
    let writer = &mut data;
    writer.push_str(" 2024");
    println!("Writer updated to: {}", writer);
}
```

### 📋 Expected Terminal Output:
```text
Readers see: Rust, Rust
Writer updated to: Rust 2024
```

Because `r1` and `r2` were never used after line 6, Rust's smart compiler knows their borrow has ended. The writer can step in with zero conflict!

---

## 5. Summary Matrix: What is Allowed?

| Reference Mix | Allowed in Rust? | Reason |
|---|:---:|---|
| Multiple `&T` | ✅ **YES** | Shared access; ordinary data cannot be mutated through `&T` |
| Single `&mut T` | ✅ **YES** | Exclusive; complete safe write control |
| `&T` + `&mut T` simultaneously | ❌ **FORBIDDEN** | Reader might observe corrupted / mutating data |
| Multiple `&mut T` simultaneously | ❌ **FORBIDDEN** | Conflicting exclusive access is forbidden, even within one thread |

---

## 6. Quick Recap Checklist
- [x] You can have unlimited readers (`&T`) OR exactly one writer (`&mut T`).
- [x] You can never mix readers and writers at the exact same moment.
- [x] Shared references prevent ordinary mutation; interior-mutability types permit controlled mutation through their APIs.
- [x] Once readers finish their last read, writers can step in safely.

---


## Symbols and Practical Boundaries

A blackboard has an inspection period, an editing period, then a final inspection. Finish each access before starting the conflicting one.

| Symbol or term | Meaning |
|---|---|
| `&T` | shared access |
| `&mut T` | exclusive access |
| `last use` | helps determine borrow end |

**Common pitfall:** The rule concerns overlapping active access to the same data, not one reference per whole function. Disjoint fields may be borrowed independently.

## 🎯 Next Steps
* Ready to practice? Complete [`tasks/step_1_core_foundations/06_ownership_and_memory/task_57_borrowing_rules/TASK.md`](../../../tasks/step_1_core_foundations/06_ownership_and_memory/task_57_borrowing_rules/TASK.md).
* Next lesson: [Lesson 58: Non-Lexical Lifetimes (NLL) & The Borrow Checker](58_borrow_checker_nll.md).
