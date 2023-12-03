fn main() {
    let file_contents = r#"

    contract contract0 {        
        mapping(uint => uint) public map0;
        mapping(uint number => uint number2) public map1;
    }
    "#;

    let source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    println!("{:#?}", source_unit);
}
