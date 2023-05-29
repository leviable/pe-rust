use big_num::big_num::BigNum;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let mut num = BigNum::new();
    num.factorial(100);
    println!("Factorial Sum is {}", num.sum());
    println!("Time to complete: {}", start.elapsed().as_secs_f64());
}
