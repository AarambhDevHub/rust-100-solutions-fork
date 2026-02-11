//! # Find Maximum
//! 
//! **Difficulty**: 🟢 Beginner
//! 
//! Find the maximum element in an array
//!
//! ## Problem Link
//! https://github.com/aarambh-darshan/100-rust-problems/blob/main/problems/009_find_maximum.md

struct Solution;

impl Solution {
    pub fn find_max(nums: Vec<i32>) -> Option<i32> {
      if nums.is_empty() {
          return None;
      }

      let mut max = nums[0];
      for num in nums {
        if num > max {
            max = num;
        }
      }
      Some(max)
    }

    pub fn find_max_iter(nums: Vec<i32>) -> Option<i32> {
        nums.iter().copied().max()
    }

    pub fn find_max_fold(nums: Vec<i32>) -> Option<i32> {
        nums.iter().copied().reduce(|a, b| if a > b { a } else { b })
    }
}

/// Solution for Find Maximum
pub fn solve() {
    // TODO: Implement your solution here
    unimplemented!("Solve Find Maximum");

        // Test Case 1: Regular array
    let nums1 = vec![3, 1, 4, 1, 5, 9, 2, 6];
    println!("Max of {:?} = {:?}", nums1, Solution::find_max(nums1));
    // Output: Max of [3, 1, 4, 1, 5, 9, 2, 6] = Some(9)
    
    // Test Case 2: Negative numbers
    let nums2 = vec![-5, -2, -8, -1, -9];
    println!("Max of {:?} = {:?}", nums2, Solution::find_max(nums2));
    // Output: Max of [-5, -2, -8, -1, -9] = Some(-1)
    
    // Test Case 3: Single element
    let nums3 = vec![42];
    println!("Max of {:?} = {:?}", nums3, Solution::find_max(nums3));
    // Output: Max of [42] = Some(42)
    
    // Test Case 4: Empty array
    let nums4: Vec<i32> = vec![];
    println!("Max of {:?} = {:?}", nums4, Solution::find_max(nums4));
    // Output: Max of [] = None
    
    // Test Case 5: All same elements
    let nums5 = vec![7, 7, 7, 7];
    println!("Max of {:?} = {:?}", nums5, Solution::find_max(nums5));
    // Output: Max of [7, 7, 7, 7] = Some(7)
    
    // Using the value safely
    if let Some(max) = Solution::find_max(nums1) {
        println!("\nThe maximum value is: {}", max);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        // TODO: Add test cases
        println!("Maximum in [1, 2, 3, 4, 5]: {:?}", Solution::find_max(vec![1, 2, 3, 4, 5]));
        assert_eq!(Solution::find_max(vec![1, 2, 3]), Some(3));
        assert_eq!(Solution::find_max(vec![-1, -2, -3]), Some(-1));
        assert_eq!(Solution::find_max(vec![10, 20, 30]), Some(30));
        assert_eq!(Solution::find_max(vec![5]), Some(5));
    }

    #[test]
    fn test_iter() {
        println!("Maximum in [1, 2, 3, 4, 5] using iter: {:?}", Solution::find_max_iter(vec![1, 2, 3, 4, 5]));
        assert_eq!(Solution::find_max_iter(vec![1, 2, 3]), Some(3));
        assert_eq!(Solution::find_max_iter(vec![-1, -2, -3]), Some(-1));
        assert_eq!(Solution::find_max_iter(vec![10, 20, 30]), Some(30));
        assert_eq!(Solution::find_max_iter(vec![5]), Some(5));
    }

    #[test]
    fn test_fold() {
        println!("Maximum in [1, 2, 3, 4, 5] using fold: {:?}", Solution::find_max_fold(vec![1, 2, 3, 4, 5]));
        assert_eq!(Solution::find_max_fold(vec![1, 2, 3]), Some(3));
        assert_eq!(Solution::find_max_fold(vec![-1, -2, -3]), Some(-1));
        assert_eq!(Solution::find_max_fold(vec![10, 20, 30]), Some(30));
        assert_eq!(Solution::find_max_fold(vec![5]), Some(5));
    }
}
