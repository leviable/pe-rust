use std::collections::HashMap;

fn pe062(target: u64) -> Vec<u64> {
    let mut hm: HashMap<String, Vec<u64>> = HashMap::new();

    for num in 1u64.. {
        let cube = num.pow(3);
        let mut cube_sorted = cube.to_string().chars().collect::<Vec<_>>();
        cube_sorted.sort();
        let cube_sorted = cube_sorted.iter().collect();

        let entry = hm.entry(cube_sorted).or_insert(vec![]);
        entry.push(cube);

        if entry.len() as u64 >= target {
            return entry.clone();
        }
    }

    unreachable!();
}

#[test]
fn test_pe062() {
    assert_eq!(pe062(3), vec![41063625, 56623104, 66430125]);
}

fn main() {
    let start = std::time::Instant::now();

    let mut perms = pe062(5);
    perms.sort();
    let solution = perms.iter().next();
    println!("Solution is : {:?}", solution);

    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
