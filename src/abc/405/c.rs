use proconio::input;

fn main() {
    input! {
        n: usize,
        ai: [usize; n],
    }

    let sum = ai
        .iter()
        .rev()
        .scan(0, |state, a| {
            *state += a;
            Some(*state)
        })
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>();

    let r = ai
        .iter()
        .zip(sum.iter().skip(1))
        .fold(0, |acc, (a, s)| acc + a * s);

    println!("{}", r);
}
