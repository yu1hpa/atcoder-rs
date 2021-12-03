use proconio::marker::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        cp: [(Usize1, u32); n],
        q: usize,
        lr: [(Usize1, Usize1); q],
    }

    let mut sums = vec![vec![0; n + 1]; 2];
    for i in 0..n {
        sums[0][i + 1] = sums[0][i];
        sums[1][i + 1] = sums[1][i];
        sums[cp[i].0][i + 1] += cp[i].1;
    }

    for (l, r) in lr {
        println!(
            "{} {}",
            sums[0][r + 1] - sums[0][l],
            sums[1][r + 1] - sums[1][l]
        );
    }
}
