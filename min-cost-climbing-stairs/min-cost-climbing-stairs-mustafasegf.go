package mincostclimbingstairs

func minCostClimbingStairs(cost []int) int {
	x, y := cost[0], cost[1]
	for i := 2; i < len(cost); i++ {
		if y < x {
			x, y = y, cost[i]+y
		} else {
			x, y = y, cost[i]+x
		}
	}

	if x < y {
		return x
	}
	return y
}
