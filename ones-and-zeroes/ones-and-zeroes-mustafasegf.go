package onesandzeroes

import "strings"

func findMaxForm(strs []string, m int, n int) int {
	table := make([][]int, m+1)
	for i := range table {
		table[i] = make([]int, n+1)
	}

	// counter
	cnt := make([][]int, len(strs))
	for i := range cnt {
		l := len(strs[i])
		zeros := strings.Count(strs[i], "0")

		cnt[i] = make([]int, 2)

		cnt[i][0] = zeros
		cnt[i][1] = l - zeros
	}

	for cntI := range cnt {
		for i := m; i >= 0; i-- {
			for j := n; j >= 0; j-- {
				// if it can fit
				if i >= cnt[cntI][0] && j >= cnt[cntI][1] {
					// get biggest
					table[i][j] = maxInt(table[i][j], table[i-cnt[cntI][0]][j-cnt[cntI][1]]+1)
					// if doesn't fit, go to smaller
				} else {
					break
				}
			}
		}
	}

	return table[m][n]
}

// golang don't have max int sadly
func maxInt(a, b int) int {
	if a > b {
		return a
	}
	return b
}
