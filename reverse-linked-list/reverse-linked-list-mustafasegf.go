package reverselinkedlist

/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func reverseList(head *ListNode) *ListNode {
	var ans *ListNode
	var n *ListNode
	p := head

	for p != nil {
		n = p.Next
		p.Next = ans
		ans, p = p, n
	}
	return ans
}
