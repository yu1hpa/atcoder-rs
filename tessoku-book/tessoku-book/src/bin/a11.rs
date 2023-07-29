use proconio::{fastout, input};

fn binary_search(a: Vec<usize>, start: usize, end: usize, v: usize) -> usize {
    let point = (start + end) / 2;
    if a[point] == v {
        point
    } else if a[point] < v {
        binary_search(a, point, end, v)
    } else {
        binary_search(a, start, point, v)
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize;n],
    }
    println!("{}", binary_search(a, 0, n, x) + 1);
}
