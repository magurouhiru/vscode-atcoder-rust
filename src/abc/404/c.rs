use petgraph::unionfind::UnionFind;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        abi: [(usize, usize); m],
    }

    if n != m {
        println!("No");
        return;
    }

    let cnt = abi.iter().fold(vec![0; n + 1], |mut acc, (a, b)| {
        acc[*a] += 1;
        acc[*b] += 1;
        acc
    });
    let cnt = cnt.iter().skip(1).all(|&x| x == 2);
    if !cnt {
        println!("No");
        return;
    }

    let mut uf: UnionFind<usize> = UnionFind::new(n);
    for (a, b) in abi {
        uf.union(a - 1, b - 1);
    }

    (1..n)
        .collect::<Vec<usize>>()
        .iter()
        .all(|x| uf.equiv(0, *x))
        .then(|| println!("Yes"))
        .unwrap_or_else(|| println!("No"));
}
