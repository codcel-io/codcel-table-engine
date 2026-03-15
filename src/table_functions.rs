// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

//! Type definitions for custom table functions.
//!
//! This module provides type aliases for defining custom functions that can be
//! invoked during table operations. These functions allow extending table
//! functionality with custom logic.

use std::collections::HashMap;
use std::error::Error;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use codcel_calculation_engine::input::Input;
use codcel_calculation_engine::value::Value;

/// Collection of named table functions (both regular and parameterized).
///
/// This struct holds optional maps of function names to their implementations.
/// Regular functions (`functions`) are looked up by name from `*F*name` markers
/// in parquet data. Parameterized functions (`param_functions`) are looked up
/// from `*P*name:const1:const2:...` markers where constants are parsed and
/// passed as `Vec<Value>` parameters.
///
/// # Example
///
/// ```ignore
/// use std::collections::HashMap;
/// use codcel_table_engine::table_functions::{TableFunctions, TableFunctionType};
///
/// let mut functions: HashMap<String, TableFunctionType> = HashMap::new();
/// // Add custom functions to the map...
/// let table_functions = TableFunctions {
///     functions: Some(functions),
///     param_functions: None,
/// };
/// ```
pub struct TableFunctions {
    pub functions: Option<HashMap<String, TableFunctionType>>,
    pub param_functions: Option<HashMap<String, ParamTableFunctionType>>,
}

impl TableFunctions {
    pub fn none() -> Self {
        TableFunctions {
            functions: None,
            param_functions: None,
        }
    }
}

/// Function signature for async table functions.
///
/// This type alias defines the signature for custom table functions that can be
/// registered and invoked during table operations. Functions receive an [`Arc<Input>`]
/// containing the calculation context and return a pinned, boxed future that
/// resolves to a [`Result<Value, Box<dyn Error + Send + Sync>>`].
pub type TableFunctionType = fn(Arc<Input>) -> Pin<Box<dyn Future<Output = Result<Value, Box<dyn Error + Send + Sync>>> + Send>>;

/// Function signature for parameterized async table functions.
///
/// Similar to [`TableFunctionType`] but also receives a `Vec<Value>` of parameters
/// that were extracted from the parquet cell's `*P*` marker string. This allows a
/// single function template to handle many structurally identical formulas that
/// differ only in their constant values.
pub type ParamTableFunctionType = fn(Arc<Input>, Vec<Value>) -> Pin<Box<dyn Future<Output = Result<Value, Box<dyn Error + Send + Sync>>> + Send>>;
