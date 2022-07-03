package twosum

func twoSum(nums []int, target int) []int {
	nMap := map[int]int{}
	for i, n := range nums {
		if ii, ok := nMap[target-n]; ok {
			return []int{i, ii}
		}
		nMap[n] = i
	}

	return nil
}
