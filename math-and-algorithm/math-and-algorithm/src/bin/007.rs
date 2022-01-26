use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize
    }

    println!("{}", (1..=n).filter(|n| n % x == 0 || n % y == 0).count());
}
