use big_num::big_num::BigNum;
use cucumber::{given, then, when, World};

#[given(regex = r"^I have a new BigNum$")]
fn i_have_a_new_bignum(w: &mut BigNumWorld) {
    w.bignum = BigNum::new();
}

#[when(regex = r"^I multiply the BigNum by (.*)$")]
fn multiply_bignum_by(w: &mut BigNumWorld, multiply_by: usize) {
    w.bignum.mul(multiply_by);
}

#[then(regex = r"^I get a vector with (.*) in it$")]
fn get_a_vector_with(w: &mut BigNumWorld, vector_contents: usize) {
    assert_eq!(w.bignum.num, vec![vector_contents]);
}

#[when(regex = r"^I calculate the factorial of (.*)$")]
fn calc_factorial_of(w: &mut BigNumWorld, num: usize) {
    w.bignum.factorial(num);
}

#[then(regex = r"^I get the number (.*)$")]
fn get_bignum_value(w: &mut BigNumWorld, expect: usize) {
    assert_eq!(
        w.bignum.value(),
        expect,
        "Expected {}, got {}",
        expect,
        w.bignum.value()
    )
}

#[then(regex = r"^the sum of the digits is (.*)$")]
fn sum_of_digits(w: &mut BigNumWorld, expect: usize) {
    assert_eq!(
        expect,
        w.bignum.sum(),
        "Expected {}, got {}",
        expect,
        w.bignum.sum()
    );
}

#[derive(Debug, Default, World)]
pub struct BigNumWorld {
    bignum: BigNum,
}

fn main() {
    futures::executor::block_on(BigNumWorld::run("tests/features"));
}
