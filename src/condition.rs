// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

//! SQL condition building with SQL injection protection.
//!
//! This module provides types for constructing SQL WHERE clause conditions in a safe manner.
//! All user inputs are properly escaped or validated to prevent SQL injection attacks.
//!
//! # Main Types
//!
//! - [`Condition`] - Represents a complete SQL condition (lhs operator rhs)
//! - [`ConditionValue`] - Represents a value that can appear on either side of a condition
//! - [`WildcardPosition`] - Specifies wildcard placement for LIKE patterns
//!
//! # Security
//!
//! This module implements several security measures:
//! - SQL identifiers (column names) are validated against a whitelist pattern
//! - String values are escaped to prevent SQL injection
//! - Operators are validated against a whitelist of allowed SQL operators
//!
//! # Example
//!
//! ```ignore
//! use codcel_table_engine::condition::{Condition, ConditionValue};
//! use codcel_calculation_engine::value::Value;
//!
//! let lhs = ConditionValue::new_columns("name", false);
//! let rhs = ConditionValue::new_value(Value::String("John".to_string()), false);
//! let condition = Condition::new(lhs, "=", rhs);
//! ```

use crate::column_type::ColumnType;
use codcel_calculation_engine::date_time_base::{date_time_to_excel, time_to_excel};
use codcel_calculation_engine::value::Value;
use codcel_calculation_engine::value_format::ValueFormat;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;

/// Regex pattern for valid SQL identifiers: alphanumeric and underscores only, must start with letter or underscore
static IDENTIFIER_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").expect("Invalid regex pattern"));

/// Validates that a string is a safe SQL identifier (column name, table name, etc.)
fn validate_sql_identifier(identifier: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    if identifier.is_empty() {
        return Err("SQL identifier cannot be empty".into());
    }
    if !IDENTIFIER_REGEX.is_match(identifier) {
        return Err(format!("Invalid SQL identifier: '{}'. Identifiers must contain only alphanumeric characters and underscores, and start with a letter or underscore.", identifier).into());
    }
    Ok(())
}

/// Escapes a string value for use in SQL queries (prevents SQL injection in string literals)
fn escape_sql_string(value: &str) -> String {
    value.replace('\'', "''")
}

/// List of allowed SQL operators for condition building
const ALLOWED_OPERATORS: &[&str] = &[
    "=", "<>", "!=", "<", "<=", ">", ">=", "LIKE", "NOT LIKE", "IN", "NOT IN", "IS", "IS NOT",
    "AND", "OR",
];

/// Validates that an operator is a safe SQL operator
fn validate_sql_operator(op: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    let op_upper = op.to_uppercase();
    if ALLOWED_OPERATORS.iter().any(|&allowed| allowed == op_upper) {
        Ok(())
    } else {
        Err(format!(
            "Invalid SQL operator: '{}'. Allowed operators are: {:?}",
            op, ALLOWED_OPERATORS
        )
        .into())
    }
}

/// Specifies where wildcard characters should be placed in a SQL LIKE pattern.
///
/// This enum is used with [`ConditionValue::WildcardValue`] to control how
/// wildcard patterns are generated for SQL LIKE comparisons.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WildcardPosition {
    /// Append wildcard at the end (e.g., `'value%'` for prefix matching).
    End,
    /// Prepend wildcard at the start (e.g., `'%value'` for suffix matching).
    Start,
    /// Add wildcards at both ends (e.g., `'%value%'` for contains matching).
    Both,
    /// No automatic wildcard addition (wildcards in the value itself are still converted).
    None,
}

/// Represents a value that can appear on either side of a SQL condition.
///
/// This enum provides various ways to represent values in SQL conditions, including
/// literal values, column references, and SQL expressions. All variants handle
/// SQL injection prevention automatically.
///
/// # Case Sensitivity
///
/// Many variants include an `is_case_sensitive` boolean parameter:
/// - When `true`, the value or column is used as-is
/// - When `false`, the value or column is wrapped in `UPPER()` for case-insensitive comparison
#[derive(Debug)]
pub enum ConditionValue {
    /// A literal value with case sensitivity control.
    ///
    /// - First field: The [`Value`] to use in the condition
    /// - Second field: `true` for case-sensitive comparison, `false` to wrap in `UPPER()`
    Value(Value, bool),

    /// A literal value with wildcard pattern support for LIKE comparisons.
    ///
    /// - First field: The [`Value`] to use as the pattern base
    /// - Second field: `true` for case-sensitive comparison, `false` to wrap in `UPPER()`
    /// - Third field: [`WildcardPosition`] specifying where to add `%` wildcards
    ///
    /// Additionally, `*` characters in the value are converted to `%` and `?` to `_`.
    WildcardValue(Value, bool, WildcardPosition),

    /// A reference to one or more column names.
    ///
    /// - First field: Comma-separated column names (e.g., `"col1"` or `"col1, col2"`)
    /// - Second field: `true` for case-sensitive comparison, `false` to wrap in `UPPER()`
    ///
    /// Column names are validated to prevent SQL injection.
    Columns(String, bool),

    /// A SUBSTR expression on a literal value.
    ///
    /// Generates: `UPPER(SUBSTR(value, 1, length))`
    ///
    /// - First field: The [`Value`] to apply SUBSTR to
    /// - Second field: The length parameter for SUBSTR
    SubstrValue(Value, Value),

    /// A SUBSTR expression on a column.
    ///
    /// Generates: `UPPER(SUBSTR(column, 1, length))`
    ///
    /// - First field: The column name
    /// - Second field: The length parameter for SUBSTR
    SubstrColumn(String, Value),

