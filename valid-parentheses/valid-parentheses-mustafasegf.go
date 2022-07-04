package validparentheses

func isValid(s string) bool {
	stk := []rune{}
	bMap := map[rune]rune{
		'(': ')',
		'[': ']',
		'{': '}',
	}

	for _, c := range s {
		if v, ok := bMap[c]; ok {
			stk = append(stk, v)
			continue
		}

		n := len(stk) - 1
		if n < 0 || c != stk[n] {
			return false
		}

		stk = stk[:n]
	}
	return len(stk) == 0
}
