use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut set = std::collections::HashSet::new();
    for (i, si) in s.iter().take(n - 1).enumerate() {
        for sj in s.iter().skip(i + 1) {
            set.insert(format!("{}{}", si, sj));
            set.insert(format!("{}{}", sj, si));
        }
    }

    println!("{}", set.len());
}
