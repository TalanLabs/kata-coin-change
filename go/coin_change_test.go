package coinchange

import "testing"

type testCase struct {
	coins    []int
	amount   int
	expected uint
}

var testCases []testCase = []testCase{
	{coins: []int{1, 2, 3}, amount: 0, expected: 1},
	{coins: []int{}, amount: 10, expected: 0},
	{coins: []int{2}, amount: 3, expected: 0},
	{coins: []int{1}, amount: 10, expected: 1},
	{coins: []int{1, 2}, amount: 3, expected: 2},
	{coins: []int{1, 2, 3}, amount: 4, expected: 4},
	{coins: []int{1, 2, 4, 5}, amount: 201, expected: 36941},
}

func TestCoinChange(t *testing.T) {
	for _, tc := range testCases {
		got := coinChange(tc.coins, tc.amount)
		if got != tc.expected {
			t.Errorf("count coins failed. got: %d, expected: %d", got, tc.expected)
		}

	}
}

func TestCoinChange2(t *testing.T) {
	for _, tc := range testCases {
		got := coinChange2(tc.coins, tc.amount)
		if got != tc.expected {
			t.Errorf("count coins failed. got: %d, expected: %d", got, tc.expected)
		}
	}
}
