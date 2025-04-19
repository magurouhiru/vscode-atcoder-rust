use proconio::input;

fn main() {
    input! {
        n: u32,
        si: [String; n],
    }

    let login = "login";
    let logout = "logout";
    let private = "private";

    let mut cnt = 0;
    let mut status = logout;
    si.iter().for_each(|s| match (s.as_str(), status) {
        (l, _) if l == login => status = login,
        (l, _) if l == logout => status = logout,
        (p, logout_status) if p == private && logout_status == logout => cnt += 1,
        _ => {}
    });

    println!("{}", cnt);
}
