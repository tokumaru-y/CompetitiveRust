struct BIT {
    // 1-indexed
    node: Vec<usize>,
    N: usize,
}

impl BIT {
    pub fn new(size: usize) -> Self {
        Self {
            node: vec![0; size + 1],
            N: size,
        }
    }

    pub fn add(&mut self, ind: usize, val: usize) {
        let mut x = ind;
        while x <= self.N {
            self.node[x] += val;
            x += x & x.wrapping_neg();
        }
    }

    pub fn sum(&self, ind: usize) -> usize {
        let mut res = 0;
        let mut x = ind;

        while (0 < x) {
            res += self.node[x];
            x -= x & x.wrapping_neg();
        }

        res
    }
}
