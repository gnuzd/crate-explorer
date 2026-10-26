# crate-explorer

> A command-line tool to discover Rust crates from [crates.io](https://crates.io) — search by query, category, or keyword directly from your terminal.

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Status](https://img.shields.io/badge/status-WIP-blue.svg)](#roadmap)

---

## Overview

`crate-explorer` is a small, focused CLI that helps you **find the right Rust library** without leaving your terminal. Instead of opening a browser, typing `cargo search`, and guessing crate names, you can filter by **category** (domain) or **keyword** (tag) — two things `cargo search` does not support out of the box.

**Real-world example:** You want a SQLite driver for Rust but don't know which crate to use. Just run:

```bash
crate-explorer keyword sqlite
```

You immediately get `rusqlite`, `sqlx`, `libsql`, and more — each with description, download count, and repository link.

This project is also designed as a **hands-on Rust learning path** — every milestone teaches a core language concept (async, error handling, TUI, serde, and so on).

---

## Features (v0.1.0)

- ✅ Free-text search: `search <query>`
- ✅ Filter by category: `category <name>` (e.g. `database`, `web-programming`)
- ✅ Filter by keyword/tag: `keyword <tag>` (e.g. `sqlite`, `cli`, `async`)
- ✅ Displays name, description, total downloads, latest version, and repository link
- ✅ Colorized, readable output

## In progress (see [Roadmap](#roadmap))

- ⏳ Interactive TUI (built with `ratatui`)
- ⏳ Tabs: `[Search] [Category] [Keyword]` switchable via keyboard
- ⏳ Crate detail popup + `a` key to add to your project
- ⏳ Response caching to reduce API calls

---

## Installation

### Requirements

- **Rust** >= 1.75 (install via [rustup](https://rustup.rs))
- Internet connection (to call the crates.io API)

### Install from source

```bash
git clone https://github.com/<username>/crate-explorer.git
cd crate-explorer
cargo install --path .
```

After installation, the `crate-explorer` command is available on your `PATH`.

### Run without installing

```bash
cargo run -- <subcommand> <args>
```

---

## Usage

### Free-text search

```bash
crate-explorer search sqlite
```

### Filter by category

```bash
crate-explorer category database
crate-explorer category web-programming
crate-explorer category command-line-interface
```

See the full list of categories at [crates.io/categories](https://crates.io/categories).

### Filter by keyword/tag

```bash
crate-explorer keyword sqlite
crate-explorer keyword async
crate-explorer keyword serialization
```

### Help

```bash
crate-explorer --help
crate-explorer search --help
```

### Example output

```text
🔎 Results for keyword: "sqlite" — 20 crates

1. rusqlite
   Ergonomic wrapper for SQLite
   ⬇ 12345678 total  |  📦 v0.31.0
   🔗 https://github.com/rusqlite/rusqlite

2. sqlx
   The Rust SQL Toolkit
   ⬇ 9876543 total  |  📦 v0.8.0
   🔗 https://github.com/launchbadge/sqlx

3. libsql
   libSQL is a fork of SQLite that adds...
   ⬇ 543210 total  |  📦 v0.4.0
   🔗 https://github.com/tursodatabase/libsql
```

---

## Project structure

```text
crate-explorer/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs        # Entry point — parse CLI, call API, print results
    ├── cli.rs         # Subcommand definitions via clap
    ├── api.rs         # crates.io API client (async with reqwest)
    └── models.rs      # Structs mapping the JSON response
```

### How it works

1. `main.rs` parses command-line arguments via `clap`.
2. Based on the subcommand, it calls the matching `CratesClient` method (`search`, `by_category`, `by_keyword`).
3. `api.rs` sends an async HTTP request to `https://crates.io/api/v1/crates` and deserializes the JSON into `SearchResponse`.
4. `main.rs` prints the result list to the terminal with colors.

---

## Tech stack

| Crate | Role |
|---|---|
| [`tokio`](https://crates.io/crates/tokio) | Async runtime |
| [`reqwest`](https://crates.io/crates/reqwest) | HTTP client |
| [`serde`](https://crates.io/crates/serde) + [`serde_json`](https://crates.io/crates/serde_json) | JSON serialization |
| [`clap`](https://crates.io/crates/clap) | CLI argument parsing (derive style) |
| [`anyhow`](https://crates.io/crates/anyhow) | Ergonomic error handling |
| [`colored`](https://crates.io/crates/colored) | Terminal colors |

**About TLS:** we use `rustls-tls` instead of `native-tls` to **avoid an OpenSSL dependency**, which makes cross-platform builds (Windows, Alpine Linux, macOS) much easier.

---

## Roadmap

### ✅ v0.1.0 — Basic CLI (done)
- [x] Search by query / category / keyword
- [x] Display description, downloads, version, repository
- [x] Colorized output

### 🚧 v0.2.0 — Better CLI
- [ ] `--limit <n>` flag to cap the number of results
- [ ] `--sort <by>` flag (downloads, recent, alpha)
- [ ] `--json` flag for script-friendly output
- [ ] Rate-limit handling + automatic retry

### 🚧 v0.3.0 — TUI
- [ ] Interactive UI with `ratatui`
- [ ] Scrollable list with arrow keys
- [ ] Real-time search input (debounced 300ms)
- [ ] Tabs: `[Search] [Category] [Keyword]`

### 🚧 v0.4.0 — Advanced interaction
- [ ] Crate detail popup
- [ ] `a` key to run `cargo add <crate>` directly
- [ ] `o` key to open repository in browser
- [ ] Result cache in `~/.cache/crate-explorer/`

### 🔮 Future (unscheduled)
- [ ] Filter by license
- [ ] Side-by-side crate comparison
- [ ] Suggestions based on current `Cargo.toml`

---

## Development

### Build and run

```bash
# Debug build
cargo build

# Run
cargo run -- search sqlite

# Release build (optimized)
cargo build --release

# Run tests
cargo test
```

### Format

```bash
cargo fmt
```

### Lint

```bash
cargo clippy -- -D warnings
```

### Common errors

| Error | Cause | Fix |
|---|---|---|
| `expected Client, found Result<Client, Error>` | Missing `?` after `.context(...)` | Add `?` right after `)` |
| `method context not found` | Missing `use anyhow::Context` | Add the import at the top of the file |
| `failed to fetch` | No internet or rate limited | Check connection, retry later |
| `user-agent rejected` | User-Agent missing contact info | Update `user_agent(...)` in `api.rs` |

---

## Notes

- **Rate limit:** crates.io limits request volume. Do not fire an API call on every keystroke (this will be addressed in v0.3.0 with debouncing).
- **User-Agent:** crates.io **requires** a User-Agent with contact information (email or URL). If you fork this project, change the email in `src/api.rs`.
- **Read-only:** this tool only reads data from crates.io; it never writes or modifies anything on the server.

---

## Contributing

Contributions are welcome! Workflow:

1. Fork the repository
2. Create a branch: `git checkout -b feature/your-feature`
3. Commit using [Conventional Commits](https://www.conventionalcommits.org/): `feat: add --limit flag`
4. Push to your branch
5. Open a Pull Request

### Code conventions

- Run `cargo fmt` before committing
- Run `cargo clippy -- -D warnings` — no warnings allowed
- Add tests for new logic (especially in `api.rs` and `models.rs`)
- Comments in English or Vietnamese are both fine as long as they're clear

---

## References

- [The Rust Programming Language](https://doc.rust-lang.org/book/) — the official book
- [crates.io API docs](https://crates.io/data-access) — official API documentation
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial) — async guide
- [ratatui docs](https://docs.rs/ratatui) — TUI library (used in the roadmap)
- [lib.rs](https://lib.rs) — inspiration for the tag/category system

---

## License

MIT License — see the [LICENSE](LICENSE) file for details.

You are free to use, modify, distribute, and commercialize this project, provided you keep the original copyright notice.

---

## Author

**Chris Nguyen** — [@gnuzd](https://github.com/gnuzd)

> This project was created as a hands-on exercise while learning Rust, to master async/await, error handling, serde, clap, and HTTP API integration.

---

## Acknowledgements

- The [Rust](https://www.rust-lang.org) team and [crates.io](https://crates.io) for providing a free, public API.
- The Rust community (Vietnam and worldwide) for the excellent documentation this project builds upon.
