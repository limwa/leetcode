use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut start = 0;
        let mut max_len = 0;
        
        let mut last_seen = HashMap::<char, i32>::new();
        
        for (i, c) in s.char_indices() {
            let index = i as i32;
            
            if let Some(last_index) = last_seen.get_mut(&c) {
                start = start.max(*last_index + 1);
                *last_index = index;
            } else {
                last_seen.insert(c, index);
            }
            
            max_len = max_len.max(index - start + 1);
        }
        
        max_len
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solution::length_of_longest_substring("abcabcbb".to_string());
        assert_eq!(result, 3);
    }
    
    #[test]
    fn test_2() {
        let result = Solution::length_of_longest_substring("bbbbb".to_string());
        assert_eq!(result, 1);
    }
    
    #[test]
    fn test_3() {
        let result = Solution::length_of_longest_substring("pwwkew".to_string());
        assert_eq!(result, 3);
    }    
}
