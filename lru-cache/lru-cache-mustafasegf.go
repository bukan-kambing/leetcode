package lrucache

type LRUCache struct {
	Head *Node
	Tail *Node
	Mem  map[int]*Node
	Cap  int
}

type Node struct {
	Key  int
	Val  int
	Prev *Node
	Next *Node
}

func Constructor(capacity int) LRUCache {
	return LRUCache{Mem: make(map[int]*Node), Cap: capacity}
}

func (l *LRUCache) Get(key int) int {
	if n, ok := l.Mem[key]; ok {
		l.Pop(n)
		l.Add(n)
		return n.Val
	}
	return -1
}

func (l *LRUCache) Put(key int, value int) {
	if n, ok := l.Mem[key]; ok {
		n.Val = value
		l.Pop(n)
		l.Add(n)
	} else {
		n = &Node{Key: key, Val: value}
		l.Mem[key] = n
		l.Add(n)
	}

	if len(l.Mem) > l.Cap {
		delete(l.Mem, l.Tail.Key)
		l.Pop(l.Tail)
	}
}

func (l *LRUCache) Add(n *Node) {
	n.Prev = nil
	n.Next = l.Head

	if l.Head != nil {
		l.Head.Prev = n
	}
	l.Head = n

	if l.Tail == nil {
		l.Tail = n
	}
}

func (l *LRUCache) Pop(n *Node) {
	if n != l.Head {
		n.Prev.Next = n.Next
	} else {
		l.Head = n.Next
	}

	if n != l.Tail {
		n.Next.Prev = n.Prev
	} else {
		l.Tail = n.Prev
	}
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * obj := Constructor(capacity);
 * param_1 := obj.Get(key);
 * obj.Put(key,value);
 */
