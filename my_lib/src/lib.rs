//! # My Lib
//!
//! `my_lib` is a demonstration library providing basic calculator functions,
//! string utilities, and greeting functionality.
//!
//! ## Examples
//!
//! ```rust
//! use my_lib::{calculator, string_utils, greet};
//!
//! // Use calculator functions
//! let sum = calculator::add(2, 3);
//! assert_eq!(sum, 5);
//!
//! // Use string utilities
//! let reversed = string_utils::reverse_string("hello");
//! assert_eq!(reversed, "olleh");
//!
//! // Use greeting function
//! let greeting = greet("World");
//! assert_eq!(greeting, "Hello, World! Welcome to my_lib!");
//! ```

/// A simple calculator module
pub mod calculator {
    pub fn add(left: u64, right: u64) -> u64 {
        left + right
    }
    
    pub fn multiply(left: u64, right: u64) -> u64 {
        left * right
    }
    
    pub fn power(base: u64, exponent: u32) -> u64 {
        base.pow(exponent)
    }
}

/// A utility module for string operations
pub mod string_utils {
    pub fn reverse_string(input: &str) -> String {
        input.chars().rev().collect()
    }
    
    pub fn count_words(input: &str) -> usize {
        input.split_whitespace().count()
    }
    
    pub fn to_title_case(input: &str) -> String {
        input.split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().chain(chars.as_str().to_lowercase().chars()).collect(),
                }
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
}

/// A simple greeting function
pub fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to my_lib!", name)
}

#[cfg(test)]
mod tests {
    use super::calculator;
    use super::string_utils;
    use super::greet;

    #[test]
    fn test_calculator_add() {
        let result = calculator::add(2, 2);
        assert_eq!(result, 4);
    }
    
    #[test]
    fn test_calculator_multiply() {
        let result = calculator::multiply(3, 4);
        assert_eq!(result, 12);
    }
    
    #[test]
    fn test_calculator_power() {
        let result = calculator::power(2, 3);
        assert_eq!(result, 8);
    }
    
    #[test]
    fn test_string_reverse() {
        let result = string_utils::reverse_string("hello");
        assert_eq!(result, "olleh");
    }
    
    #[test]
    fn test_string_count_words() {
        let result = string_utils::count_words("hello world rust");
        assert_eq!(result, 3);
    }
    
    #[test]
    fn test_string_title_case() {
        let result = string_utils::to_title_case("hello world rust");
        assert_eq!(result, "Hello World Rust");
    }
    
    #[test]
    fn test_greet() {
        let result = greet("Alice");
        assert_eq!(result, "Hello, Alice! Welcome to my_lib!");
    }
}
