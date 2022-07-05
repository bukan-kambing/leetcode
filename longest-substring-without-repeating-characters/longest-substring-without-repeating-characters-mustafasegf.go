package longestsubstringwithoutrepeatingcharacters

func lengthOfLongestSubstring(s string) int {
	lo, hi, ans := 0, 0, 0
	smap := make(map[byte]bool)

	for hi < len(s) {
		c := s[hi]
		hi++

		for lo < hi {
			if _, ok := smap[c]; !ok {
				break
			}
			t := s[lo]
			delete(smap, t)
			lo++
		}

		smap[c] = true
		ans = max(ans, hi-lo)
	}

	return ans
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}
