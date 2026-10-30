# Master Rust Zero-to-Hero Roadmap (80 Lessons: Child to Adult)

Welcome to the **complete, definitive Rust learning curriculum**. This roadmap takes you from an absolute beginner who has never written code ("child level") all the way to building professional, fast, multi-threaded software ("adult developer level").

Every lesson has:
1. **Plain-English Explanation** with real-world analogies.
2. **Hands-On Code Practice** in your terminal.
3. **Anti-Spoiler Solution** in a separate folder if you get stuck.

Check off `[x]` as you complete each lesson!

---

## 🟢 Level 1: Absolute Baby Steps (Lessons 01 – 10)
*How computers follow instructions, printing text, reusable functions, and variable boxes.*

- [ ] **Lesson 01:** [What is Code & Instructions](lessons/01_baby_steps/01_what_is_code.md) — *Understanding how computers follow instructions, fn main(), and curly brackets {}*
- [ ] **Lesson 02:** [Speaking to the Screen (println! and semicolon)](lessons/01_baby_steps/02_speaking_to_screen.md) — *Displaying text with println!, quotes, and semicolons ;*
- [ ] **Lesson 03:** [Multi-line Printing & Comments](lessons/01_baby_steps/03_printing_and_comments.md) — *Writing comments with // and printing multiple formatted lines*
- [ ] **Lesson 04:** [What is a Function? (Reusable Recipes)](lessons/01_baby_steps/04_what_is_a_function.md) — *Why functions exist and how to define your own reusable recipes*
- [ ] **Lesson 05:** [Calling Functions & Execution Flow](lessons/01_baby_steps/05_calling_functions.md) — *How Rust executes code line by line from top to bottom*
- [ ] **Lesson 06:** [Function Parameters (Giving Inputs)](lessons/01_baby_steps/06_function_parameters.md) — *Passing arguments and ingredients into functions*
- [ ] **Lesson 07:** [Function Return Values (Getting Answers Back)](lessons/01_baby_steps/07_return_values.md) — *Returning computed values from functions using ->*
- [ ] **Lesson 08:** [Variables & Labeled Memory Boxes](lessons/01_baby_steps/08_variables_let.md) — *Storing data in memory with let and giving names to values*
- [ ] **Lesson 09:** [Mutability (mut - Locked vs Open Boxes)](lessons/01_baby_steps/09_mutability_mut.md) — *Why Rust variables are locked by default and how mut unlocks them*
- [ ] **Lesson 10:** [Variable Shadowing & Constants (const)](lessons/01_baby_steps/10_shadowing_and_constants.md) — *Re-declaring variables with let vs permanently fixed constants*

**⭐ Milestone 1: Personal Profile Terminal Card CLI**

---

## 🟡 Level 2: Basic Types & Arithmetic (Lessons 11 – 20)
*Numbers, fractions, true/false, math calculations, and reading compiler messages.*

