//! # Sum of Array
//! 
//! **Difficulty**: 🟢 Beginner
//! 
//! Calculate sum of all array elements
//!
//! ## Problem Link
//! https://github.com/aarambh-darshan/100-rust-problems/blob/main/problems/008_sum_of_array.md


struct Solution;

impl Solution {
    pub fn sum_array(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        for num in nums {
            sum += num;
        }
        sum
    }

    pub fn sum_array_iter(nums: Vec<i32>) -> i32 {
        nums.iter().sum()
    }

    pub fn sum_array_fold(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(0, |acc, x| acc + x)
    }
}

/// Solution for Sum of Array
pub fn solve() {
    // TODO: Implement your solution here
    unimplemented!("Solve Sum of Array");

    // Test Case 1: Regular array
    let nums1 = vec![1, 2, 3, 4, 5];
    println!("Sum of {:?} = {}", nums1, Solution::sum_array(nums1.clone()));
    // Output: Sum of [1, 2, 3, 4, 5] = 15
    
    // Test Case 2: Array with negative numbers
    let nums2 = vec![-1, 2, -3, 4, -5];
    println!("Sum of {:?} = {}", nums2, Solution::sum_array(nums2.clone()));
    // Output: Sum of [-1, 2, -3, 4, -5] = -3
    
    // Test Case 3: Empty array
    let nums3: Vec<i32> = vec![];
    println!("Sum of {:?} = {}", nums3, Solution::sum_array(nums3.clone()));
    // Output: Sum of [] = 0
    
    // Test Case 4: Single element
    let nums4 = vec![42];
    println!("Sum of {:?} = {}", nums4, Solution::sum_array(nums4.clone()));
    // Output: Sum of [42] = 42
    
    // Compare all implementations
    let nums = vec![1, 2, 3, 4, 5];
    println!("\nAll methods give same result:");
    println!("Loop:     {}", Solution::sum_array(nums.clone()));
    println!("Iterator: {}", Solution::sum_array_iter(nums.clone()));
    println!("Fold:     {}", Solution::sum_array_fold(nums));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        // TODO: Add test cases
        println!("Sum of [1, 2, 3, 4, 5]: {:?}", Solution::sum_array(vec![1, 2, 3, 4, 5]));
        assert_eq!(Solution::sum_array(vec![]), 0);
        assert_eq!(Solution::sum_array(vec![1, 2, 3]), 6);
        assert_eq!(Solution::sum_array(vec![-1, -2, -3]), -6);
        assert_eq!(Solution::sum_array(vec![10, 20, 30]), 60);
    }

    #[test]
    fn test_iter_method() {
        assert_eq!(Solution::sum_array_iter(vec![]), 0);
        assert_eq!(Solution::sum_array_iter(vec![1, 2, 3]), 6);
        assert_eq!(Solution::sum_array_iter(vec![-1, -2, -3]), -6);
        assert_eq!(Solution::sum_array_iter(vec![10, 20, 30]), 60);
    }

    #[test]
    fn test_fold_method() {
        assert_eq!(Solution::sum_array_fold(vec![]), 0);
        assert_eq!(Solution::sum_array_fold(vec![1, 2, 3]), 6);
        assert_eq!(Solution::sum_array_fold(vec![-1, -2, -3]), -6);
        assert_eq!(Solution::sum_array_fold(vec![10, 20, 30]), 60);
    }
}