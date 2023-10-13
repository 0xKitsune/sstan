fn main() {
    let file_contents = r#"
    
    import "Errors.sol";

    error HasNoParameters();
    error HasParameters(uint256 a, uint256 b);

    contract contract0 {
        function test() public {
            revert HasNoParameters();
        }

        function test2() public {
            revert ImportedErrorWithoutParameters();
        }

        function test3() public {
            revert HasParameters(1, 2);
        }
        
    }
    "#;

    let source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    println!("{:#?}", source_unit);
}
