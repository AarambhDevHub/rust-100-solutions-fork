//! # Even or Odd
//! 
//! **Difficulty**: 🟢 Beginner
//! 
//! Determine if a number is even or odd
//!
//! ## Problem Link
//! https://github.com/aarambh-darshan/100-rust-problems/blob/main/problems/011_even_or_odd.md

struct Solution;

impl Solution {
    pub fn even_or_odd(n: i32) -> String {
        if n % 2 == 0 {
            "Even".to_string()
        } else {
            "Odd".to_string()
        }       
    }
}

/// Solution for Even or Odd
pub fn solve() {
    // TODO: Implement your solution here
    unimplemented!("Solve Even or Odd")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        // TODO: Add test cases
        assert_eq!(Solution::even_or_odd(2), "Even");
        assert_eq!(Solution::even_or_odd(3), "Odd");
        assert_eq!(Solution::even_or_odd(0), "Even");
        assert_eq!(Solution::even_or_odd(-1), "Odd");
        assert_eq!(Solution::even_or_odd(-2), "Even");
    }
}
