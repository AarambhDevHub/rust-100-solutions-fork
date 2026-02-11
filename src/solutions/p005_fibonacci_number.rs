//! # Fibonacci Number
//! 
//! **Difficulty**: 🟢 Beginner
//! 
//! Calculate the nth Fibonacci number
//!
//! ## Problem Link
//! https://github.com/aarambh-darshan/100-rust-problems/blob/main/problems/005_fibonacci_number.md


struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n <= 1 {
            return n;
        }

        let mut perv2 = 0;
        let mut perv1 = 1;

        for _ in 2..=n {
            let current = perv1 + perv2;
            perv2 = perv1;
            perv1 = current;
        }

        perv1
    }
}

/// Solution for Fibonacci Number
pub fn solve() {
    // TODO: Implement your solution here
    unimplemented!("Solve Fibonacci Number");

    println!("Fibonacci Sequence:");
    for i in 0..15 {
        println!("F({}) = {}", i, Solution::fib(i));
    }
    
    // Test specific cases
    println!("\nTest Cases:");
    println!("fib(0) = {}", Solution::fib(0));  // 0
    println!("fib(1) = {}", Solution::fib(1));  // 1
    println!("fib(2) = {}", Solution::fib(2));  // 1
    println!("fib(10) = {}", Solution::fib(10)); // 55
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        // TODO: Add test cases
            assert_eq!(Solution::fib(0), 0);
            assert_eq!(Solution::fib(1), 1);
            assert_eq!(Solution::fib(2), 1);
            assert_eq!(Solution::fib(3), 2);
            assert_eq!(Solution::fib(4), 3);
            assert_eq!(Solution::fib(5), 5);
            assert_eq!(Solution::fib(6), 8);
    }
}
