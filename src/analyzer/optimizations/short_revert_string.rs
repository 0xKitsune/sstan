use std::collections::HashSet;

use solang_parser::pt::{self, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::analyzer::extractors::primitive::FunctionCallExtractor;
use crate::analyzer::extractors::{compound::SolidityVerisonExtractor, Extractor};

pub fn short_revert_string_optimization(
    source_unit: &mut SourceUnit,
) -> eyre::Result<HashSet<Loc>> {
    //Create a new hashset that stores the location of each optimization target identified
    let mut optimization_locations = HashSet::<Loc>::new();

    let solidity_versions = SolidityVerisonExtractor::extract(source_unit)?;
    for version in solidity_versions {
        if let Some(version) = version {
            if !(version.minor >= 8 && version.patch >= 4) {
                let target_nodes = FunctionCallExtractor::extract(source_unit)?;

                for node in target_nodes {
                    if let pt::Expression::FunctionCall(_, ident, expressions) = node {
                        match (*ident, expressions.last()) {
                            (
                                // identifier is variable
                                pt::Expression::Variable(identifier),
                                // last expression is string literal
                                Some(pt::Expression::StringLiteral(literals)),
                            ) if identifier.name.eq("require") => {
                                if let Some(literal) = literals.get(0) {
                                    if literal.string.len() >= 32 {
                                        optimization_locations.insert(literal.loc);
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
    Ok(optimization_locations)
}

#[test]
fn test_short_revert_string() {
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

    let mut source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let optimization_locations = short_revert_string_optimization(&mut source_unit);
    assert_eq!(optimization_locations.unwrap().len(), 1);

    let invalid_version_content = r#"
    pragma solidity 0.8.14;
    
    contract Contract0 {
        function expensiveRevertStrings() {
            require(a < b, "long revert string over 32 bytes");
        }
    }
    "#;

    let mut source_unit = solang_parser::parse(invalid_version_content, 0).unwrap().0;

    let optimization_locations = short_revert_string_optimization(&mut source_unit);
    assert_eq!(optimization_locations.unwrap().len(), 0);
}
