use proconio::{fastout, input};
 
#[fastout]
fn main() {
    input! {
        n: u64,
    }
    let v = gen(n);
    println!("{}", v.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
}

fn gen(n: u64) -> Vec<u64> {
    return match n {
        1 => vec![1],
        _ => {
            let v0: Vec<u64> = gen(n - 1);
            let v1 = v0.clone();
            [v0, vec![n], v1].concat()
        },
    }
}
