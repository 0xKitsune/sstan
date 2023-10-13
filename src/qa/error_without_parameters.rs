use std::{collections::HashMap, path::PathBuf};

use solang_parser::pt::{Loc, SourceUnit};


use crate::{
    engine::{EngineError, Outcome, Pushable},
    extractors::{Extractor, primitive::ErrorExtractor},
};

use super::{ErrorWithoutParams, QAPattern, QualityAssuranceOutcome};
impl QAPattern for ErrorWithoutParams {
    fn find(
        source: &mut HashMap<PathBuf, SourceUnit>,
    ) -> Result<QualityAssuranceOutcome, EngineError> {
        let mut outcome: HashMap<PathBuf, Vec<(Loc, String)>> = Outcome::new();

        for (path_buf, source_unit) in source {
            let errors = ErrorExtractor::extract(source_unit)?;
            for error in errors {
                if error.fields.is_empty() {
                    outcome.push_or_insert(
                        path_buf.clone(),
                        error.loc,
                        error.to_string(),
                    );
                }
            }
        }

        Ok(QualityAssuranceOutcome::ErrorWithoutParams(outcome))
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused)]
    use super::*;
    #[allow(unused)]
    use crate::utils::MockSource;
    #[test]
    fn test_error_without_params() -> eyre::Result<()> {
        let file_contents = r#"
    import "filename.sol";
    error HasNoParameters();
    error HasParameters(uint256 a, uint256 b);
    
    contract contract0 {
        function test() public {
            revert HasNoParameters();
        }

        function test2() public {
            revert ImportedErrorWithoutParameters(); //imported shouldn't throw since itll be a duplicate
        }

        function test3() public {
            revert HasParameters(1, 2);
        }
        
    }
 
    "#;

        let mut mock_source = MockSource::new().add_source(
            "error_without_parameters.sol",
            file_contents,
        );
        let qa_locations = ErrorWithoutParams::find(&mut mock_source.source)?;

        assert_eq!(qa_locations.len(), 1);

        Ok(())
    }
}
