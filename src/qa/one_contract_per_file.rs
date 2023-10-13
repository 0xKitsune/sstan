use std::{collections::HashMap, path::PathBuf};

use solang_parser::pt::{self, Expression, Loc, SourceUnit};

use crate::{
    engine::{EngineError, Outcome, Pushable},
    extractors::{
        compound::ContractExtractor,
        primitive::{FunctionCallExtractor, PlainImportExtractor},
        Extractor,
    },
};

use super::{
    ImportIdentifiers, OneContractPerFile, QAPattern, QualityAssuranceOutcome, RemoveConsole,
};
impl QAPattern for RemoveConsole {
    fn find(
        source: &mut HashMap<PathBuf, SourceUnit>,
    ) -> Result<QualityAssuranceOutcome, EngineError> {
        let mut outcome: HashMap<PathBuf, Vec<(Loc, String)>> = Outcome::new();

        for (path_buf, source_unit) in source {
            let function_calls = FunctionCallExtractor::extract(source_unit)?;

            for call in function_calls {
                if let Expression::FunctionCall(
                    loc,
                    function_identifier,
                    _function_call_expressions,
                ) = call.clone()
                //TODO: update this to not use clone
                {
                    if let Expression::Variable(identifier) = *function_identifier {
                        if identifier.name == "console" {
                            outcome.push_or_insert(path_buf.clone(), loc, call.to_string());
                        }
                    }
                }
            }
        }

        Ok(QualityAssuranceOutcome::ImportIdentifiers(outcome))
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Write};

    use crate::{report::ReportSectionFragment, utils::MockSource};

    use super::*;
    #[test]
    fn test_import_identifiers() -> eyre::Result<()> {
        let file_contents = r#"
    import "filename.sol";
    contract Contract0 {
       function someFunction(){

        console.log("hello world");
        console.log("some other string");
       }
    }

    "#;

        let mut mock_source = MockSource::new().add_source("console_log.sol", file_contents);
        let qa_locations = ImportIdentifiers::find(&mut mock_source.source)?;

        assert_eq!(qa_locations.len(), 2);
      
        Ok(())
    }
}
