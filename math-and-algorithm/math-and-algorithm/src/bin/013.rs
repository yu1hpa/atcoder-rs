use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64
    }
    for ans in divisor(n) {
        println!("{}", ans);
    }
}

fn divisor(x: u64) -> Vec<u64> {
    let mut p = 1;
    let mut ans = Vec::new();
    while p * p <= x {
        if x % p == 0 {
            ans.push(p);
            if p * p != x {
                ans.push(x / p);
            }
        }
        p += 1;
    }
    ans
}
