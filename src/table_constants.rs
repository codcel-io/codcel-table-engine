// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

//! Constants for match modes and search modes used in XLOOKUP and XMATCH operations.
//!
//! These constants mirror the behavior of Excel's XLOOKUP and XMATCH functions,
//! providing different matching and searching strategies.

/// Exact match mode for XLOOKUP/XMATCH operations.
///
/// When used, the lookup will only return a result if an exact match is found.
/// If no exact match exists, the operation returns an error or the `if_not_found` value.
pub const X_MATCH_MODE_EXACT: i32 = 0;

/// Exact match or next smallest mode for XLOOKUP/XMATCH operations.
///
/// When used, the lookup will return an exact match if found. If no exact match exists,
/// it returns the next smallest value that is less than the lookup value.
/// The search column should be sorted in ascending order for correct results.
pub const X_MATCH_MODE_EXACT_NEXT_SMALLEST: i32 = -1;

/// Exact match or next largest mode for XLOOKUP/XMATCH operations.
///
/// When used, the lookup will return an exact match if found. If no exact match exists,
/// it returns the next largest value that is greater than the lookup value.
/// The search column should be sorted in ascending order for correct results.
pub const X_MATCH_MODE_EXACT_NEXT_LARGEST: i32 = 1;

/// Wildcard match mode for XLOOKUP/XMATCH operations.
///
/// When used, the lookup value can contain wildcard characters:
/// - `*` matches any sequence of characters
/// - `?` matches any single character
pub const X_MATCH_MODE_WILDCARD: i32 = 2;

/// Search from first to last (forward search).
///
/// The search starts from the first element and proceeds toward the last.
/// This is the default search direction.
pub const X_SEARCH_MODE_FIRST: i32 = 1;

/// Search from last to first (reverse search).
///
/// The search starts from the last element and proceeds toward the first.
/// Useful when you expect matches to be near the end of the data.
pub const X_SEARCH_MODE_REVERSE: i32 = -1;

/// Binary search that returns the first match.
///
/// Performs a binary search on sorted data. The search column must be sorted
/// in ascending order. Returns the first matching element found.
pub const X_SEARCH_MODE_BINARY_FIRST: i32 = 2;

/// Binary search that returns the last match.
///
/// Performs a binary search on sorted data. The search column must be sorted
/// in descending order. Returns the last matching element found.
pub const X_SEARCH_MODE_BINARY_LAST: i32 = -2;
