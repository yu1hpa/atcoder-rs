use proconio::{fastout, input};
 
#[fastout]
fn main() {
    input! {
        n: u64,
        mut an: [u64;n],
    }
    an.sort();
    an.reverse();
    let (ae, ao): (Vec<u64>, Vec<u64>) = an.into_iter().partition(|n| n % 2 == 0);
    let max_ae = ae.get(0..2).map(|v| v[0]+v[1]);
    let max_ao = ao.get(0..2).map(|v| v[0]+v[1]);
    let max = vec![max_ae, max_ao].into_iter().max().unwrap();
    match max {
        Some(m) => println!("{}", m),
        None => println!("-1"),
    }
}
