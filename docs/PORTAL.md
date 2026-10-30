# Rust Learning Portal

This React, TypeScript, and Vite app reads the course directly from this repository. It does not read the older `portal_data.js`. The original HTML portal is preserved in `legacy/`; the course sources keep their existing locations.

## Run locally

Use Bun 1.4 or later. Run every command from the **repository root**:

```sh
bun install
bun run dev
```

Open the local address printed by Vite. The development server refreshes course content when the repository Markdown or task Rust files change. Stop it with Ctrl+C.

From the repository root, build and check the app:

```sh
bun run build
bun run typecheck
bun run preview
```

`build` regenerates content before TypeScript checking and bundling. `preview` serves the generated `dist` directory locally. It is a preview server, not a production hosting service.

## How course content works

`scripts/generate-content.mjs` discovers the eight foundation levels, numbered lessons, matching task folders, and project checkpoints. Missing required task files and duplicate entry IDs stop generation with an error. Steps 2–4 use their current README outlines.

The generated `src/generated/content.json` contains the lesson, practice, starter, reference solution, explanation, and document-library text. The library includes repository Markdown such as the setup guide, roadmap, testing guide, task instructions, and solution explanations. It excludes Git metadata, dependencies, Rust build output, web build output, generated public copies, application source, and the legacy archive. Hidden paths and symbolic links are skipped.

Repository Markdown and task `.rs`, `Cargo.toml`, and `Cargo.lock` files are copied to `public/course/` for source links and downloads. That directory is generated output. The generator only removes stale files recorded in its own manifest. Course publishing includes reference answers and Markdown documents; review source content before putting the site on a public host.

From the repository root, manual generation and a standalone watcher are available:

```sh
bun scripts/generate-content.mjs
bun scripts/generate-content.mjs --watch
```

Edit the original files under `lessons`, `tasks`, `guides`, or the other repository source folders. Generated copies will be replaced. Saving code in the browser does not edit the repository's Rust starter files or mark the tutor's roadmap complete.

## Deploy

Build from the **repository root**. Install with `bun install --frozen-lockfile`, build with `bun run build`, and publish `dist/`. Keep `bun.lock` in the repository. No nested application directory is required. The full repository must be available at build time so the generator can read the course sources. The static deployed site needs only the generated dist files. Markdown changes appear after rebuilding/redeploying; development mode regenerates on save.

## Browser practice and privacy

The browser editor runs Rust through the remote Rust Playground API. Running code requires an internet connection and sends the code to that service; do not put private information or secrets in it. The API is an external dependency and can be unavailable or reject requests. It does not provide an interactive terminal for stdin. Use the real local Cargo task for input lessons, companion files, dependencies, and filesystem exercises. Remote files belong to the Playground environment, not your computer.

Progress and editor drafts are saved in this browser's local storage. They do not sync to another browser, device, the cloud, or the tutor's Markdown tracker. Clearing site data can remove them. Keep important work in your local task files and use the app's export controls when available.
