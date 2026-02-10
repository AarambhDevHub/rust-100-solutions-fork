//! # Two Sum
//!
//! **Difficulty**: 🟢 Beginner
//!
//! Find indices of two numbers that add up to target
//!
//! ## Problem Link
//! https://github.com/aarambh-darshan/100-rust-problems/blob/main/problems/001_two_sum.md

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&index) = map.get(&complement) {
                return vec![index as i32, i as i32];
            }
            map.insert(num, i);
        }
        vec![]
    }
}

/// Solution for Two Sum
pub fn solve() {
    // Test Case 1
    let nums1 = vec![2, 7, 11, 15];
    let target1 = 9;
    let result1 = Solution::two_sum(nums1, target1);
    println!("Test 1: {:?}", result1); // Output: [0, 1]
    
    // Test Case 2
    let nums2 = vec![3, 2, 4];
    let target2 = 6;
    let result2 = Solution::two_sum(nums2, target2);
    println!("Test 2: {:?}", result2); // Output: [1, 2]
    
    // Test Case 3
    let nums3 = vec![3, 3];
    let target3 = 6;
    let result3 = Solution::two_sum(nums3, target3);
    println!("Test 3: {:?}", result3); // Output: [0, 1]
    // TODO: Implement your solution here
    // unimplemented!("Solve Two Sum")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        // TODO: Add test cases
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
