package minstack

type MinStack struct {
	Min   []int
	Stack []int
}

func Constructor() MinStack {
	return MinStack{Stack: make([]int, 0, 30000), Min: make([]int, 0, 30000)}
}

func (this *MinStack) Push(val int) {
	this.Stack = append(this.Stack, val)
	if len(this.Min) == 0 {
		this.Min = append(this.Min, 0)
	} else {
		// last min is less then val
		if this.Stack[this.Min[len(this.Min)-1]] < val {
			this.Min = append(this.Min, this.Min[len(this.Min)-1])
		} else {
			this.Min = append(this.Min, len(this.Stack)-1)
		}
	}
}

func (this *MinStack) Pop() {
	this.Stack = this.Stack[:len(this.Stack)-1]
	this.Min = this.Min[:len(this.Min)-1]
}

func (this *MinStack) Top() int {
	return this.Stack[len(this.Stack)-1]
}

func (this *MinStack) GetMin() int {
	return this.Stack[this.Min[len(this.Min)-1]]
}

/**
 * Your MinStack object will be instantiated and called as such:
 * obj := Constructor();
 * obj.Push(val);
 * obj.Pop();
 * param_3 := obj.Top();
 * param_4 := obj.GetMin();
 */
