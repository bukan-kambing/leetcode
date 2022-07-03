package implementstrstr

func strStr(haystack string, needle string) int {
	n := len(needle)
	for i := 0; i <= len(haystack)-n; i++ {
		if haystack[i:i+n] == needle {
			return i
		}
	}

	return -1
}
