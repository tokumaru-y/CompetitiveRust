fn divisor(n: &usize) -> HashMap<usize, usize>{
    let mut res = HashMap::new();
    let mut num = *n;
    for i in 2..=((*n as f64).sqrt() as usize) {
        while num % i == 0 {
            *res.entry(i).or_insert(0) += 1;
            num /= i
        }
    }
    if num != 1 && num > 0 { *res.entry(num).or_insert(0) += 1; }
    res
}