use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let binding = s.iter().fold(
        "abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<_>>(),
        |state, c| state.into_iter().filter(|x| *c != *x).collect::<Vec<_>>(),
    );
    let r = binding.first().unwrap();
    println!("{}", *r);
}
