use proconio::input;

fn main() {
    input! {
        n: usize,
        ai: [usize; n],
    }

    let result = ai
        .iter()
        .enumerate()
        .filter_map(|(i, a)| if i % 2 == 0 { Some(*a) } else { None })
        .sum::<usize>();

    println!("{}", result);
}
