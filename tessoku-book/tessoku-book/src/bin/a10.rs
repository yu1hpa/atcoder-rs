use proconio::{fastout, input};
use std::cmp;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize;n],
        d: usize,
        lr: [(usize, usize);d],
    }
    let mut p = vec![0; n + 1];
    p[1] = a[0];
    for i in 2..=n {
        p[i] = cmp::max(p[i - 1], a[i - 1]);
    }

    let mut q = vec![0; n + 1];
    q[n] = a[n - 1];
    for i in (2..=n).rev() {
        q[i - 1] = cmp::max(q[i], a[i - 2]);
    }

    for (l, r) in lr {
        println!("{}", cmp::max(p[l - 1], q[r + 1]));
    }
}
