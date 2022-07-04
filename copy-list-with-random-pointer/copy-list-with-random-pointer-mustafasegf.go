package copylistwithrandompointer

/**
 * Definition for a Node.
 * type Node struct {
 *     Val int
 *     Next *Node
 *     Random *Node
 * }
 */

func copyRandomList(head *Node) *Node {
	if head == nil {
		return nil
	}

	set := map[*Node]*Node{}
	p := head
	for p != nil {
		set[p] = &Node{Val: p.Val}
		p = p.Next
	}

	p = head
	for p != nil {
		set[p].Next = set[p.Next]
		set[p].Random = set[p.Random]

		p = p.Next
	}

	return set[head]
}