    /// A LENGTH expression on a column.
    ///
    /// Generates: `LENGTH(column)`
    ///
    /// - First field: The column name
    LengthColumn(String),

    /// A nested condition (for building complex expressions).
    ///
    /// Allows combining conditions using AND/OR operators.
    Condition(Box<Condition>),

    /// A modulo expression on a column.
    ///
    /// Generates: `column % divisor`
    ///
    /// - First field: The column name
    /// - Second field: The divisor value
    ModColumn(String, Value),

    /// A SUBSTRING expression on a column with case sensitivity control.
    ///
    /// Generates: `SUBSTRING(column, start)` or `UPPER(SUBSTRING(column, start))`
    ///
    /// - First field: The column name
    /// - Second field: The start position
    /// - Third field: `true` for case-sensitive, `false` to wrap in `UPPER()`
    SubStringColumn(String, Value, bool),

    /// An EXTRACT expression on a column for date/time parts.
    ///
    /// Generates: `EXTRACT(part FROM column)` (e.g., `EXTRACT(HOUR FROM c2)`)
    ///
    /// - First field: The column name
    /// - Second field: The date/time part name (HOUR, MINUTE, SECOND, DAY, MONTH, YEAR)
    ExtractColumn(String, String),
}

impl ConditionValue {
    /// Creates a new literal value condition.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to use in the condition
    /// * `is_case_sensitive` - If `false`, the value will be wrapped in `UPPER()` for case-insensitive comparison
    ///
    /// # Returns
    ///
    /// A [`ConditionValue::Value`] variant.
    pub fn new_value(value: Value, is_case_sensitive: bool) -> ConditionValue {
        ConditionValue::Value(value, is_case_sensitive)
    }

    /// Creates a new wildcard pattern value for LIKE comparisons.
    ///
    /// # Arguments
    ///
    /// * `value` - The base value for the pattern
    /// * `is_case_sensitive` - If `false`, the pattern will be wrapped in `UPPER()`
    /// * `wildcard_position` - Where to add `%` wildcards in the pattern
    ///
    /// # Returns
    ///
    /// A [`ConditionValue::WildcardValue`] variant. Characters `*` and `?` in the value
    /// are automatically converted to SQL wildcards `%` and `_` respectively.
    pub fn new_wildcard_value(
        value: Value,
        is_case_sensitive: bool,
        wildcard_position: WildcardPosition,
    ) -> ConditionValue {
        ConditionValue::WildcardValue(value, is_case_sensitive, wildcard_position)
    }

    /// Creates a reference to one or more columns.
    ///
    /// # Arguments
    ///
    /// * `columns` - A comma-separated list of column names
    /// * `is_case_sensitive` - If `false`, columns will be wrapped in `UPPER()`
    ///
    /// # Returns
    ///
    /// A [`ConditionValue::Columns`] variant. Column names are validated when
    /// [`value()`](Self::value) is called to prevent SQL injection.
    pub fn new_columns(columns: &str, is_case_sensitive: bool) -> ConditionValue {
        ConditionValue::Columns(columns.to_string(), is_case_sensitive)
    }

    /// Creates a SUBSTR expression on a literal value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to apply SUBSTR to
    /// * `length` - The length parameter for the SUBSTR function
    ///
    /// # Returns
    ///
    /// A [`ConditionValue::SubstrValue`] variant that generates `UPPER(SUBSTR(value, 1, length))`.
    pub fn new_substr_value(value: Value, length: Value) -> ConditionValue {
        ConditionValue::SubstrValue(value, length)
    }

    /// Creates a modulo expression on a column.
    ///
    /// # Arguments
    ///
    /// * `column` - The column name to apply modulo to
    /// * `divisor` - The divisor value
    ///
    /// # Returns
    ///
    /// A [`ConditionValue::ModColumn`] variant that generates `column % divisor`.
    pub fn new_column_mod(column: &str, divisor: Value) -> ConditionValue {
        ConditionValue::ModColumn(column.to_string(), divisor)
    }

    /// Creates a SUBSTR expression on a column.
    ///
    /// # Arguments
    ///
    /// * `column` - The column name to apply SUBSTR to
    /// * `length` - The length parameter for the SUBSTR function
    ///
    /// # Returns
    ///
    /// A [`ConditionValue::SubstrColumn`] variant that generates `UPPER(SUBSTR(column, 1, length))`.
    pub fn new_substr_columns(column: &str, length: Value) -> ConditionValue {
        ConditionValue::SubstrColumn(column.to_string(), length)
    }

    /// Creates a SUBSTRING expression on a column.
    ///
    /// # Arguments
    ///
    /// * `column` - The column name to apply SUBSTRING to
    /// * `length` - The start position for the SUBSTRING function
    /// * `is_case_sensitive` - If `false`, the result will be wrapped in `UPPER()`
    ///
    /// # Returns
    ///
    /// A [`ConditionValue::SubStringColumn`] variant that generates
    /// `SUBSTRING(column, start)` or `UPPER(SUBSTRING(column, start))`.
    pub fn new_substring_columns(
        column: &str,
        length: Value,
        is_case_sensitive: bool,
    ) -> ConditionValue {
        ConditionValue::SubStringColumn(column.to_string(), length, is_case_sensitive)
    }

    /// Creates an EXTRACT expression on a column for date/time parts.
    ///
    /// # Arguments
    ///
    /// * `column` - The column name to extract from
    /// * `part` - The date/time part (HOUR, MINUTE, SECOND, DAY, MONTH, YEAR)
    ///
    /// # Returns
    ///
    /// A [`ConditionValue::ExtractColumn`] variant that generates `EXTRACT(part FROM column)`.
    pub fn new_extract_column(column: &str, part: &str) -> ConditionValue {
        ConditionValue::ExtractColumn(column.to_string(), part.to_uppercase())
    }

