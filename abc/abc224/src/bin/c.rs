use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n]
    }

    let mut ans = 0;
    for ((x0, y0), (mut x1, mut y1), (mut x2, mut y2)) in xy.iter().tuple_combinations() {
        x1 -= x0;
        y1 -= y0;
        x2 -= x0;
        y2 -= y0;
        if x1 * y2 != x2 * y1 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
