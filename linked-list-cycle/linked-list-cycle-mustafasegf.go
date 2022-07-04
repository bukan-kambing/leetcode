package linkedlistcycle

/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func hasCycle(head *ListNode) bool {
	for head != nil {
		if head.Val == 100001 {
			return true
		}
		head.Val = 100001
		head = head.Next

	}

	return false
}
