use std::{collections::HashMap, panic};
use itertools::enumerate;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut needed_indices = HashMap::with_capacity(nums.len());
        
        for (index, num) in enumerate(nums) {
            if let Some(needed_index) = needed_indices.get(&num) {
                return vec![*needed_index as i32, index as i32];
            }
            
            let needed = target - num;
            needed_indices.insert(needed, index);
        }
        
        panic!("No two sum solution");
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solution::two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(result, vec![0, 1]);
    }
    
    #[test]
    fn test_2() {
        let result = Solution::two_sum(vec![3, 2, 4], 6);
        assert_eq!(result, vec![1, 2]);
    }
    
    #[test]
    fn test_3() {
        let result = Solution::two_sum(vec![3, 3], 6);
        assert_eq!(result, vec![0, 1]);
    }
}
