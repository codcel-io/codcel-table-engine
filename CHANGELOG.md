<!--
SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Changelog

## 0.1.10

Requires `codcel-calculation-engine` 0.1.10.

### Breaking

- **Follows the calculation engine's move to `DateSemantics`.** Date criteria
  are converted with `ValueFormat::date_semantics()` rather than the raw
  `allow_lotus_1_2_3_1900_date_bug` flag. Implementations of `CodcelTable` that
  call `date_time_to_excel` themselves need the same change; see the
  calculation engine's changelog.

### Changed

- **The crate denies `clippy::unwrap_used`, `clippy::expect_used` and
  `clippy::panic`**, and production code contains none of them.
- SQL identifier validation in criteria no longer goes through a lazily
  compiled regex, so there is no fallible initialisation left in a `static`.
  Accepted identifiers are unchanged: `^[a-zA-Z_][a-zA-Z0-9_]*$`.
