package longestconsecutivesequence

func longestConsecutive(nums []int) int {
	nSet := make(map[int]bool, len(nums))

	for _, v := range nums {
		nSet[v] = true
	}
	ans := 0
	for _, n := range nums {
		if !nSet[n-1] {
			t := 0
			for nSet[n] {
				t++
				n++
			}

			if t > ans {
				ans = t
			}
		}
	}

	return ans
}
