use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut n: u64,
    }

    let mut ans = "".to_string();

    while n > 0 {
        if n % 2 != 0 {
            ans.push_str("A");
            n -= 1;
        } else {
            ans.push_str("B");
            n /= 2;
        }
    }
    println!("{}", ans.chars().rev().collect::<String>());
}
