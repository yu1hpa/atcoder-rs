use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: [usize; 3]
    }

    let ans = a.iter().fold(1, |v, x| v * x);
    println!("{}", ans);
}