    /// Creates a nested condition value.
    ///
    /// # Arguments
    ///
    /// * `condition` - The condition to nest
    ///
    /// # Returns
    ///
    /// A [`ConditionValue::Condition`] variant that allows building complex expressions
    /// by combining conditions with AND/OR operators.
    pub fn new_condition(condition: Condition) -> ConditionValue {
        ConditionValue::Condition(Box::new(condition))
    }

    /// Creates a LENGTH expression on a column.
    ///
    /// # Arguments
    ///
    /// * `column` - The column name to get the length of
    ///
    /// # Returns
    ///
    /// A [`ConditionValue::LengthColumn`] variant that generates `LENGTH(column)`.
    pub fn new_column_length(column: &str) -> ConditionValue {
        ConditionValue::LengthColumn(column.to_string())
    }

    /// Generates the SQL string representation of this condition value.
    ///
    /// This method converts the condition value into a SQL-safe string that can be
    /// used in a WHERE clause. All inputs are validated or escaped to prevent SQL injection.
    ///
    /// # Arguments
    ///
    /// * `column_types` - A map of column names to their types, used to determine
    ///   whether to apply `UPPER()` to non-text columns
    /// * `value_format` - Format settings for value conversion
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The SQL string representation
    /// * `Err` - If the value type is not supported or validation fails
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - An unsupported value type is used (e.g., `Bool`, `VecValue`, `None`)
    /// - A column name contains invalid characters
    /// - An invalid SQL operator is used in a nested condition
    pub fn value(
        &self,
        column_types: &HashMap<String, ColumnType>,
        value_format: &ValueFormat,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        match self {
            ConditionValue::Value(value, is_case_sensitive) => {
                match value {
                    Value::String(val) => {
                        // Escape the string value to prevent SQL injection
                        let escaped = escape_sql_string(val);
                        if *is_case_sensitive {
                            Ok(format!("'{escaped}'"))
                        } else {
                            Ok(format!("UPPER('{escaped}')"))
                        }
                    }
                    Value::OptionString(val) => {
                        match val {
                            None => Ok("''".to_string()),
                            Some(v) => {
                                // Escape the string value to prevent SQL injection
                                let escaped = escape_sql_string(v);
                                if *is_case_sensitive {
                                    Ok(format!("'{escaped}'"))
                                } else {
                                    Ok(format!("UPPER('{escaped}')"))
                                }
                            }
                        }
                    }
                    Value::OptionBool(val) => match val {
                        Some(b) => Ok(if *b {
                            "true".to_string()
                        } else {
                            "false".to_string()
                        }),
                        None => Ok("NULL".to_string()),
                    },
                    Value::VecValue(_) => {
                        Err("Vec values are not supported yet in a Condition Value".into())
                    }
                    Value::OptionVecValue(_) => {
                        Err("Option vec values are not supported yet in a Condition Value".into())
                    }
                    Value::AreaValue(value) => {
                        if value.len() == 1 {
                            if let Some(inner) = value.first() {
                                if inner.len() == 1 {
                                    let val = inner.first().unwrap();
                                    let new_conditon_value =
                                        ConditionValue::new_value(val.clone(), *is_case_sensitive);
                                    let val =
                                        new_conditon_value.value(column_types, value_format)?;
                                    return Ok(val);
                                }
                            }
                        }

                        Err("Area values are not supported yet in a Condition Value".into())
                    }
                    Value::OptionAreaValue(_) => {
                        Err("Option Area values are not supported yet in a Condition Value".into())
                    }
                    Value::None => Err("None is not supported yet in a Condition Value".into()),
                    Value::F64(val) => Ok(format!("{val}")),
                    Value::I32(val) => Ok(format!("{val}")),
                    Value::Bool(val) => Ok(if *val {
                        "true".to_string()
                    } else {
                        "false".to_string()
                    }),
                    Value::OptionF64(val) => {
                        match val {
                            None => {
                                // TODO CHECK IF THIS CORRECT
                                Ok("0.0".to_string())
                            }
                            Some(v) => Ok(format!("{v}")),
                        }
                    }
                    Value::OptionI32(val) => {
                        match val {
                            None => {
                                // TODO CHECK IF THIS CORRECT
                                Ok("0".to_string())
                            }
                            Some(v) => Ok(format!("{v}")),
                        }
                    }
                    Value::ChronoDateTime(val) => Ok(date_time_to_excel(
                        val,
                        value_format.date_semantics(),
                    )?
                    .to_string()),
                    Value::OptionChronoDateTime(value) => {
                        if let Some(value) = value {
                            Ok(date_time_to_excel(
                                value,
                                value_format.date_semantics(),
                            )?
                            .to_string())
                        } else {
                            Err("Empty datetime is not supported in a Condition Value".into())
                        }
                    }
                    Value::OptionTime(value) => {
                        if let Some(value) = value {
                            Ok(time_to_excel(value)?.to_string())
                        } else {
                            Err("Empty time is not supported in a Condition Value".into())
                        }
                    }
                    Value::Time(val) => Ok(time_to_excel(val)?.to_string()),
                    Value::Error(e) => Err(format!(
                        "Excel error {} is not supported in a Condition Value",
                        e.display()
                    )
                    .into()),
                }
            }
            ConditionValue::Columns(columns, is_case_sensitive) => {
                let columns_vec: Vec<String> =
                    columns.split(',').map(|s| s.trim().to_string()).collect();
                // Validate all column identifiers
                for col in &columns_vec {
                    validate_sql_identifier(col)?;
                }
                let is_not_upper =
                    columns_vec
                        .iter()
                        .any(|column| match column_types.get(column) {
                            Some(column_type) => is_non_upper_type(column_type),
                            None => false,
                        });
                if is_not_upper || *is_case_sensitive {
                    Ok(columns.to_string())
                } else {
                    Ok(format!("UPPER({columns})"))
                }
            }
            ConditionValue::SubstrValue(value, length) => {
                let condition_value = ConditionValue::new_value(value.clone(), false);
                let val = condition_value.value(column_types, value_format)?;
                Ok(format!(
                    "UPPER(SUBSTR({val}, 1, {:}))",
                    length.i32(value_format)?
                ))
            }
            ConditionValue::SubstrColumn(column, length) => {
                validate_sql_identifier(column)?;
                Ok(format!(
                    "UPPER(SUBSTR({column}, 1, {:}))",
                    length.i32(value_format)?
                ))
            }
            ConditionValue::LengthColumn(column) => {
                validate_sql_identifier(column)?;
                Ok(format!("LENGTH({column})"))
            }
            ConditionValue::ModColumn(column, divisor) => {
                validate_sql_identifier(column)?;
                let divisor_value = ConditionValue::new_value(divisor.clone(), false);
                let val = divisor_value.value(column_types, value_format)?;
                Ok(format!("{column} % {val}"))
            }
            ConditionValue::Condition(condition) => {
                // Validate operator
                validate_sql_operator(&condition.op)?;
                let lhs = condition.lhs.value(column_types, value_format)?;
                let rhs = condition.rhs.value(column_types, value_format)?;

                Ok(format!("{} {} {}", lhs, condition.op, rhs))
            }
            ConditionValue::SubStringColumn(column, length, is_case_sensitive) => {
                validate_sql_identifier(column)?;
                if *is_case_sensitive {
                    Ok(format!(
                        "SUBSTRING({column}, {:})",
                        length.i32(value_format)?
                    ))
                } else {
                    Ok(format!(
                        "UPPER(SUBSTRING({column}, {:}))",
                        length.i32(value_format)?
                    ))
                }
            }
            ConditionValue::ExtractColumn(column, part) => {
                const VALID_PARTS: &[&str] = &["HOUR", "MINUTE", "SECOND", "DAY", "MONTH", "YEAR"];
                if !VALID_PARTS.contains(&part.as_str()) {
                    return Err(format!(
                        "Invalid EXTRACT part: '{}'. Allowed parts are: {:?}",
                        part, VALID_PARTS
                    )
                    .into());
                }
                validate_sql_identifier(column)?;

                // Check if the column is a numeric type (Excel serial number) vs native timestamp
                let is_numeric = column_types
                    .get(column)
                    .map(|ct| ct.is_numeric())
                    .unwrap_or(false);

                if is_numeric {
                    // Column stores Excel serial numbers as float — use arithmetic extraction
                    // Excel serial: integer part = days since 1900-01-01, fractional part = time of day
                    match part.as_str() {
                        "HOUR" => Ok(format!("CAST(FLOOR(({column} - FLOOR({column})) * 24) AS INTEGER)")),
                        "MINUTE" => Ok(format!("CAST(FLOOR(MOD(({column} - FLOOR({column})) * 24, 1) * 60) AS INTEGER)")),
                        "SECOND" => Ok(format!("CAST(FLOOR(MOD(({column} - FLOOR({column})) * 1440, 1) * 60) AS INTEGER)")),
                        _ => {
                            // DAY, MONTH, YEAR require full date conversion — use EXTRACT on converted timestamp
                            // Convert Excel serial to Unix timestamp: (serial - 25569) * 86400
                            Ok(format!("EXTRACT({part} FROM CAST(({column} - 25569) * 86400 AS TIMESTAMP)"))
                        }
                    }
                } else {
                    // Native timestamp/date column — use standard EXTRACT
                    Ok(format!("EXTRACT({} FROM {})", part, column))
                }
            }
            ConditionValue::WildcardValue(value, is_case_sensitive, wildcard_position) => {
                match value {
                    Value::String(val) => {
                        // Escape first, then apply wildcard transformations
                        let escaped = escape_sql_string(val);
                        let val = wildcard(&escaped, wildcard_position);
                        if *is_case_sensitive {
                            Ok(format!("'{val}'"))
                        } else {
                            Ok(format!("UPPER('{val}')"))
                        }
                    }
                    Value::OptionString(val) => {
                        match val {
                            None => Ok("''".to_string()),
                            Some(v) => {
                                // Escape first, then apply wildcard transformations
                                let escaped = escape_sql_string(v);
                                let v = wildcard(&escaped, wildcard_position);
                                if *is_case_sensitive {
                                    Ok(format!("'{v}'"))
                                } else {
                                    Ok(format!("UPPER('{v}')"))
                                }
                            }
                        }
                    }
                    Value::OptionBool(_) => Err(
                        "Option bool value are not supported yet in a Condition Wildcard Value"
                            .into(),
                    ),
                    Value::VecValue(_) => {
                        Err("Vec values are not supported yet in a Condition Wildcard Value".into())
                    }
                    Value::OptionVecValue(_) => Err(
                        "Option vec values are not supported yet in a Condition Wildcard Value"
                            .into(),
                    ),
                    Value::AreaValue(_) => Err(
                        "Area values are not supported yet in a Condition Wildcard Value".into(),
                    ),
                    Value::OptionAreaValue(_) => Err(
                        "Option Area values are not supported yet in a Condition Wildcard Value"
                            .into(),
                    ),
                    Value::None => {
                        Err("None is not supported yet in a Condition Wildcard Value".into())
                    }
                    Value::F64(val) => Ok(format!("{val}")),
                    Value::I32(val) => Ok(format!("{val}")),
                    Value::Bool(_) => {
                        Err("Bool value are not supported yet in a Condition Wildcard Value".into())
                    }
                    Value::OptionF64(val) => {
                        match val {
                            None => {
                                // TODO CHECK IF THIS CORRECT
                                Ok("0.0".to_string())
                            }
                            Some(v) => Ok(format!("{v}")),
                        }
                    }
                    Value::OptionI32(val) => {
                        match val {
                            None => {
                                // TODO CHECK IF THIS CORRECT
                                Ok("0".to_string())
                            }
                            Some(v) => Ok(format!("{v}")),
                        }
                    }
                    Value::ChronoDateTime(val) => Ok(date_time_to_excel(
                        val,
                        value_format.date_semantics(),
                    )?
                    .to_string()),
                    Value::OptionChronoDateTime(value) => {
                        if let Some(value) = value {
                            Ok(date_time_to_excel(
                                value,
                                value_format.date_semantics(),
                            )?
                            .to_string())
                        } else {
                            Err("Empty datetime is not supported in a Condition Value".into())
                        }
                    }
                    Value::OptionTime(value) => {
                        if let Some(value) = value {
                            Ok(time_to_excel(value)?.to_string())
                        } else {
                            Err("Empty time is not supported in a Condition Value".into())
                        }
                    }
                    Value::Time(val) => Ok(time_to_excel(val)?.to_string()),
                    Value::Error(e) => Err(format!(
                        "Excel error {} is not supported in a Condition Wildcard Value",
                        e.display()
                    )
                    .into()),
                }
            }
        }
    }
}

