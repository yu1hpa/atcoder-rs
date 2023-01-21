use proconio::{fastout, input};
 
#[fastout]
fn main() {
    input! {
        a: usize,
        mut b: usize
    }
    if b < a || 6*a < b {
        println!("No");
    } else {
        println!("Yes");
    }
}
