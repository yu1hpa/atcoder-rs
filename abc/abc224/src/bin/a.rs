use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { s: String }
    println!(
        "{}",
        if s.chars().rev().next().unwrap() == 'r' {
            "er"
        } else {
            "ist"
        }
    )
}
