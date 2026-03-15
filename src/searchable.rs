// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

//! Search and sort operations for collections.
//!
//! This module provides the [`Searchable`] trait for finding elements in collections
//! and sorting them. It includes implementations for common vector types
//! ([`Vec<f64>`], [`Vec<i32>`], [`Vec<String>`]) and convenience functions for
//! performing searches through trait objects.

/// A trait for searching and sorting collections.
///
/// This trait provides methods for finding elements based on various matching
/// criteria (exact match, less than or equal, greater than or equal) as well as
/// methods for reordering the collection.
///
/// # Implementations
///
/// This trait is implemented for:
/// - [`Vec<f64>`] - Floating-point number collections
/// - [`Vec<i32>`] - Integer collections
/// - [`Vec<String>`] - String collections (case-insensitive matching)
pub trait Searchable {
    /// Finds the position of an exact match for the given value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to search for, as a string that will be parsed
    ///
    /// # Returns
    ///
    /// * `Some(usize)` - The zero-based index of the first matching element
    /// * `None` - If no exact match is found or the value cannot be parsed
    fn find_exact_pos(&self, value: &str) -> Option<usize>;

    /// Finds the position of the largest element less than or equal to the given value.
    ///
    /// # Arguments
    ///
    /// * `value` - The target value to compare against, as a string
    ///
    /// # Returns
    ///
    /// * `Some(usize)` - The zero-based index of the largest element that is <= the target
    /// * `None` - If no such element exists or the value cannot be parsed
    fn find_largest_less_than_or_equal(&self, value: &str) -> Option<usize>;

    /// Finds the position of the smallest element greater than or equal to the given value.
    ///
    /// # Arguments
    ///
    /// * `value` - The target value to compare against, as a string
    ///
    /// # Returns
    ///
    /// * `Some(usize)` - The zero-based index of the smallest element that is >= the target
    /// * `None` - If no such element exists or the value cannot be parsed
    fn find_smallest_greater_than_or_equal(&self, value: &str) -> Option<usize>;

    /// Reverses the order of elements in the collection.
    ///
    /// This modifies the collection in place, reversing the position of all elements.
    fn reverse_order(&mut self);

    /// Sorts the collection in ascending order.
    ///
    /// This modifies the collection in place, ordering elements from smallest to largest.
    fn sort_ascending(&mut self);

    /// Sorts the collection in descending order.
    ///
    /// This modifies the collection in place, ordering elements from largest to smallest.
    fn sort_descending(&mut self);
}

impl Searchable for Vec<f64> {
    fn find_exact_pos(&self, value: &str) -> Option<usize> {
        value.parse::<f64>().ok().and_then(|val| self.iter().position(|&x| x == val))
    }

    fn find_largest_less_than_or_equal(&self, value: &str) -> Option<usize> {
        let val = value.parse::<f64>().ok()?;
        self.iter().enumerate().filter_map(|(i, &x)| if x <= val { Some((i, x)) } else { None })
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map(|(i, _)| i)
    }

    fn find_smallest_greater_than_or_equal(&self, value: &str) -> Option<usize> {
        let val = value.parse::<f64>().ok()?;
        self.iter().enumerate().filter_map(|(i, &x)| if x >= val { Some((i, x)) } else { None })
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map(|(i, _)| i)
    }

    fn reverse_order(&mut self) {
        self.reverse();
    }

    fn sort_ascending(&mut self) {
        self.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }

    fn sort_descending(&mut self) {
        self.sort_by(|a, b| b.partial_cmp(a).unwrap());
    }

}

impl Searchable for Vec<i32> {
    fn find_exact_pos(&self, value: &str) -> Option<usize> {
        value.parse::<i32>().ok().and_then(|val| self.iter().position(|&x| x == val))
    }

    fn find_largest_less_than_or_equal(&self, value: &str) -> Option<usize> {
        let val = value.parse::<i32>().ok()?;
        self.iter().enumerate().filter_map(|(i, &x)| if x <= val { Some((i, x)) } else { None })
            .max_by(|a, b| a.1.cmp(&b.1))
            .map(|(i, _)| i)
    }

    fn find_smallest_greater_than_or_equal(&self, value: &str) -> Option<usize> {
        let val = value.parse::<i32>().ok()?;
        self.iter().enumerate().filter_map(|(i, &x)| if x >= val { Some((i, x)) } else { None })
            .min_by(|a, b| a.1.cmp(&b.1))
            .map(|(i, _)| i)
    }

    fn reverse_order(&mut self) {
        self.reverse();
    }

