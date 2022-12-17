// 素因数分解
fn divisor(n: &usize) -> HashMap<usize, usize> {
    let mut res = HashMap::new();
    let mut num = *n;
    for i in 2..=((*n as f64).sqrt() as usize) {
        while num % i == 0 {
            *res.entry(i).or_insert(0) += 1;
            num /= i
        }
    }
    if num != 1 && num > 0 {
        *res.entry(num).or_insert(0) += 1;
    }
    res
}

fn divisor_list(n: &usize) -> Vec<usize> {
    let mut res = Vec::new();
    for i in 1..=((*n as f64).sqrt() as usize) {
        if n % i == 0 {
            res.push(i);
            if i != n / i {
                res.push(n / i);
            }
        }
    }
    res
}

//
