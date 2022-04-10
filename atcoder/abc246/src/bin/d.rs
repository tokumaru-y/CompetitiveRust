use proconio::input;

fn calc(a: &usize, b: &usize) -> usize{
    a*a*a + (a*a)*b + a*(b*b) + b*b*b
}

fn main() {
    input!{
        N: usize,
    }
    let mut ans = 1_000_000_000_000_000_000;
    for a in 0..1000_001{
        let mut left = 0;
        let mut right = 1000_001;
        while right - left > 1 {
            let middle = (right+left) / 2;
            if calc(&a, &middle) >= N {
                right = middle;
            } else {
                left = middle;
            }
        }
        let cmp_num = if calc(&a, &left) >= N { left } else { right };
        ans = std::cmp::min(ans, calc(&a, &cmp_num));
    }
    println!("{}", ans);
}