- [ ] **Lesson 11:** [Whole Numbers (Integers: i32, u32, bit sizes)](lessons/02_types_and_math/11_integers.md) — *Understanding integer sizes (i8, i16, i32, i64, u8) and positive vs signed numbers*
- [ ] **Lesson 12:** [Decimal Numbers (Floating Points: f32, f64)](lessons/02_types_and_math/12_floats.md) — *Working with fractions, money, and scientific decimals*
- [ ] **Lesson 13:** [True or False (Booleans: bool)](lessons/02_types_and_math/13_booleans.md) — *Making binary choices with true and false*
- [ ] **Lesson 14:** [Single Letters & Emojis (Characters: char)](lessons/02_types_and_math/14_characters.md) — *Unicode characters, single quotes, and emojis in Rust*
- [ ] **Lesson 15:** [Math in Rust (Operators: +, -, *, /, %)](lessons/02_types_and_math/15_math_operators.md) — *Addition, subtraction, multiplication, integer division, and modulus remainder*
- [ ] **Lesson 16:** [Type Casting & Explicit Conversion (as i32, as f64)](lessons/02_types_and_math/16_type_casting.md) — *Converting numbers between integer and decimal safely*
- [ ] **Lesson 17:** [Scope & Code Blocks ({ let x = 5; })](lessons/02_types_and_math/17_scope_and_blocks.md) — *Where variables live and die inside curly braces {}*
- [ ] **Lesson 18:** [Formatted Output & Placeholders ({}, {:.2})](lessons/02_types_and_math/18_formatting_placeholders.md) — *Controlling decimal precision and multi-variable printing*
- [ ] **Lesson 19:** [Debug Printing ({:?} and {:#?})](lessons/02_types_and_math/19_debug_printing.md) — *Printing complex data on screen for debugging*
- [ ] **Lesson 20:** [Understanding Beginner Compiler Errors](lessons/02_types_and_math/20_compiler_errors_decoded.md) — *Reading Rust compiler hints like a friendly teacher*

**⭐ Milestone 2: Interactive Tip & Price Calculator**

---

## 🟠 Level 3: Logic, Choices & Loops (Lessons 21 – 30)
*Teaching the computer how to make smart choices and repeat work automatically.*

- [ ] **Lesson 21:** [Making Decisions: The if Statement](lessons/03_logic_and_loops/21_if_statement.md) — *Teaching your code to take different paths based on conditions*
- [ ] **Lesson 22:** [Alternative Paths: else and else if](lessons/03_logic_and_loops/22_else_and_else_if.md) — *Handling fallback paths and multiple decision branches*
- [ ] **Lesson 23:** [Comparison Operators (==, !=, <, >, <=, >=)](lessons/03_logic_and_loops/23_comparison_operators.md) — *Comparing values in code*
- [ ] **Lesson 24:** [Logical Operators (&&, ||, !)](lessons/03_logic_and_loops/24_logical_operators.md) — *Combining multiple conditions with AND, OR, and NOT*
- [ ] **Lesson 25:** [If as an Expression (let status = if ...)](lessons/03_logic_and_loops/25_if_expressions.md) — *Assigning variables directly from an if-else decision*
- [ ] **Lesson 26:** [Infinite Loops with loop and break](lessons/03_logic_and_loops/26_loop_and_break.md) — *Running instructions repeatedly and breaking out*
- [ ] **Lesson 27:** [Returning Values from Loops (break value;)](lessons/03_logic_and_loops/27_break_with_value.md) — *Computing and retrieving a result from inside a loop*
- [ ] **Lesson 28:** [Conditional Repetition: while Loops](lessons/03_logic_and_loops/28_while_loops.md) — *Looping as long as a condition remains true*
- [ ] **Lesson 29:** [Counting & Ranges: for Loops (1..=10)](lessons/03_logic_and_loops/29_for_loops_and_ranges.md) — *Iterating cleanly through number ranges*
- [ ] **Lesson 30:** [Loop Labels & Controlling Nested Loops](lessons/03_logic_and_loops/30_loop_labels.md) — *Breaking out of outer loops with named labels*

**⭐ Milestone 3: Secret Number Guessing Game**

---

## 🔵 Level 4: Compound Types & Collections (Lessons 31 – 40)
*Grouping related data: lists, shopping carts, vectors, and string slices.*

- [ ] **Lesson 31:** [Grouping Fixed Data: Tuples](lessons/04_collections/31_tuples.md) — *Grouping multiple data types into a single tuple (x, y, z)*
- [ ] **Lesson 32:** [Destructuring Tuples (let (x, y) = point)](lessons/04_collections/32_tuple_destructuring.md) — *Unpacking tuple values into separate variables*
- [ ] **Lesson 33:** [Fixed-Size Lists: Arrays ([i32; 5])](lessons/04_collections/33_arrays.md) — *Storing multiple items of the same type in fixed-size arrays*
- [ ] **Lesson 34:** [Accessing Array Elements & Index Bounds Checking](lessons/04_collections/34_array_indexing.md) — *Zero-based indexing and how Rust prevents buffer overflow crashes*
- [ ] **Lesson 35:** [Dynamic Lists: Vectors (Vec<T>, vec![])](lessons/04_collections/35_vectors.md) — *Lists that can grow and shrink in memory*
- [ ] **Lesson 36:** [Modifying Vectors (push, pop, len)](lessons/04_collections/36_vector_operations.md) — *Adding, removing, and inspecting vector elements*
- [ ] **Lesson 37:** [Iterating over Vectors with for loops](lessons/04_collections/37_vector_iteration.md) — *Looping through collections item by item*
- [ ] **Lesson 38:** [Text in Memory: String vs String Slices (&str)](lessons/04_collections/38_string_vs_str.md) — *The difference between growable String and view-only &str*
- [ ] **Lesson 39:** [String Methods (len, contains, replace, push_str)](lessons/04_collections/39_string_methods.md) — *Manipulating text safely in Rust*
- [ ] **Lesson 40:** [Key-Value Stores: HashMaps (HashMap<K, V>)](lessons/04_collections/40_hashmaps.md) — *Looking up values by names and keys like a dictionary*

**⭐ Milestone 4: Shopping Cart & Inventory Tracker**

---

## 🟣 Level 5: Structs, Enums & Pattern Matching (Lessons 41 – 50)
*Designing custom data types and exhaustive pattern matching.*

- [ ] **Lesson 41:** [Custom Types: Named Structs (struct User)](lessons/05_structs_and_enums/41_named_structs.md) — *Creating your own custom blueprint with named fields*
- [ ] **Lesson 42:** [Creating & Updating Struct Instances (.. syntax)](lessons/05_structs_and_enums/42_struct_instances.md) — *Creating new struct objects and copying fields with ..*
- [ ] **Lesson 43:** [Tuple Structs & Unit-like Structs](lessons/05_structs_and_enums/43_tuple_and_unit_structs.md) — *Lightweight structs without named fields*
- [ ] **Lesson 44:** [Methods: Attaching Functions to Structs (&self)](lessons/05_structs_and_enums/44_struct_methods.md) — *Giving behaviors and abilities to your structs using impl*
- [ ] **Lesson 45:** [Associated Functions & Constructors (Self::new)](lessons/05_structs_and_enums/45_associated_functions.md) — *Functions attached to types without needing an instance*
- [ ] **Lesson 46:** [Enums: Representing Fixed Choices (enum Direction)](lessons/05_structs_and_enums/46_enums_basics.md) — *Defining types that can be one of several possibilities*
- [ ] **Lesson 47:** [Enums Holding Data (enum Message)](lessons/05_structs_and_enums/47_enums_with_data.md) — *Attaching rich data directly inside enum variants*
- [ ] **Lesson 48:** [The match Expression: Exhaustive Matching](lessons/05_structs_and_enums/48_match_exhaustive.md) — *Handling every single possible enum state safely*
- [ ] **Lesson 49:** [Match Guards (Adding if inside match arms)](lessons/05_structs_and_enums/49_match_guards.md) — *Adding extra condition checks inside pattern matching*
- [ ] **Lesson 50:** [Concise Matching: if let and let else](lessons/05_structs_and_enums/50_if_let_and_let_else.md) — *Shortcuts for handling single enum variants cleanly*

**⭐ Milestone 5: Student Gradebook & Report Card System**

---

## 🔴 Level 6: Rust's Sacred Core — Ownership & Memory (Lessons 51 – 60)
*How Rust manages computer memory with zero garbage collection and zero crashes.*

- [ ] **Lesson 51:** [Computer Memory Demystified: Stack vs Heap](lessons/06_ownership_and_memory/51_stack_vs_heap.md) — *Fast stack memory vs flexible heap memory*
- [ ] **Lesson 52:** [The Three Sacred Rules of Rust Ownership](lessons/06_ownership_and_memory/52_three_ownership_rules.md) — *The golden principles that make Rust memory-safe without a garbage collector*
- [ ] **Lesson 53:** [Variable Moves vs Primitive Copies (Move Semantics)](lessons/06_ownership_and_memory/53_move_vs_copy.md) — *Why passing a String moves ownership while integers copy*
- [ ] **Lesson 54:** [Deep Copying with .clone()](lessons/06_ownership_and_memory/54_clone_deep_copy.md) — *Making an explicit duplicate of heap data*
- [ ] **Lesson 55:** [Borrowing: Shared References (&T - Read Only)](lessons/06_ownership_and_memory/55_borrowing_shared.md) — *Looking at data without taking ownership using &*
- [ ] **Lesson 56:** [Mutable Borrowing: Exclusive References (&mut T)](lessons/06_ownership_and_memory/56_borrowing_mutable.md) — *Changing borrowed data safely with &mut*
- [ ] **Lesson 57:** [The Golden Borrowing Rule: Many Readers OR One Writer](lessons/06_ownership_and_memory/57_borrowing_rules.md) — *Preventing data races and pointer corruption*
- [ ] **Lesson 58:** [Non-Lexical Lifetimes (NLL) & The Borrow Checker](lessons/06_ownership_and_memory/58_borrow_checker_nll.md) — *How Rust knows when a reference is done being used*
- [ ] **Lesson 59:** [Slices: Borrowing Parts of Arrays and Strings](lessons/06_ownership_and_memory/59_slices_deep_dive.md) — *Referencing a contiguous sequence of items without copying*
- [ ] **Lesson 60:** [Introduction to Lifetimes ('a annotations)](lessons/06_ownership_and_memory/60_lifetimes_intro.md) — *Ensuring references never outlive the data they point to*

**⭐ Milestone 6: Safe Text Analyzer & Word Counter**

---

## 🟤 Level 7: Robustness, Error Handling & System I/O (Lessons 61 – 70)
*Handling errors cleanly, reading keyboard input, and saving data to disk.*

- [ ] **Lesson 61:** [The Billion-Dollar Mistake: Why Rust Has No NULL](lessons/07_errors_and_io/61_no_null_in_rust.md) — *How Rust protects you from crashes caused by missing values*
- [ ] **Lesson 62:** [The Option<T> Enum (Some vs None)](lessons/07_errors_and_io/62_option_enum.md) — *Working with values that might or might not exist*
- [ ] **Lesson 63:** [Unwrapping Options Safely (unwrap_or, match)](lessons/07_errors_and_io/63_safe_option_unwrapping.md) — *Extracting values from Option without crashing*
- [ ] **Lesson 64:** [Recoverable Errors: The Result<T, E> Enum](lessons/07_errors_and_io/64_result_enum.md) — *Handling functions that can succeed (Ok) or fail (Err)*
- [ ] **Lesson 65:** [Unrecoverable Errors: panic!](lessons/07_errors_and_io/65_panic_macro.md) — *When and why a program should stop immediately*
- [ ] **Lesson 66:** [Propagating Errors: The ? (Question Mark) Operator](lessons/07_errors_and_io/66_question_mark_operator.md) — *Passing errors up the call stack with zero boilerplate*
- [ ] **Lesson 67:** [Reading Keyboard Input from Terminal (std::io::stdin)](lessons/07_errors_and_io/67_terminal_user_input.md) — *Interacting with users by reading what they type*
- [ ] **Lesson 68:** [Reading Files from Hard Drive (std::fs::read_to_string)](lessons/07_errors_and_io/68_reading_files.md) — *Loading saved text files from disk*
- [ ] **Lesson 69:** [Writing Data to Files (std::fs::write)](lessons/07_errors_and_io/69_writing_files.md) — *Saving application data permanently to text files*
- [ ] **Lesson 70:** [Organizing Code with Modules (mod, pub, use)](lessons/07_errors_and_io/70_modules_and_packages.md) — *Splitting large programs cleanly across multiple files*

**⭐ Milestone 7: Persistent Terminal Todo App (Saves to Disk)**

---

## 🏆 Level 8: Advanced Mastery & Concurrency (Lessons 71 – 80)
*Generics, Traits, Closures, Iterators, Multithreading, and Channels.*

- [ ] **Lesson 71:** [Generics: Writing Code for Any Type (fn print<T>)](lessons/08_advanced_and_concurrency/71_generics_functions.md) — *Writing flexible code that works with integers, floats, or custom types*
- [ ] **Lesson 72:** [Generic Structs & Implementation Blocks](lessons/08_advanced_and_concurrency/72_generic_structs.md) — *Creating reusable data structures for any type*
- [ ] **Lesson 73:** [Traits: Defining Shared Behavior (trait Summarizable)](lessons/08_advanced_and_concurrency/73_traits_basics.md) — *Defining interfaces and common abilities across different types*
- [ ] **Lesson 74:** [Implementing Traits for Custom Types](lessons/08_advanced_and_concurrency/74_implementing_traits.md) — *Giving standard abilities to your own structs*
- [ ] **Lesson 75:** [Essential Standard Library Traits (Display, Debug, Clone)](lessons/08_advanced_and_concurrency/75_standard_traits.md) — *The most common traits every Rust developer uses daily*
- [ ] **Lesson 76:** [Closures: Anonymous Inline Functions (|x| x + 1)](lessons/08_advanced_and_concurrency/76_closures.md) — *Passing small functions around as variables*
- [ ] **Lesson 77:** [Iterators: Transforming Data (map, filter, collect)](lessons/08_advanced_and_concurrency/77_iterators.md) — *Functional data processing pipelines*
- [ ] **Lesson 78:** [Smart Pointers: Unique Heap Allocation (Box<T>)](lessons/08_advanced_and_concurrency/78_box_smart_pointer.md) — *Storing large or recursive data safely on the heap*
- [ ] **Lesson 79:** [Concurrency: Spawning OS Threads (thread::spawn)](lessons/08_advanced_and_concurrency/79_threads_spawn.md) — *Running multiple tasks at the exact same time on your CPU*
- [ ] **Lesson 80:** [Thread Communication: Message Passing with Channels (mpsc)](lessons/08_advanced_and_concurrency/80_channels_mpsc.md) — *Sending safe messages between concurrent threads*

**⭐ Milestone 8: Multi-Threaded CLI Tool & Mini Server**

---

