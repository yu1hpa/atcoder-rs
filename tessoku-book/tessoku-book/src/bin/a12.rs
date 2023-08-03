use proconio::{fastout, input};

fn binary_search(a: Vec<usize>, start: usize, end: usize, v: usize) -> usize {
    let mut sum = 0;
    let point = (start + end) / 2;
    for ai in a.iter() {
        sum += point / ai
    }
    if (end - start) == 1 {
        end
    } else if sum < v {
        binary_search(a, point, end, v)
    } else {
        binary_search(a, start, point, v)
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize;n]
    }
    println!("{}", binary_search(a, 0, 1000000000, k));
}
