package mergeksortedlists

/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func mergeKLists(lists []*ListNode) *ListNode {
	n := len(lists)
	if n < 1 {
		return nil
	}
	if n == 1 {
		return lists[0]
	}

	m := n / 2
	l := mergeKLists(lists[:m])
	r := mergeKLists(lists[m:])
	return mergeTwoLists(l, r)
}

func mergeTwoLists(l1 *ListNode, l2 *ListNode) *ListNode {
	ans := &ListNode{}
	p := ans

	for l1 != nil || l2 != nil {
		if l1 == nil {
			p.Next = l2
			return ans.Next
		}

		if l2 == nil {
			p.Next = l1
			return ans.Next
		}

		if l1.Val < l2.Val {
			p.Next = l1
			l1 = l1.Next
		} else {
			p.Next = l2
			l2 = l2.Next
		}
		p = p.Next
	}
	return ans.Next
}
