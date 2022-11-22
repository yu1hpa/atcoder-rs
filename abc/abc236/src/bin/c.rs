use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: u64,
        m: u64,
        s: [String;n],
        t: [String;m],
    }

    let uniq_t: HashSet<String> = t.into_iter().collect();

    for si in s {
        if uniq_t.contains(&si) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
