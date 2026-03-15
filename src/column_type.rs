// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

/// Abstract column type that works across both Parquet and PostgreSQL table implementations.
/// This enum provides a common abstraction layer so that shared utilities like `condition.rs`
/// don't need to depend on datafusion-specific types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColumnType {
    Text,
    Integer,
    BigInt,
    Float,
    Double,
    Boolean,
    Date,
    Timestamp,
    Binary,
}

impl ColumnType {
    /// Returns true if the type should not be uppercased in SQL comparisons.
    /// Only Text types should be uppercased for case-insensitive comparisons.
    pub fn is_non_text_type(&self) -> bool {
        !matches!(self, ColumnType::Text)
    }
}
