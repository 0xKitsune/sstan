use std::{collections::HashMap, path::PathBuf};

use solang_parser::{
    helpers::CodeLocation,
    pt::{Expression, Loc, SourceUnit, Type},
};

use crate::{
    engine::{EngineError, Outcome, Pushable},
    extractors::{primitive::VariableDefinitionExtractor, Extractor},
};

use super::{QAPattern, QualityAssuranceOutcome, VariableInitializedWithDefault};
impl QAPattern for VariableInitializedWithDefault {
    fn find(
        source: &mut HashMap<PathBuf, SourceUnit>,
    ) -> Result<QualityAssuranceOutcome, EngineError> {
        let mut outcome: HashMap<PathBuf, Vec<(Loc, String)>> = Outcome::new();

        for (path_buf, source_unit) in source {
            let variables = VariableDefinitionExtractor::extract(source_unit)?;
            for variable in variables {
                if let Expression::Type(_, ty) = &variable.ty {
                    match ty {
                        Type::Uint(_) | Type::Int(_) => {
                            if variable.initializer.is_some() {
                                let initializer = variable.clone().initializer.unwrap();
                                if let Expression::NumberLiteral(_, val, _, _) = &initializer {
                                    if val == "0" {
                                        outcome.push_or_insert(
                                            path_buf.clone(),
                                            variable.clone().loc(),
                                            variable.to_string(),
                                        );
                                    }
                                }

                                if let Expression::HexNumberLiteral(_, val, _) = &initializer {
                                    if val == "0x0" {
                                        outcome.push_or_insert(
                                            path_buf.clone(),
                                            variable.loc(),
                                            variable.to_string(),
                                        );
                                    }
                                }
                            }
                        }
                        Type::Bool => {
                            if variable.initializer.is_some() {
                                let initializer = variable.clone().initializer.unwrap();
                                if let Expression::BoolLiteral(_, val) = &initializer {
                                    if !(*val) {
                                        outcome.push_or_insert(
                                            path_buf.clone(),
                                            variable.clone().loc(),
                                            variable.to_string(),
                                        );
                                    }
                                }
                            }
                        }

                        Type::Address => {
                            if variable.initializer.is_some() {
                                let initializer = variable.clone().initializer.unwrap();
                                if let Expression::FunctionCall(_, _, attrs) = initializer.clone() {
                                    for attr in attrs {
                                        match attr {
                                            Expression::NumberLiteral(_, val, _, _) => {
                                                if val == "0" {
                                                    outcome.push_or_insert(
                                                        path_buf.clone(),
                                                        variable.clone().loc(),
                                                        variable.to_string(),
                                                    );
                                                }
                                            }
                                            Expression::HexNumberLiteral(_, val, _) => {
                                                match val.parse::<u128>() {
                                                    Ok(val) => {
                                                        if val == 0 {
                                                            outcome.push_or_insert(
                                                                path_buf.clone(),
                                                                variable.clone().loc(),
                                                                variable.to_string(),
                                                            );
                                                        }
                                                    }
                                                    Err(_) => continue,
                                                }
                                            }
                                            _ => continue,
                                        }
                                    }
                                }
                            }
                        }

                        _ => continue,
                    }
                }
            }
        }

        Ok(QualityAssuranceOutcome::VariableInitializedWithDefault(
            outcome,
        ))
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused)]
    use super::*;
    #[allow(unused)]
    use crate::utils::MockSource;
    #[test]
    fn test_default_initialization() -> eyre::Result<()> {
        let file_contents = r#"
    import "filename.sol";
    contract contract0 {

        bool private isStupid = false; 
        bool private isFine = true;
        address owner = address(0);
        uint256 private constant CONSTANT_NAME = 0;
        uint256 private constant constantName = 5;
        int128 constant_name = 0;
    }
 
    "#;

        let mut mock_source = MockSource::new().add_source("default_value.sol", file_contents);
        let qa_locations = VariableInitializedWithDefault::find(&mut mock_source.source)?;

        assert_eq!(qa_locations.len(), 4);

        Ok(())
    }
}
