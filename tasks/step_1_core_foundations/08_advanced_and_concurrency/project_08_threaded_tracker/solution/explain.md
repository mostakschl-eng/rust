# Checkpoint 8: How the continuing Study Tracker works

## What changed from the previous checkpoint?

Keep checkpoint 7's commands, validation, session model, and saved-file format. Give one worker an owned snapshot, send its report through a channel, and join the worker. Only the main thread prints the report.

The earlier snapshots stay runnable. Here the final goal is still a learner profile and a useful study report, rather than a new unrelated program. This is the combined Level 1–8 app: profile/functions, arithmetic, decisions/loops, vectors/strings, structs/enums, borrowed slices, input/files/errors, and iterator/thread/channel reporting. It does not need every individual Rust feature merely to tick a box.

## Real-life analogy

Think of a study notebook and a helper. Give the helper a photocopy of the current pages; they compute one report and send it back. The original notebook stays with you. Wait for the helper to finish, then read the returned report in order.

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
| `.iter().map(...).sum()` | Borrow sessions, convert their minutes, then add them |
| `.to_vec()` | Clone the slice's sessions into an owned worker snapshot |
| `move` | Capture the snapshot and sender by value |
| `mpsc::channel()` | Create a sender and receiver |
| `.recv()` / `.join()` | Wait for the message / wait for worker completion |

## Follow one add command

1. The command loop reads a line into a fresh String. A read count of zero means EOF.
2. Splitting produces optional pieces. Matching their exact count prevents silently ignoring extra input.
3. `make_session` checks the subject, integer range, and allowed kind. It returns a Session only on success.
4. `append_session` writes one complete encoded row using `write_all`. That call handles short writes by trying the remaining bytes, but can still fail partway through.
5. Only after success does the vector receive the new Session. Storage errors leave the command loop through `?`; an invalid command only prints a helpful input error.

Loading uses `parse_record` and the same data validation. `.lines()` supports normal LF and CRLF endings. The file must end on a complete newline before another row can be appended. Invalid existing content is never treated as an empty list.

## Follow a report

The report borrows a slice. Minutes are widened to u64 before summing. Goal status has separate below/equal/above cases, and remaining minutes are clamped at zero. `longest_subject` returns a view into an existing subject; it cannot outlive the borrowed sessions. `.chars().count()` counts Unicode scalar values, not bytes or human-perceived characters.

The worker owns cloned Sessions and their Strings. It returns a report String through the channel; the main thread joins it and checks both channel and worker outcomes. Only the main thread prints. The original vector remains available for the next command. This demonstration uses a worker to practise ownership and communication; it does not claim that a tiny report runs faster on a thread.

## A small check, not a second implementation

Inside the reference program's `check_logic`, these are assertion excerpts, not standalone programs:

```rust
assert_eq!(goal_status(89, 90), "below goal");
assert_eq!(goal_status(90, 90), "goal reached");
assert_eq!(goal_status(91, 90), "above goal");
assert_eq!(longest_subject(&[]), None);
```

Successful assertions print nothing. A wrong result panics and prevents the demonstration from claiming success. The complete reference also rejects invalid saved records and checks a Unicode example. It compares worker and direct reports for empty and nonempty snapshots. See [the testing guide](../../../../../guides/testing_basics.md). Exact sample stdout is captured in [TASK.md](../TASK.md).

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
- `move` captures by value; cloning supplies owned data. Moving a short-lived reference would not make its referent live longer.
- Receiving the result alone does not join the thread. Keep the explicit join and error handling.

Primary references: [OpenOptions](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html), [write_all](https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all), [str::chars](https://doc.rust-lang.org/std/primitive.str.html#method.chars), [threads](https://doc.rust-lang.org/std/thread/fn.spawn.html), [channels](https://doc.rust-lang.org/std/sync/mpsc/index.html).
