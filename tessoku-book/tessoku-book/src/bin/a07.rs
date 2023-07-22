use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        d: usize,
        n: usize,
        lr: [(usize,usize);n],
    }
    let mut table = vec![0; d + 2];
    for lri in lr {
        table[lri.0] += 1;
        table[lri.1 + 1] -= 1;
    }

    for i in 1..=d {
        table[i] += table[i - 1];
        println!("{}", table[i]);
    }
}
