use std::collections::HashMap;
use std::path::PathBuf;

use solang_parser::pt::{self, CodeLocation};
use solang_parser::{self, pt::SourceUnit};

use crate::extractors::primitive::FunctionCallExtractor;
use crate::extractors::{compound::SolidityVerisonExtractor, Extractor};

use super::{OptimizationOutcome, OptimizationPattern, ShortRevertString};
use crate::engine::{EngineError, Outcome, Pushable};

impl OptimizationPattern for ShortRevertString {
    fn find(source: &mut HashMap<PathBuf, SourceUnit>) -> Result<OptimizationOutcome, EngineError> {
        let mut outcome = Outcome::new();
        for (path_buf, source_unit) in source {
            let solidity_versions = SolidityVerisonExtractor::extract(source_unit)?;
            for version in solidity_versions.into_iter().flatten() {
                if !(version.minor >= 8 && version.patch >= 4) {
                    let target_nodes = FunctionCallExtractor::extract(source_unit)?;

                    for node in target_nodes {
                        if let pt::Expression::FunctionCall(_, ident, expressions) = node.clone() {
                            match (*ident, expressions.last()) {
                                (
                                    // identifier is variable
                                    pt::Expression::Variable(identifier),
                                    // last expression is string literal
                                    Some(pt::Expression::StringLiteral(literals)),
                                ) if identifier.name.eq("require") => {
                                    if let Some(literal) = literals.get(0) {
                                        if literal.string.len() >= 32 {
                                            outcome.push_or_insert(
                                                path_buf.clone(),
                                                node.loc(),
                                                node.to_string(),
                                            );
                                        }
                                    }
                                }
                                _ => (),
                            };
                        }
                    }
                }
            }
        }
        //Return the identified optimization locations
        Ok(OptimizationOutcome::ShortRevertString(outcome))
    }
}
mod test {
    #[allow(unused)]
    use super::*;
    #[allow(unused)]
    use crate::utils::MockSource;

    #[test]
    fn test_short_revert_string() -> eyre::Result<()> {
        let file_contents = r#"
    pragma solidity 0.8.0;
    
    contract Contract0 {
        function expensiveRevertStrings() {
            require(a < b, "long revert string over 32 bytes");
        }

        function cheapRevertStrings() {
            require(a < b, "a");
        }

        function noRevertMessage() {
            require(a < b);
        }
    }
    "#;

        let mut source = MockSource::new().add_source("short_revert_string_0.sol", file_contents);
        let optimization_locations = ShortRevertString::find(&mut source.source)?;
        assert_eq!(optimization_locations.len(), 1);

        let invalid_version_content = r#"
    pragma solidity 0.8.14;
    
    contract Contract0 {
        function expensiveRevertStrings() {
            require(a < b, "long revert string over 32 bytes");
        }
    }
    "#;

        source = MockSource::new().add_source("short_revert_string_1.sol", invalid_version_content);
        let optimization_locations = ShortRevertString::find(&mut source.source)?;
        assert_eq!(optimization_locations.len(), 0);

        Ok(())
    }
}
