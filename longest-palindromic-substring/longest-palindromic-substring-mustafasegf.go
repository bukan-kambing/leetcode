package longestpalindromicsubstring

func longestPalindrome(s string) string {
	start, end := 0, 0
	for i := 0; i < len(s); i++ {
		s1, e1 := search(s, i, i)
		s2, e2 := search(s, i, i+1)
		if e1-s1 < e2-s2 {
			s1, e1 = s2, e2
		}
		if end-start < e1-s1 {
			start, end = s1, e1
		}
	}
	return s[start : end+1]
}

func search(s string, l, r int) (int, int) {
	for l >= 0 && r < len(s) && s[l] == s[r] {
		l--
		r++
	}
	return l + 1, r - 1
}
