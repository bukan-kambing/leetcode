package subsets

func subsets(nums []int) [][]int {
	res := make([][]int, 0)
	solve(0, nums, nil, &res)
	return res
}

func solve(index int, nums []int, cur []int, res *[][]int) {
	if index == len(nums) {
		*res = append(*res, append([]int{}, cur...))
		return
	}

	solve(index+1, nums, cur, res)
	solve(index+1, nums, append(cur, nums[index]), res)
}
