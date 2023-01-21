use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut p: usize
    }
    let mut fac = vec![0; 11];
    fac[0] = 1;
    for i in 1..=10 {
        fac[i] = fac[i - 1] * i;
    }

    let mut ans = 0;
    for i in (1..=10).rev() {
        while p >= fac[i] {
            ans += 1;
            p -= fac[i]
        }
    }
    println!("{}", ans);
}
