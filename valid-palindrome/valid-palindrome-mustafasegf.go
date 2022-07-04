package validpalindrome

import "unicode"

func isPalindrome(s string) bool {
	ss := make([]rune, 0, len(s))

	for _, c := range s {
		if unicode.IsLetter(c) || unicode.IsNumber(c) {
			ss = append(ss, unicode.ToLower(c))
		}
	}

	n := len(ss)
	for i := 0; i < len(ss)/2; i++ {
		if ss[i] != ss[n-i-1] {
			return false
		}
	}

	return true
}
