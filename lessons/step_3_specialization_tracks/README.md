# Step 3: Choose one specialization

This is a planned outline. Full lessons and task packages have not been written yet. Choose **one** path first; you do not need to complete all tracks. Add libraries only when the project needs them.

- **CLI tools and automation:** useful terminal commands, arguments, files, and helpful errors. Consider clap for argument parsing; terminal UI and progress-bar libraries are optional.
- **Web backends:** HTTP, validation, storage, and error responses before adding framework or database libraries. Possible tools include axum and SQLx. Query checking does not guarantee correct application logic or zero database/runtime cost. JWT handling is only one part of authentication; a token library alone does not provide a whole secure login and authorization system.
- **Systems, networking or WebAssembly:** choose one concrete platform problem. Learn its constraints before selecting networking or browser libraries. UI libraries are optional.

Keep the scope small. Build and check a useful working slice before adding more.

[Back to the four-step roadmap](../../ROADMAP.md)
