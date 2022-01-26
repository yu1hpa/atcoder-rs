use proconio::{fastout, input};
use itertools::iproduct;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: usize
    }
    println!("{}", iproduct!(1..=n, 1..=n).filter(|&x| (x.0 + x.1) <= s).count());
}
