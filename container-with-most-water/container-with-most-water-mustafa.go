package containerwithmostwater

func maxArea(height []int) int {
	ans := 0
	l, r := 0, len(height)-1

	for l < r {
		t := min(height[l], height[r]) * (r - l)
		ans = max(ans, t)

		if height[l] <= height[r] {
			l++
		} else {
			r--
		}
	}
	return ans
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}
