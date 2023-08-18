use std::collections::HashSet;

use solang_parser::pt::{self, Loc, SourceUnit};

use crate::analyzer::extractors::{compound::YulShiftExtractor, Extractor};

pub fn incorrect_shift_math_vulnerability(
    source_unit: &mut SourceUnit,
) -> eyre::Result<HashSet<Loc>> {
    let mut vulnerability_locations: HashSet<Loc> = HashSet::new();
    let yul_shift_function_calls = YulShiftExtractor::extract(source_unit)?;
    //Iterate through each shift. If the first element of the shift is a Variable, and the second is a NumberLiteral | HexNumberLiteral this is likely a vulnerability.
    for shift_function_call in yul_shift_function_calls {
        let shift_amount = shift_function_call.arguments.first();
        let shifted_variable = shift_function_call.arguments.last();

        if let Some(shift_amount) = shift_amount {
            if let Some(shifted_variable) = shifted_variable {
                if let pt::YulExpression::Variable(_) = shift_amount {
                    if let pt::YulExpression::NumberLiteral(_, _, _, _) = shifted_variable {
                        vulnerability_locations.insert(shift_function_call.loc);
                    }
                    if let pt::YulExpression::HexNumberLiteral(_, _, _) = shifted_variable {
                        vulnerability_locations.insert(shift_function_call.loc);
                    }
                }
            }
        }
    }
    Ok(vulnerability_locations)
}

#[test]
fn test_incorrect_shift_math() {
    let file_contents = r#"
    
    contract Contract0 {
        function incorrectShiftMath() public returns (uint256 x) {
            assembly {
                let x := shl(x, 1)
                let x := shr(x, 1)
            }
        }

        function correctShiftMath() public returns (uint256 x) {
            assembly {
                let x := shl(1, x)
                let x := shr(1, x)
            }
        }
    }
    "#;

    let mut source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let vulnerability_locations = incorrect_shift_math_vulnerability(&mut source_unit);
    assert_eq!(vulnerability_locations.unwrap().len(), 2)
}
