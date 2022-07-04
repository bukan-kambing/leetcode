package removenthnodefromendoflist

/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func removeNthFromEnd(head *ListNode, n int) *ListNode {
	if head == nil {
		return nil
	}

	p := head
	cnt := 0
	for p != nil {
		cnt++
		p = p.Next
	}

	cnt -= n
	ans := &ListNode{Next: head}
	p = ans
	for cnt != 0 {
		p = p.Next
		cnt--
	}

	p.Next = p.Next.Next

	return ans.Next
}
