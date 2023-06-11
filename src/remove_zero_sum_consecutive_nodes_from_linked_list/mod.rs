// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

struct Solution;

impl Solution {
    pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        let mut sum = 0;
        let mut cursor = head.clone();
        while cursor.is_some() {
            sum += cursor.as_ref().unwrap().val;
            if sum == 0 {
                return Self::remove_zero_sum_sublists(cursor.unwrap().next);
            }
            cursor = cursor.unwrap().next;
        }
        let next = head.clone().unwrap().next;
        let mut node = head.clone().unwrap();
        node.next = Self::remove_zero_sum_sublists(next);
        Some(node)
    }
}

#[cfg(test)]
mod tests {
    use crate::remove_zero_sum_consecutive_nodes_from_linked_list::*;

    #[test]
    fn test_remove_zero_sum_sublists() {
        let head = Some(
            Box::new(ListNode {
                val: 1,
                next: Some(
                    Box::new(ListNode {
                        val: 2,
                        next: Some(
                            Box::new(ListNode {
                                val: -3,
                                next: Some(
                                    Box::new(ListNode {
                                        val: 3,
                                        next: Some(Box::new(ListNode::new(1))),
                                    })
                                ),
                            })
                        ),
                    })
                ),
            })
        );
        let ans = Solution::remove_zero_sum_sublists(head);
        assert_eq!(
            ans,
            Some(
                Box::new(ListNode {
                    val: 3,
                    next: Some(
                        Box::new(ListNode {
                            val: 1,
                            next: None,
                        })
                    ),
                })
            )
        );
        let head = Some(
            Box::new(ListNode {
                val: 1,
                next: Some(
                    Box::new(ListNode {
                        val: 2,
                        next: Some(
                            Box::new(ListNode {
                                val: 3,
                                next: Some(
                                    Box::new(ListNode {
                                        val: -3,
                                        next: Some(Box::new(ListNode::new(-2))),
                                    })
                                ),
                            })
                        ),
                    })
                ),
            })
        );
        let ans = Solution::remove_zero_sum_sublists(head);
        assert_eq!(ans, Some(Box::new(ListNode::new(1))));
    }
}
