use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[u32; w]; h],
    }

    let mut f = true;
    for (i0, i1) in (0..h).tuple_combinations() {
        for (j0, j1) in (0..w).tuple_combinations() {
            f &= a[i0][j0] + a[i1][j1] <= a[i1][j0] + a[i0][j1];
        }
    }
    println!("{}", if f { "Yes" } else { "No" });
}
