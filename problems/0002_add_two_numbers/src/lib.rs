impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result = None;
        let mut current = &mut result;
        let mut carry = 0;
        
        let mut head1 = &l1;
        let mut head2 = &l2;
        
        while let Some(d1) = head1 && let Some(d2) = head2 {
            let sum = d1.val + d2.val + carry;
            carry = sum / 10;
            
            *current = Some(Box::new(ListNode::new(sum % 10)));
            current = &mut current.as_mut().unwrap().next;
            
            head1 = &d1.next;
            head2 = &d2.next;
        }
        
        while let Some(d1) = head1 {
            let sum = d1.val + carry;
            carry = sum / 10;
            
            *current = Some(Box::new(ListNode::new(sum % 10)));
            current = &mut current.as_mut().unwrap().next;
            
            head1 = &d1.next;
        }
        
        while let Some(d2) = head2 {
            let sum = d2.val + carry;
            carry = sum / 10;
            
            *current = Some(Box::new(ListNode::new(sum % 10)));
            current = &mut current.as_mut().unwrap().next;
            
            head2 = &d2.next;
        }
        
        if carry > 0 {
            *current = Some(Box::new(ListNode::new(carry)));
        }
        
        result
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    
    fn make_linked_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut current = &mut head;
        
        for num in nums {
            let new_node = Box::new(ListNode::new(num));
            *current = Some(new_node);
            current = &mut current.as_mut().unwrap().next;
        }
        
        head
    }

    #[test]
    fn test_1() {
        let result = Solution::add_two_numbers(
            make_linked_list(vec![2, 4, 3]),
            make_linked_list(vec![5, 6, 4])
        );
        
        assert_eq!(result, make_linked_list(vec![7, 0, 8]));
    }
    
    #[test]
    fn test_2() {
        let result = Solution::add_two_numbers(
            make_linked_list(vec![0]),
            make_linked_list(vec![0])
        );
        
        assert_eq!(result, make_linked_list(vec![0]));
    }
    
    #[test]
    fn test_3() {
        let result = Solution::add_two_numbers(
            make_linked_list(vec![9, 9, 9, 9, 9, 9, 9]),
            make_linked_list(vec![9, 9, 9, 9])
        );
        
        assert_eq!(result, make_linked_list(vec![8, 9, 9, 9, 0, 0, 0, 1]));
    }
}
