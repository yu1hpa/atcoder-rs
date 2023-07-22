use proconio::{fastout, input};

fn dec2bin(num: u32) -> Vec<u32> {
    let mut a: Vec<u32> = Vec::new();
    let mut n = num;
    while n > 0 {
        let bit = n % 2;
        a.push(bit);
        n /= 2;
    }
    a.reverse();
    a
}

fn make_ten_digit(nums: Vec<u32>) -> Vec<u32> {
    let mut mut_nums = nums;
    if mut_nums.len() != 10 {
        let lack_len = 10 - mut_nums.len();
        for _ in 0..lack_len {
            mut_nums.insert(0, 0);
        }
    }
    mut_nums
}

#[fastout]
fn main() {
    input! {
        n: u32,
    }
    let bin = make_ten_digit(dec2bin(n));
    let result = bin
        .into_iter()
        .map(|num| num.to_string())
        .collect::<Vec<String>>()
        .join("");
    println!("{}", result);
}