/// Represents a complete SQL condition consisting of a left-hand side, operator, and right-hand side.
///
/// This struct is used to build SQL WHERE clause conditions. The operator is validated
/// against a whitelist of allowed SQL operators to prevent SQL injection.
///
/// # Example
///
/// ```ignore
/// use codcel_table_engine::condition::{Condition, ConditionValue};
/// use codcel_calculation_engine::value::Value;
///
/// // Create: name = 'John'
/// let lhs = ConditionValue::new_columns("name", false);
/// let rhs = ConditionValue::new_value(Value::String("John".to_string()), false);
/// let condition = Condition::new(lhs, "=", rhs);
///
/// // Create: age > 18 AND name = 'John'
/// let age_lhs = ConditionValue::new_columns("age", true);
/// let age_rhs = ConditionValue::new_value(Value::I32(18), true);
/// let age_condition = Condition::new(age_lhs, ">", age_rhs);
///
/// let combined_lhs = ConditionValue::new_condition(age_condition);
/// let combined_rhs = ConditionValue::new_condition(condition);
/// let combined = Condition::new(combined_lhs, "AND", combined_rhs);
/// ```
#[derive(Debug)]
pub struct Condition {
    /// The left-hand side of the condition.
    pub lhs: ConditionValue,

    /// The SQL operator (e.g., `"="`, `"<>"`, `"LIKE"`, `"AND"`, `"OR"`).
    ///
    /// Must be one of the allowed operators: `=`, `<>`, `!=`, `<`, `<=`, `>`, `>=`,
    /// `LIKE`, `NOT LIKE`, `IN`, `NOT IN`, `IS`, `IS NOT`, `AND`, `OR`.
    pub op: String,

