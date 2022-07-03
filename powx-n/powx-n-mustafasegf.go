package powxn

func myPow(x float64, n int) float64 {
	if n == 0 {
		return float64(1)
	}
	if n == 1 {
		return x
	}
	if n < 0 {
		return 1 / myPow(x, -n)
	}

	t := myPow(x, n/2)
	if n%2 == 0 {
		return t * t
	} else {
		return t * t * x
	}
}
