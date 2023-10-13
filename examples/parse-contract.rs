fn main() {
    let file_contents = r#"

    contract contract0 {
        uint128 constant doubleCast = uint128(uint256(1));
        
    }
    "#;

    let source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    println!("{:#?}", source_unit);
}
