use cucumber::{given, then, when, World};
use pe042::{calc_word_score, count_triangle_words, is_triangle_number};

#[derive(Debug, Default, World)]
struct Pe42World {
    num: usize,
    actual: bool,
    word_to_convert: String,
    word_score: usize,
    actual_word_total: usize,
    word_buf: Vec<String>,
}

#[when(regex = r"^I am given the number (.*)$")]
fn get_and_store_number(w: &mut Pe42World, num: usize) {
    w.num = num;
}

#[when(regex = "^I test if the number is a triangle number$")]
fn get_and_store_is_triangle_result(w: &mut Pe42World) {
    w.actual = is_triangle_number(w.num);
}

#[then(regex = r"^I find that it (.*) a triangle number$")]
fn then_it_is_or_is_not_triangle(w: &mut Pe42World, word_tense: String) {
    let expected = match &word_tense[..] {
        "is" => true,
        "is not" => false,
        _ => panic!("Must use either \"is\" or \"is not\""),
    };

    assert_eq!(
        expected, w.actual,
        "Expected {}, found {}",
        expected, w.actual
    );
}

#[when(regex = r"^I compute the word score of '(.*)'$")]
fn get_word_score(w: &mut Pe42World, word: String) {
    w.word_score = calc_word_score(&word);
    w.word_to_convert = word;
}

#[then(regex = r"^I get (.*)$")]
fn compare_calculated_score(w: &mut Pe42World, expected: usize) {
    let actual = w.word_score;
    assert_eq!(actual, expected, "Got {}, expected {}", actual, expected);
}

#[given(regex = r"^I have a buffer of words")]
fn given_i_have_a_word_buffer(w: &mut Pe42World) {
    w.word_buf = vec![
        "SKX".to_string(),
        "SKY".to_string(),
        "SKZ".to_string(),
        "SKY".to_string(),
    ];
}

#[when(regex = r"^I calculate the total score of the buffer")]
fn calculate_the_total_score_of_the_buffer(w: &mut Pe42World) {
    w.actual_word_total = count_triangle_words(&w.word_buf);
}

#[then(regex = r"^the buffer total is (.*)$")]
fn compare_actual_word_total_to_expected(w: &mut Pe42World, expected_word_total: usize) {
    assert_eq!(
        w.actual_word_total, expected_word_total,
        "Got {}, expected {}",
        w.actual_word_total, expected_word_total
    );
}

fn main() {
    println!("In tests main");
    futures::executor::block_on(Pe42World::run("tests/features"));
}
