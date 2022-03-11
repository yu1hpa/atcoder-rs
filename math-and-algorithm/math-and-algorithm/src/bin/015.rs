use proconio::{fastout, input};
use num::integer::gcd;

#[fastout]
fn main() {
    input! {
        a: u64,
        b: u64
    }

    println!("{}", gcd(a, b));
}
