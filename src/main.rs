fn next_str(st: &str) => &str {
    st[1..]
}

fn main() {
    let file = open();
    let line = String::new();
    call(file.read_line(&mut line)).filter(lv, md).unwrap();
}
