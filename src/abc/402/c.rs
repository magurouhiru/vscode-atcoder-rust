use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ki: [[usize];m],
        bi: [usize; n],
    }

    let mut ib = vec![0; n + 1];
    for (i, b) in bi.iter().enumerate() {
        ib[*b] = i;
    }

    let mut tmp = vec![0; n];
    for k in ki.iter() {
        let j = k.iter().map(|x| ib[*x]).max().unwrap();
        tmp[j] += 1;
    }

    let r = tmp
        .iter()
        .scan(0, |state, x| {
            *state += *x;
            Some(*state)
        })
        .join("\n");
    println!("{}", r);
}
