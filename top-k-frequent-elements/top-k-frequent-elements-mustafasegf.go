package topkfrequentelements

import "sort"

func topKFrequent(nums []int, k int) []int {
	nMap := make(map[int]int)

	for _, n := range nums {
		nMap[n]++
	}

	type kv struct {
		Key   int
		Value int
	}

	numUniq := make([]kv, 0, len(nMap))

	for k, v := range nMap {
		numUniq = append(numUniq, kv{k, v})
	}

	sort.Slice(numUniq, func(i, j int) bool {
		return numUniq[i].Value > numUniq[j].Value
	})

	ans := make([]int, k)

	for i := 0; i < k; i++ {
		ans[i] = numUniq[i].Key
	}

	return ans
}
