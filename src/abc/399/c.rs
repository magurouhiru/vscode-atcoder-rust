use im_rc::HashSet;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    if m == 0 {
        println!("0");
        return;
    }

    input! {
        uvi: [(Usize1, Usize1); m],
    }

    let mut uf = UnionFind::new(n);
    for (u, v) in uvi {
        uf.union(u, v);
    }

    let k: usize = uf.len();
    let cnt = i64::try_from(m).unwrap() - i64::try_from(n - k).unwrap();
    if cnt < 0 {
        println!("{}", 0);
    } else {
        println!("{}", cnt);
    }
}

struct UnionFind {
    roots: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        Self {
            roots: (0..size).collect(),
        }
    }

    fn find_mut(&mut self, x: usize) -> usize {
        if self.roots[x] != x {
            self.roots[x] = self.find(self.roots[x]);
        }
        self.roots[x]
    }

    fn find(&self, x: usize) -> usize {
        if self.roots[x] != x {
            return self.find(self.roots[x]);
        }
        self.roots[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find_mut(x);
        let root_y = self.find_mut(y);
        if root_x != root_y {
            self.roots[root_y] = root_x;
        }
    }

    fn len(&mut self) -> usize {
        let mut s = HashSet::new();
        for x in &self.roots {
            s.insert(self.find(*x));
        }
        s.len()
    }
}
