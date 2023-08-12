use std::collections::HashSet;
use std::u32;

use solang_parser::pt::{Expression, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::analyzer::extractors::{primitive::UrnaryOpteratorExtractor, Extractor};

pub fn shift_math_optimization(source_unit: &mut SourceUnit) -> eyre::Result<HashSet<Loc>> {
    let mut optimization_locations: HashSet<Loc> = HashSet::new();

    let urnary_operator_nodes = UrnaryOpteratorExtractor::extract(source_unit)?;

    for node in urnary_operator_nodes {
        //We can use expect because both Target::Multiply and Target::Divide are expressions
        match node {
            Expression::Multiply(loc, box_expression, box_expression_1) => {
                if check_if_inputs_are_power_of_two(*box_expression, *box_expression_1) {
                    optimization_locations.insert(loc);
                }
            }

            Expression::Divide(loc, box_expression, box_expression_1) => {
                if check_if_inputs_are_power_of_two(*box_expression, *box_expression_1) {
                    optimization_locations.insert(loc);
                }
            }

            _ => {}
        }
    }
    Ok(optimization_locations)
}

fn check_if_inputs_are_power_of_two(
    box_expression: Expression,
    box_expression_1: Expression,
) -> bool {
    //create a boolean to determine if either of the inputs are a power of two
    let mut is_even: bool = false;

    //if the first expression is a number literal that is a power of 2
    if let Expression::NumberLiteral(_, val_string, _, _) = box_expression {
        let value = val_string
            .parse::<u32>()
            .expect("Could not parse NumberLiteral value from string to u32");

        if (value != 0) && ((value & (value - 1)) == 0) {
            is_even = true;
        }
    }

    //if the first expression is a number literal that is a power of 2
    if let Expression::NumberLiteral(_, val_string, _, _) = box_expression_1 {
        let value = val_string
            .parse::<u32>()
            .expect("Could not parse NumberLiteral value from string to u32");

        if (value != 0) && ((value & (value - 1)) == 0) {
            is_even = true;
        }
    }

    is_even
}

#[test]
fn test_shift_math_optimization() {
    let file_contents = r#"

    contract Contract0 {

        function mul2(uint256 a, uint256 b) public pure {
            uint256 a = 10 * 2;

            uint256 b = 2 * a;
            uint256 c = a * b;

            uint256 d = (a * b) * 2;
        }
    }
    "#;

    let mut source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let optimization_locations = shift_math_optimization(&mut source_unit);

    assert_eq!(optimization_locations.unwrap().len(), 3)
}
