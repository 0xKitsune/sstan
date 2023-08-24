use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use solang_parser::pt::{Loc, SourceUnit};
use crate::{
    create_test_source,
    engine::{EngineError, Outcome, Pushable},
    extractors::{
        compound::ConstructorExtractor,
        primitive::{
            ContractDefinitionExtractor, EqualityExtractor, ParameterExtractor, VariableExtractor,
        },
        Extractor,
    },
};

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

    use crate::engine::Report;

    use super::*;
    #[test]
    fn test_constructor_var_initialization() -> eyre::Result<()> {
        let file_contents_1 = r#"
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

        let source = create_test_source!(file_contents_1);
        let qa_locations = ConstructorVarInitialization::find(source)?;
        
        assert_eq!(qa_locations.len(), 1);
        let report: Report = qa_locations.into();
        let mut f = File::options().append(true).open("src/qa/test_report/mock_report.md")?;
        writeln!(&mut f, "{}", report)?;
        
        Ok(())
    }
}
