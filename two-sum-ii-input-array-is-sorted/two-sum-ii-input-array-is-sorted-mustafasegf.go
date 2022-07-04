package twosumiiinputarrayissorted

func twoSum(numbers []int, target int) []int {
	l := 0
	r := len(numbers) - 1

	ll := numbers[l]
	rr := numbers[r]
	for l < r {
		if ll+rr == target {
			return []int{l + 1, r + 1}
		}

		if rr > target-ll {
			r--
			rr = numbers[r]
		} else {
			l++
			ll = numbers[l]
		}
	}

	return nil
}
