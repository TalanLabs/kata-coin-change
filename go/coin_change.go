package coinchange

func coinChange(coins []int, amout int) uint {

	coinsSize := len(coins)
	if amout == 0 {
		return 1
	}

	if amout < 0 {
		return 0
	}

	if coinsSize <= 0 {
		return 0
	}

	return coinChange(coins[:(coinsSize-1)], amout) + coinChange(coins, amout-coins[coinsSize-1])
}

func coinChange2(coins []int, amout int) uint {
	coinsSize := len(coins)
	table := make([]uint, amout+1)

	for i := 0; i <= amout; i++ {
		if i == 0 {
			table[i] = 1
		} else {
			table[i] = 0
		}
	}

	for i := 0; i < coinsSize; i++ {
		for j := coins[i]; j < amout+1; j++ {
			table[j] += table[j-coins[i]]
		}
	}
	return table[amout]
}
