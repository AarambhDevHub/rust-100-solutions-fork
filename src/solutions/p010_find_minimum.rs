//! # Find Minimum
//! 
//! **Difficulty**: 🟢 Beginner
//! 
//! Find the minimum element in an array
//!
//! ## Problem Link
//! https://github.com/aarambh-darshan/100-rust-problems/blob/main/problems/010_find_minimum.md

struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> Option<i32> {
        if nums.is_empty() {
            return None;
        }
        let mut min = nums[0];
        for num in nums {
            if num < min {
                min = num;
            }
        }
        Some(min)
    }

    pub fn find_min_iter(nums: Vec<i32>) -> Option<i32> {
        nums.iter().copied().min()
    }

    pub fn find_min_fold(nums: Vec<i32>) -> Option<i32> {
        nums.into_iter().reduce(|min, x| if x < min { x } else { min })
    }
}

/// Solution for Find Minimum
pub fn solve() {
    // TODO: Implement your solution here
        // Test Case 1: Regular array
    let nums1 = vec![3, 1, 4, 1, 5, 9, 2, 6];
    println!("Min of {:?} = {:?}", nums1, Solution::find_min(nums1.clone()));
    // Output: Min of [3, 1, 4, 1, 5, 9, 2, 6] = Some(1)
    
    // Test Case 2: Negative numbers
    let nums2 = vec![-5, -2, -8, -1, -9];
    println!("Min of {:?} = {:?}", nums2, Solution::find_min(nums2.clone()));
    // Output: Min of [-5, -2, -8, -1, -9] = Some(-9)
    
    // Test Case 3: Single element
    let nums3 = vec![42];
    println!("Min of {:?} = {:?}", nums3, Solution::find_min(nums3.clone()));
    // Output: Min of [42] = Some(42)
    
    // Test Case 4: Empty
    let nums4: Vec<i32> = vec![];
    println!("Min of {:?} = {:?}", nums4, Solution::find_min(nums4.clone()));
    // Output: Min of [] = None
    
    // Finding both min and max in one pass
    let nums = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let min = Solution::find_min(nums.clone());
    let max = nums.iter().copied().max();
    println!("\nRange: {:?} to {:?}", min, max);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        // TODO: Add test cases
        println!("Minimum in [3, 1, 4, 1, 5]: {:?}", Solution::find_min(vec![3, 1, 4, 1, 5]));
        assert_eq!(Solution::find_min(vec![3, 1, 4, 1, 5]), Some(1));
        assert_eq!(Solution::find_min(vec![-5, -2, -8, -1, -9]), Some(-9));
        assert_eq!(Solution::find_min(vec![10, 20, 30]), Some(10));
        assert_eq!(Solution::find_min(vec![5]), Some(5));
        assert_eq!(Solution::find_min(vec![]), None);
    }

    #[test]
    fn test_iter() {
        println!("Minimum in [3, 1, 4, 1, 5] using iter: {:?}", Solution::find_min_iter(vec![3, 1, 4, 1, 5]));
        assert_eq!(Solution::find_min_iter(vec![3, 1, 4, 1, 5]), Some(1));
        assert_eq!(Solution::find_min_iter(vec![-5, -2, -8, -1, -9]), Some(-9));
        assert_eq!(Solution::find_min_iter(vec![10, 20, 30]), Some(10));
        assert_eq!(Solution::find_min_iter(vec![5]), Some(5));
        assert_eq!(Solution::find_min_iter(vec![]), None);
    }

    #[test]
    fn test_fold() {
        println!("Minimum in [3, 1, 4, 1, 5] using fold: {:?}", Solution::find_min_fold(vec![3, 1, 4, 1, 5]));
        assert_eq!(Solution::find_min_fold(vec![3, 1, 4, 1, 5]), Some(1));
        assert_eq!(Solution::find_min_fold(vec![-5, -2, -8, -1, -9]), Some(-9));
        assert_eq!(Solution::find_min_fold(vec![10, 20, 30]), Some(10));
        assert_eq!(Solution::find_min_fold(vec![5]), Some(5));
        assert_eq!(Solution::find_min_fold(vec![]), None);
    }
}
