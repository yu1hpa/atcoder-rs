use num::pow;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        b: usize,
    }
    for a in 1..=15 {
        if pow(a, a) == b {
            println!("{}", a);
            return;
        }
    }
    println!("-1");
}
