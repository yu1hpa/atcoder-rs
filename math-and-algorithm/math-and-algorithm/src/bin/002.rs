use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: [usize; 3]
    }
    let sum: usize = a.iter().sum();
    println!("{}", sum);
}
