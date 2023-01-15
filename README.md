# Coin Change


## Instruction 

You are given an integer array `coins` representing coins of different denominations and an integer `amount` representing a total amount of money.

Return ***the number of way*** to make up that amount by using different combinations from `coins`. The order do not not matter.


You may assume that you have an infinite number of each kind of coin.

**Example 1:**

```text
Input: coins = [1,2], amount = 3
Output: 2
Explanation: 
    3 = 1 + 1 + 1
    3 = 2 + 1
```


**Example 2:**

```text
Input: coins = [2], amount = 3
Output: 0
```

**Example 3:**

```text
Input: coins = [1], amount = 0
Output: 0
```

**Example 4:**

```text
Input: coins = [1,2,3], amount = 4
Output: 4
Explanation: 
    4 = 1 + 1 + 1 + 1
    4 = 2 + 1 + 1
    4 = 2 + 2
    4 = 1 + 3
```

## Constraints:

1 <= coins.length <= 12
1 <= coins[i] <= 231 - 1
0 <= amount <= 104