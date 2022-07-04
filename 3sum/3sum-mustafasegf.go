package threesum

import "sort"

func threeSum(nums []int) [][]int {
	sort.Ints(nums)
	n := len(nums)

	ans := [][]int{}

	for n1 := 0; n1 < n-2; n1++ {
		if n1 > 0 && nums[n1] == nums[n1-1] {
			continue
		}
		n2 := n1 + 1
		n3 := n - 1

		for n2 < n3 {
			sum := nums[n1] + nums[n2] + nums[n3]
			if sum == 0 {
				ans = append(ans, []int{nums[n1], nums[n2], nums[n3]})
				n3--

				for n2 < n3 && nums[n3] == nums[n3+1] {
					n3--
				}
			} else if sum > 0 {
				n3--
			} else {
				n2++
			}
		}

	}

	return ans
}
