package plusone

func plusOne(digits []int) []int {
	n := len(digits) - 1
	carry := true
	for i := range digits {
		carry = true
		digits[n-i] = (digits[n-i] + 1) % 10
		if digits[n-i] != 0 {
			carry = false
			break
		}
	}
	if carry {
		digits = append([]int{1}, digits...)
	}
	return digits
}
