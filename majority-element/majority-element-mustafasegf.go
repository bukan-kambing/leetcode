package majorityelement

func majorityElement(nums []int) int {
	c := 1
	num := nums[0]
	tab := map[int]int{}
	for _, n := range nums {
		if _, ok := tab[n]; !ok {
			tab[n] = 0
		}

		tab[n]++

		if tab[n] > c {
			c = tab[n]
			num = n
		}
	}

	return num
}
