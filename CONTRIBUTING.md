# Contributing to Codcel Table Engine

Thank you for your interest in contributing to the Codcel Table Engine.

This repository contains the Rust implementation of Codcel's shared table engine utilities, including the core table trait, lookup and search operations, SQL condition building, and cross-backend abstractions used by Codcel's storage backends.

We welcome contributions that improve correctness, compatibility, performance, maintainability, and documentation.

---

## Scope of This Repository

This repository is intended for work related to the Rust table engine, including:

- Table trait implementations and method signatures
- Lookup function behaviour (VLOOKUP, HLOOKUP, XLOOKUP, MATCH, XMATCH, INDEX, FILTER)
- SQL condition building and injection protection
- Search and sort utilities
- Column type abstractions
- Cross-backend compatibility improvements
- Tests and compatibility validation
- Internal engine refactoring
- Developer documentation for the engine

If your issue is about the Codcel product itself rather than this specific Rust engine repository, please use the Codcel contact channels:

- General contact: https://codcel.io/contact
- Product bug reports: https://codcel.io/contact/bugs
- Feature requests: https://codcel.io/contact/features

---

## Before You Start

Before opening a Pull Request, please:

- Check whether a similar issue or pull request already exists
- Keep the change focused and limited in scope
- Prefer behaviour that matches Excel as closely as practical
- Add or update tests for any behaviour change
- Avoid unrelated formatting-only changes in the same PR

---

## Reporting Bugs

If you find a bug in this repository, please open a GitHub issue.

A good bug report includes:

- The table operation or function involved (e.g. VLOOKUP, XLOOKUP, condition building)
- The input values used
- The actual result
- The expected result
- Whether the expected result was verified against Excel (for lookup/match operations)
- A minimal reproducible example
- Any relevant logs, panic messages, or failing test output

Examples of useful issue titles:

- `XLOOKUP returns incorrect result for wildcard match mode`
- `Condition builder produces invalid SQL for LIKE with special characters`
- `Searchable find_largest_less_than_or_equal fails on unsorted float vectors`

If the problem is in the Codcel application rather than this Rust engine, report it via:

- https://codcel.io/contact/bugs
- bugs@codcel.io

---

## Suggesting Enhancements

Enhancements are welcome.

Examples include:

- New table operation support
- Better compatibility with Excel lookup and match edge cases
- Condition builder improvements or new operator support
- Search and sort performance improvements
- Column type additions for new storage backends
- Refactoring that improves readability or maintainability
- Improved test coverage
- Developer tooling improvements

For broader product ideas, language targets, or commercial feature requests, please use:

- https://codcel.io/contact/features
- features@codcel.io

---

## Contribution Workflow

All changes must come through Pull Requests.

Direct commits to protected branches should not be used except by repository owners when absolutely necessary.

Typical workflow:

1. Fork the repository
2. Create a branch for your change
3. Make your changes
4. Add or update tests
5. Run the test suite
6. Open a Pull Request

Example branch names:

- `fix-xlookup-wildcard`
- `add-condition-between-operator`
- `refactor-searchable-trait`

---

## Pull Request Guidelines

Please keep Pull Requests focused and clearly explained.

Each Pull Request should ideally include:

- A short summary of the change
- The reason for the change
- The Excel behaviour being matched or improved (for lookup/match operations)
- Notes on any edge cases
- Tests added or updated
- Any compatibility considerations

Please avoid mixing multiple unrelated changes into a single PR.

---

## Excel Compatibility Expectations

The lookup and match operations in this project aim to match Excel behaviour as closely as practical.

When contributing table operation logic:

- Prefer verified Excel behaviour over assumptions
- Document any known deviations from Excel
- Be careful with match mode and search mode behaviour
- Consider wildcard matching, case sensitivity, and approximate matching edge cases
- Consider error propagation behaviour
- Preserve backward compatibility where practical unless a bug fix requires otherwise

Where possible, include:

- Example Excel inputs and outputs
- Boundary cases
- Invalid input cases
- Cross-checks against known Excel results

---

## SQL Safety

The condition builder includes SQL injection protection. When contributing to condition-related code:

- Maintain SQL identifier validation
- Preserve string escaping behaviour
- Keep operator whitelisting intact
- Do not introduce raw string interpolation into SQL output
- Add tests for any new condition types or operators

---

## Tests

Tests are required for behaviour changes.

Please add or update tests when you:

- Fix a bug
- Add a table operation
- Change lookup, match, or condition behaviour
- Refactor logic that could affect results

Where useful, tests should include:

- Standard cases
- Edge cases
- Invalid argument cases
- Excel compatibility cases (for lookup/match operations)
- Regression tests for previous bugs

If the repository already has conventions for test placement or naming, follow those conventions.

Before submitting a PR, run:

```bash
cargo test
```

If applicable, also run:

```bash
cargo fmt
cargo clippy --all-targets --all-features
```

---

## Coding Style

Please follow normal Rust best practices:

- Keep functions focused
- Prefer clear naming over cleverness
- Avoid unnecessary allocations where possible
- Keep public behaviour stable unless intentionally changing it
- Add comments where Excel behaviour is surprising or non-obvious
- Prefer small, reviewable refactors

Use `cargo fmt` formatting conventions.

---

## Documentation

If your change affects public behaviour, update relevant documentation as appropriate.

Examples:

- Supported table operations
- Behaviour notes
- Known limitations
- Examples
- Compatibility notes

If the change is primarily documentation-related, consider whether it belongs in `codcel-docs` instead of this repository.

---

## Confidentiality and Example Files

Do not submit confidential spreadsheets, customer models, or regulated data.

If a data example is needed:

- Use anonymized or synthetic data
- Reduce the example to the smallest reproducible case
- Remove sensitive business information

---

## Review and Merge Process

All Pull Requests are reviewed by a maintainer.

Maintainers may request changes for:

- correctness
- Excel compatibility
- SQL safety
- test coverage
- code clarity
- repository scope

A Pull Request may be rejected if it:

- lacks tests for behavioural changes
- changes unrelated areas unnecessarily
- introduces SQL injection vulnerabilities
- introduces unclear behaviour differences from Excel
- belongs in another repository

---

## Licensing

By submitting a contribution to this repository, you agree that your contribution will be licensed under the same terms as this project.

See the repository licensing files for details.

---

## Thank You

We appreciate contributions that help improve table operation compatibility, correctness, and developer experience in the Codcel Table Engine.
