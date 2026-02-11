//! # Factorial
//! 
//! **Difficulty**: 🟢 Beginner
//! 
//! Calculate factorial of a number
//!
//! ## Problem Link
//! https://github.com/aarambh-darshan/100-rust-problems/blob/main/problems/006_factorial.md

struct Solution;

impl Solution {
    pub fn factorial(n: i32) -> i32 {
        if n == 0 || n == 1 {
            return 1;
        }       

        let mut result = 1;
        for i in 2..=n {
            result *= i;
        }
        result
    }
}

/// Solution for Factorial
pub fn solve() {
    // TODO: Implement your solution here
    unimplemented!("Solve Factorial");

    println!("Factorials:");
    for i in 0..11 {
        println!("{}! = {}", i, Solution::factorial(i));
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        // TODO: Add test 
        println!("Factorials of 5: {:?}", Solution::factorial(5));
        assert_eq!(Solution::factorial(0), 1);
        assert_eq!(Solution::factorial(1), 1);
        assert_eq!(Solution::factorial(2), 2);
        assert_eq!(Solution::factorial(3), 6);
        assert_eq!(Solution::factorial(4), 24);
        assert_eq!(Solution::factorial(5), 120);
    }
}
