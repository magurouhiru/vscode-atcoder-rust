use proconio::input;

fn main() {
    input! {
        n: usize,
        pi: [usize; n],
    }

    let mut yama_tani = pi
        .iter()
        .skip(1)
        .scan(pi[0], |state, p| {
            let buf = *state;
            *state = *p;
            if buf < *p {
                Some('<')
            } else {
                Some('>')
            }
        })
        .collect::<Vec<_>>();

    yama_tani.insert(0, '>'); // Prepend '>' to the start
    yama_tani.push('>'); // Append '>' to the end

    let mut yama = vec![];
    let mut tani = vec![];
    for i in 0..(yama_tani.len() - 1) {
        if yama_tani[i] == '<' && yama_tani[i + 1] == '>' {
            yama.push(i);
        } else if yama_tani[i] == '>' && yama_tani[i + 1] == '<' {
            tani.push(i);
        }
    }

    let cnt = yama
        .iter()
        .zip(tani.iter())
        .map(|(y, t)| y - t)
        .collect::<Vec<_>>();

    let cnt = cnt
        .iter()
        .zip(cnt.iter().skip(1))
        .fold(0, |acc, (a, b)| acc + a * b);

    println!("{:?}", cnt);
}
