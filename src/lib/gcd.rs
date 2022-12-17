fn gcd(x: usize, y: usize) -> usize {
    let mut x = x;
    let mut y = y;
    if x < y {
        std::mem::swap(&mut x, &mut y);
    }
    if y == 0 {
        return x;
    }
    return gcd(y, x % y);
}

fn lcm(x: usize, y: usize) -> usize {
    let mut x = x;
    let mut y = y;
    if x < y {
        std::mem::swap(&mut x, &mut y);
    }
    x / gcd(x, y) * y
}

fn ext_gcd(a: isize, b: isize, x: &mut isize, y: &mut isize) -> isize {
    if b == 0 {
        *x = 1;
        *y = 0;
        return a;
    }
    let d = ext_gcd(b, a % b, y, x);
    *y -= a / b * (*x);
    d
}
