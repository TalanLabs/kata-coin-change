
pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {

    match amount {
        a if a < 0 => 0,
        0 => 1,
        _ => {
            if coins.len() == 0 {
                return 0;
            }
            let mut remain_coins =  coins.clone();
            remain_coins.pop();

            let remain_amont = amount - coins[coins.len() - 1];

            return coin_change(remain_coins, amount) + coin_change(coins, remain_amont);
        }
    }
}


pub fn coin_change_2(coins: Vec<i32>, amount: i32) -> i32 {

    let mut table: Vec<i32> = vec![0; (amount + 1) as usize];
    table[0] = 1;

    for coin in coins {
        for j in coin..=amount {
            table[(j as usize)] +=  table[((j - coin) as usize)]
        }
    }

    table[amount as usize]

  
}