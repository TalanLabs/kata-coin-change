function count(coins, amount) {
    if (amount == 0)
        return 1;
        
    if (amount < 0)
        return 0;

    if (coins.length <= 0) 
        return 0;

    return count(coins.slice(0, -1), amount) + count(coins, amount - coins[coins.length - 1])
}

function count2(coins, amount) {
    let table = new Array(amount + 1).fill(0);
    table[0] = 1;

    for (let i = 0; i < coins.length; i++) {
        for (let j = coins[i]; j <= amount; j++) {
            table[j] += table[j - coins[i]]
        }
    }
    return table[amount];
}

module.exports.count = count;