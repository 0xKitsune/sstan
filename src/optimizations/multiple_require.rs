use std::collections::HashMap;
use std::path::PathBuf;

use super::{MultipleRequire, OptimizationOutcome, OptimizationPattern};
use solang_parser::pt::{CodeLocation, Expression};
use solang_parser::{self, pt::SourceUnit};

use crate::engine::{EngineError, Outcome, Pushable};
use crate::extractors::{primitive::FunctionCallExtractor, Extractor};

//Use multiple require statements instead of one single require statement with multiple conditions
impl OptimizationPattern for MultipleRequire {
    fn find(source: &mut HashMap<PathBuf, SourceUnit>) -> Result<OptimizationOutcome, EngineError> {
        let mut outcome = Outcome::new();
        for (path_buf, source_unit) in source {
            let target_nodes = FunctionCallExtractor::extract(source_unit)?;

            for node in target_nodes {
                if let Expression::FunctionCall(
                    _loc,
                    function_identifier,
                    function_call_expressions,
                ) = node.clone()
                {
                    //if the function call identifier is a variable
                    if let Expression::Variable(identifier) = *function_identifier {
                        //if the identifier name is "require"
                        if identifier.name == *"require" {
                            //for each expression in the function call expressions
                            for func_call_expression in function_call_expressions {
                                //if there is an and expression (ie. &&)
                                if let Expression::And(_, _, _) = func_call_expression {
                                    //add the location to the list of optimization locations
                                    outcome.push_or_insert(
                                        path_buf.clone(),
                                        node.loc(),
                                        node.to_string(),
                                    );
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(OptimizationOutcome::MultipleRequire(outcome))
    }
}
mod test {
    #[allow(unused)]
    use super::*;
    #[allow(unused)]
    use crate::utils::MockSource;

    #[test]
    fn test_multiple_require_optimization() -> eyre::Result<()> {
        let file_contents = r#"
    contract Contract0 {
        function addressInternalBalance() public returns (uint256) {

            uint256 a = 100;
            uint256 b = 100;
            uint256 c = 100;

            require(true, "some message");

            require(true && a==b, "some message");
            require(true && a==b && b==c, "thing");

            return address(this).balance;


        }
    }
    "#;

        let mut source = MockSource::new().add_source("multiple_require.sol", file_contents);
        let optimization_locations = MultipleRequire::find(&mut source.source)?;

        assert_eq!(optimization_locations.len(), 2);

        Ok(())
    }
}
