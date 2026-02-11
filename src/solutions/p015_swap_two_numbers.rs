//! # Swap Two Numbers
//! 
//! **Difficulty**: 🟢 Beginner
//! 
//! Swap two numbers without temp
//!
//! ## Problem Link
//! https://github.com/aarambh-darshan/100-rust-problems/blob/main/problems/015_swap_two_numbers.md
//! 

struct Solution;

impl Solution {
    pub fn swap(a: i32, b: i32) -> (i32, i32) {
        let a = a ^ b;
        let b = a ^ b;
        let a = a ^ b;
        (a, b)
    }

    pub fn swap_tuple(a: i32, b: i32) -> (i32, i32) {
        (b, a)
    }

    pub fn swap_add_sub(a: i32, b: i32) -> (i32, i32) {
        let a = a + b;
        let b = a - b;
        let a = a - b;
        (a, b)
    }
}

/// Solution for Swap Two Numbers
pub fn solve() {
    // TODO: Implement your solution here
    unimplemented!("Solve Swap Two Numbers");

    // Test the swap function
    let a = 3;
    let b = 5;
    println!("Before swap: a = {}, b = {}", a, b);
    let (a, b) = Solution::swap(a, b);
    println!("After swap: a = {}, b = {}", a, b);
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        // TODO: Add test cases
        assert_eq!(Solution::swap(3, 5), (5, 3));
        assert_eq!(Solution::swap(10, 20), (20, 10));
        assert_eq!(Solution::swap(7, 9), (9, 7));
        assert_eq!(Solution::swap(15, 25), (25, 15));
    }

    #[test]
    fn test_swap_tuple() {
        assert_eq!(Solution::swap_tuple(3, 5), (5, 3));
        assert_eq!(Solution::swap_tuple(10, 20), (20, 10));
        assert_eq!(Solution::swap_tuple(7, 9), (9, 7));
        assert_eq!(Solution::swap_tuple(15, 25), (25, 15));
    }

    #[test]
    fn test_swap_add_sub() {
        assert_eq!(Solution::swap_add_sub(3, 5), (5, 3));
        assert_eq!(Solution::swap_add_sub(10, 20), (20, 10));
        assert_eq!(Solution::swap_add_sub(7, 9), (9, 7));
        assert_eq!(Solution::swap_add_sub(15, 25), (25, 15));
    }
}
