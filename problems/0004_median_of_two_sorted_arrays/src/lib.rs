// Disclaimer: My main reason for this solution is that the problem statement
// states that "The overall run time complexity should be O(log (m+n))".
// 
// My solution's overall run time complexity is O(log(min(m, n))), which
// is better.

trait OptionOrd<T> {
    fn optional_max(&self, other: Option<T>) -> Option<T>;
    fn optional_min(&self, other: Option<T>) -> Option<T>;
}

impl<T: Ord + Copy> OptionOrd<T> for Option<T> {
    fn optional_max(&self, other: Option<T>) -> Option<T> {
        match (self, other) {
            (Some(i), None) => Some(*i),
            (None, Some(j)) => Some(j),
            (Some(i), Some(j)) => Some((*i).max(j)),
            _ => None,
        }
    }
    
    fn optional_min(&self, other: Option<T>) -> Option<T> {
        match (self, other) {
            (Some(i), None) => Some(*i),
            (None, Some(j)) => Some(j),
            (Some(i), Some(j)) => Some((*i).min(j)),
            _ => None,
        }
    }
}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // My solution for this problem is as follows:
        // 
        // 1. Finding the median of N (i.e. 13) numbers is the same as
        // finding the number (or numbers) that comes after the (N-1)/2 (i.e. 6)
        // smallest numbers.
        // 
        // 2. As such, we will run a binary search on the smallest vector
        // and try to find sets of `nums1` and `nums2` that, when appended,
        // have (N-1)/2 elements.

        let total_len = nums1.len() + nums2.len();
        let nums_before = (total_len - 1) / 2 ;
        
        let (smallest_nums, biggest_nums) = if nums1.len() < nums2.len() {
            (nums1, nums2)
        } else {
            (nums2, nums1)
        };
        
        let nums_used_from_smallest = if smallest_nums.is_empty() {
            0
        } else {
            let mut left = 0;
            let mut right = smallest_nums.len() - 1;
            
            loop {
                if left > right {
                    break smallest_nums.len();
                }
                
                let mid = (left + right) / 2;
                let nums_needed_from_biggest = nums_before - mid;
                
                let first_out_biggest = biggest_nums[nums_needed_from_biggest];
                let last_in_biggest = if nums_needed_from_biggest > 0 {
                    biggest_nums.get(nums_needed_from_biggest - 1)
                } else {
                    None 
                };
                
                let first_out_smallest = smallest_nums[mid];
                let last_in_smallest = if mid > 0 {
                    smallest_nums.get(mid - 1)
                } else {
                    None
                };
                
                if let Some(lis) = last_in_smallest && *lis > first_out_biggest {
                    right = mid - 1;
                } else if let Some(lib) = last_in_biggest && *lib > first_out_smallest {
                    left = mid + 1;
                } else {
                    break mid;
                }
            }
        };
        
        let nums_used_from_biggest = nums_before - nums_used_from_smallest;
        
        // Now that we know how many numbers we need to use from the smallest
        // vector, we know that the following is true for the median:
        //
        // - if there is an odd number of numbers, the median is the smallest
        // number from the smallest and biggest vectors, after excluding the
        // (N-1)/2 smallest numbers.
        // 
        // - if there is an even number of numbers, the median is the average of
        // two numbers, after excluding the (N-1)/2 smallest numbers. Those two
        // numbers can both come from the smallest vector, or from the biggest
        // vector, or one can come from the smallest vector and the other one 
        // from the biggest vector.
        
        let first_out_smallest_idx = nums_used_from_smallest;        
        let first_out_biggest_idx = nums_used_from_biggest;
        
        if total_len % 2 == 1 {
            let first_out_smallest = smallest_nums.get(first_out_smallest_idx);
            let first_out_biggest = biggest_nums.get(first_out_biggest_idx);
            
            return *first_out_smallest.optional_min(first_out_biggest)
                .expect("both arrays use all elements") as f64;
        } else {
            let first_out_smallest = smallest_nums.get(first_out_smallest_idx);
            let second_out_smallest = smallest_nums.get(first_out_smallest_idx + 1);
            
            let first_out_biggest = biggest_nums.get(first_out_biggest_idx);
            let second_out_biggest = biggest_nums.get(first_out_biggest_idx + 1);
            
            let first_min = *first_out_smallest.optional_min(first_out_biggest)
                .expect("both arrays use all elements") as f64;
            
            let maybe_second_min = second_out_smallest.optional_min(second_out_biggest);
            
            let second_min = *first_out_smallest.or(maybe_second_min)
                .optional_max(first_out_biggest.or(maybe_second_min))
                .expect("there is no second element for median") as f64;
            
            return (first_min + second_min) / 2.0;
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solution::find_median_sorted_arrays(vec![1, 3], vec![2]);
        assert_eq!(result, 2.0);
    }
    
    #[test]
    fn test_2() {
        let result = Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]);
        assert_eq!(result, 2.5);
    }
    
    #[test]
    fn test_3() {
        let result = Solution::find_median_sorted_arrays(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 7, 11, 20, 25]);
        assert_eq!(result, 6.0);
    }
    
    #[test]
    fn test_4() {
        let result = Solution::find_median_sorted_arrays(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![1, 1, 1, 1, 1]);
        assert_eq!(result, 2.0);
    }
    
    #[test]
    fn test_5() {
        let result = Solution::find_median_sorted_arrays(vec![1], vec![]);
        assert_eq!(result, 1.0);
    }
    
    #[test]
    fn test_6() {
        let result = Solution::find_median_sorted_arrays(vec![1], vec![7]);
        assert_eq!(result, 4.0);
    }
    
    #[test]
    fn test_7() {
        let result = Solution::find_median_sorted_arrays(vec![], vec![2, 3]);
        assert_eq!(result, 2.5);
    }
}
