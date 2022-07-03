package addtwonumbers

/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func addTwoNumbers(l1 *ListNode, l2 *ListNode) *ListNode {
	carry := 0
	ans := &ListNode{}
	p := ans

	for l1 != nil || l2 != nil {
		t := &ListNode{Val: carry}
		if l1 != nil {
			t.Val += l1.Val
			l1 = l1.Next
		}
		if l2 != nil {
			t.Val += l2.Val
			l2 = l2.Next
		}
		carry = t.Val / 10
		t.Val %= 10
		p.Next = t
		p = p.Next
	}

	if carry == 1 {
		p.Next = &ListNode{Val: carry}
	}

	return ans.Next
}
