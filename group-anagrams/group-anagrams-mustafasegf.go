package groupanagrams

func groupAnagrams(strs []string) [][]string {
	ans := [][]string{}
	sMap := map[[26]int][]string{}

	for _, str := range strs {
		t := [26]int{}

		for _, c := range str {
			t[c-'a']++
		}

		sMap[t] = append(sMap[t], str)
	}

	for _, v := range sMap {
		ans = append(ans, v)
	}
	return ans
}
