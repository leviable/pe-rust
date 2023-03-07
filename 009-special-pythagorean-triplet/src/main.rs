// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,

// a2 + b2 = c2
// For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.

// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
// Find the product abc.

fn main() {
    let (a, b, c) = find_triplet(1000);
    println!("Triplet product is: {}", a * b * c);
}

fn is_triplet(a: u32, b: u32, c: u32) -> bool {
    a.pow(2) + b.pow(2) == c.pow(2)
}

#[test]
fn test_is_triplet() {
    assert_eq!(is_triplet(1, 2, 3), false);
    assert_eq!(is_triplet(3, 4, 5), true);
}

fn find_triplet(upper_limit: u32) -> (u32, u32, u32) {
    for a in 1..upper_limit - 1 {
        for b in 2..upper_limit {
            for c in 3..upper_limit + 1 {
                if is_triplet(a, b, c) && a + b + c == 1000 {
                    return (a, b, c);
                }
            }
        }
    }
    (0, 0, 0)
}

#[test]
fn test_find_triplet() {
    assert_eq!(find_triplet(12), (3, 4, 5));
}
