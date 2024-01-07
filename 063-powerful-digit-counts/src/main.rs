// The
// -digit number,
// , is also a fifth power. Similarly, the
// -digit number,
// , is a ninth power.
//
// How many
// -digit positive integers exist which are also an
// th power?

use num_bigint::ToBigUint;

fn pe063() -> u32 {
    let mut count = 0;

    for base in 1..20 {
        for power in 1..40 {
            let prod = base.to_biguint().unwrap().pow(power);
            let prod_len = prod.to_string().len();
            if prod_len == power as usize {
                count += 1;
                println!("{} -> {}", base, power);
            }
        }
    }

    count
}

fn main() {
    let start = std::time::Instant::now();

    let solution = pe063();
    println!("Solution: {solution}");
    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
