// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

//! SQL modifiers for pushdown optimization.
//!
//! When the transpiler detects patterns like `=SORT(FILTER(table, condition))`, it can
//! push the outer operation (SORT → ORDER BY) into the SQL query itself rather than
//! performing it in-memory on the result. This module defines the modifier types that
//! are passed from generated code to the table engines.

/// SQL modifiers that can be pushed down into table operation queries.
///
/// These are detected at transpile time and passed to the table engine
/// so it can incorporate them directly into the SQL query.
///
/// # Examples
///
/// ```
/// use codcel_table_engine::sql_modifiers::{SqlModifiers, SqlAggregate};
///
/// // SORT(FILTER(...), 2, -1) → ORDER BY col2 DESC
/// let modifiers = SqlModifiers {
///     order_by: Some(vec![(2, true)]),
///     ..Default::default()
/// };
///
/// // UNIQUE(FILTER(...)) → SELECT DISTINCT
/// let modifiers = SqlModifiers {
///     distinct: true,
///     ..Default::default()
/// };
///
/// // SUM(FILTER(...)) → SELECT SUM(col)
/// let modifiers = SqlModifiers {
///     aggregate: Some(SqlAggregate::Sum),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default)]
pub struct SqlModifiers {
    /// ORDER BY clause: Vec of (column_index_1_based, descending).
    /// Maps from Excel SORT parameters to SQL ORDER BY.
    pub order_by: Option<Vec<(usize, bool)>>,
    /// When true, use SELECT DISTINCT instead of SELECT.
    /// Maps from Excel UNIQUE function.
    pub distinct: bool,
    /// Aggregate function to apply to selected columns.
    /// Changes the query from returning rows to returning a scalar.
    pub aggregate: Option<SqlAggregate>,
    /// LIMIT and OFFSET for positional access (e.g., LARGE/SMALL).
    /// Format: (limit, offset) — both 0-based for SQL.
    pub limit_offset: Option<(usize, usize)>,
}

impl SqlModifiers {
    /// Returns true if no modifiers are set (equivalent to default).
    pub fn is_empty(&self) -> bool {
        self.order_by.is_none() && !self.distinct && self.aggregate.is_none() && self.limit_offset.is_none()
    }
}

/// SQL aggregate functions that can replace in-memory Excel aggregation.
#[derive(Debug, Clone)]
pub enum SqlAggregate {
    /// SUM(col) — Excel SUM
    Sum,
    /// COUNT(*) — Excel COUNT
    Count,
    /// COUNT(col) — Excel COUNTA (counts non-null values)
    CountA,
    /// AVG(col) — Excel AVERAGE
    Average,
    /// MIN(col) — Excel MIN
    Min,
    /// MAX(col) — Excel MAX
    Max,
}

impl SqlAggregate {
    /// Returns the SQL function name for this aggregate.
    pub fn sql_function(&self) -> &'static str {
        match self {
            SqlAggregate::Sum => "SUM",
            SqlAggregate::Count => "COUNT",
            SqlAggregate::CountA => "COUNT",
            SqlAggregate::Average => "AVG",
            SqlAggregate::Min => "MIN",
            SqlAggregate::Max => "MAX",
        }
    }
}
