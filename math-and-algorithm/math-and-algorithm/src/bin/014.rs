use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64,
    }

    let mut p = 2;
    let mut a = n;
 
    let mut v: Vec<u64> = Vec::new();
 
    while p * p <= n {
        if a % p == 0 {
            a /= p;
            v.push(p);
            continue;
        }
        p += 1;
    }
 
    if a != 1 {
        v.push(a);
    }
 
    println!("{}", v.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}
