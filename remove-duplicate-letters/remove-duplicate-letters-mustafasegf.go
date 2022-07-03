package removeduplicateletters

func removeDuplicateLetters(s string) string {
	m := [26]int{}
	v := [26]bool{}

	for i := range s {
		m[int(s[i]-'a')] = i
	}

	var res []byte
	for i := range s {
		ch := s[i]
		for len(res) > 0 {
			last := res[len(res)-1]
			if ch <= last &&
				m[int(last-'a')] >= i &&
				!v[int(ch-'a')] {
				v[int(last-'a')] = false
				res = res[:len(res)-1]
				continue
			}
			break
		}

		if !v[int(ch-'a')] {
			res = append(res, ch)
			v[int(ch-'a')] = true
		}
	}
	return string(res)
}
