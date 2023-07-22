use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize
    }

    let mut ans = 0;
    for i in 1..=n {
        for j in 1..=n {
            if i + j < k && i + j + n >= k {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