    /// The right-hand side of the condition.
    pub rhs: ConditionValue,
}

impl Condition {
    /// Creates a new condition with the given left-hand side, operator, and right-hand side.
    ///
    /// # Arguments
    ///
    /// * `lhs` - The left-hand side value of the condition
    /// * `op` - The SQL operator (e.g., `"="`, `"LIKE"`, `"AND"`)
    /// * `rhs` - The right-hand side value of the condition
    ///
    /// # Returns
    ///
    /// A new [`Condition`] instance. Note that operator validation happens when
    /// [`condition()`](Self::condition) is called, not during construction.
    pub fn new(lhs: ConditionValue, op: &str, rhs: ConditionValue) -> Condition {
        Condition {
            lhs,
            op: op.to_string(),
            rhs,
        }
    }

    /// Generates the SQL string representation of this condition.
    ///
    /// This method converts the condition into a SQL-safe string in the format
    /// `lhs op rhs`. All components are validated or escaped to prevent SQL injection.
    ///
    /// # Arguments
    ///
    /// * `column_types` - A map of column names to their types
    /// * `value_format` - Format settings for value conversion
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The SQL condition string (e.g., `"UPPER(name) = UPPER('John')"`)
    /// * `Err` - If validation fails or an unsupported value type is used
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The operator is not in the allowed list
    /// - A column name contains invalid characters
    /// - A value type is not supported
    pub fn condition(
        &self,
        column_types: &HashMap<String, ColumnType>,
        value_format: &ValueFormat,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        // Validate operator to prevent SQL injection
        validate_sql_operator(&self.op)?;

        let lhs = self.lhs.value(column_types, value_format)?;
        let rhs = self.rhs.value(column_types, value_format)?;

        Ok(format!("{} {} {}", lhs, self.op, rhs))
    }
}

fn is_non_upper_type(column_type: &ColumnType) -> bool {
    column_type.is_non_text_type()
}

fn wildcard(value: &str, wildcard_position: &WildcardPosition) -> String {
    let value = match wildcard_position {
        WildcardPosition::End => &format!("{value}%"),
        WildcardPosition::Start => &format!("%{value}"),
        WildcardPosition::Both => &format!("%{value}%"),
        WildcardPosition::None => value,
    };

    let value = value.replace("*", "%");
    value.replace("?", "_")
}

