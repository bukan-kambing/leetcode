package nthtribonaccinumber

func tribonacci(n int) int {
	temp := make([]int, n+3)
	temp[0] = 0
	temp[1] = 1
	temp[2] = 1
	for i := 3; i <= n; i++ {
		temp[i] = temp[i-1] + temp[i-2] + temp[i-3]
	}
	return temp[n]
}
