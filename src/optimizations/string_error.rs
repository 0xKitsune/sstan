use super::{OptimizationOutcome, OptimizationPattern, StringError};
use crate::engine::{EngineError, Outcome, Pushable};

use std::collections::HashMap;
use std::path::PathBuf;

use solang_parser::pt::{self, CodeLocation};
use solang_parser::{self, pt::SourceUnit};

use crate::extractors::primitive::FunctionCallExtractor;
use crate::extractors::{compound::SolidityVerisonExtractor, Extractor};

impl OptimizationPattern for StringError {
    fn find(source: &mut HashMap<PathBuf, SourceUnit>) -> Result<OptimizationOutcome, EngineError> {
        let mut outcome = Outcome::new();
        for (path_buf, source_unit) in source {
            let solidity_versions = SolidityVerisonExtractor::extract(source_unit)?;
            for version in solidity_versions.into_iter().flatten() {
                if version.minor >= 8 && version.patch >= 4 {
                    //Extract the target nodes from the source_unit
                    let target_nodes = FunctionCallExtractor::extract(source_unit)?;

                    for node in target_nodes {
                        if let pt::Expression::FunctionCall(
                            _,
                            function_identifier,
                            func_call_expressions,
                        ) = node.clone()
                        {
                            //if the function call identifier is a variable
                            if let pt::Expression::Variable(identifier) = *function_identifier {
                                //if the identifier name is "require"
                                if identifier.name == *"require" {
                                    //If the require statement contains strings
                                    if let Some(pt::Expression::StringLiteral(
                                        _vec_string_literal,
                                    )) = func_call_expressions.last()
                                    {
                                        outcome.push_or_insert(
                                            path_buf.clone(),
                                            node.loc(),
                                            node.to_string(),
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        //Return the identified optimization locations
        Ok(OptimizationOutcome::StringError(outcome))
    }
}

mod test {
    use crate::{
        optimizations::{OptimizationPattern, StringError},
        utils::MockSource,
    };

    #[test]
    fn test_string_error_optimization() -> eyre::Result<()> {
        //test when base solidiy version is > than 0.8.4
        let file_contents = r#"
                 pragma solidity >=0.8.13;
             
                 contract Contract0 {
                     function addressInternalBalance() public returns (uint256) {
             
                         require(true, "some message");
             
                         require(true && a==b, "some message");
                         require(true && a==b && b==c, "thing");
             
                         return address(this).balance;
                     }
                 }
                 "#;

        let mut source = MockSource::new().add_source("string_error.solx", file_contents);
        let optimization_locations = StringError::find(&mut source.source);

        assert_eq!(optimization_locations.unwrap().len(), 3);

        //test when base solidiy version is < than 0.8.4
        let file_contents_1 = r#"
                 pragma solidity <= 0.8.3;
             
                 contract Contract0 {
                     function addressInternalBalance() public returns (uint256) {
             
                         require(true, "some message");
             
                         require(true && a==b, "some message");
                         require(true && a==b && b==c, "thing");
             
                         return address(this).balance;
                     }
                 }
                 "#;

        source = MockSource::new().add_source("string_error_0.sol", file_contents_1);

        let optimization_locations_1 = StringError::find(&mut source.source)?;

        assert_eq!(optimization_locations_1.len(), 0);

        Ok(())
    }
}
