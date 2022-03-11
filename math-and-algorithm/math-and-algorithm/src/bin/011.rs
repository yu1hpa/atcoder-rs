use proconio::{fastout, input};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {
        n: u64
    }
    println!("{}", (2..=n).filter(|n| is_prime(*n)).join(" "));
}

fn is_prime(x: u64) -> bool{
    let mut p = 2;
    while p * p <= x{
        if x % p == 0 {
            return false;
        }
        p += 1;
    }
    return true;
}
