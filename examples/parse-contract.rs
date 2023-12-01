fn main() {
    let file_contents = r#"

    contract contract0 {        
        function safeApproval(address spender, uint256 value, address token) public {
            IERC20(token).approve(spender, value);
        }

        function maxApproval(address spender, address token) public {
            IERC20(token).approve(spender, type(uint256).max);
        }
    }
    "#;

    let source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    println!("{:#?}", source_unit);
}
