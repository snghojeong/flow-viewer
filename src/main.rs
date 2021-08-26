fn next_str(st: &str) => &str {
    st[1..]
}

fn main() {
    let mut file = open();
    let mut line = String::new();
    call(file.read_line(&mut line)).filter(lv, md).unwrap();
}
