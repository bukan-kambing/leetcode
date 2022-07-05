package besttimetobuyandsellstock

func maxProfit(prices []int) int {
	low, max := prices[0], 0

	for _, price := range prices {
		if price < low {
			low = price
		}

		if price-low > max {
			max = price - low
		}
	}

	return max
}
