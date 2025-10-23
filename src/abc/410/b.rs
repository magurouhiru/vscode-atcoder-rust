use proconio::input;

fn main() {
    input! {
        (n, q): (usize, usize),
        xi: [usize; q],
    }

    let mut counts = vec![0; n];
    let mut results = vec![0; q];
    for (i, &x) in xi.iter().enumerate() {
        let r = if x == 0 {
            let min = counts.iter().min().unwrap();
            counts.iter().position(|&c| c == *min).unwrap() + 1
        } else {
            x
        };
        results[i] = r;
        counts[r - 1] += 1;
    }

    let result_str = results
        .iter()
        .map(|r| r.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", result_str);
}
