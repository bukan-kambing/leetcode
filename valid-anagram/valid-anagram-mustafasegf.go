package validanagram

func isAnagram(s string, t string) bool {
	cMap := map[rune]int{}

	for _, c := range s {
		cMap[c]++
	}

	for _, c := range t {
		cMap[c]--
	}

	for _, v := range cMap {
		if v != 0 {
			return false
		}
	}
	return true
}
