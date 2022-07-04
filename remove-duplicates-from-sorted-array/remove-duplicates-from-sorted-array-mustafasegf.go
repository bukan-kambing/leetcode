package removeduplicatesfromsortedarray

func removeDuplicates(nums []int) int {
	n := len(nums)
	if n == 0 {
		return 0
	}

	ans := 1
	last := nums[0]
	for i := 0; i < n-2; i++ {
		if nums[i] != nums[i+1] {
			nums[ans] = nums[i+1]
			last = nums[i+1]

			ans++
		}
	}

	if last != nums[n-1] {
		nums[ans] = nums[n-1]
		ans++
	}

	return ans
}
