use std::{collections::HashMap, path::PathBuf};

use solang_parser::pt::{self, Expression, Loc, SourceUnit};

use crate::{
    engine::{EngineError, Outcome, Pushable},
    extractors::{
        compound::{
            ConstantStorageVariableExtractor, ContractExtractor, ImmutableStorageVariableExtractor,
        },
        primitive::{
            ContractDefinitionExtractor, EventExtractor, FunctionCallExtractor,
            PlainImportExtractor,
        },
        Extractor,
    },
    utils::{is_camel_case, is_pascal_case, is_screaming_snake_case},
};

use super::{
    ConstantImmutableNameScreamingSnakeCase, ContractNamePascalCase, EventNamePascalCase,
    ImportIdentifiers, OneContractPerFile, QAPattern, QualityAssuranceOutcome, RemoveConsole,
};
impl QAPattern for ConstantImmutableNameScreamingSnakeCase {
    fn find(
        source: &mut HashMap<PathBuf, SourceUnit>,
    ) -> Result<QualityAssuranceOutcome, EngineError> {
        let mut outcome: HashMap<PathBuf, Vec<(Loc, String)>> = Outcome::new();

        for (path_buf, source_unit) in source {
            let immutable_variables = ImmutableStorageVariableExtractor::extract(source_unit)?;
            let constant_variables = ConstantStorageVariableExtractor::extract(source_unit)?;

            for var in constant_variables {
                if let Some(name) = &var.name {
                    if !is_screaming_snake_case(&name.name) {
                        outcome.push_or_insert(path_buf.clone(), var.loc, var.to_string());
                    }
                }
            }

            for var in immutable_variables {
                if let Some(name) = &var.name {
                    if !is_screaming_snake_case(&name.name) {
                        outcome.push_or_insert(path_buf.clone(), var.loc, var.to_string());
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
    contract contract0 {

        uint256 constant CONSTANT_NAME = 1;
        uint256 constant constantName = 1;
        uint256 immutable constant_name = 1;
        uint256 immutable IMMUTABLE_NAME = 1;
        uint256 immutable immutable_name = 1;
        uint256 immutable immutableName = 1;
    }
 
    "#;

        let mut mock_source = MockSource::new().add_source(
            "constant_immutable_name_screaming_snake_case.sol",
            file_contents,
        );
        let qa_locations = ImportIdentifiers::find(&mut mock_source.source)?;

        assert_eq!(qa_locations.len(), 4);
        let report: Option<ReportSectionFragment> = qa_locations.into();
        if let Some(report) = report {
            let mut f = File::options().append(true).open("qa_report_sections.md")?;
            writeln!(&mut f, "{}", &String::from(report))?;
        }
        Ok(())
    }
}
