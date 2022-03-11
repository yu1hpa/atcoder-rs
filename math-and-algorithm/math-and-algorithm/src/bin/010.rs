use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize
    }

    println!("{}", factorial(n));
}

fn factorial(n: usize) -> usize {
    if n != 0 {
        factorial(n - 1) * n
    } else {
        1
    }
}
