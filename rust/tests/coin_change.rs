use coinchange::*;

#[test]
fn zero_amount() {
    let coins = vec![1, 2, 3];
    let amount = 0;
    let expected = 1;
    assert_eq!(expected, coin_change(coins, amount));
}

#[test]
fn no_coins() {
    let coins :Vec<i32>= Vec::new();
    let amount = 10;
    let expected = 0;
    assert_eq!(expected, coin_change(coins, amount));
}

#[test]
fn no_combinaison_possible() {
    let coins :Vec<i32>= vec![3];
    let amount = 2;
    let expected = 0;
    assert_eq!(expected, coin_change(coins, amount));
}

#[test]
fn one_coins() {
    let coins :Vec<i32>= vec![1];
    let amount = 11;
    let expected = 1;
    assert_eq!(expected, coin_change(coins, amount));
}

#[test]
fn two_coins() {
    let coins :Vec<i32>= vec![1, 2];
    let amount = 3;
    let expected = 2;
    assert_eq!(expected, coin_change(coins, amount));
}


#[test]
fn three_coins() {
    let coins :Vec<i32>= vec![1, 2, 3];
    let amount = 4;
    let expected = 4;
    assert_eq!(expected, coin_change(coins, amount));
}

#[test]
fn four_coins() {
    let coins :Vec<i32>= vec![1, 2, 4, 5];
    let amount = 201;
    let expected = 36941;
    assert_eq!(expected, coin_change(coins, amount));
}