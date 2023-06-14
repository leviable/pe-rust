// The nth term of the sequence of triangle numbers is given by, tn = ½n(n+1);
// so the first ten triangle numbers are:

// 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...

// By converting each letter in a word to a number corresponding to its alphabetical
// position and adding these values we form a word value. For example, the word
// value for SKY is 19 + 11 + 25 = 55 = t10. If the word value is a triangle number
// then we shall call the word a triangle word.

// Using words.txt (right click and 'Save Link/Target As...'), a 16K text file
// containing nearly two-thousand common English words, how many are triangle words?

fn main() {
    let start = std::time::Instant::now();

    let mut words: Vec<String> = vec![];
    for line in pe042::read_lines("./src/0042_words.txt".to_string()) {
        for word in line.expect("Didn't get a word").split(",") {
            words.push(word.replace("\"", ""));
        }
    }

    let total = pe042::count_triangle_words(&words);
    println!("Triangle sum total: {}", total);

    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
