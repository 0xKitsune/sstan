use std::{collections::HashMap, path::PathBuf};

use solang_parser::pt::{Loc, SourceUnit};

use crate::{
    engine::{EngineError, Outcome, Pushable},
    extractors::{primitive::FunctionExtractor, Extractor},
    utils::is_camel_case,
};

use super::{FunctionNameCamelCase, QAPattern, QualityAssuranceOutcome};
impl QAPattern for FunctionNameCamelCase {
    fn find(
        source: &mut HashMap<PathBuf, SourceUnit>,
    ) -> Result<QualityAssuranceOutcome, EngineError> {
        let mut outcome: HashMap<PathBuf, Vec<(Loc, String)>> = Outcome::new();

        for (path_buf, source_unit) in source {
            let function_definition = FunctionExtractor::extract(source_unit)?;
            for function in function_definition {
                if let Some(name) = &function.name {
                    if !is_camel_case(&name.name) {
                        outcome.push_or_insert(
                            path_buf.clone(),
                            function.loc,
                            function.to_string(),
                        );
                    }
                }
            }
        }

        Ok(QualityAssuranceOutcome::FunctionNameCamelCase(outcome))
    }
}

#[cfg(test)]
mod tests {

    use crate::utils::MockSource;

    use super::*;
    #[test]
    fn test_function_name_camel_case() -> eyre::Result<()> {
        let file_contents = r#"
    import "filename.sol";
    contract Contract0 {
      function some_function(){}
      function AnotherFunction(){}
    function functionThree(){}

    }

    "#;

        let mut mock_source =
            MockSource::new().add_source("function_name_camel_case.sol", file_contents);
        let qa_locations = FunctionNameCamelCase::find(&mut mock_source.source)?;

        assert_eq!(qa_locations.len(), 2);

        Ok(())
    }
}
