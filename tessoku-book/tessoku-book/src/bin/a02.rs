use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize;n],
    }

    for ai in a {
        if ai == x {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
