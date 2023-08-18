fn main() {
    let file_contents = r#"
    
        pragma solidity ^0.8.16;

        contract SimpleStore {
           
            address hardcoded = 0xCF93bC53DA6D3543ec2B39EB9Fb3eb1472502afA;

            function foo() public returns (uint256 x) {
                x = 1;
                assembly {
                    x := shl(x, 1)
                }
            }
            
        }
    "#;

    let source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    println!("{:#?}", source_unit);
}
