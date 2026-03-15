// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

//! Core trait for table operations.
//!
//! This module defines the [`CodcelTable`] trait which provides spreadsheet-like
//! table operations including lookup functions (VLOOKUP, HLOOKUP, XLOOKUP),
//! matching functions (MATCH, XMATCH), and CRUD operations.

use async_trait::async_trait;
use std::error::Error;
use codcel_calculation_engine::input::Input;
use codcel_calculation_engine::value::Value;
use codcel_calculation_engine::value_format::ValueFormat;
use crate::condition::Condition;
use crate::sql_modifiers::SqlModifiers;
use crate::table_functions::TableFunctions;

/// A trait defining spreadsheet-like table operations.
///
/// This trait provides methods for performing lookups, matching, filtering,
/// and CRUD operations on tabular data. It is designed to be implemented by
/// different table backends (e.g., Parquet files, PostgreSQL databases).
///
/// All methods are async and thread-safe (`Send + Sync`), making them suitable
/// for concurrent access in web applications and multi-threaded environments.
///
/// # Lookup Functions
///
/// The trait provides several Excel-compatible lookup functions:
/// - [`v_lookup`](Self::v_lookup) - Vertical lookup (VLOOKUP)
/// - [`h_lookup`](Self::h_lookup) - Horizontal lookup (HLOOKUP)
/// - [`x_lookup`](Self::x_lookup) - Extended lookup with advanced matching (XLOOKUP)
/// - [`lookup`](Self::lookup) - Simplified lookup
///
/// # Matching Functions
///
/// - [`match_table`](Self::match_table) - Find position of a value (MATCH)
/// - [`x_match`](Self::x_match) - Extended match with advanced modes (XMATCH)
///
/// # Data Access
///
/// - [`index`](Self::index) - Retrieve value at specific row/column
/// - [`filter`](Self::filter) - Filter rows by condition
/// - [`select_all`](Self::select_all) - Select all rows from columns
///
/// # CRUD Operations
///
/// - [`add_row`](Self::add_row) - Insert a new row
/// - [`read_row`](Self::read_row) - Read a row by ID
/// - [`update_row`](Self::update_row) - Update an existing row
/// - [`delete_row`](Self::delete_row) - Delete a row by ID
#[async_trait]
pub trait CodcelTable: Send + Sync {
    /// Performs a vertical lookup similar to Excel's VLOOKUP function.
    ///
    /// Searches for a value in the first column of a range (or specified search column)
    /// and returns a value in the same row from a specified column.
    ///
    /// # Arguments
    ///
    /// * `lookup_value` - The value to search for
    /// * `result_column_index` - The column index or name to return the result from
    /// * `search_column_index` - The column index or name to search in
    /// * `range` - If `Some(true)`, finds approximate match; if `Some(false)` or `None`, finds exact match
    /// * `table_functions` - Optional custom functions available during the operation
    /// * `input` - The calculation input context
    /// * `value_format` - Format settings for value conversion
    ///
    /// # Returns
    ///
    /// * `Ok(Value)` - The value found at the intersection of the matching row and result column
    /// * `Err` - If no match is found or an error occurs
    #[allow(clippy::too_many_arguments)]
    async fn v_lookup(&self, lookup_value: &str, result_column_index: &str, search_column_index: &str, range: Option<bool>, table_functions: &TableFunctions, input: &Input, value_format: &ValueFormat) -> Result<Value, Box<dyn Error + Send + Sync>>;

    /// Returns the relative position of a value in a column.
    ///
    /// Similar to Excel's MATCH function, this finds the position of a value
    /// within a column based on the specified match type.
    ///
    /// # Arguments
    ///
    /// * `match_value` - The value to search for
    /// * `match_type` - The type of match:
    ///   - `Some(1)` or `None`: Finds largest value <= match_value (data must be ascending)
    ///   - `Some(0)`: Finds exact match
    ///   - `Some(-1)`: Finds smallest value >= match_value (data must be descending)
    /// * `column` - The column name to search in
    /// * `row` - Starting row for the search
    /// * `value_format` - Format settings for value conversion
    ///
    /// # Returns
    ///
    /// * `Ok(Value)` - The 1-based position of the match
    /// * `Err` - If no match is found or an error occurs
    async fn match_table(&self, match_value: &str, match_type: Option<i32>, column: &str, row: u32, value_format: &ValueFormat) -> Result<Value, Box<dyn Error + Send + Sync>>;