    fn sort_ascending(&mut self) {
        self.sort();
    }

    fn sort_descending(&mut self) {
        self.sort_by(|a, b| b.cmp(a));
    }
}

impl Searchable for Vec<String> {
    fn find_exact_pos(&self, value: &str) -> Option<usize> {
        self.iter().position(|x| x.eq_ignore_ascii_case(value))
    }

    fn find_largest_less_than_or_equal(&self, value: &str) -> Option<usize> {
        self.iter().enumerate().filter_map(|(i, x)| {
            if x.to_uppercase() <= value.to_uppercase() { Some((i, x)) } else { None }
        })
            .max_by(|a, b| a.1.to_uppercase().cmp(&b.1.to_uppercase()))
            .map(|(i, _)| i)
    }

    fn find_smallest_greater_than_or_equal(&self, value: &str) -> Option<usize> {
        self.iter().enumerate().filter_map(|(i, x)| {
            if x.to_uppercase() >= value.to_uppercase() { Some((i, x)) } else { None }
        })
            .min_by(|a, b| a.1.to_uppercase().cmp(&b.1.to_uppercase()))
            .map(|(i, _)| i)
    }

    fn reverse_order(&mut self) {
        self.reverse();
    }

    fn sort_ascending(&mut self) {
        self.sort();
    }

    fn sort_descending(&mut self) {
        self.sort_by(|a, b| b.cmp(a));
    }
}

/// Finds the exact position of a value in a searchable collection.
///
/// This is a convenience function that calls [`Searchable::find_exact_pos`] on the
/// provided collection through a trait object.
///
/// # Arguments
///
/// * `collection` - A reference to any type implementing [`Searchable`]
/// * `value` - The value to search for, as a string
///
/// # Returns
///
/// * `Some(usize)` - The zero-based index of the matching element
/// * `None` - If no exact match is found
pub fn find_exact_position(collection: &dyn Searchable, value: &str) -> Option<usize> {
    collection.find_exact_pos(value)
}

/// Finds the position of the largest element less than or equal to a value.
///
/// This is a convenience function that calls [`Searchable::find_largest_less_than_or_equal`]
/// on the provided collection through a trait object.
///
/// # Arguments
///
/// * `collection` - A reference to any type implementing [`Searchable`]
/// * `value` - The target value to compare against, as a string
///
/// # Returns
///
/// * `Some(usize)` - The zero-based index of the largest element that is <= the target
/// * `None` - If no such element exists
pub fn find_largest_position(collection: &dyn Searchable, value: &str) -> Option<usize> {
    collection.find_largest_less_than_or_equal(value)
}

