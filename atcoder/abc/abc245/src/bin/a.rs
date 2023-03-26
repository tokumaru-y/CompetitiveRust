use proconio::input;

fn main() {
    input!{
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }
    let ans;
    if c>a || (c == a && d >= b) {
        ans = "Takahashi";
    } else {
        ans = "Aoki";
    }
    println!("{}", ans);
}