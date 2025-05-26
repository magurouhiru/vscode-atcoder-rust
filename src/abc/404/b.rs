use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        sii: [Chars; n],
        tii: [Chars; n],
    }

    let ftii: Vec<char> = tii.iter().flat_map(|x| x.iter()).copied().collect();
    let (_, r) = [0, 1, 2, 3]
        .iter()
        .fold((sii, usize::MAX), |(sii, diff), &x| {
            let mut tmp_diff = sii
                .iter()
                .flat_map(|x| x.iter())
                .copied()
                .zip(ftii.iter())
                .filter(|(s, t)| s != *t)
                .count();
            tmp_diff += x;
            let mut rotate = vec![vec!['?'; n]; n];
            for (i, row) in rotate.iter_mut().enumerate() {
                for (j, cell) in row.iter_mut().enumerate() {
                    *cell = sii[n - 1 - j][i];
                }
            }
            (rotate, diff.min(tmp_diff))
        });

    println!("{}", r);
}
