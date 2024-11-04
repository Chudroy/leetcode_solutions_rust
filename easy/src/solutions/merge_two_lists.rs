use crate::models::Solution;
use crate::models::ListNode;
/**
 * 21. Merge Two Sorted Lists
 * You are given the heads of two sorted linked lists list1 and list2.
 * Merge the two lists into one sorted list. The list should be made by
 * splicing together the nodes of the first two lists.
 * Return the head of the merged linked list.
 */

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut list1 = list1;
        let mut list2 = list2;

        let mut prehead = ListNode::new(-1);
        let mut cur_node = &mut prehead;

        while let (Some(node1), Some(node2)) = (&list1, &list2) {
            if node1.val < node2.val {
                cur_node.next = list1.take(); // take() moves the node from list1 to cur_node.next
                cur_node = cur_node.next.as_mut().unwrap(); //
                list1 = cur_node.next.take(); // take() moves the node from cur_node.next to list1
            } else {
                cur_node.next = list2.take();
                cur_node = cur_node.next.as_mut().unwrap();
                list2 = cur_node.next.take();
            }
        }
        cur_node.next = list1.or(list2);
        prehead.next
    }
}
