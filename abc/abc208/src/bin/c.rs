use proconio::{fastout, input};
 
#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize;n],
    }
    let mut order = vec![(0, 0); n];
    for i in 0..n {
        order[i] = (a[i], i);
    }
    order.sort();
    let mut ans = vec![k / n; n]; // できるだけ配り切る
    for i in 0..k%n { // 配りきれなかったものを配る
        ans[order[i].1] += 1;
    }
    for a in ans {
        println!("{}", a);
    }
}
