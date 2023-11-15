fn main() {
    let file_contents = r#"

    interface contract0 {        
    }
    "#;

    let source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    println!("{:#?}", source_unit);
}
