fn main() {
    match grammar::parse("test:=1") {
        Ok(ast) => println!("{:?}", ast),
        Err(e) => println!("{:?}", e),
    }
}
