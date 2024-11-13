use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _n: usize,
        s: String,
    }
    println!(
        "{}",
        if s.contains("ab") || s.contains("ba") {
            "Yes"
        } else {
            "No"
        }
    );
}
