// Starting with 1 and spiralling anticlockwise in the following way, a square
// spiral with side length 7 is formed.
//
//                       37 36 35 34 33 32 31
//                       38 17 16 15 14 13 30
//                       39 18  5  4  3 12 29
//                       40 19  6  1  2 11 28
//                       41 20  7  8  9 10 27
//                       42 21 22 23 24 25 26
//                       43 44 45 46 47 48 49
//
// It is interesting to note that the odd squares lie along the bottom right
// diagonal, but what is more interesting is that 8 out of the 13 numbers lying
// along both diagonals are prime; that is, a ratio of 8/13 ≈ 62%.
//
// If one complete new layer is wrapped around the spiral above, a square spiral
// with side length 9 will be formed. If this process is continued, what is the
// side length of the square spiral for which the ratio of primes along both
// diagonals first falls below 10%?
//
fn pe058() -> u64 {
    let mut prime_corners = 0.0;
    let mut non_prime_corners = 1.0;
    for layer in (1u64..).step_by(2) {
        let layer_start = layer.pow(2) + 1;
        let layer_end = (layer + 2).pow(2);
        for corner in ((layer_start + layer)..=layer_end).step_by((layer + 1) as usize) {
            if primes::is_prime(corner) {
                prime_corners += 1.0;
            } else {
                non_prime_corners += 1.0;
            }
        }

        if prime_corners / (prime_corners + non_prime_corners) < 0.1 {
            return layer + 2;
        }
    }

    0
}

fn main() {
    let start = std::time::Instant::now();
    let solution = pe058();
    println!("Solution: {solution}");
    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
