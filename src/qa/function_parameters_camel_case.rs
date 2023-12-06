use std::{collections::HashMap, path::PathBuf};

use solang_parser::pt::{Loc, SourceUnit};

use crate::{
    engine::{EngineError, Outcome, Pushable},
    extractors::{
        primitive::{FunctionExtractor, ParameterExtractor},
        Extractor,
    },
    utils::is_camel_case,
};

use super::{FunctionParametersCamelCase, QAPattern, QualityAssuranceOutcome};
impl QAPattern for FunctionParametersCamelCase {
    fn find(
        source: &mut HashMap<PathBuf, SourceUnit>,
    ) -> Result<QualityAssuranceOutcome, EngineError> {
        let mut outcome: HashMap<PathBuf, Vec<(Loc, String)>> = Outcome::new();
        for (path_buf, source_unit) in source {
            let mut function_definitions = FunctionExtractor::extract(source_unit)?;
            for function in function_definitions.iter_mut() {
                let parameters = ParameterExtractor::extract(function)?;
                for parameter in parameters.iter() {
                    if let Some(identifier) = &parameter.name {
                        if !is_camel_case(&identifier.name) {
                            outcome.push_or_insert(
                                path_buf.clone(),
                                identifier.loc,
                                identifier.to_string(),
                            );
                        }
                    }
                }
            }
        }

        Ok(QualityAssuranceOutcome::FunctionParametersCamelCase(
            outcome,
        ))
    }
}

#[cfg(test)]
mod tests {

    use crate::{qa::FunctionParametersCamelCase, utils::MockSource};

    use super::*;
    #[test]
    fn test_function_parameters_camel_case() -> eyre::Result<()> {
        let file_contents = r#"
    import "filename.sol";
    contract Contract0 {
      function someFunction(uint256 someParameter2) public {}
      function someFunction2(uint256 _someparameter, uint256 _someParameter) public {}
      function someFunction3(uint256 SOME_PARAMETER, uint256 some_parameter) public {}
    }

    "#;

        let mut mock_source =
            MockSource::new().add_source("function_parameters_camel_case.sol", file_contents);
        let qa_locations = FunctionParametersCamelCase::find(&mut mock_source.source)?;

        assert_eq!(qa_locations.len(), 3);

        Ok(())
    }
}
