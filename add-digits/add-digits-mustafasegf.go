package adddigits

func addDigits(num int) int {
	for num > 9 {
		c := 0
		for num > 0 {
			temp := num % 10
			num /= 10
			c += temp
		}
		num = c
	}
	return num
}
