package reorderlist

/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */

func reorderList(head *ListNode) {
	l := listSize(head)

	mid := head

	for i := 0; 2*i < l-2; i++ {
		mid = mid.Next
	}
	mid, mid.Next = mid.Next, nil

	m1 := head
	m2 := reverseList(mid)

	for m1 != nil && m2 != nil {
		m1.Next, m1, m2.Next, m2 = m2, m1.Next, m1.Next, m2.Next
	}
	res := head
	_ = res
}

func listSize(h *ListNode) int {
	if h == nil {
		return 0
	} else {
		size := 0
		for h != nil {
			size++
			h = h.Next
		}
		return size
	}
}

func reverseList(head *ListNode) *ListNode {
	if head == nil {
		return nil
	} else if head.Next == nil {
		return head
	} else {
		next := head.Next
		head.Next = nil
		return rlHelper(next, head)
	}
}

func rlHelper(list *ListNode, tail *ListNode) *ListNode {
	if list.Next == nil {
		list.Next = tail
		return list
	} else {
		tmp := list.Next
		list.Next = tail
		return rlHelper(tmp, list)
	}
}
