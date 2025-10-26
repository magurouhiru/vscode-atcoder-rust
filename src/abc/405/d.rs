use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        mut s: [Chars; h],
    }
    let mut next = s
        .iter()
        .enumerate()
        .flat_map(|(i, si)| {
            si.iter()
                .enumerate()
                .filter(|(_, sij)| **sij == 'E')
                .map(|(j, _)| (i, j))
                .collect::<Vec<(usize, usize)>>()
        })
        .collect::<Vec<(usize, usize)>>();

    while !next.is_empty() {
        let mut tmp = vec![];
        for (ny, nx) in next {
            // top
            if ny > 0 && s[ny - 1][nx] == '.' {
                s[ny - 1][nx] = 'v';
                tmp.push((ny - 1, nx));
            }
            // bottom
            if ny < h - 1 && s[ny + 1][nx] == '.' {
                s[ny + 1][nx] = '^';
                tmp.push((ny + 1, nx));
            }
            // left
            if nx > 0 && s[ny][nx - 1] == '.' {
                s[ny][nx - 1] = '>';
                tmp.push((ny, nx - 1));
            }
            // right
            if nx < w - 1 && s[ny][nx + 1] == '.' {
                s[ny][nx + 1] = '<';
                tmp.push((ny, nx + 1));
            }
        }
        next = tmp;
    }
    println!(
        "{}",
        s.iter()
            .map(|si| si.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    );
}
