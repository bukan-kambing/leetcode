package longestcommonprefix

import "strings"

func longestCommonPrefix(strs []string) string {
	pre := ""
	i := 0
	end := false

	for !end {
		if len(strs[0]) == 0 {
			break
		}
		temp := pre + string(strs[0][i])
		i++
		for _, str := range strs {
			if !strings.HasPrefix(str, temp) {
				return pre
			}

			if len(str) == i {
				end = true
			}
		}
		pre = temp

	}
	return pre
}
