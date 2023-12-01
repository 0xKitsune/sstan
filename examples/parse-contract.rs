fn main() {
    let file_contents = r#"

contract Contract0 {
        
        function blindDecimalsCall(addrress arbitraryToken) public returns (uint8) {
            return IERC20(arbitraryToken).decimals();
        }
}
    "#;

    let source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    println!("{:#?}", source_unit);
}
