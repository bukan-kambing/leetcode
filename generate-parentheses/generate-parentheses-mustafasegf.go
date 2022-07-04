package generateparentheses

func generateParenthesis(n int) []string {
	ans := make([]string, 0)
	rec(&ans, "", n, n)

	return ans
}

func rec(ans *[]string, prev string, l, r int) {
	if l == 0 && r == 0 {
		*ans = append(*ans, prev)
		return
	}

	if r < l {
		return
	}

	if l > 0 {
		rec(ans, prev+"(", l-1, r)
	}

	if r > 0 {
		rec(ans, prev+")", l, r-1)
	}
}
