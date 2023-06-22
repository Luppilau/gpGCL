extern crate lalrpop;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // lalrpop::process_root().unwrap();
    lalrpop::Configuration::new()
        .generate_in_source_tree()
        .process()
}
