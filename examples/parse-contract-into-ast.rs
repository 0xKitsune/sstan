fn main() {
    let file_contents = r#"
    
        pragma solidity ^0.8.16;

        contract SimpleStore {
            uint x;
            address z = address(1);
            function set(uint newValue) {
                x = 100000;
                uint256 y = 1e18;
                address p = address(1);
            }
            
            function get() returns (uint) {
                return x;
            }

            function get2() public {
                get();
            }
        }
    "#;

    let source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    println!("{:#?}", source_unit);
}
