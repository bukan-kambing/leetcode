package happynumber

func isHappy(n int) bool {
	set := make(map[int]bool)
	for n != 1 {
		if _, ok := set[n]; ok {
			return false
		}
		set[n] = true
		c := 0
		for n > 0 {
			temp := n % 10
			n /= 10
			c += temp * temp
		}
		n = c
	}
	return true
}
