package longestincreasingsubsequence

func lengthOfLIS(nums []int) int {
	arr := make([]int, 0)
	arr = append(arr, nums[0])

	for i := 1; i < len(nums); i++ {
		num := nums[i]
		if num > arr[len(arr)-1] {
			arr = append(arr, num)
		} else {
			j := bin(arr, num)
			arr[j] = num
		}
	}
	return len(arr)
}

func bin(arr []int, num int) int {
	l, r := 0, len(arr)-1
	for l < r {
		m := l + (r-l)/2
		if arr[m] == num {
			return m
		}
		if arr[m] < num {
			l = m + 1
		} else {
			r = m
		}
	}
	return l
}
