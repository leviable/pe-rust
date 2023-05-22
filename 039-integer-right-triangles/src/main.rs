// If p is the perimeter of a right angle triangle with integral length sides, {a,b,c}, there are exactly three solutions for p = 120.

// {20,48,52}, {24,45,51}, {30,40,50}

// For which value of p ≤ 1000, is the number of solutions maximised?

use std::collections::HashMap;
use std::time::Instant;

fn is_right_triangle((x, y, z): (u64, u64, u64)) -> bool {
    x * x + y * y == z * z
}

#[test]
fn test_is_right_triangle() {
    assert!(is_right_triangle((20, 48, 52)));
    assert!(!is_right_triangle((20, 48, 51)));
}

fn pe039() -> u64 {
    let mut hm: HashMap<u64, u64> = HashMap::new();
    for x in 1..(1000 / 3) {
        for y in x..(1000 * 2 / 3) {
            for z in y..1000 {
                if is_right_triangle((x, y, z)) {
                    let perim = x + y + z;
                    match hm.get(&perim) {
                        Some(count) => {
                            hm.insert(perim, count + 1);
                        }
                        None => {
                            hm.insert(perim, 1);
                        }
                    }
                }
            }
        }
    }

    let mut p = 0;
    let mut max_values = 0;
    for (k, v) in hm {
        if v > max_values {
            p = k;
            max_values = v;
        }
    }

    p
}

fn main() {
    let start = Instant::now();
    let solution = pe039();
    println!("Solution: {solution}");
    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
