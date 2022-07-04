package productofarrayexceptself

func productExceptSelf(nums []int) []int {
	sum := 1
	zero := false
	zeroAgainLmao := false

	for _, n := range nums {
		if n == 0 && zero {
			zeroAgainLmao = true
			break
		}

		if n == 0 {
			zero = true
			continue
		}
		sum *= n
	}

	for i := range nums {
		if zeroAgainLmao || (zero && nums[i] != 0) {
			nums[i] = 0
			continue
		}

		if nums[i] == 0 {
			nums[i] = sum
			continue
		}
		nums[i] = sum / nums[i]
	}

	return nums
}
