package evaluatereversepolishnotation

import "strconv"

func evalRPN(tokens []string) int {
	stk := make([]int, 0, 10000)
	n1, n2 := 0, 0

	op := map[string]bool{
		"+": true,
		"-": true,
		"*": true,
		"/": true,
	}

	for _, t := range tokens {
		if _, ok := op[t]; ok {
			n2 = stk[len(stk)-1]
			n1 = stk[len(stk)-2]
			stk = stk[:len(stk)-2]
			switch t {
			case "+":
				stk = append(stk, n1+n2)
			case "-":
				stk = append(stk, n1-n2)
			case "*":
				stk = append(stk, n1*n2)
			case "/":
				stk = append(stk, n1/n2)
			}
		} else {
			num, _ := strconv.Atoi(t)
			stk = append(stk, num)
		}
	}
	return stk[0]
}
