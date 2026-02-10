//! # Reverse String
//! 
//! **Difficulty**: 🟢 Beginner
//! 
//! Reverse a string in-place
//!
//! ## Problem Link
//! https://github.com/aarambh-darshan/100-rust-problems/blob/main/problems/002_reverse_string.md


struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let n = s.len();
        let mut left = 0;
        let mut right = n.saturating_sub(1);
        while left < right {
            s.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
}

/// Solution for Reverse String
pub fn solve() {
    // TODO: Implement your solution here
    unimplemented!("Solve Reverse String");
    // Test Case 1
    let mut s1 = vec!['h', 'e', 'l', 'l', 'o'];
    println!("Before: {:?}", s1);
    Solution::reverse_string(&mut s1);
    println!("After:  {:?}", s1); // ['o', 'l', 'l', 'e', 'h']
    
    // Test Case 2
    let mut s2 = vec!['H', 'a', 'n', 'n', 'a', 'h'];
    println!("\nBefore: {:?}", s2);
    Solution::reverse_string(&mut s2);
    println!("After:  {:?}", s2); // ['h', 'a', 'n', 'n', 'a', 'H']
    
    // Test Case 3 - Single character
    let mut s3 = vec!['a'];
    Solution::reverse_string(&mut s3);
    println!("\nSingle char: {:?}", s3); // ['a']
    
    // Test Case 4 - Empty
    let mut s4: Vec<char> = vec![];
    Solution::reverse_string(&mut s4);
    println!("Empty: {:?}", s4); // []
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        // TODO: Add test cases
            let mut s = vec!['h', 'e', 'l', 'l', 'o'];
            Solution::reverse_string(&mut s);
            assert_eq!(s, vec!['o', 'l', 'l', 'e', 'h']);

            let mut s = vec!['H', 'a', 'n', 'n', 'a', 'h'];
            Solution::reverse_string(&mut s);
            assert_eq!(s, vec!['h', 'a', 'n', 'n', 'a', 'H']);
    }
}
