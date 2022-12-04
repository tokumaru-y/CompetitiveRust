// reference: https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/dsu.rs
struct UnionFind {
    node: usize,
    parent_or_size: Vec<isize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            node: size,
            // parent_or_size[x] < 0:x is parent
            // parent_or_size[x] > 0:x's parent is parent_or_size[x]
            parent_or_size: vec![-1; size],
        }
    }

    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    pub fn root(&mut self, x: usize) -> usize {
        if self.parent_or_size[x] < 0 {
            return x;
        }
        self.parent_or_size[x] = self.root(self.parent_or_size[x] as usize) as isize;
        self.parent_or_size[x] as usize
    }

    pub fn unite(&mut self, x: usize, y: usize) -> usize {
        let (mut a, mut b) = (self.root(x), self.root(y));
        if a == b {
            return a;
        }
        if -self.parent_or_size[a] < -self.parent_or_size[b] {
            std::mem::swap(&mut a, &mut b);
        }
        self.parent_or_size[a] += self.parent_or_size[b];
        self.parent_or_size[b] = a as isize;
        a
    }

    pub fn size(&mut self, x: usize) -> usize {
        let a = self.root(x);
        -self.parent_or_size[a] as usize
    }

    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut roots = vec![0; self.node];
        let mut group_size = vec![0; self.node];
        for i in 0..self.node {
            roots[i] = self.root(i)
            group_size[roots[i]] += 1;
        }
        let mut res = vec![Vec::new(), self.node];
        for i in 0..self.node {
            res[i].reserve(group_size[i]);
        }
        for i in 0..self.node {
            res[roots[i]].push(i);
        }

        res.into_iter().filter(|x| !x.is_empty()).collect::<Vec<Vec<usize>>>()
    }
}
