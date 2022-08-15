package removeelement

func removeElement(nums []int, val int) int {
	ans := 0

	for _, v := range nums {
		if v != val {
			nums[ans] = v
			ans++
		}
	}

	return ans
}
