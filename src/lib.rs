// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

//! Codcel Table Engine
//!
//! This crate provides shared utilities for Codcel table engines, enabling spreadsheet-like
//! table operations across different storage backends (Parquet, PostgreSQL, etc.).
//!
//! # Modules
//!
//! - [`codcel_table`] - Core trait defining table operations like VLOOKUP, HLOOKUP, XLOOKUP, INDEX, MATCH, and CRUD operations.
//! - [`condition`] - SQL condition building with built-in SQL injection protection.
//! - [`searchable`] - Traits and functions for searching and sorting collections.
//! - [`table_constants`] - Constants for match modes and search modes used in lookup operations.
//! - [`table_functions`] - Type definitions for custom table functions.
//! - [`column_type`] - Abstract column type enumeration for cross-backend compatibility.

pub mod codcel_table;
pub mod column_type;
pub mod condition;
pub mod searchable;
pub mod sql_modifiers;
pub mod table_constants;
pub mod table_functions;
