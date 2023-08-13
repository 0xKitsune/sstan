use std::collections::HashSet;

use solang_parser::pt::{self, Loc, SourceUnit};

use crate::analyzer::extractors::{primitive::EqualityExtractor, Extractor};

pub const ZERO: &str = "0";

pub fn address_zero_optimization(source_unit: &mut SourceUnit) -> eyre::Result<HashSet<Loc>> {
    let mut optimization_locations: HashSet<Loc> = HashSet::new();

    let equality_nodes = EqualityExtractor::extract(source_unit)?;

    for node in equality_nodes {
        //We can use unwrap because Target::Equal and Target::NotEqual are expressions

        match node {
            pt::Expression::NotEqual(loc, box_expression, box_expression_1) => {
                if check_for_address_zero(*box_expression)
                    || check_for_address_zero(*box_expression_1)
                {
                    optimization_locations.insert(loc);
                }
            }
            pt::Expression::Equal(loc, box_expression, box_expression_1) => {
                if check_for_address_zero(*box_expression)
                    || check_for_address_zero(*box_expression_1)
                {
                    optimization_locations.insert(loc);
                }
            }
            _ => {}
        }
    }

    Ok(optimization_locations)
}

fn check_for_address_zero(box_expression: pt::Expression) -> bool {
    //create a boolean to determine if address(0) is present
    let mut address_zero: bool = false;

    //if the first expression is address(0)
    if let pt::Expression::FunctionCall(_, func_call_box_expression, vec_expression) =
        box_expression
    {
        if let pt::Expression::Type(_, pt::Type::Address) = *func_call_box_expression {
            if let pt::Expression::NumberLiteral(_, val, _, _) = &vec_expression[0] {
                if val == ZERO {
                    address_zero = true;
                }
            }
        }
    }

    //return true or false for address_zero
    address_zero
}

#[test]
fn test_address_zero_optimization() {
    let file_contents = r#"
    
    contract Contract0 {

        function ownerNotZero(address _addr) public pure {
            require(_addr == address(0), "zero address");
        }

        function ownerNotZero(address _addr) public pure {
            require(_addr != address(0), "zero address");
        }

        function ownerNotZero1(address _addr) public pure {
            require(address(0) == _addr, "zero address");
        }

        function ownerNotZero1(address _addr) public pure {
            require(address(0) != _addr, "zero address");
        }

     }
    "#;

    let mut source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let optimization_locations =
        address_zero_optimization(&mut source_unit).expect("TODO: propagate this instead");

    assert_eq!(optimization_locations.len(), 4)
}
