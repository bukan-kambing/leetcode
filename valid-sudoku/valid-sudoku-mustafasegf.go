package validsudoku

func isValidSudoku(board [][]byte) bool {
	row := make([]map[byte]bool, 9)
	col := make([]map[byte]bool, 9)
	box := make([][]map[byte]bool, 3)

	for i := range row {
		row[i] = make(map[byte]bool)
	}

	for i := range col {
		col[i] = make(map[byte]bool)
	}

	for j := range box {
		box[j] = make([]map[byte]bool, 3)
		for i := range box[j] {
			box[j][i] = make(map[byte]bool)
		}
	}

	for i, arr := range board {
		for j, c := range arr {
			if c == 46 {
				continue
			}
			if _, ok := row[j][c]; ok {
				return false
			}
			row[j][c] = true

			if _, ok := col[i][c]; ok {
				return false
			}
			col[i][c] = true

			if _, ok := box[i/3][j/3][c]; ok {
				return false
			}
			box[i/3][j/3][c] = true

		}
	}

	return true
}
