// for x in 1010101010..1389026623 {

fn is_match(num: u128) -> bool {
    let mut num = num.to_string().chars().collect::<Vec<_>>();

    if num[0] == '1'
        && num[2] == '2'
        && num[4] == '3'
        && num[6] == '4'
        && num[8] == '5'
        && num[10] == '6'
        && num[12] == '7'
        && num[14] == '8'
        && num[16] == '9'
        && num[18] == '0'
    {
        return true;
    } else {
        return false;
    }
}

fn pe202() -> u128 {
    for num in 1010101010u128..1389026623 {
        if is_match(num.pow(2)) {
            return num;
        }
    }

    1
}

fn main() {
    let start = std::time::Instant::now();
    let solution = pe202();
    println!("Solution: {solution}");
    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
