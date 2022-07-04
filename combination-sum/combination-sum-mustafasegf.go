package combinationsum

func combinationSum(candidates []int, target int) [][]int {
	ans := [][]int{}
	rec(&ans, candidates, nil, target, 0)
	return ans
}

func rec(ans *[][]int, nums, prev []int, target, idx int) {
	if target < 0 || idx > len(nums) {
		return
	}

	if target == 0 {
		t := make([]int, len(prev))
		copy(t, prev)
		*ans = append(*ans, t)
		return
	}

	for i := idx; i < len(nums); i++ {
		prev := append(prev, nums[i])
		rec(ans, nums, prev, target-nums[i], i)
		prev = prev[:len(prev)-1]
	}
}
