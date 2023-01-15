def count(coins: list[int], amount: int) -> int:
    """
    Params:
        coins list[int]
        amount int
    Returns:
        int: the number of possible change making the amount
    """
    n = len(coins)
    # If amount is 0 then there is one solution (do not include any coin)
    if (amount == 0):
        return 1

    # If sum is less than 0 then no solution exists
    if (amount < 0):
        return 0
 
    # If there are no coins and sum is greater than 0, then no solution exist
    if (n <= 0):
        return 0
 
    # count is sum of solutions (i) including coins[n-1] (ii) excluding coins[n-1]
    return count(coins[: -1], amount) + count(coins, amount - coins[n-1])


def count2(coins: list[int],  sum: int) -> int:
    """
    Params:
        coins list[int]
        amount int
    Returns:
        int: the number of possible change making the amount
    """
    n = len(coins)
    table = [0 for k in range(sum+1)]
    table[0] = 1
 
    # Pick all coins one by one and update the table[] values
    # after the index greater than or equal to the value of the
    # picked coin
    for i in range(0, n):
        for j in range(coins[i], sum+1):
            table[j] += table[j-coins[i]]
 
    return table[sum]
 