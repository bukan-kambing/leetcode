package dailytemperatures

func dailyTemperatures(temperatures []int) []int {
	ans := make([]int, len(temperatures))
	stack := make([]int, len(temperatures))

	for i := range temperatures {
		for len(stack) > 0 && temperatures[i] > temperatures[stack[len(stack)-1]] {
			t := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			ans[t] = i - t
		}

		stack = append(stack, i)
	}

	return ans
}
