package problem130

/*
  the idea behind it.
  we check from the edges if there's any O and any connecting O. if there's is O then we mark it.
  after all iteration, then all the O that haven't turned into M doesn't touch the side.
  hence it need to be converted into X
  all the M is O that touch the side. so we convert it back into O.
*/

func solve(board [][]byte) {
	m := len(board) - 1
	n := len(board[0]) - 1

	// check from the left and right
	for i := range board {
		dfs(board, i, 0)
		dfs(board, i, n)
	}

	// check from the top and bottom
	for j := range board[0] {
		dfs(board, 0, j)
		dfs(board, m, j)
	}

	// convert the marked into O, and the O into X
	for i := range board {
		for j := range board[0] {
			if board[i][j] == 'M' {
				board[i][j] = 'O'
			} else if board[i][j] == 'O' {
				board[i][j] = 'X'
			}
		}
	}
}

func dfs(board [][]byte, i, j int) {
	m := len(board)
	n := len(board[0])

	if i >= 0 && j >= 0 && i < m && j < n {
		if board[i][j] == 'O' {
			// we mark it
			board[i][j] = 'M'
			dfs(board, i+1, j)
			dfs(board, i-1, j)
			dfs(board, i, j+1)
			dfs(board, i, j-1)
		}
	}
}