    /// Returns a value from a specific row and column position.
    ///
    /// Similar to Excel's INDEX function, retrieves a value from the table
    /// at the specified coordinates.
    ///
    /// # Arguments
    ///
    /// * `row` - The 1-based row number (0 returns entire column)
    /// * `column` - The 1-based column number, or `None` for the entire row
    /// * `table_functions` - Optional custom functions available during the operation
    /// * `input` - The calculation input context
    /// * `value_format` - Format settings for value conversion
    ///
    /// # Returns
    ///
    /// * `Ok(Value)` - The value at the specified position
    /// * `Err` - If the position is out of bounds or an error occurs
    async fn index(&self, row: i32, column: Option<i32>, table_functions: &TableFunctions, input: &Input, value_format: &ValueFormat) -> Result<Value, Box<dyn Error + Send + Sync>>;

    /// Performs a horizontal lookup similar to Excel's HLOOKUP function.
    ///
    /// Searches for a value in the first row of a range and returns a value
    /// in the same column from a specified row.
    ///
    /// # Arguments
    ///
    /// * `lookup_value` - The value to search for
    /// * `row_index` - The row number to return the result from (1-based)
    /// * `range` - If `Some(true)`, finds approximate match; if `Some(false)` or `None`, finds exact match
    /// * `table_functions` - Optional custom functions available during the operation
    /// * `input` - The calculation input context
    /// * `column` - The column to search in
    /// * `value_format` - Format settings for value conversion
    ///
    /// # Returns
    ///
    /// * `Ok(Value)` - The value found at the intersection of the result row and matching column
    /// * `Err` - If no match is found or an error occurs
    #[allow(clippy::too_many_arguments)]
    async fn h_lookup(&self, lookup_value: &str, row_index: i32, range: Option<bool>, table_functions: &TableFunctions, input: &Input, column: &str, value_format: &ValueFormat) -> Result<Value, Box<dyn Error + Send + Sync>>;

    /// Performs an extended lookup with advanced match and search modes.
    ///
    /// Similar to Excel's XLOOKUP function, this provides more flexible searching
    /// with multiple match modes and search directions.
    ///
    /// # Arguments
    ///
    /// * `lookup_value` - The value to search for
    /// * `search_column` - The column to search in
    /// * `columns` - Comma-separated column names to return values from
    /// * `row` - Starting row for the search
    /// * `if_not_found` - Value to return if no match is found
    /// * `match_mode` - The matching strategy (see [`crate::table_constants`]):
    ///   - `0`: Exact match
    ///   - `-1`: Exact match or next smallest
    ///   - `1`: Exact match or next largest
    ///   - `2`: Wildcard match
    /// * `search_mode` - The search direction (see [`crate::table_constants`]):
    ///   - `1`: Search first to last
    ///   - `-1`: Search last to first
    ///   - `2`: Binary search ascending
    ///   - `-2`: Binary search descending
    /// * `table_functions` - Optional custom functions available during the operation
    /// * `input` - The calculation input context
    /// * `value_format` - Format settings for value conversion
    ///
    /// # Returns
    ///
    /// * `Ok(Value)` - The value(s) from the return columns for the matching row
    /// * `Err` - If an error occurs (not-found returns `if_not_found` value if provided)
    #[allow(clippy::too_many_arguments)]
    async fn x_lookup(&self, lookup_value: &str, search_column: &str, columns: &str, row: u32, if_not_found: Option<String>, match_mode: Option<i32>, search_mode: Option<i32>, table_functions: &TableFunctions, input: &Input, value_format: &ValueFormat) -> Result<Value, Box<dyn Error + Send + Sync>>;

