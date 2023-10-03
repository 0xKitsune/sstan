use std::collections::HashMap;
use std::path::PathBuf;

use solang_parser::pt::{self, CodeLocation};
use solang_parser::{self, pt::SourceUnit};

use crate::engine::{EngineError, Outcome, Pushable};
use crate::extractors::{primitive::EqualityExtractor, Extractor};

use super::{BoolEqualsBool, OptimizationOutcome, OptimizationPattern};

impl OptimizationPattern for BoolEqualsBool {
    fn find(source: &mut HashMap<PathBuf, SourceUnit>) -> Result<OptimizationOutcome, EngineError> {
        let mut outcome = Outcome::new();
        for (path_buf, source_unit) in source {
            //Extract the target nodes from the source_unit
            let equality_nodes = EqualityExtractor::extract(source_unit).unwrap();

            //For each target node that was extracted, check for the optimization patterns
            for node in equality_nodes {
                match node.clone() {
                    pt::Expression::NotEqual(_loc, box_expression, box_expression_1) => {
                        if check_for_bool_equals_bool(*box_expression, *box_expression_1) {
                            outcome.push_or_insert(path_buf.clone(), node.loc(), node.to_string())
                        }
                    }

                    pt::Expression::Equal(_loc, box_expression, box_expression_1) => {
                        if check_for_bool_equals_bool(*box_expression, *box_expression_1) {
                            outcome.push_or_insert(path_buf.clone(), node.loc(), node.to_string())
                        }
                    }

                    _ => {}
                }
            }
        }

        //Return the identified optimization locations
        Ok(OptimizationOutcome::AssignUpdateArrayValue(outcome))
    }
}

fn check_for_bool_equals_bool(
    box_expression: pt::Expression,
    box_expression_1: pt::Expression,
) -> bool {
    //create a boolean to determine if address(0) is present
    let mut bool_equals_bool: bool = false;

    //if the first expression is true or false
    if let pt::Expression::BoolLiteral(_, _) = box_expression {
        bool_equals_bool = true;
    }
    //if the second expression is true or false
    if let pt::Expression::BoolLiteral(_, _) = box_expression_1 {
        bool_equals_bool = true;
    }

    //return true or false for bool equals bool
    bool_equals_bool
}
mod test {

    use crate::utils::MockSource;

    use super::*;

    #[test]
    fn test_analyze_for_if_bool_equals_bool_optimization() -> eyre::Result<()> {
        let file_contents = r#"
    

    contract Contract0 {

        function boolEqualsBool0(bool check) public pure {
            if (check == true){
                return;
            }
        }


        function boolEqualsBool1(bool check) public pure {
            if (check == false){
                return;
            }
        }

        function boolEqualsBool2(bool check) public pure {
            if (false == check){
                return;
            }
        }

        function boolEqualsBool3(bool check) public pure {
            if (true == check){
                return;
            }
        }

        function boolEqualsBool4(bool check) public pure {
            if (check != true){
                return;
            }
        }


        function boolEqualsBool5(bool check) public pure {
            if (check != false){
                return;
            }
        }

        function boolEqualsBool6(bool check) public pure {
            if (false != check){
                return;
            }
        }

        function boolEqualsBool7(bool check) public pure {
            if (true != check){
                return;
            }
        }

    }
    "#;

        let mut source = MockSource::new().add_source("bool_equals_bool.sol", file_contents);
        let optimization_locations = BoolEqualsBool::find(&mut source.source)?;

        assert_eq!(optimization_locations.len(), 8);

        Ok(())
    }
}
