use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        n:usize,
        abcd: [(usize, usize, usize,usize);n],
    }
    let mut table = vec![vec![0; w + 1]; h + 1];

    for (a, b, c, d) in abcd {
        table[a - 1][b - 1] += 1;
        table[a - 1][d] -= 1;
        table[c][b - 1] -= 1;
        table[c][d] += 1;
    }

    let mut ex = vec![vec![0; w + 1]; h + 1];
    // 横方向
    for iy in 1..=h {
        for ix in 1..=w {
            ex[iy][ix] = table[iy - 1][ix - 1] + ex[iy][ix - 1];
        }
    }

    // 縦方向
    for iy in 1..=h {
        for ix in 1..=w {
            ex[iy][ix] += ex[iy - 1][ix];
        }
    }

    for iy in 1..=h {
        for ix in 1..=w {
            if ix != w {
                // with whitespace
                print!("{} ", ex[iy][ix]);
            } else {
                // without whitespace
                print!("{}", ex[iy][ix]);
            }
        }
        println!("");
    }
}