    /// Performs a simplified lookup operation.
    ///
    /// A convenience method that performs a lookup with default settings,
    /// equivalent to XLOOKUP with exact matching.
    ///
    /// # Arguments
    ///
    /// * `lookup_value` - The value to search for
    /// * `search_column` - The column to search in
    /// * `columns` - Comma-separated column names to return values from
    /// * `row` - Starting row for the search
    /// * `table_functions` - Optional custom functions available during the operation
    /// * `input` - The calculation input context
    /// * `value_format` - Format settings for value conversion
    ///
    /// # Returns
    ///
    /// * `Ok(Value)` - The value(s) from the return columns for the matching row
    /// * `Err` - If no match is found or an error occurs
    #[allow(clippy::too_many_arguments)]
    async fn lookup(&self, lookup_value: &str, search_column: &str, columns: &str, row: u32, table_functions: &TableFunctions, input: &Input, value_format: &ValueFormat) -> Result<Value, Box<dyn Error + Send + Sync>>;

    /// Returns the position of a value with extended matching options.
    ///
    /// Similar to Excel's XMATCH function, this provides flexible position
    /// finding with multiple match and search modes.
    ///
    /// # Arguments
    ///
    /// * `match_value` - The value to search for
    /// * `match_mode` - The matching strategy (see [`crate::table_constants`])
    /// * `search_mode` - The search direction (see [`crate::table_constants`])
    /// * `column` - The column to search in
    /// * `row` - Starting row for the search
    /// * `value_format` - Format settings for value conversion
    ///
    /// # Returns
    ///
    /// * `Ok(Value)` - The 1-based position of the match
    /// * `Err` - If no match is found or an error occurs
    #[allow(clippy::too_many_arguments)]
    async fn x_match(&self, match_value: &str, match_mode: Option<i32>, search_mode: Option<i32>, column: &str, row: u32, value_format: &ValueFormat) -> Result<Value, Box<dyn Error + Send + Sync>>;

    /// Filters table rows based on a condition.
    ///
    /// Returns all rows that match the specified condition, similar to
    /// Excel's FILTER function.
    ///
    /// # Arguments
    ///
    /// * `condition` - The filter condition to apply
    /// * `if_empty` - Value to return if no rows match the condition
    /// * `columns` - Comma-separated column names to include in the result
    /// * `table_functions` - Optional custom functions available during the operation
    /// * `input` - The calculation input context
    /// * `value_format` - Format settings for value conversion
    ///
    /// # Returns
    ///
    /// * `Ok(Value)` - The filtered rows as a 2D array, or `if_empty` if no matches
    /// * `Err` - If an error occurs during filtering
    #[allow(clippy::too_many_arguments)]
    async fn filter(&self, condition: Condition, if_empty: &str, columns: &str, table_functions: &TableFunctions, input: &Input, value_format: &ValueFormat) -> Result<Value, Box<dyn Error + Send + Sync>>;

    /// Selects all rows from the specified columns.
    ///
    /// # Arguments
    ///
    /// * `columns` - Comma-separated column names to select
    /// * `table_functions` - Optional custom functions available during the operation
    /// * `input` - The calculation input context
    /// * `value_format` - Format settings for value conversion
    ///
    /// # Returns
    ///
    /// * `Ok(Value)` - All rows from the specified columns as a 2D array
    /// * `Err` - If a column doesn't exist or an error occurs
    async fn select_all(&self, columns: &str, table_functions: &TableFunctions, input: &Input, value_format: &ValueFormat) -> Result<Value, Box<dyn Error + Send + Sync>>;

    /// Adds a new row to the table.
    ///
    /// # Arguments
    ///
    /// * `values` - The values for the new row, in column order
    /// * `table_functions` - Optional custom functions available during the operation
    /// * `input` - The calculation input context
    /// * `value_format` - Format settings for value conversion
    ///
    /// # Returns
    ///
    /// * `Ok(Value)` - Confirmation of the insert (typically the new row's ID or count)
    /// * `Err` - If the insert fails
    async fn add_row(&self, values: Vec<Value>, table_functions: &TableFunctions, input: &Input, value_format: &ValueFormat) -> Result<Value, Box<dyn Error + Send + Sync>>;

