use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use crate::{
    engine::{EngineError, Outcome, Pushable},
    extractors::{
        compound::ConstructorExtractor,
        primitive::{
            ContractDefinitionExtractor, EqualityExtractor, ParameterExtractor, VariableExtractor,
        },
        Extractor,
    },
};
use solang_parser::pt::{Loc, SourceUnit};

use super::{ConstructorVarInitialization, QAPattern, QualityAssuranceOutcome};

impl QAPattern for ConstructorVarInitialization {
    fn find(
        source: HashMap<PathBuf, &mut SourceUnit>,
    ) -> Result<QualityAssuranceOutcome, EngineError> {
        let mut outcome: HashMap<PathBuf, Vec<(Loc, String)>> = Outcome::new();

        for (path_buf, source_unit) in source {
            let mut contracts = ContractDefinitionExtractor::extract(source_unit)?;

            for contract in contracts.iter_mut() {
                let mut constructors = ConstructorExtractor::extract(contract)?;
                for constructor in constructors.iter_mut() {
                    let parameter_names = ParameterExtractor::extract_names(
                        ParameterExtractor::extract(constructor)?,
                    );
                    let mut not_equal_checks = EqualityExtractor::extract_not_equal(
                        EqualityExtractor::extract(constructor)?,
                    );
                    let mut not_equal_check_variable_names: HashSet<String> = HashSet::new();
                    for not_equal_check in not_equal_checks.iter_mut() {
                        not_equal_check_variable_names.extend(VariableExtractor::extract_names(
                            VariableExtractor::extract(not_equal_check)?,
                        ));
                    }
                    for parameter in parameter_names {
                        if !not_equal_check_variable_names.contains(&parameter) {
                            outcome.push_or_insert(
                                path_buf.clone(),
                                constructor.loc,
                                constructor.to_string(),
                            );
                        }
                    }
                }
            }
        }

        Ok(QualityAssuranceOutcome::ConstructorVarInitialization(
            outcome,
        ))
    }
}
#[cfg(test)]
mod tests {
    use std::{fs::File, io::Write};

    use crate::{report::ReportSectionFragment, utils::MockSource};

    use super::*;
    #[test]
    fn test_constructor_var_initialization() -> eyre::Result<()> {
        let file_contents = r#"
    contract Contract0 {
        address public owner;
        
        constructor(address _owner) {
            owner = _owner;
        }
    }
    contract Contract0 {
        address public owner;
        
        constructor(address _owner) {
            require(_owner != address(0), "Owner cannot be zero address");
            owner = _owner;
        }
    }
    "#;

        let mut mock_source = MockSource::new().add_source(file_contents);
        let source = std::mem::take(&mut mock_source.source);
        let qa_locations = ConstructorVarInitialization::find(source)?;

        assert_eq!(qa_locations.len(), 1);
        let report: Option<ReportSectionFragment> = qa_locations.into();
        if let Some(report) = report {
            let mut f = File::options()
                .append(true)
                .open("src/report/mock_report.md")?;
            writeln!(&mut f, "{}", &String::from(report))?;
        }
        Ok(())
    }
}
