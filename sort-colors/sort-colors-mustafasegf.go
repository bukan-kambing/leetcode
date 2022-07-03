package sortcolors

func sortColors(nums []int) {
	red := 0
	white := 0
	blue := 0
	for _, num := range nums {
		if num == 0 {
			red++
		} else if num == 1 {
			white++
		} else {
			blue++
		}
	}
	c := 0
	for i := 0; i < red; i++ {
		nums[c] = 0
		c++
	}

	for i := 0; i < white; i++ {
		nums[c] = 1
		c++
	}

	for i := 0; i < blue; i++ {
		nums[c] = 2
		c++
	}
}
