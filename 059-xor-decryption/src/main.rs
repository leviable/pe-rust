// Each character on a computer is assigned a unique code and the preferred
// standard is ASCII (American Standard Code for Information Interchange).
// For example, uppercase A = 65, asterisk (*) = 42, and lowercase k = 107.
//
// A modern encryption method is to take a text file, convert the bytes to
// ASCII, then XOR each byte with a given value, taken from a secret key. The
// advantage with the XOR function is that using the same encryption key on the
// cipher text, restores the plain text; for example, 65 XOR 42 = 107,
// then 107 XOR 42 = 65.
//
// For unbreakable encryption, the key is the same length as the plain text message,
// and the key is made up of random bytes. The user would keep the encrypted message
// and the encryption key in different locations, and without both "halves", it is
// impossible to decrypt the message.
//
// Unfortunately, this method is impractical for most users, so the modified method
// is to use a password as a key. If the password is shorter than the message, which
// is likely, the key is repeated cyclically throughout the message. The balance for
// this method is using a sufficiently long password key for security, but short
// enough to be memorable.
//
// Your task has been made easy, as the encryption key consists of three lower case
// characters. Using 0059_cipher.txt (right click and 'Save Link/Target As...'), a
// file containing the encrypted ASCII codes, and the knowledge that the plain text
// must contain common English words, decrypt the message and find the sum of the
// ASCII values in the original text.
//
use std::fs::File;
use std::io::{self, BufRead, BufReader};

const ASCII_CHARS: &str = "abcdefghijklmnopqrstuvwxyz";

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}

fn read_file() -> Vec<u8> {
    let mut lines: Vec<String> = vec![];
    for line in read_lines("./src/pe059-cipher.txt".to_string()) {
        lines.push(line.unwrap());
    }

    lines[0]
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u8>>()
}

fn decrypt(encrypted: Vec<u8>, key: String) -> String {
    let key = Vec::from_iter(key.chars());
    encrypted
        .iter()
        .enumerate()
        .map(|(idx, x)| (x ^ key[idx % 3] as u8) as char)
        .collect::<String>()
}

fn check(text: &String) -> bool {
    text.matches("the").collect::<Vec<_>>().len() > 10
}

fn pe059() -> u32 {
    let encrypted = read_file();
    for x in ASCII_CHARS.chars() {
        for y in ASCII_CHARS.chars() {
            for z in ASCII_CHARS.chars() {
                let decrypted = decrypt(encrypted.clone(), format!("{x}{y}{z}"));
                if check(&decrypted) {
                    return decrypted
                        .chars()
                        .map(|x| x as u32)
                        .collect::<Vec<u32>>()
                        .iter()
                        .sum::<u32>();
                }
            }
        }
    }
    0
}

fn main() {
    let start = std::time::Instant::now();
    let solution = pe059();
    println!("Solution: {solution}");
    println!("Time elapsed: {}", start.elapsed().as_secs_f64());
}