#[cfg(test)]
mod tests {
    use super::{Condition, ConditionValue, WildcardPosition};
    use crate::column_type::ColumnType;
    use codcel_calculation_engine::value::Value;
    use codcel_calculation_engine::value_format::ValueFormat;
    use std::collections::HashMap;

    // Helper function to create a ValueFormat instance
    fn create_value_format() -> ValueFormat {
        ValueFormat {
            currency_symbol: "".to_string(),
            decimal_separator: ".".to_string(),
            language: "en".to_string(),
            thousands_separator: ",".to_string(),
            use_excel_rounding: false,
            allow_lotus_1_2_3_1900_date_bug: true,
            ..Default::default()
        }
    }

    #[test]
    fn test_wildcard_position_end() {
        // Test WildcardPosition::End with a string value
        let value = Value::String("test".to_string());
        let condition_value =
            ConditionValue::new_wildcard_value(value, true, WildcardPosition::End);

        let column_types = HashMap::new();
        let value_format = create_value_format();

        let result = condition_value.value(&column_types, &value_format).unwrap();
        assert_eq!(result, "'test%'");
    }

    #[test]
    fn test_wildcard_position_start() {
        // Test WildcardPosition::Start with a string value
        let value = Value::String("test".to_string());
        let condition_value =
            ConditionValue::new_wildcard_value(value, true, WildcardPosition::Start);

        let column_types = HashMap::new();
        let value_format = create_value_format();

        let result = condition_value.value(&column_types, &value_format).unwrap();
        assert_eq!(result, "'%test'");
    }

    #[test]
    fn test_wildcard_position_both() {
        // Test WildcardPosition::Both with a string value
        let value = Value::String("test".to_string());
        let condition_value =
            ConditionValue::new_wildcard_value(value, true, WildcardPosition::Both);

        let column_types = HashMap::new();
        let value_format = create_value_format();

        let result = condition_value.value(&column_types, &value_format).unwrap();
        assert_eq!(result, "'%test%'");
    }

    #[test]
    fn test_wildcard_position_none() {
        // Test WildcardPosition::None with a string value
        let value = Value::String("test".to_string());
        let condition_value =
            ConditionValue::new_wildcard_value(value, true, WildcardPosition::None);

        let column_types = HashMap::new();
        let value_format = create_value_format();

        let result = condition_value.value(&column_types, &value_format).unwrap();
        assert_eq!(result, "'test'");
    }

    #[test]
    fn test_wildcard_with_special_chars() {
        // Test wildcard function with special characters
        let value = Value::String("test*with?wildcards".to_string());
        let condition_value =
            ConditionValue::new_wildcard_value(value, true, WildcardPosition::None);

        let column_types = HashMap::new();
        let value_format = create_value_format();

        let result = condition_value.value(&column_types, &value_format).unwrap();
        assert_eq!(result, "'test%with_wildcards'");
    }

    #[test]
    fn test_condition_value_string() {
        // Test ConditionValue::Value with a string value
        let value = Value::String("test".to_string());
        let condition_value = ConditionValue::new_value(value, true);

        let column_types = HashMap::new();
        let value_format = create_value_format();

        let result = condition_value.value(&column_types, &value_format).unwrap();
        assert_eq!(result, "'test'");
    }

    #[test]
    fn test_condition_value_string_case_insensitive() {
        // Test ConditionValue::Value with a string value and case insensitivity
        let value = Value::String("Test".to_string());
        let condition_value = ConditionValue::new_value(value, false);

        let column_types = HashMap::new();
        let value_format = create_value_format();

        let result = condition_value.value(&column_types, &value_format).unwrap();
        assert_eq!(result, "UPPER('Test')");
    }

    #[test]
    fn test_condition_value_option_string_some() {
        // Test ConditionValue::Value with Some(String)
        let value = Value::OptionString(Some("test".to_string()));
        let condition_value = ConditionValue::new_value(value, true);

        let column_types = HashMap::new();
        let value_format = create_value_format();

        let result = condition_value.value(&column_types, &value_format).unwrap();
        assert_eq!(result, "'test'");
    }

    #[test]
    fn test_condition_value_option_string_none() {
        // Test ConditionValue::Value with None
        let value = Value::OptionString(None);
        let condition_value = ConditionValue::new_value(value, true);

        let column_types = HashMap::new();
        let value_format = create_value_format();

        let result = condition_value.value(&column_types, &value_format).unwrap();
        assert_eq!(result, "''");
    }

    #[test]
    fn test_condition_value_f64() {
        // Test ConditionValue::Value with f64
        let value = Value::F64(42.5);
        let condition_value = ConditionValue::new_value(value, true);

        let column_types = HashMap::new();
        let value_format = create_value_format();

        let result = condition_value.value(&column_types, &value_format).unwrap();
        assert_eq!(result, "42.5");
    }

    #[test]
    fn test_condition_value_i32() {
        // Test ConditionValue::Value with i32
        let value = Value::I32(42);
        let condition_value = ConditionValue::new_value(value, true);

        let column_types = HashMap::new();
        let value_format = create_value_format();

        let result = condition_value.value(&column_types, &value_format).unwrap();
        assert_eq!(result, "42");
    }

    #[test]
    fn test_condition_value_option_f64_some() {
        // Test ConditionValue::Value with Some(f64)
        let value = Value::OptionF64(Some(42.5));
        let condition_value = ConditionValue::new_value(value, true);

        let column_types = HashMap::new();
        let value_format = create_value_format();

        let result = condition_value.value(&column_types, &value_format).unwrap();
        assert_eq!(result, "42.5");
    }

