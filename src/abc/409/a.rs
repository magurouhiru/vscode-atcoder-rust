use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        t: Chars,
        a: Chars,
    }

    let result = t
        .iter()
        .zip(a.iter())
        .any(|(tc, ac)| matches!((tc, ac), ('o', 'o')));
    if result {
        println!("Yes");
    } else {
        println!("No");
    }
}