/// Finds the position of the smallest element greater than or equal to a value.
///
/// This is a convenience function that calls [`Searchable::find_smallest_greater_than_or_equal`]
/// on the provided collection through a trait object.
///
/// # Arguments
///
/// * `collection` - A reference to any type implementing [`Searchable`]
/// * `value` - The target value to compare against, as a string
///
/// # Returns
///
/// * `Some(usize)` - The zero-based index of the smallest element that is >= the target
/// * `None` - If no such element exists
pub fn find_smallest_position(collection: &dyn Searchable, value: &str) -> Option<usize> {
    collection.find_smallest_greater_than_or_equal(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for Vec<f64> implementation

    #[test]
    fn test_vec_f64_find_exact_pos() {
        // Test with a value that exists in the vector
        let vec = vec![1.0, 2.5, 3.7, 4.2, 5.0];
        let result = vec.find_exact_pos("3.7");
        assert_eq!(result, Some(2));

        // Test with a value that doesn't exist in the vector
        let result = vec.find_exact_pos("3.8");
        assert_eq!(result, None);

        // Test with an empty vector
        let empty_vec: Vec<f64> = vec![];
        let result = empty_vec.find_exact_pos("1.0");
        assert_eq!(result, None);

        // Test with invalid input
        let result = vec.find_exact_pos("not_a_number");
        assert_eq!(result, None);

        // Test with a value that appears multiple times
        let vec_with_duplicates = vec![1.0, 2.5, 3.7, 2.5, 5.0];
        let result = vec_with_duplicates.find_exact_pos("2.5");
        assert_eq!(result, Some(1)); // Should return the first occurrence
    }

    #[test]
    fn test_vec_f64_find_largest_less_than_or_equal() {
        // Test with a value that has an exact match
        let vec = vec![1.0, 2.5, 3.7, 4.2, 5.0];
        let result = vec.find_largest_less_than_or_equal("3.7");
        assert_eq!(result, Some(2));

        // Test with a value that doesn't have an exact match
        let result = vec.find_largest_less_than_or_equal("3.8");
        assert_eq!(result, Some(2)); // Should return index of 3.7

        // Test with a value smaller than all elements
        let result = vec.find_largest_less_than_or_equal("0.5");
        assert_eq!(result, None);

        // Test with a value larger than all elements
        let result = vec.find_largest_less_than_or_equal("6.0");
        assert_eq!(result, Some(4)); // Should return index of 5.0

        // Test with an empty vector
        let empty_vec: Vec<f64> = vec![];
        let result = empty_vec.find_largest_less_than_or_equal("1.0");
        assert_eq!(result, None);

        // Test with invalid input
        let result = vec.find_largest_less_than_or_equal("not_a_number");
        assert_eq!(result, None);

        // Test with unsorted vector
        let unsorted_vec = vec![3.7, 1.0, 5.0, 2.5, 4.2];
        let result = unsorted_vec.find_largest_less_than_or_equal("3.8");
        assert_eq!(result, Some(0)); // Should return index of 3.7
    }

    #[test]
    fn test_vec_f64_find_smallest_greater_than_or_equal() {
        // Test with a value that has an exact match
        let vec = vec![1.0, 2.5, 3.7, 4.2, 5.0];
        let result = vec.find_smallest_greater_than_or_equal("3.7");
        assert_eq!(result, Some(2));

        // Test with a value that doesn't have an exact match
        let result = vec.find_smallest_greater_than_or_equal("3.8");
        assert_eq!(result, Some(3)); // Should return index of 4.2

        // Test with a value smaller than all elements
        let result = vec.find_smallest_greater_than_or_equal("0.5");
        assert_eq!(result, Some(0)); // Should return index of 1.0

        // Test with a value larger than all elements
        let result = vec.find_smallest_greater_than_or_equal("6.0");
        assert_eq!(result, None);

        // Test with an empty vector
        let empty_vec: Vec<f64> = vec![];
        let result = empty_vec.find_smallest_greater_than_or_equal("1.0");
        assert_eq!(result, None);

        // Test with invalid input
        let result = vec.find_smallest_greater_than_or_equal("not_a_number");
        assert_eq!(result, None);

        // Test with unsorted vector
        let unsorted_vec = vec![3.7, 1.0, 5.0, 2.5, 4.2];
        let result = unsorted_vec.find_smallest_greater_than_or_equal("3.8");
        assert_eq!(result, Some(4)); // Returns index of 4.2 based on actual implementation behavior
    }

    #[test]
    fn test_vec_f64_reverse_order() {
        // Test with a non-empty vector
        let mut vec = vec![1.0, 2.5, 3.7, 4.2, 5.0];
        vec.reverse_order();
        assert_eq!(vec, vec![5.0, 4.2, 3.7, 2.5, 1.0]);

        // Test with a single element
        let mut single_vec = vec![1.0];
        single_vec.reverse_order();
        assert_eq!(single_vec, vec![1.0]);

        // Test with an empty vector
        let mut empty_vec: Vec<f64> = vec![];
        empty_vec.reverse_order();
        assert_eq!(empty_vec, vec![] as Vec<f64>);
    }

    #[test]
    fn test_vec_f64_sort_ascending() {
        // Test with an unsorted vector
        let mut vec = vec![3.7, 1.0, 5.0, 2.5, 4.2];
        vec.sort_ascending();
        assert_eq!(vec, vec![1.0, 2.5, 3.7, 4.2, 5.0]);

        // Test with an already sorted vector
        let mut sorted_vec = vec![1.0, 2.5, 3.7, 4.2, 5.0];
        sorted_vec.sort_ascending();
        assert_eq!(sorted_vec, vec![1.0, 2.5, 3.7, 4.2, 5.0]);

        // Test with a single element
        let mut single_vec = vec![1.0];
        single_vec.sort_ascending();
        assert_eq!(single_vec, vec![1.0]);

        // Test with an empty vector
        let mut empty_vec: Vec<f64> = vec![];
        empty_vec.sort_ascending();
        assert_eq!(empty_vec, vec![] as Vec<f64>);

        // Test with duplicate elements
        let mut vec_with_duplicates = vec![3.7, 1.0, 3.7, 2.5, 1.0];
        vec_with_duplicates.sort_ascending();
        assert_eq!(vec_with_duplicates, vec![1.0, 1.0, 2.5, 3.7, 3.7]);
    }

    #[test]
    fn test_vec_f64_sort_descending() {
        // Test with an unsorted vector
        let mut vec = vec![3.7, 1.0, 5.0, 2.5, 4.2];
        vec.sort_descending();
        assert_eq!(vec, vec![5.0, 4.2, 3.7, 2.5, 1.0]);

        // Test with an already sorted vector (in ascending order)
        let mut sorted_vec = vec![1.0, 2.5, 3.7, 4.2, 5.0];
        sorted_vec.sort_descending();
        assert_eq!(sorted_vec, vec![5.0, 4.2, 3.7, 2.5, 1.0]);

        // Test with a single element
        let mut single_vec = vec![1.0];
        single_vec.sort_descending();
        assert_eq!(single_vec, vec![1.0]);

        // Test with an empty vector
        let mut empty_vec: Vec<f64> = vec![];
        empty_vec.sort_descending();
        assert_eq!(empty_vec, vec![] as Vec<f64>);

        // Test with duplicate elements
        let mut vec_with_duplicates = vec![3.7, 1.0, 3.7, 2.5, 1.0];
        vec_with_duplicates.sort_descending();
        assert_eq!(vec_with_duplicates, vec![3.7, 3.7, 2.5, 1.0, 1.0]);
    }

    // Tests for Vec<i32> implementation

    #[test]
    fn test_vec_i32_find_exact_pos() {
        // Test with a value that exists in the vector
        let vec = vec![1, 2, 3, 4, 5];
        let result = vec.find_exact_pos("3");
        assert_eq!(result, Some(2));

        // Test with a value that doesn't exist in the vector
        let result = vec.find_exact_pos("6");
        assert_eq!(result, None);

        // Test with an empty vector
        let empty_vec: Vec<i32> = vec![];
        let result = empty_vec.find_exact_pos("1");
        assert_eq!(result, None);

        // Test with invalid input
        let result = vec.find_exact_pos("not_a_number");
        assert_eq!(result, None);

        // Test with a value that appears multiple times
        let vec_with_duplicates = vec![1, 2, 3, 2, 5];
        let result = vec_with_duplicates.find_exact_pos("2");
        assert_eq!(result, Some(1)); // Should return the first occurrence
    }

    #[test]
    fn test_vec_i32_find_largest_less_than_or_equal() {
        // Test with a value that has an exact match
        let vec = vec![1, 2, 3, 4, 5];
        let result = vec.find_largest_less_than_or_equal("3");
        assert_eq!(result, Some(2));

        // Test with a value that doesn't have an exact match
        let result = vec.find_largest_less_than_or_equal("3.5");
        assert_eq!(result, None); // Returns None based on actual implementation behavior

        // Test with a value smaller than all elements
        let result = vec.find_largest_less_than_or_equal("0");
        assert_eq!(result, None);

        // Test with a value larger than all elements
        let result = vec.find_largest_less_than_or_equal("6");
        assert_eq!(result, Some(4)); // Should return index of 5

        // Test with an empty vector
        let empty_vec: Vec<i32> = vec![];
        let result = empty_vec.find_largest_less_than_or_equal("1");
        assert_eq!(result, None);

        // Test with invalid input
        let result = vec.find_largest_less_than_or_equal("not_a_number");
        assert_eq!(result, None);

        // Test with unsorted vector
        let unsorted_vec = vec![3, 1, 5, 2, 4];
        let result = unsorted_vec.find_largest_less_than_or_equal("3");
        assert_eq!(result, Some(0)); // Should return index of 3
    }

    #[test]
    fn test_vec_i32_find_smallest_greater_than_or_equal() {
        // Test with a value that has an exact match
        let vec = vec![1, 2, 3, 4, 5];
        let result = vec.find_smallest_greater_than_or_equal("3");
        assert_eq!(result, Some(2));

        // Test with a value that doesn't have an exact match
        let result = vec.find_smallest_greater_than_or_equal("3.5");
        assert_eq!(result, None); // Returns None based on actual implementation behavior

        // Test with a value smaller than all elements
        let result = vec.find_smallest_greater_than_or_equal("0");
        assert_eq!(result, Some(0)); // Should return index of 1

        // Test with a value larger than all elements
        let result = vec.find_smallest_greater_than_or_equal("6");
        assert_eq!(result, None);

        // Test with an empty vector
        let empty_vec: Vec<i32> = vec![];
        let result = empty_vec.find_smallest_greater_than_or_equal("1");
        assert_eq!(result, None);

        // Test with invalid input
        let result = vec.find_smallest_greater_than_or_equal("not_a_number");
        assert_eq!(result, None);

        // Test with unsorted vector
        let unsorted_vec = vec![3, 1, 5, 2, 4];
        let result = unsorted_vec.find_smallest_greater_than_or_equal("3.5");
        assert_eq!(result, None); // Returns None based on actual implementation behavior
    }

    #[test]
    fn test_vec_i32_reverse_order() {
        // Test with a non-empty vector
        let mut vec = vec![1, 2, 3, 4, 5];
        vec.reverse_order();
        assert_eq!(vec, vec![5, 4, 3, 2, 1]);

        // Test with a single element
        let mut single_vec = vec![1];
        single_vec.reverse_order();
        assert_eq!(single_vec, vec![1]);

        // Test with an empty vector
        let mut empty_vec: Vec<i32> = vec![];
        empty_vec.reverse_order();
        assert_eq!(empty_vec, vec![] as Vec<i32>);
    }

    #[test]
    fn test_vec_i32_sort_ascending() {
        // Test with an unsorted vector
        let mut vec = vec![3, 1, 5, 2, 4];
        vec.sort_ascending();
        assert_eq!(vec, vec![1, 2, 3, 4, 5]);

        // Test with an already sorted vector
        let mut sorted_vec = vec![1, 2, 3, 4, 5];
        sorted_vec.sort_ascending();
        assert_eq!(sorted_vec, vec![1, 2, 3, 4, 5]);

        // Test with a single element
        let mut single_vec = vec![1];
        single_vec.sort_ascending();
        assert_eq!(single_vec, vec![1]);

        // Test with an empty vector
        let mut empty_vec: Vec<i32> = vec![];
        empty_vec.sort_ascending();
        assert_eq!(empty_vec, vec![] as Vec<i32>);

        // Test with duplicate elements
        let mut vec_with_duplicates = vec![3, 1, 3, 2, 1];
        vec_with_duplicates.sort_ascending();
        assert_eq!(vec_with_duplicates, vec![1, 1, 2, 3, 3]);
    }

    #[test]
    fn test_vec_i32_sort_descending() {
        // Test with an unsorted vector
        let mut vec = vec![3, 1, 5, 2, 4];
        vec.sort_descending();
        assert_eq!(vec, vec![5, 4, 3, 2, 1]);

        // Test with an already sorted vector (in ascending order)
        let mut sorted_vec = vec![1, 2, 3, 4, 5];
        sorted_vec.sort_descending();
        assert_eq!(sorted_vec, vec![5, 4, 3, 2, 1]);

        // Test with a single element
        let mut single_vec = vec![1];
        single_vec.sort_descending();
        assert_eq!(single_vec, vec![1]);

        // Test with an empty vector
        let mut empty_vec: Vec<i32> = vec![];
        empty_vec.sort_descending();
        assert_eq!(empty_vec, vec![] as Vec<i32>);

        // Test with duplicate elements
        let mut vec_with_duplicates = vec![3, 1, 3, 2, 1];
        vec_with_duplicates.sort_descending();
        assert_eq!(vec_with_duplicates, vec![3, 3, 2, 1, 1]);
    }

    // Tests for Vec<String> implementation

    #[test]
    fn test_vec_string_find_exact_pos() {
        // Test with a value that exists in the vector
        let vec = vec![
            "apple".to_string(),
            "banana".to_string(),
            "cherry".to_string(),
            "date".to_string(),
            "elderberry".to_string(),
        ];
        let result = vec.find_exact_pos("cherry");
        assert_eq!(result, Some(2));

        // Test with a value that doesn't exist in the vector
        let result = vec.find_exact_pos("fig");
        assert_eq!(result, None);

        // Test with an empty vector
        let empty_vec: Vec<String> = vec![];
        let result = empty_vec.find_exact_pos("apple");
        assert_eq!(result, None);

        // Test with case insensitivity
        let result = vec.find_exact_pos("CHERRY");
        assert_eq!(result, Some(2));

        // Test with a value that appears multiple times
        let vec_with_duplicates = vec![
            "apple".to_string(),
            "banana".to_string(),
            "cherry".to_string(),
            "banana".to_string(),
            "elderberry".to_string(),
        ];
        let result = vec_with_duplicates.find_exact_pos("banana");
        assert_eq!(result, Some(1)); // Should return the first occurrence
    }

    #[test]
    fn test_vec_string_find_largest_less_than_or_equal() {
        // Test with a value that has an exact match
        let vec = vec![
            "apple".to_string(),
            "banana".to_string(),
            "cherry".to_string(),
            "date".to_string(),
            "elderberry".to_string(),
        ];
        let result = vec.find_largest_less_than_or_equal("cherry");
        assert_eq!(result, Some(2));

        // Test with a value that doesn't have an exact match
        let result = vec.find_largest_less_than_or_equal("cucumber");
        assert_eq!(result, Some(2)); // Should return index of "cherry"

        // Test with a value smaller than all elements
        let result = vec.find_largest_less_than_or_equal("aardvark");
        assert_eq!(result, None);

        // Test with a value larger than all elements
        let result = vec.find_largest_less_than_or_equal("zebra");
        assert_eq!(result, Some(4)); // Should return index of "elderberry"

        // Test with an empty vector
        let empty_vec: Vec<String> = vec![];
        let result = empty_vec.find_largest_less_than_or_equal("apple");
        assert_eq!(result, None);

        // Test with case insensitivity
        let result = vec.find_largest_less_than_or_equal("CHERRY");
        assert_eq!(result, Some(2));

        // Test with unsorted vector
        let unsorted_vec = vec![
            "cherry".to_string(),
            "apple".to_string(),
            "elderberry".to_string(),
            "banana".to_string(),
            "date".to_string(),
        ];
        let result = unsorted_vec.find_largest_less_than_or_equal("cucumber");
        assert_eq!(result, Some(0)); // Should return index of "cherry"
    }

    #[test]
    fn test_vec_string_find_smallest_greater_than_or_equal() {
        // Test with a value that has an exact match
        let vec = vec![
            "apple".to_string(),
            "banana".to_string(),
            "cherry".to_string(),
            "date".to_string(),
            "elderberry".to_string(),
        ];
        let result = vec.find_smallest_greater_than_or_equal("cherry");
        assert_eq!(result, Some(2));

        // Test with a value that doesn't have an exact match
        let result = vec.find_smallest_greater_than_or_equal("cucumber");
        assert_eq!(result, Some(3)); // Should return index of "date"

        // Test with a value smaller than all elements
        let result = vec.find_smallest_greater_than_or_equal("aardvark");
        assert_eq!(result, Some(0)); // Should return index of "apple"

        // Test with a value larger than all elements
        let result = vec.find_smallest_greater_than_or_equal("zebra");
        assert_eq!(result, None);

        // Test with an empty vector
        let empty_vec: Vec<String> = vec![];
        let result = empty_vec.find_smallest_greater_than_or_equal("apple");
        assert_eq!(result, None);

        // Test with case insensitivity
        let result = vec.find_smallest_greater_than_or_equal("CHERRY");
        assert_eq!(result, Some(2));

        // Test with unsorted vector
        let unsorted_vec = vec![
            "cherry".to_string(),
            "apple".to_string(),
            "elderberry".to_string(),
            "banana".to_string(),
            "date".to_string(),
        ];
        let result = unsorted_vec.find_smallest_greater_than_or_equal("cucumber");
        assert_eq!(result, Some(4)); // Should return index of "date"
    }

    #[test]
    fn test_vec_string_reverse_order() {
        // Test with a non-empty vector
        let mut vec = vec![
            "apple".to_string(),
            "banana".to_string(),
            "cherry".to_string(),
            "date".to_string(),
            "elderberry".to_string(),
        ];
        vec.reverse_order();
        assert_eq!(
            vec,
            vec![
                "elderberry".to_string(),
                "date".to_string(),
                "cherry".to_string(),
                "banana".to_string(),
                "apple".to_string(),
            ]
        );

        // Test with a single element
        let mut single_vec = vec!["apple".to_string()];
        single_vec.reverse_order();
        assert_eq!(single_vec, vec!["apple".to_string()]);

        // Test with an empty vector
        let mut empty_vec: Vec<String> = vec![];
        empty_vec.reverse_order();
        assert_eq!(empty_vec, vec![] as Vec<String>);
    }

    #[test]
    fn test_vec_string_sort_ascending() {
        // Test with an unsorted vector
        let mut vec = vec![
            "cherry".to_string(),
            "apple".to_string(),
            "elderberry".to_string(),
            "banana".to_string(),
            "date".to_string(),
        ];
        vec.sort_ascending();
        assert_eq!(
            vec,
            vec![
                "apple".to_string(),
                "banana".to_string(),
                "cherry".to_string(),
                "date".to_string(),
                "elderberry".to_string(),
            ]
        );

        // Test with an already sorted vector
        let mut sorted_vec = vec![
            "apple".to_string(),
            "banana".to_string(),
            "cherry".to_string(),
            "date".to_string(),
            "elderberry".to_string(),
        ];
        sorted_vec.sort_ascending();
        assert_eq!(
            sorted_vec,
            vec![
                "apple".to_string(),
                "banana".to_string(),
                "cherry".to_string(),
                "date".to_string(),
                "elderberry".to_string(),
            ]
        );

        // Test with a single element
        let mut single_vec = vec!["apple".to_string()];
        single_vec.sort_ascending();
        assert_eq!(single_vec, vec!["apple".to_string()]);

        // Test with an empty vector
        let mut empty_vec: Vec<String> = vec![];
        empty_vec.sort_ascending();
        assert_eq!(empty_vec, vec![] as Vec<String>);

        // Test with duplicate elements
        let mut vec_with_duplicates = vec![
            "cherry".to_string(),
            "apple".to_string(),
            "cherry".to_string(),
            "banana".to_string(),
            "apple".to_string(),
        ];
        vec_with_duplicates.sort_ascending();
        assert_eq!(
            vec_with_duplicates,
            vec![
                "apple".to_string(),
                "apple".to_string(),
                "banana".to_string(),
                "cherry".to_string(),
                "cherry".to_string(),
            ]
        );
    }

    #[test]
    fn test_vec_string_sort_descending() {
        // Test with an unsorted vector
        let mut vec = vec![
            "cherry".to_string(),
            "apple".to_string(),
            "elderberry".to_string(),
            "banana".to_string(),
            "date".to_string(),
        ];
        vec.sort_descending();
        assert_eq!(
            vec,
            vec![
                "elderberry".to_string(),
                "date".to_string(),
                "cherry".to_string(),
                "banana".to_string(),
                "apple".to_string(),
            ]
        );

        // Test with an already sorted vector (in ascending order)
        let mut sorted_vec = vec![
            "apple".to_string(),
            "banana".to_string(),
            "cherry".to_string(),
            "date".to_string(),
            "elderberry".to_string(),
        ];
        sorted_vec.sort_descending();
        assert_eq!(
            sorted_vec,
            vec![
                "elderberry".to_string(),
                "date".to_string(),
                "cherry".to_string(),
                "banana".to_string(),
                "apple".to_string(),
            ]
        );

        // Test with a single element
        let mut single_vec = vec!["apple".to_string()];
        single_vec.sort_descending();
        assert_eq!(single_vec, vec!["apple".to_string()]);

        // Test with an empty vector
        let mut empty_vec: Vec<String> = vec![];
        empty_vec.sort_descending();
        assert_eq!(empty_vec, vec![] as Vec<String>);

        // Test with duplicate elements
        let mut vec_with_duplicates = vec![
            "cherry".to_string(),
            "apple".to_string(),
            "cherry".to_string(),
            "banana".to_string(),
            "apple".to_string(),
        ];
        vec_with_duplicates.sort_descending();
        assert_eq!(
            vec_with_duplicates,
            vec![
                "cherry".to_string(),
                "cherry".to_string(),
                "banana".to_string(),
                "apple".to_string(),
                "apple".to_string(),
            ]
        );
    }

    // Tests for helper functions

    #[test]
    fn test_find_exact_position() {
        // Test with Vec<f64>
        let vec_f64 = vec![1.0, 2.5, 3.7, 4.2, 5.0];
        let result = find_exact_position(&vec_f64, "3.7");
        assert_eq!(result, Some(2));

        // Test with Vec<i32>
        let vec_i32 = vec![1, 2, 3, 4, 5];
        let result = find_exact_position(&vec_i32, "3");
        assert_eq!(result, Some(2));

        // Test with Vec<String>
        let vec_string = vec![
            "apple".to_string(),
            "banana".to_string(),
            "cherry".to_string(),
            "date".to_string(),
            "elderberry".to_string(),
        ];
        let result = find_exact_position(&vec_string, "cherry");
        assert_eq!(result, Some(2));

        // Test with a value that doesn't exist
        let result = find_exact_position(&vec_f64, "6.0");
        assert_eq!(result, None);
    }

    #[test]
    fn test_find_largest_position() {
        // Test with Vec<f64>
        let vec_f64 = vec![1.0, 2.5, 3.7, 4.2, 5.0];
        let result = find_largest_position(&vec_f64, "3.7");
        assert_eq!(result, Some(2));

        // Test with Vec<i32>
        let vec_i32 = vec![1, 2, 3, 4, 5];
        let result = find_largest_position(&vec_i32, "3");
        assert_eq!(result, Some(2));

        // Test with Vec<String>
        let vec_string = vec![
            "apple".to_string(),
            "banana".to_string(),
            "cherry".to_string(),
            "date".to_string(),
            "elderberry".to_string(),
        ];
        let result = find_largest_position(&vec_string, "cherry");
        assert_eq!(result, Some(2));

        // Test with a value that doesn't have an exact match
        let result = find_largest_position(&vec_f64, "3.8");
        assert_eq!(result, Some(2)); // Should return index of 3.7
    }

    #[test]
    fn test_find_smallest_position() {
        // Test with Vec<f64>
        let vec_f64 = vec![1.0, 2.5, 3.7, 4.2, 5.0];
        let result = find_smallest_position(&vec_f64, "3.7");
        assert_eq!(result, Some(2));

        // Test with Vec<i32>
        let vec_i32 = vec![1, 2, 3, 4, 5];
        let result = find_smallest_position(&vec_i32, "3");
        assert_eq!(result, Some(2));

        // Test with Vec<String>
        let vec_string = vec![
            "apple".to_string(),
            "banana".to_string(),
            "cherry".to_string(),
            "date".to_string(),
            "elderberry".to_string(),
        ];
        let result = find_smallest_position(&vec_string, "cherry");
        assert_eq!(result, Some(2));

        // Test with a value that doesn't have an exact match
        let result = find_smallest_position(&vec_f64, "3.8");
        assert_eq!(result, Some(3)); // Should return index of 4.2
    }

    // Additional edge cases and combinations

    #[test]
    fn test_edge_cases_and_combinations() {
        // Test with negative numbers in Vec<f64>
        let vec_f64_negative = vec![-5.0, -3.7, -2.5, -1.0, 0.0, 1.0, 2.5, 3.7, 5.0];

        let result = vec_f64_negative.find_exact_pos("-3.7");
        assert_eq!(result, Some(1));

        let result = vec_f64_negative.find_largest_less_than_or_equal("-2.0");
        assert_eq!(result, Some(2)); // Should return index of -2.5

        let result = vec_f64_negative.find_smallest_greater_than_or_equal("-2.0");
        assert_eq!(result, Some(3)); // Should return index of -1.0

        // Test with negative numbers in Vec<i32>
        let vec_i32_negative = vec![-5, -3, -2, -1, 0, 1, 2, 3, 5];

        let result = vec_i32_negative.find_exact_pos("-3");
        assert_eq!(result, Some(1));

        let result = vec_i32_negative.find_largest_less_than_or_equal("-2");
        assert_eq!(result, Some(2)); // Should return index of -2

        let result = vec_i32_negative.find_smallest_greater_than_or_equal("-2");
        assert_eq!(result, Some(2)); // Should return index of -2

        // Test with mixed case strings
        let vec_mixed_case = vec![
            "Apple".to_string(),
            "BANANA".to_string(),
            "Cherry".to_string(),
            "DATE".to_string(),
            "elderberry".to_string(),
        ];

        let result = vec_mixed_case.find_exact_pos("apple");
        assert_eq!(result, Some(0)); // Case insensitive match

        let result = vec_mixed_case.find_largest_less_than_or_equal("CHERRY");
        assert_eq!(result, Some(2)); // Case insensitive match

        let result = vec_mixed_case.find_smallest_greater_than_or_equal("cherry");
        assert_eq!(result, Some(2)); // Case insensitive match

        // Test with very large and very small floating point numbers
        let vec_f64_extreme = vec![1e-10, 1e-5, 1.0, 1e5, 1e10];

        let result = vec_f64_extreme.find_exact_pos("1e-10");
        assert_eq!(result, Some(0));

        let result = vec_f64_extreme.find_largest_less_than_or_equal("1e-7");
        assert_eq!(result, Some(0)); // Returns index of 1e-10 based on actual implementation behavior

        let result = vec_f64_extreme.find_smallest_greater_than_or_equal("1e7");
        assert_eq!(result, Some(4)); // Returns index of 1e10 based on actual implementation behavior

        // Test with very large and very small integers
        let vec_i32_extreme = vec![i32::MIN, -1000, 0, 1000, i32::MAX];

        let result = vec_i32_extreme.find_exact_pos(&i32::MIN.to_string());
        assert_eq!(result, Some(0));

        let result = vec_i32_extreme.find_largest_less_than_or_equal("-500");
        assert_eq!(result, Some(1)); // Should return index of -1000

        let result = vec_i32_extreme.find_smallest_greater_than_or_equal(&i32::MAX.to_string());
        assert_eq!(result, Some(4)); // Should return index of i32::MAX
    }
}
