package trappingrainwater

func trap(height []int) int {
	n := len(height)
	if n == 0 {
		return 0
	}
	ans := 0

	l := make([]int, n)
	r := make([]int, n)

	l[0] = height[0]
	r[n-1] = height[n-1]

	for i := 1; i < n; i++ {
		l[i] = max(height[i], l[i-1])
	}

	for i := n - 2; i >= 0; i-- {
		r[i] = max(height[i], r[i+1])
	}

	for i := 0; i < n; i++ {
		ans += min(l[i], r[i]) - height[i]
	}

	return ans
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}
