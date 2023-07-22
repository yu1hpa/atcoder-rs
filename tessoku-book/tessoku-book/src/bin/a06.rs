use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize;n],
        lr: [(usize, usize);q]
    }

    let mut sums: Vec<usize> = Vec::new();
    for lri in lr {
        let r = lri.0 - 1;
        let l = lri.1 - 1;
        let mut sum = 0;
        for s in &a[r..=l] {
            sum += s;
        }
        sums.push(sum);
    }

    for sum in sums {
        println!("{}", sum);
    }
}
