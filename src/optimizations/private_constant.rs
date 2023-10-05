use std::{collections::HashMap, path::PathBuf};

use solang_parser::pt::{self, CodeLocation, SourceUnit};

use super::{OptimizationOutcome, OptimizationPattern, PrivateConstant};
use crate::engine::{EngineError, Outcome, Pushable};
use crate::extractors::{compound::ConstantStorageVariableExtractor, Extractor};

impl OptimizationPattern for PrivateConstant {
    fn find(source: &mut HashMap<PathBuf, SourceUnit>) -> Result<OptimizationOutcome, EngineError> {
        let mut outcome = Outcome::new();
        for (path_buf, source_unit) in source {
            let constant_target_nodes =
                ConstantStorageVariableExtractor::extract(source_unit).unwrap();
            for node in constant_target_nodes {
                let mut has_visibility_attribute = false;

                for attr in &node.attrs {
                    match attr {
                        pt::VariableAttribute::Visibility(pt::Visibility::External(_))
                        | pt::VariableAttribute::Visibility(pt::Visibility::Public(_))
                        | pt::VariableAttribute::Visibility(pt::Visibility::Internal(_)) => {
                            outcome.push_or_insert(path_buf.clone(), node.loc(), node.to_string());
                            has_visibility_attribute = true;
                        }
                        pt::VariableAttribute::Visibility(pt::Visibility::Private(_)) => {
                            has_visibility_attribute = true;
                        }
                        _ => {}
                    }
                }
                if !has_visibility_attribute {
                    outcome.push_or_insert(path_buf.clone(), node.loc(), node.to_string())
                }
            }
        }
        Ok(OptimizationOutcome::PrivateConstant(outcome))
    }
}
mod test {
    #[allow(unused)]
    use crate::utils::MockSource;
    #[allow(unused)]
    use super::*;

    #[test]
    fn test_private_constant_optimization() -> eyre::Result<()> {
        let file_contents = r#"
    
contract Contract0 {

    uint256 constant public x = 100;
    uint256 constant private y = 100;
    uint256 constant z = 100;


    function addPublicConstant(uint256 a) external pure returns (uint256) {
        return a + x;
    }


    function addPrivateConstant(uint256 a) external pure returns (uint256) {
        return a +x;
    }
}

    "#;
        let mut source = MockSource::new().add_source("private_constant.sol", file_contents);
        let optimization_locations = PrivateConstant::find(&mut source.source)?;
        assert_eq!(optimization_locations.len(), 2);

        Ok(())
    }
}