    #[test]
    fn test_condition_value_option_f64_none() {
        // Test ConditionValue::Value with None for f64
        let value = Value::OptionF64(None);
        let condition_value = ConditionValue::new_value(value, true);

        let column_types = HashMap::new();
        let value_format = create_value_format();

        let result = condition_value.value(&column_types, &value_format).unwrap();
        assert_eq!(result, "0.0");
    }

    #[test]
    fn test_condition_value_option_i32_some() {
        // Test ConditionValue::Value with Some(i32)
        let value = Value::OptionI32(Some(42));
        let condition_value = ConditionValue::new_value(value, true);

        let column_types = HashMap::new();
        let value_format = create_value_format();

        let result = condition_value.value(&column_types, &value_format).unwrap();
        assert_eq!(result, "42");
    }

    #[test]
    fn test_condition_value_option_i32_none() {
        // Test ConditionValue::Value with None for i32
        let value = Value::OptionI32(None);
        let condition_value = ConditionValue::new_value(value, true);

        let column_types = HashMap::new();
        let value_format = create_value_format();

        let result = condition_value.value(&column_types, &value_format).unwrap();
        assert_eq!(result, "0");
    }

    #[test]
    fn test_condition_value_columns() {
        // Test ConditionValue::Columns
        let condition_value = ConditionValue::new_columns("column1", true);

        let column_types = HashMap::new();
        let value_format = create_value_format();

        let result = condition_value.value(&column_types, &value_format).unwrap();
        assert_eq!(result, "column1");
    }

    #[test]
    fn test_condition_value_columns_case_insensitive() {
        // Test ConditionValue::Columns with case insensitivity
        let condition_value = ConditionValue::new_columns("column1", false);

        let column_types = HashMap::new();
        let value_format = create_value_format();

        let result = condition_value.value(&column_types, &value_format).unwrap();
        assert_eq!(result, "UPPER(column1)");
    }

    #[test]
    fn test_condition_value_columns_with_non_upper_type() {
        // Test ConditionValue::Columns with a non-Text column type
        let condition_value = ConditionValue::new_columns("column1", false);

        let mut column_types = HashMap::new();
        // Use a different ColumnType that's not Text
        column_types.insert("column1".to_string(), ColumnType::Date);
        let value_format = create_value_format();

        let result = condition_value.value(&column_types, &value_format).unwrap();
        assert_eq!(result, "column1");
    }

    #[test]
    fn test_condition_value_substr_value() {
        // Test ConditionValue::SubstrValue
        let value = Value::String("test".to_string());
        let length = Value::I32(2);
        let condition_value = ConditionValue::new_substr_value(value, length);

        let column_types = HashMap::new();
        let value_format = create_value_format();

        let result = condition_value.value(&column_types, &value_format).unwrap();
        assert_eq!(result, "UPPER(SUBSTR(UPPER('test'), 1, 2))");
    }

    #[test]
    fn test_condition_value_substr_column() {
        // Test ConditionValue::SubstrColumn
        let length = Value::I32(2);
        let condition_value = ConditionValue::new_substr_columns("column1", length);

        let column_types = HashMap::new();
        let value_format = create_value_format();

        let result = condition_value.value(&column_types, &value_format).unwrap();
        assert_eq!(result, "UPPER(SUBSTR(column1, 1, 2))");
    }

    #[test]
    fn test_condition_value_length_column() {
        // Test ConditionValue::LengthColumn
        let condition_value = ConditionValue::new_column_length("column1");

        let column_types = HashMap::new();
        let value_format = create_value_format();

        let result = condition_value.value(&column_types, &value_format).unwrap();
        assert_eq!(result, "LENGTH(column1)");
    }

    #[test]
    fn test_condition_value_mod_column() {
        // Test ConditionValue::ModColumn
        let divisor = Value::I32(5);
        let condition_value = ConditionValue::new_column_mod("column1", divisor);

        let column_types = HashMap::new();
        let value_format = create_value_format();

        let result = condition_value.value(&column_types, &value_format).unwrap();
        assert_eq!(result, "column1 % 5");
    }

    #[test]
    fn test_condition_value_substring_column_case_sensitive() {
        // Test ConditionValue::SubStringColumn with case sensitivity
        let length = Value::I32(2);
        let condition_value = ConditionValue::new_substring_columns("column1", length, true);

        let column_types = HashMap::new();
        let value_format = create_value_format();

        let result = condition_value.value(&column_types, &value_format).unwrap();
        assert_eq!(result, "SUBSTRING(column1, 2)");
    }

    #[test]
    fn test_condition_value_substring_column_case_insensitive() {
        // Test ConditionValue::SubStringColumn with case insensitivity
        let length = Value::I32(2);
        let condition_value = ConditionValue::new_substring_columns("column1", length, false);

        let column_types = HashMap::new();
        let value_format = create_value_format();

        let result = condition_value.value(&column_types, &value_format).unwrap();
        assert_eq!(result, "UPPER(SUBSTRING(column1, 2))");
    }

    #[test]
    fn test_condition_value_condition() {
        // Test ConditionValue::Condition
        let lhs = ConditionValue::new_value(Value::String("test".to_string()), true);
        let rhs = ConditionValue::new_value(Value::I32(42), true);
        let condition = Condition::new(lhs, "=", rhs);
        let condition_value = ConditionValue::new_condition(condition);

        let column_types = HashMap::new();
        let value_format = create_value_format();

        let result = condition_value.value(&column_types, &value_format).unwrap();
        assert_eq!(result, "'test' = 42");
    }

