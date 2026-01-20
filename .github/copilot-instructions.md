# Copilot / AI agent instructions for libshrindo

This file gives focused, actionable guidance for AI coding agents working in this Rust + Svelte/Tauri monorepo.

- **Big picture (workspace roots):** The workspace is defined in [Cargo.toml](Cargo.toml#L1). Major members: `shrine-network`, `shrine-raft`, `muscarine-*` crates, and `liberato-*` crates. Treat each crate as an independent library/service with clearly separated responsibilities.

- **Major components & responsibilities:**
  - `shrine-network`: networking, protocols, node/bootstrap logic. See `shrine-network/src/networking` and internal modules like [shrine-network/src/networking/internals/behavior/mod.rs](shrine-network/src/networking/internals/behavior/mod.rs#L1).
  - `muscarine-*`: reusable subsystems (auth, bootstrap, fileio, types, vrf-chain). Shared types often live under `muscarine-types/src/prelude`.
  - `liberato-*`: addressing and protocol helpers used across other crates.
  - `PeersLotus`: Svelte + Tauri front-end (see [PeersLotus/package.json](PeersLotus/package.json#L1)). Frontend ↔ Rust integration lives under `PeersLotus/src-tauri`.

- **Project patterns & conventions to follow:**
  - Workspace-first: prefer `cargo build --workspace` / `cargo test --workspace` for global tasks; target a single crate with `-p <crate>` when making focused changes.
  - Crate-local preludes: many crates expose a `prelude` module for commonly-used types; import from those rather than deep paths when possible.
  - Protocols are organized in numbered directories (example: `protocols/__0_Assignment_Protocol`, `protocols/__1_Broadcast`) — preserve ordering and naming when adding protocol steps.
  - Keep public APIs stable: changing signature or crate-level types typically requires updating multiple crates.

- **Build / test / dev commands (concrete examples):**
  - Build whole workspace: `cargo build --workspace`
  - Build one crate: `cargo build -p shrine-network`
  - Run tests for a crate: `cargo test -p shrine-network`
  - Run all tests: `cargo test --workspace`
  - Run a crate example (if present): `cargo run -p <crate> --example <name>`
  - PeersLotus front-end dev: `cd PeersLotus && npm install && npm run dev` (project uses Vite + Svelte; Node/Tauri required). The `package.json` scripts include `dev`, `build`, `preview`, and `tauri`.

- **Integration points & dependencies:**
  - Inter-crate communication uses Rust types and module imports; check `muscarine-types` for canonical types and cert/key modules.
  - Frontend (PeersLotus) communicates with native Rust via Tauri; coordination outside Rust code may require editing `PeersLotus/src-tauri` and `PeersLotus/tauri.conf.json`.

- **Where to look first when starting a change:**
  - Workspace manifest: [Cargo.toml](Cargo.toml#L1)
  - Networking internals: [shrine-network/src/networking](shrine-network/src/networking#L1)
  - Shared types: [muscarine-types/src](muscarine-types/src#L1)
  - Frontend: [PeersLotus/package.json](PeersLotus/package.json#L1) and `PeersLotus/src-tauri`

- **Safe-change checklist for AI edits:**
  1. Update only code inside a single crate unless the change is API-compatible across crates.
  2. If changing a public type or signature, search workspace for usages (`rg "TypeName" -S`).
  3. Run `cargo test -p <crate>` locally after edits (or `cargo test --workspace` for cross-cutting changes).
  4. For UI changes, run `cd PeersLotus && npm run dev` and prefer minimal JS/TS edits; coordinate with `src-tauri` if native calls are affected.

- **Examples of repository-specific patterns to emulate:**
  - Use the crate `prelude` exports instead of repeating full module paths (e.g. import common types from `muscarine-types::prelude`).
  - Protocol implementations live in `protocols/` subfolders and are often namespaced; add new protocol steps using the existing numeric prefix convention.

If anything above is unclear or you want additional examples (e.g., walk-through of changing a protocol, or running a specific example), tell me which area and I will expand or update this file.
