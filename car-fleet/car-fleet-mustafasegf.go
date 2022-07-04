package carfleet

import "sort"

func carFleet(target int, position []int, speed []int) int {
	type car struct {
		pos int
		spd int
	}

	n := len(position)
	cars := make([]car, n)

	for i := range position {
		cars[i].pos = position[i]
		cars[i].spd = speed[i]
	}

	sort.Slice(cars, func(i, j int) bool {
		return cars[i].pos < cars[j].pos
	})

	stack := make([]float64, 0, len(position))

	for _, c := range cars {
		t := float64(target-c.pos) / float64(c.spd)
		n := len(stack)

		for n > 0 && t >= stack[n-1] {
			stack = stack[:n-1]
			n--
		}

		stack = append(stack, t)
	}

	return len(stack)
}