    #[test]
    fn test_condition_struct() {
        // Test Condition struct
        let lhs = ConditionValue::new_value(Value::String("test".to_string()), true);
        let rhs = ConditionValue::new_value(Value::I32(42), true);
        let condition = Condition::new(lhs, "=", rhs);

        let column_types = HashMap::new();
        let value_format = create_value_format();

        let result = condition.condition(&column_types, &value_format).unwrap();
        assert_eq!(result, "'test' = 42");
    }

    #[test]
    fn test_condition_with_complex_values() {
        // Test Condition with more complex values
        let lhs = ConditionValue::new_columns("column1", false);
        let rhs =
            ConditionValue::new_substr_value(Value::String("test".to_string()), Value::I32(2));
        let condition = Condition::new(lhs, "LIKE", rhs);

        let column_types = HashMap::new();
        let value_format = create_value_format();

        let result = condition.condition(&column_types, &value_format).unwrap();
        assert_eq!(
            result,
            "UPPER(column1) LIKE UPPER(SUBSTR(UPPER('test'), 1, 2))"
        );
    }

    #[test]
    fn test_error_cases() {
        // Test error cases for unsupported value types
        let column_types = HashMap::new();
        let value_format = create_value_format();

        // Test Bool value - now supported
        let value = Value::Bool(true);
        let condition_value = ConditionValue::new_value(value, true);
        let result = condition_value.value(&column_types, &value_format);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "true");

        // Test OptionBool value - now supported
        let value = Value::OptionBool(Some(true));
        let condition_value = ConditionValue::new_value(value, true);
        let result = condition_value.value(&column_types, &value_format);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "true");

        // Test VecValue
        let value = Value::VecValue(vec![]);
        let condition_value = ConditionValue::new_value(value, true);
        let result = condition_value.value(&column_types, &value_format);
        assert!(result.is_err());

        // Test None value
        let value = Value::None;
        let condition_value = ConditionValue::new_value(value, true);
        let result = condition_value.value(&column_types, &value_format);
        assert!(result.is_err());
    }

    #[test]
    fn test_wildcard_value_with_different_types() {
        let column_types = HashMap::new();
        let value_format = create_value_format();

        // Test with F64
        let value = Value::F64(42.5);
        let condition_value =
            ConditionValue::new_wildcard_value(value, true, WildcardPosition::End);
        let result = condition_value.value(&column_types, &value_format).unwrap();
        assert_eq!(result, "42.5");

        // Test with I32
        let value = Value::I32(42);
        let condition_value =
            ConditionValue::new_wildcard_value(value, true, WildcardPosition::End);
        let result = condition_value.value(&column_types, &value_format).unwrap();
        assert_eq!(result, "42");

        // Test with OptionF64
        let value = Value::OptionF64(Some(42.5));
        let condition_value =
            ConditionValue::new_wildcard_value(value, true, WildcardPosition::End);
        let result = condition_value.value(&column_types, &value_format).unwrap();
        assert_eq!(result, "42.5");

        // Test with OptionI32
        let value = Value::OptionI32(Some(42));
        let condition_value =
            ConditionValue::new_wildcard_value(value, true, WildcardPosition::End);
        let result = condition_value.value(&column_types, &value_format).unwrap();
        assert_eq!(result, "42");
    }

    #[test]
    fn test_multiple_columns() {
        // Test with multiple columns
        let condition_value = ConditionValue::new_columns("column1, column2", false);

        let column_types = HashMap::new();
        let value_format = create_value_format();

        let result = condition_value.value(&column_types, &value_format).unwrap();
        assert_eq!(result, "UPPER(column1, column2)");

        // Test with mixed column types
        let mut column_types = HashMap::new();
        column_types.insert("column1".to_string(), ColumnType::Date);
        column_types.insert("column2".to_string(), ColumnType::Text);

        let result = condition_value.value(&column_types, &value_format).unwrap();
        assert_eq!(result, "column1, column2");
    }

    #[test]
    fn test_nested_conditions() {
        // Test nested conditions
        let inner_lhs = ConditionValue::new_value(Value::String("inner_test".to_string()), true);
        let inner_rhs = ConditionValue::new_value(Value::I32(10), true);
        let inner_condition = Condition::new(inner_lhs, "<", inner_rhs);

        let lhs = ConditionValue::new_condition(inner_condition);
        let rhs = ConditionValue::new_value(Value::Bool(true), true);

        let outer_condition = Condition::new(lhs, "AND", rhs);

        let column_types = HashMap::new();
        let value_format = create_value_format();

        // Bool is now supported, so this should succeed
        let result = outer_condition.condition(&column_types, &value_format);
        assert!(result.is_ok());
    }

    #[test]
    fn test_complex_condition_chain() {
        // Test a more complex chain of conditions
        let c1_lhs = ConditionValue::new_columns("column1", false);
        let c1_rhs = ConditionValue::new_value(Value::String("value1".to_string()), false);
        let condition1 = Condition::new(c1_lhs, "=", c1_rhs);

        let c2_lhs = ConditionValue::new_columns("column2", true);
        let c2_rhs = ConditionValue::new_value(Value::I32(42), true);
        let condition2 = Condition::new(c2_lhs, ">", c2_rhs);

        let c3_lhs = ConditionValue::new_condition(condition1);
        let c3_rhs = ConditionValue::new_condition(condition2);
        let condition3 = Condition::new(c3_lhs, "OR", c3_rhs);

        let column_types = HashMap::new();
        let value_format = create_value_format();

        let result = condition3.condition(&column_types, &value_format).unwrap();
        assert_eq!(result, "UPPER(column1) = UPPER('value1') OR column2 > 42");
    }
}
