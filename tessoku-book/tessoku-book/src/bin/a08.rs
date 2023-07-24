use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        x: [[usize;w];h],
        q: usize,
        abcd: [(usize, usize, usize, usize);q],
    }
    let mut ex = vec![vec![0; w + 1]; h + 1];
    // 横方向
    for iy in 1..=h {
        for ix in 1..=w {
            ex[iy][ix] = x[iy - 1][ix - 1] + ex[iy][ix - 1];
        }
    }

    // 縦方向
    for iy in 1..=h {
        for ix in 1..=w {
            ex[iy][ix] += ex[iy - 1][ix];
        }
    }

    for (a, b, c, d) in abcd {
        println!(
            "{}",
            ex[c][d] + ex[a - 1][b - 1] - ex[a - 1][d] - ex[c][b - 1]
        )
    }
}
