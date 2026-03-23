use std::collections::HashMap;

fn is_palindrome(s: &str) -> bool {
    let half_length = (s.len() + 1) / 2;
    s.chars().take(half_length).eq(s.chars().rev().take(half_length))
}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        // We could simply make this with a few for loops, but to make it faster
        // we can simply index the positions of every character in order. That's
        // helpful because a palindrome must start and end with the same characters.

        let mut char_indices = HashMap::<char, Vec<usize>>::new();
        
        for (i, c) in s.char_indices() {
            if let Some(indices) = char_indices.get_mut(&c) {
                indices.push(i);
            } else {
                char_indices.insert(c, vec![i]);
            }
        }
        
        // Now, we iterate once more over every character, check for all possible
        // endings and, for each possibility, check if it's a palindrome.
        
        let mut longest_palindrome = "";
        
        for (i, c) in s.char_indices() {
            let remaining_length = s.len() - i;
            if remaining_length <= longest_palindrome.len() {
                // Early break because, at this point, no matter the palindrome
                // we find, it's impossible to get one that's longer.
                break;
            }
            
            let indices = char_indices.get(&c).unwrap();
            
            // We iterate in reverse order so we find the biggest palindromes
            // first, hopefully skipping a lot of shorter palindromes.
            for j in indices.iter().rev().take_while(|x| **x >= i) {
                // Now that we have a possible ending, we need to check if it's a palindrome.
                 
                // Furthermore, we can just ignore it if its length won't be
                // greater than the best one we already have.
                let length = j - i + 1;
                if length < longest_palindrome.len() {
                    break;
                }
                
                let substr = &s[i..=*j];
                if is_palindrome(substr) {
                    longest_palindrome = substr;
                }
            }
        }
        
        return longest_palindrome.to_owned()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let possible_answers = vec!["bab".to_owned(), "aba".to_owned()];
        let answer = Solution::longest_palindrome("babad".to_owned());
        assert!(possible_answers.contains(&answer), "{possible_answers:?}.contains({answer:?}) is false");
    }
    
    #[test]
    fn test_2() {
        let answer = Solution::longest_palindrome("cbbd".to_owned());
        assert_eq!(answer, "bb");
    }
    
    #[test]
    fn test_3() {
        let answer = Solution::longest_palindrome("a".to_owned());
        assert_eq!(answer, "a");
    }
    
    #[test]
    fn test_4() {
        let answer = Solution::longest_palindrome("bb".to_owned());
        assert_eq!(answer, "bb");
    }
}
