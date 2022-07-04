package binarysearch

func search(nums []int, target int) int {
	l, h := 0, len(nums)-1
	var m int
	for l <= h {
		m = (l + h) / 2
		if nums[m] == target {
			return m
		}

		if nums[m] < target {
			l = m + 1
		} else {
			h = m - 1
		}
	}
	return -1
}
