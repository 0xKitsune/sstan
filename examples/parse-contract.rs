fn main() {
    let file_contents = r#"

    contract Contract0 {
        modifier someModifier() {
            _;
        }
        function some_function() public {}
    }
    "#;

    let source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    println!("{:#?}", source_unit);
}
