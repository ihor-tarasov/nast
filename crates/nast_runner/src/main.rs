
fn main() {
    match nast_dylib::run(std::fs::read_to_string("tests/factorial.json").unwrap().as_str()) {
        Ok(v) => println!("{v:?}"),
        Err(e) => println!("{e}"),
    }
}
