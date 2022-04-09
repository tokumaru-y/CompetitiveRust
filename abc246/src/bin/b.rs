use proconio::{input, marker::Chars};
fn main() {
    input! {
        A: isize,
        B: isize,
    }
    let len : f64 = (((A * A) as f64) + ((B * B) as f64)).sqrt();
    let ans_x = A as f64 / len;
    let ans_y = B as f64 / len;
    println!("{} {}", ans_x, ans_y);
}