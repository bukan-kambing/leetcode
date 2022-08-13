package miniparser

/**
 * // This is the interface that allows for creating nested lists.
 * // You should not implement it, or speculate about its implementation
 * type NestedInteger struct {
 * }
 *
 * // Return true if this NestedInteger holds a single integer, rather than a nested list.
 * func (n NestedInteger) IsInteger() bool {}
 *
 * // Return the single integer that this NestedInteger holds, if it holds a single integer
 * // The result is undefined if this NestedInteger holds a nested list
 * // So before calling this method, you should have a check
 * func (n NestedInteger) GetInteger() int {}
 *
 * // Set this NestedInteger to hold a single integer.
 * func (n *NestedInteger) SetInteger(value int) {}
 *
 * // Set this NestedInteger to hold a nested list and adds a nested integer to it.
 * func (n *NestedInteger) Add(elem NestedInteger) {}
 *
 * // Return the nested list that this NestedInteger holds, if it holds a nested list
 * // The list length is zero if this NestedInteger holds a single integer
 * // You can access NestedInteger's List element directly if you want to modify it
 * func (n NestedInteger) GetList() []*NestedInteger {}
 */
func deserialize(s string) *NestedInteger {
	stack := []*NestedInteger{}

	i := 0
	for i < len(s) {
		if s[i] >= '0' && s[i] <= '9' || s[i] == '-' {
			sign := 1
			if s[i] == '-' {
				sign = -1
				i++
			}

			tmp := 0
			for i < len(s) && s[i] >= '0' && s[i] <= '9' {
				tmp = tmp*10 + int(s[i]-'0')
				i++
			}

			inner := &NestedInteger{}
			inner.SetInteger(sign * tmp)

			if len(stack) == 0 {
				return inner
			}

			stack[len(stack)-1].Add(*inner)
			i--
		} else if s[i] == '[' {
			stack = append(stack, &NestedInteger{})
		} else if s[i] == ']' {
			if len(stack) == 1 {
				break
			} else {
				stack[len(stack)-2].Add(*stack[len(stack)-1])
				stack = stack[:len(stack)-1]
			}
		}

		i++
	}

	return stack[0]
}
