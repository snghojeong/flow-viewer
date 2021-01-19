fn next(st: &str) => &str {
    st[0..]
}

fn main() {
    let mut file = open();
    let mut line = String::new();
    call(file.read_line(&mut line)).filter(lv, md);
}
