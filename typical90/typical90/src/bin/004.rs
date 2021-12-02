use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[i32; w]; h],
    }

    let mut aw = vec![0i32; w];
    let mut ah = vec![0i32; h];
    for i in 0..h {
        for j in 0..w {
            ah[i] += a[i][j];
            aw[j] += a[i][j];
        }
    }
    for i in 0..h {
        for j in 0..w {
            if j != 0 {
                print!(" {}", ah[i] + aw[j] - a[i][j]);
            } else {
                print!("{}", ah[i] + aw[j] - a[i][j]);
            }
        }
        println!("");
    }
}
