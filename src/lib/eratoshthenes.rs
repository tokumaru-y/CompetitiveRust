fn eratoshthenes(n: usize) -> Vec<usize> {
    let mut res = Vec::new();
    let mut checked_num = vec![false; n + 1];

    if n <= 1 {
        return Vec::new();
    }
    checked_num[0] = true;
    checked_num[1] = true;
    for i in (2..=n) {
        if checked_num[i] {
            continue;
        }

        let mut tmp = i;
        while tmp <= n {
            checked_num[tmp] = true;
            tmp += i;
        }

        res.push(i);
    }

    res
}
