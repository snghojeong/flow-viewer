fn main() {
    let mut file = open();
    let mut line = String::new();
    call(read_file(&mut file)).read_line().filter();
}
