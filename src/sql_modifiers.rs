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
    /// STDDEV_SAMP(col) — Excel STDEV / STDEV.S / DSTDEV (sample stdev, n-1 divisor)
    StdevS,
    /// STDDEV_POP(col) — Excel STDEVP / STDEV.P / DSTDEVP (population stdev, n divisor)
    StdevP,
    /// VAR_SAMP(col) — Excel VAR / VAR.S / DVAR (sample variance)
    VarS,
    /// VAR_POP(col) — Excel VARP / VAR.P / DVARP (population variance)
    VarP,
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
            SqlAggregate::StdevS => "STDDEV_SAMP",
            SqlAggregate::StdevP => "STDDEV_POP",
            SqlAggregate::VarS => "VAR_SAMP",
            SqlAggregate::VarP => "VAR_POP",
        }
    }

    /// Builds a SQL SELECT expression for an aggregate over one or more numeric columns.
    ///
    /// Matches Excel behavior where aggregate functions (AVERAGE, SUM, MIN, MAX) ignore
    /// non-numeric values. When applied to a multi-column range, they operate on ALL
    /// numeric cells as one flat set.
    ///
    /// The `numeric_cols` should already be formatted/quoted by the caller (e.g. `"c3"` for
    /// DataFusion or `"\"c3\""` for PostgreSQL).
    ///
    /// Returns a SQL expression suitable for use in `SELECT {expr} FROM ...`.
    pub fn build_aggregate_select(&self, numeric_cols: &[String]) -> String {
        match self {
            SqlAggregate::Count | SqlAggregate::CountA => {
                // COUNT already uses "*" - handled by caller
                "COUNT(*)".to_string()
            }
            SqlAggregate::Average => {
                if numeric_cols.len() == 1 {
                    format!("AVG({})", numeric_cols[0])
                } else {
                    // Excel AVERAGE: flat average across all numeric cells
                    // (SUM(c2) + SUM(c3) + ...) / NULLIF(COUNT(c2) + COUNT(c3) + ..., 0)
                    let sums: Vec<String> = numeric_cols.iter()
                        .map(|c| format!("COALESCE(SUM({}), 0)", c))
                        .collect();
                    let counts: Vec<String> = numeric_cols.iter()
                        .map(|c| format!("COUNT({})", c))
                        .collect();
                    format!("({}) / NULLIF({}, 0)", sums.join(" + "), counts.join(" + "))
                }
            }
            SqlAggregate::Sum => {
                if numeric_cols.len() == 1 {
                    format!("SUM({})", numeric_cols[0])
                } else {
                    let parts: Vec<String> = numeric_cols.iter()
                        .map(|c| format!("COALESCE(SUM({}), 0)", c))
                        .collect();
                    parts.join(" + ")
                }
            }
            SqlAggregate::Min => {
                if numeric_cols.len() == 1 {
                    format!("MIN({})", numeric_cols[0])
                } else {
                    let parts: Vec<String> = numeric_cols.iter()
                        .map(|c| format!("MIN({})", c))
                        .collect();
                    format!("LEAST({})", parts.join(", "))
                }
            }
            SqlAggregate::Max => {
                if numeric_cols.len() == 1 {
                    format!("MAX({})", numeric_cols[0])
                } else {
                    let parts: Vec<String> = numeric_cols.iter()
                        .map(|c| format!("MAX({})", c))
                        .collect();
                    format!("GREATEST({})", parts.join(", "))
                }
            }
            // D-function pushdown uses a single column from the database range,
            // so the single-column branch is the expected path for these variants.
            // A multi-column fallback materializes the column expressions.
            SqlAggregate::StdevS => {
                if numeric_cols.len() == 1 {
                    format!("STDDEV_SAMP({})", numeric_cols[0])
                } else {
                    let parts: Vec<String> = numeric_cols.iter()
                        .map(|c| format!("STDDEV_SAMP({})", c))
                        .collect();
                    parts.join(", ")
                }
            }
            SqlAggregate::StdevP => {
                if numeric_cols.len() == 1 {
                    format!("STDDEV_POP({})", numeric_cols[0])
                } else {
                    let parts: Vec<String> = numeric_cols.iter()
                        .map(|c| format!("STDDEV_POP({})", c))
                        .collect();
                    parts.join(", ")
                }
            }
            SqlAggregate::VarS => {
                if numeric_cols.len() == 1 {
                    format!("VAR_SAMP({})", numeric_cols[0])
                } else {
                    let parts: Vec<String> = numeric_cols.iter()
                        .map(|c| format!("VAR_SAMP({})", c))
                        .collect();
                    parts.join(", ")
                }
            }
            SqlAggregate::VarP => {
                if numeric_cols.len() == 1 {
                    format!("VAR_POP({})", numeric_cols[0])
                } else {
                    let parts: Vec<String> = numeric_cols.iter()
                        .map(|c| format!("VAR_POP({})", c))
                        .collect();
                    parts.join(", ")
                }
            }
        }
    }
}