    /// Updates an existing row by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the row to update
    /// * `values` - The new values for the row, in column order
    /// * `table_functions` - Optional custom functions available during the operation
    /// * `input` - The calculation input context
    /// * `value_format` - Format settings for value conversion
    ///
    /// # Returns
    ///
    /// * `Ok(Value)` - Confirmation of the update (typically the number of rows affected)
    /// * `Err` - If the row doesn't exist or the update fails
    async fn update_row(&self, id: &str, values: Vec<Value>, table_functions: &TableFunctions, input: &Input, value_format: &ValueFormat) -> Result<Value, Box<dyn Error + Send + Sync>>;

    /// Deletes a row by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the row to delete
    /// * `table_functions` - Optional custom functions available during the operation
    /// * `input` - The calculation input context
    /// * `value_format` - Format settings for value conversion
    ///
    /// # Returns
    ///
    /// * `Ok(Value)` - Confirmation of the deletion (typically the number of rows affected)
    /// * `Err` - If the row doesn't exist or the deletion fails
    async fn delete_row(&self, id: &str, table_functions: &TableFunctions, input: &Input, value_format: &ValueFormat) -> Result<Value, Box<dyn Error + Send + Sync>>;

    /// Reads a single row by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the row to read
    /// * `table_functions` - Optional custom functions available during the operation
    /// * `input` - The calculation input context
    /// * `value_format` - Format settings for value conversion
    ///
    /// # Returns
    ///
    /// * `Ok(Value)` - The row data as an array of values
    /// * `Err` - If the row doesn't exist or an error occurs
    async fn read_row(&self, id: &str, table_functions: &TableFunctions, input: &Input, value_format: &ValueFormat) -> Result<Value, Box<dyn Error + Send + Sync>>;

    /// Filters table rows with SQL pushdown modifiers (ORDER BY, DISTINCT, aggregates).
    ///
    /// Default implementation ignores modifiers and delegates to [`filter`](Self::filter).
    /// Engines that support SQL pushdown should override this to incorporate modifiers
    /// directly into the SQL query.
    #[allow(clippy::too_many_arguments)]
    async fn filter_with_modifiers(&self, condition: Condition, if_empty: &str, columns: &str, table_functions: &TableFunctions, input: &Input, value_format: &ValueFormat, _modifiers: &SqlModifiers) -> Result<Value, Box<dyn Error + Send + Sync>> {
        self.filter(condition, if_empty, columns, table_functions, input, value_format).await
    }

    /// Selects all rows with SQL pushdown modifiers.
    ///
    /// Default implementation ignores modifiers and delegates to [`select_all`](Self::select_all).
    async fn select_all_with_modifiers(&self, columns: &str, table_functions: &TableFunctions, input: &Input, value_format: &ValueFormat, _modifiers: &SqlModifiers) -> Result<Value, Box<dyn Error + Send + Sync>> {
        self.select_all(columns, table_functions, input, value_format).await
    }

    /// Performs an extended lookup with SQL pushdown modifiers.
    ///
    /// Default implementation ignores modifiers and delegates to [`x_lookup`](Self::x_lookup).
    #[allow(clippy::too_many_arguments)]
    async fn x_lookup_with_modifiers(&self, lookup_value: &str, search_column: &str, columns: &str, row: u32, if_not_found: Option<String>, match_mode: Option<i32>, search_mode: Option<i32>, table_functions: &TableFunctions, input: &Input, value_format: &ValueFormat, _modifiers: &SqlModifiers) -> Result<Value, Box<dyn Error + Send + Sync>> {
        self.x_lookup(lookup_value, search_column, columns, row, if_not_found, match_mode, search_mode, table_functions, input, value_format).await
    }
}
