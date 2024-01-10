// Consider quadratic Diophantine equations of the form:
//
// For example, when
// , the minimal solution in
//  is
// .
//
// It can be assumed that there are no solutions in positive integers when
//  is square.
//
// By finding minimal solutions in
//  for
// , we obtain the following:
//
//
// Hence, by considering minimal solutions in
//  for
// , the largest
//  is obtained when
// .
//
// Find the value of
//  in minimal solutions of
//  for which the largest value of
//  is obtained.

fn diophantine(d: u64, y: u64) -> f64 {
    (1.0 + d as f64 * (y as f64).powi(2)).sqrt()
}
#[test]
fn test_diophantine() {
    assert_eq!(diophantine(2, 1), 3.0f64.sqrt());
    assert_eq!(diophantine(5, 3), 46.0f64.sqrt());
    assert_eq!(diophantine(5, 4), 81.0f64.sqrt());
    assert_eq!(diophantine(13, 180), 421201.0f64.sqrt());
    assert_eq!(diophantine(661, 15209887), 391044982.0f64);
}

fn is_integer(num: f64) -> bool {
    num == num as u64 as f64
}

#[test]
fn test_is_integer() {
    assert!(!is_integer(3.0f64.sqrt()));
    assert!(is_integer(81.0f64.sqrt()));
}

fn pe066(limit: u64) -> u64 {
    let mut largest_d = 0u64;
    let mut largest_x = 0u64;
    // range over positive non-squares
    for d in 2..=limit {
        let squares = (2u64..32).map(|x| x.pow(2)).collect::<Vec<_>>();
        if squares.contains(&d) {
            continue;
        }

        let smallest_x = 1_000_000_000_000;
        for y in 1.. {
            let x = diophantine(d, y);
            if d == 661 {
                // eprintln!("222222222222222222222222222222222222222222222222222");
                // eprintln!("{d} -> {y} -> {x}");
            }
            if is_integer(x) && (x as u64) < smallest_x {
                let smallest_x = x as u64;
                if smallest_x > largest_x {
                    eprintln!("1111111111111111111111111111111111111111111111111");
                    eprintln!("New largest: {d} at {y} and {x}");
                    largest_d = d;
                    largest_x = x as u64;
                }
                break;
            }
        }
    }

    largest_d
}

#[test]
fn test_pe066() {
    assert_eq!(pe066(3), 2);
    assert_eq!(pe066(7), 5);
    assert_eq!(pe066(8), 5);
    assert_eq!(pe066(10), 10);
    assert_eq!(pe066(11), 10);
    assert_eq!(pe066(13), 13);
}

fn main() {
    let start = std::time::Instant::now();
    let solution = pe066(1000);
    println!("Solution: {}", solution);
    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
