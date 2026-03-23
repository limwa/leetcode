impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let len = s.len() as i32;
        let input_letters = s.chars().collect::<Vec<_>>();
        let mut output_letters = Vec::with_capacity(s.len());
        
        for curr_row in 0..num_rows {
            let mut curr_index = curr_row;
            let mut going_down = true;
            
            let rows_below = num_rows - curr_row - 1;
            let rows_above = curr_row;
            
            let (delta_down, delta_up) = match (2 * rows_below, 2 * rows_above) {
                (0, 0) => (1, 1),
                (down, 0) => (down, down),
                (0, up) => (up, up),
                (down, up) => (down, up),
            };
            
            while curr_index < len {
                output_letters.push(input_letters[curr_index as usize]);
                
                let delta = if going_down { delta_down } else { delta_up };
                
                curr_index = curr_index + delta;
                going_down = !going_down;
            }
        }
        
        output_letters.iter().collect()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!("PAHNAPLSIIGYIR", Solution::convert("PAYPALISHIRING".to_owned(), 3))
    }
    
    #[test]
    fn test_2() {
        assert_eq!("PINALSIGYAHRPI", Solution::convert("PAYPALISHIRING".to_owned(), 4))
    }
    
    #[test]
    fn test_3() {
        assert_eq!("A", Solution::convert("A".to_owned(), 1))
    }
}
