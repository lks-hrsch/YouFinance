# youfinance

A simple, secure, and self-hosted personal finance manager. YouFinance gives you full control over where your data flows and how it is stored. Built with Tauri, Rust, Next.js, and TypeScript.

## Tech Stack

- [Tauri](https://tauri.app/) — Desktop application shell
- [Rust](https://www.rust-lang.org/) — Backend logic and database access
- [Next.js](https://nextjs.org/) — Frontend framework
- [TypeScript](https://www.typescriptlang.org/) — Frontend type safety
- [Bun](https://bun.sh/) — JavaScript package manager and runtime
- [Tailwind CSS](https://tailwindcss.com/) — Utility-first CSS framework
- [shadcn/ui](https://ui.shadcn.com/) — UI component library
- [Diesel](https://diesel.rs/) — Rust ORM for SQLite
- [Typeshare](https://github.com/1password/typeshare) — Generates TypeScript types from Rust structs
- [GoCardless](https://gocardless.com/) — Bank data provider (Open Banking)

## Development Setup

### Prerequisites

This project uses a [Nix flake](./flake.nix) to manage dependencies. Enter the dev shell with:

```bash
nix develop
```

This provides: Rust toolchain, Bun, Biome, `typeshare-cli`, and SQLite.

### Run in development mode

```bash
bun tauri dev
# On Linux, if needed:
WEBKIT_DISABLE_COMPOSITING_MODE=1 bun tauri dev
```

### Generate TypeScript types from Rust

The project uses [Typeshare](https://github.com/1password/typeshare) to keep the TypeScript models in sync with the Rust data structures. After modifying Rust structs in `src-tauri/src/model.rs`, regenerate the types:

```bash
typeshare . --lang=typescript --output-file=src/models/typeshare_definitions.ts
```

> **Note:** `typeshare` is included in the Nix dev shell. If running outside Nix, install it with `cargo install typeshare-cli`.

### Database Migrations (Diesel)

### Running Tests

To run the backend unit tests, execute the following command:

```bash
cargo test --manifest-path src-tauri/Cargo.toml
```

### Testing the Parsers with CLI

You can test real-world `.csv` or `.mta` files against the application parsers via the built-in CLI helper. From the project root, run:

```bash
cargo run --manifest-path src-tauri/Cargo.toml --bin parse -- path/to/real/file.csv
# or
cargo run --manifest-path src-tauri/Cargo.toml --bin parse -- path/to/real/file.mta
```

### Database Migrations (Diesel)

Run all pending migrations:

```bash
cd src-tauri
diesel migration run --database-url <path/to/database>
```

Generate a new migration:

```bash
cd src-tauri
diesel migration generate <migration_name>
```

Redo migrations (e.g., to reapply schema changes):

```bash
cd src-tauri
diesel migration redo --database-url <path/to/database> --number <depth>
```

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) with:
  - [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
  - [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Disclaimer

See [doc/DISCLAIMER-AI.md](./doc/DISCLAIMER-AI.md) for AI usage disclosure.
