<p align="center">
  <a href="https://codcel.io">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="assets/codcel-logo-lockup-dark.svg">
      <img src="assets/codcel-logo-lockup.svg" alt="Codcel" width="320">
    </picture>
  </a>
</p>

# Codcel Table Engine

Shared Rust traits and utilities for Excel-like table operations — lookups, filtering, search, and CRUD.

## Overview

Codcel Table Engine defines the `CodcelTable` trait — the common interface that both the [Parquet Engine](https://github.com/codcel-io/codcel-parquet-engine) and [PostgreSQL Engine](https://github.com/codcel-io/codcel-postgresql-engine) implement. It provides Excel-compatible lookup operations, search and sort utilities, SQL condition building with injection protection, and a backend-agnostic column type system.

This is one of the open-source components of [Codcel](https://codcel.io). Codcel converts your Excel spreadsheets into clean, human-readable source code — in Rust, Python, Java, C#, TypeScript, Go, Swift, and more. You get the full source code, and this engine is part of what you get: your generated projects use these table traits directly to handle lookups, joins, and mappings. No black boxes.

## Capabilities

### Lookup Operations

| Operation | Description |
|---|---|
| `v_lookup` | Vertical lookup (Excel VLOOKUP) |
| `h_lookup` | Horizontal lookup (Excel HLOOKUP) |
| `x_lookup` | Extended lookup with match and search modes (Excel XLOOKUP) |
| `lookup` | Standard lookup |
| `match_table` | Find position of a value (Excel MATCH) |
| `x_match` | Extended position matching (Excel XMATCH) |
| `index` | Retrieve value at a specific row/column (Excel INDEX) |
| `filter` | Filter rows by condition (Excel FILTER) |
| `select_all` | Retrieve all rows for specified columns |

### CRUD Operations

| Operation | Description |
|---|---|
| `add_row` | Insert a new row |
| `read_row` | Read a row by ID |
| `update_row` | Update an existing row |
| `delete_row` | Delete a row by ID |

### Search Modes

- **Exact match** — find an identical value
- **Next smallest** — exact or nearest smaller value (sorted ascending)
- **Next largest** — exact or nearest larger value (sorted ascending)
- **Wildcard** — `*` and `?` pattern matching
- **Binary search** — ascending or descending sorted data

### Additional Features

- SQL condition builder with injection protection (identifier validation, string escaping)
- Backend-agnostic column types: Text, Integer, BigInt, Float, Double, Boolean, Date, Timestamp, Binary
- Async trait — all operations are `async` and thread-safe (`Send + Sync`)

## Quick Start

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
codcel-table-engine = { git = "https://github.com/codcel-io/codcel-table-engine.git", branch = "main" }
```

Import and implement the trait:

```rust
use codcel_table_engine::codcel_table::CodcelTable;
```

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) before opening a pull request.

## About Codcel

[Codcel](https://codcel.io) turns Excel spreadsheets into production-ready software — real source code in Rust, Python, Java, C#, TypeScript, Go, Swift, and more, with zero platform lock-in.

This table engine is one of several open-source components that power Codcel. Learn more at [codcel.io](https://codcel.io).

## Licensing

This project is available under **multiple licenses**:

- **MIT License** (LICENSE-MIT)
- **Apache License 2.0** (LICENSE-APACHE)
- **Codcel Commercial License** (LICENSE-CODCEL-COMMERCIAL)

This licensing model is intended to keep the Excel calculation logic
open and auditable, while protecting Codcel from direct competition
in automated spreadsheet-to-code platforms.

### Which license should I use?

- If you are an individual developer, company, or library author using
  these Excel functions directly in your application, you may choose
  **MIT or Apache-2.0**.

- If you are using this software as part of an **Automated Code Generation
  System (excluding Codcel itself)**, you must use the
  **Codcel Commercial License**.

See LICENSE-CODCEL-COMMERCIAL for details.
