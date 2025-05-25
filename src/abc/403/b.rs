use proconio::{input, marker::Chars};

fn main() {
    input! {
        ti: Chars,
        ui: Chars,
    }

    let mut flag = false;
    for i in 0..(ti.len() - ui.len() + 1) {
        let f = ti
            .iter()
            .skip(i)
            .take(ui.len())
            .zip(ui.iter())
            .all(|(t, u)| t == u || *t == '?');
        if f {
            flag = true;
            break;
        }
    }

    if flag {
        println!("Yes");
    } else {
        println!("No");
    }
}
