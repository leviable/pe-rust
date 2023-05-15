// In the United Kingdom the currency is made up of pound (£) and pence (p). There are eight coins in general circulation:

// 1p, 2p, 5p, 10p, 20p, 50p, £1 (100p), and £2 (200p).
// It is possible to make £2 in the following way:

// 1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p
// How many different ways can £2 be made using any number of coins?

use std::time::Instant;

fn fill_wallet(wallet: &mut Vec<Vec<u8>>, coin_list: Vec<u8>, current_coins: Vec<u8>) {
    let current_total: u8 = current_coins.iter().sum();

    if current_total >= 200 {
        if current_total == 200 {
            wallet.push(current_coins);
        }

        return;
    }

    for (idx, coin) in coin_list.iter().enumerate() {
        let mut new_coins = current_coins.clone();
        new_coins.push(*coin);
        fill_wallet(wallet, coin_list[idx..].to_vec(), new_coins);
    }
}

fn main() {
    let start = Instant::now();

    let mut wallet: Vec<Vec<u8>> = vec![];
    let coin_list = vec![200, 100, 50, 20, 10, 5, 2, 1];
    let current_coins: Vec<u8> = vec![];
    fill_wallet(&mut wallet, coin_list, current_coins);
    println!("Total: {}", wallet.len());
    println!("Time Elapsed: {}", start.elapsed().as_secs_f64());
}
