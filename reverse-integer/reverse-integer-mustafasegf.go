package reverseinteger

import "math"

func reverse(x int) int {
	ans := 0
	neg := 1
	if x < 0 {
		neg = -1
		x *= -1
	}

	max := math.MaxInt32 / 10
	min := math.MinInt32 / 10

	for x > 0 {
		p := x % 10

		if (ans > max) || (ans == max && p > 7) {
			return 0
		}

		if (neg*ans < min) || (ans == min && p*neg < -8) {
			return 0
		}

		ans = (ans * 10) + p
		x /= 10
	}

	return neg * ans
}
