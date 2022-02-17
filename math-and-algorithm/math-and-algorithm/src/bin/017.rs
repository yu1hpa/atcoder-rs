use proconio::{fastout, input};
use num::integer::Integer;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    println!("{}", a.iter().fold(1, |b, c| b.lcm(c)));
}
