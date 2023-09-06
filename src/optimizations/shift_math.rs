use std::collections::HashMap;
use std::path::PathBuf;

use solang_parser::pt::{CodeLocation, Expression};
use solang_parser::{self, pt::SourceUnit};

use crate::extractors::{primitive::UrnaryOpteratorExtractor, Extractor};

use super::{OptimizationOutcome, OptimizationPattern, ShiftMath};
use crate::engine::{EngineError, Outcome, Pushable};

impl OptimizationPattern for ShiftMath {
    fn find(source: &mut HashMap<PathBuf, SourceUnit>) -> Result<OptimizationOutcome, EngineError> {
        let mut outcome = Outcome::new();
        for (path_buf, source_unit) in source {
            let urnary_operator_nodes = UrnaryOpteratorExtractor::extract(source_unit)?;

            for node in urnary_operator_nodes {
                //We can use expect because both Target::Multiply and Target::Divide are expressions
                match node.clone() {
                    Expression::Multiply(_loc, box_expression, box_expression_1) => {
                        if check_if_inputs_are_power_of_two(*box_expression, *box_expression_1) {
                            outcome.push_or_insert(path_buf.clone(), node.loc(), node.to_string())
                        }
                    }

                    Expression::Divide(_loc, box_expression, box_expression_1) => {
                        if check_if_inputs_are_power_of_two(*box_expression, *box_expression_1) {
                            outcome.push_or_insert(path_buf.clone(), node.loc(), node.to_string())
                        }
                    }

                    _ => {}
                }
            }
        }
        Ok(OptimizationOutcome::ShiftMath(outcome))
    }
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
            .parse::<u128>()
            .expect("Could not parse NumberLiteral value from string to u128");

        if (value != 0) && ((value & (value - 1)) == 0) {
            is_even = true;
        }
    }

    //if the first expression is a number literal that is a power of 2
    if let Expression::NumberLiteral(_, val_string, _, _) = box_expression_1 {
        let value = val_string
            .parse::<u128>()
            .expect("Could not parse NumberLiteral value from string to u128");

        if (value != 0) && ((value & (value - 1)) == 0) {
            is_even = true;
        }
    }

    is_even
}
mod test {
    use crate::{
        optimizations::{OptimizationPattern, ShiftMath},
        utils::MockSource,
    };

    #[test]
    fn test_shift_math_optimization() -> eyre::Result<()> {
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

        let mut source = MockSource::new().add_source("shift_math.sol", file_contents);
        let optimization_locations = ShiftMath::find(&mut source.source)?;

        assert_eq!(optimization_locations.len(), 3);

        Ok(())
    }
}
