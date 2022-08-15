package romantointeger

func romanToInt(s string) int {
	roman := map[byte]int{
		'I': 1,
		'V': 5,
		'X': 10,
		'L': 50,
		'C': 100,
		'D': 500,
		'M': 1000,
	}

	doubleRoman := map[string]int{
		"IV": 4,
		"IX": 9,
		"XL": 40,
		"XC": 90,
		"CD": 400,
		"CM": 900,
	}

	n := len(s)
	ans := 0

	for i := 0; i < n; i++ {
		if (i != (n - 1)) && (doubleRoman[s[i:i+2]] != 0) {
			ans += doubleRoman[s[i:i+2]]
			i++
		} else {
			ans += roman[s[i]]
		}
	}

	return ans
}
