use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        ai: [usize; q],
    }

    let mut cnt = 0usize;
    let mut tmp = vec![false; n + 2];
    let mut result = vec![0usize; q];
    for (i, &a) in ai.iter().enumerate() {
        match (tmp[a - 1], tmp[a], tmp[a + 1]) {
            (false, false, false) => {
                cnt += 1;
                tmp[a] = true;
            }
            (false, true, false) => {
                cnt -= 1;
                tmp[a] = false;
            }
            (true, false, false) => {
                tmp[a] = true;
            }
            (true, true, false) => {
                tmp[a] = false;
            }
            (false, false, true) => {
                tmp[a] = true;
            }
            (false, true, true) => {
                tmp[a] = false;
            }
            (true, false, true) => {
                cnt -= 1;
                tmp[a] = true;
            }
            (true, true, true) => {
                cnt += 1;
                tmp[a] = false;
            }
        }
        result[i] = cnt;
    }
    println!(
        "{}",
        result
            .iter()
            .map(|&x| x.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );
}
