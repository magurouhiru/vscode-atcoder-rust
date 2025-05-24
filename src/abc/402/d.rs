use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        abi: [(usize,usize); m],
    }

    let mut tmp: Vec<usize> = vec![0; n];
    for (a, b) in abi.iter() {
        tmp[(a + b) % n] += 1;
    }

    let tmp: Vec<usize> = tmp.into_iter().filter(|x| *x != 0).collect();
    let cnt: Vec<usize> = tmp
        .iter()
        .scan(0, |state, x| {
            *state += *x;
            Some(*state)
        })
        .collect();

    if tmp.len() < 2 {
        println!("0");
        return;
    }

    let mut r = 0;
    for (i, &t) in tmp.iter().enumerate().skip(1) {
        r += cnt[i - 1] * t;
    }
    println!("{}", r);
}
