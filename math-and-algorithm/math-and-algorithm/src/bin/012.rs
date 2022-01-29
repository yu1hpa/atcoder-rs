use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64
    }
    println!("{}", if is_prime(n) { "Yes" } else { "No" })
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
