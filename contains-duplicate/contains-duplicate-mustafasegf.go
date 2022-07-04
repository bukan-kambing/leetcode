package containsduplicate

func containsDuplicate(nums []int) bool {
	set := map[int]bool{}

	for _, n := range nums {
		if _, ok := set[n]; ok {
			return true
		}

		set[n] = true
	}
	return false
}
