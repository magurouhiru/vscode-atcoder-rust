use proconio::input;

fn main() {
    input! {
        ai: [usize; 7],
    }

    let mut ci = vec![0; 14];
    for a in ai {
        ci[a] += 1;
    }

    let mut f3 = false;
    for c in ci.iter_mut() {
        if *c >= 3 {
            *c = 0;
            f3 = true;
            break;
        }
    }

    let mut f2 = false;
    for c in ci.iter_mut() {
        if *c >= 2 {
            *c = 0;
            f2 = true;
            break;
        }
    }

    if f3 && f2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
