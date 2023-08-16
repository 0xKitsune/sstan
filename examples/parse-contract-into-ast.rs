fn main() {
    let file_contents = r#"
    
        pragma solidity ^0.8.16;

        contract SimpleStore {
            uint x;

            function set(uint newValue) {
                x = 100000;
                uint256 y = 1e18;
            }
            
            function get() returns (uint) {
                return x;
            }
        }
    "#;

    let source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    println!("{:#?}", source_unit);
}
