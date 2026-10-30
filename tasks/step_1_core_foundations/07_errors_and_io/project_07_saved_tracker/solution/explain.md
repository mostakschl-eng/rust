# Checkpoint 7: How the continuing Study Tracker works

## What changed from the previous checkpoint?

Replace the fixed sample vector with a real command loop and saved records. Start with no sessions when the log is missing. A valid add must be saved before it enters the vector.

The earlier snapshots stay runnable. Here the final goal is still a learner profile and a useful study report, rather than a new unrelated program. The old fixed sample becomes user-entered history. You can recreate the same three sample sessions through add commands.

## Real-life analogy

Think of an attendance notebook. Read its existing pages first. Check a new entry, write it onto a new line, then add it to the day's display. An unreadable page is a reason to stop and inspect it, not to erase the notebook and begin again.

## Read the symbols

| Code | Meaning |
|---|---|
| `Result<T, E>` | A success value or a reason for failure |
| `?` | Return an error to the caller instead of continuing |
| `&[Session]` | Borrow the session list without taking ownership |
| `Option<&str>` | A borrowed name, or no name for an empty list |
| `'static` | The goal labels are string literals valid for the program's lifetime |
| `.next()` | Take the next piece produced by splitting the input |
| `io::ErrorKind::NotFound` | Specifically a missing path, not every possible read error |
| `OpenOptions::append(true)` | Write at the end without truncating existing records |

## Follow one add command

1. The command loop reads a line into a fresh String. A read count of zero means EOF.
2. Splitting produces optional pieces. Matching their exact count prevents silently ignoring extra input.
3. `make_session` checks the subject, integer range, and allowed kind. It returns a Session only on success.
4. `append_session` writes one complete encoded row using `write_all`. That call handles short writes by trying the remaining bytes, but can still fail partway through.
5. Only after success does the vector receive the new Session. Storage errors leave the command loop through `?`; an invalid command only prints a helpful input error.

Loading uses `parse_record` and the same data validation. `.lines()` supports normal LF and CRLF endings. The file must end on a complete newline before another row can be appended. Invalid existing content is never treated as an empty list.

## Follow a report

The report borrows a slice. Minutes are widened to u64 before summing. Goal status has separate below/equal/above cases, and remaining minutes are clamped at zero. `longest_subject` returns a view into an existing subject; it cannot outlive the borrowed sessions. `.chars().count()` counts Unicode scalar values, not bytes or human-perceived characters.

The main thread computes and prints the report directly. Threading is deliberately left for checkpoint 8, after Lessons 79–80.

## A small check, not a second implementation

Inside the reference program's `check_logic`, these are assertion excerpts, not standalone programs:

```rust
assert_eq!(goal_status(89, 90), "below goal");
assert_eq!(goal_status(90, 90), "goal reached");
assert_eq!(goal_status(91, 90), "above goal");
assert_eq!(longest_subject(&[]), None);
```

Successful assertions print nothing. A wrong result panics and prevents the demonstration from claiming success. The complete reference also rejects invalid saved records and checks a Unicode example.  See [the testing guide](../../../../../guides/testing_basics.md). Exact sample stdout is captured in [TASK.md](../TASK.md).

## Try the reference without touching your own data

In Windows PowerShell, start **inside this checkpoint folder**. These commands create a new temporary working directory and keep your project-folder log separate:

```powershell
$trackerManifest = (Resolve-Path Cargo.toml).Path
$trackerDemo = Join-Path ([IO.Path]::GetTempPath()) ("study-tracker-" + [guid]::NewGuid().ToString("N"))
New-Item -ItemType Directory -Path $trackerDemo | Out-Null
Push-Location $trackerDemo
try {
    cargo run --manifest-path $trackerManifest --example solution
} finally {
    Pop-Location
}
```

Type the sample commands from TASK.md. Your demo log remains in the printed/selected temporary folder; inspect it there. The program's file path is relative to the working directory, not the location of Cargo.toml.

## Common pitfalls and honest limits

- Running the three sample add commands twice creates six sessions. Restart with report/list to check reloading without duplicating data.
- Do not replace an existing log with a blank file to hide a parse error. Inspect a copy and preserve the original.
- Subjects are one word in this small command format. Names containing spaces need an explicit future command/encoding design.
- A successful write is not an atomic, crash-proof database transaction. There is no file locking, multi-process coordination, or automatic repair of partial records.
- A long-running app still needs a deliberate storage-size policy. This course app is for small local histories.

Primary references: [OpenOptions](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html), [write_all](https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all), [str::chars](https://doc.rust-lang.org/std/primitive.str.html#method.chars).
