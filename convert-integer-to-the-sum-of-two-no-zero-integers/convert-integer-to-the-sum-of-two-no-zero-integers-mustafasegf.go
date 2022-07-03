package convertintegertothesumoftwonozerointegers

import (
	"strconv"
	"strings"
)

func getNoZeroIntegers(n int) []int {
	a := 1
	b := n - a
	for {
		aStr := strconv.Itoa(a)
		bStr := strconv.Itoa(b)

		if !(strings.Contains(aStr, "0") || strings.Contains(bStr, "0")) {
			return []int{a, b}
		}
		a++
		b--
	}
}
