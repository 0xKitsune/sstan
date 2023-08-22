use std::{
    collections::{HashMap, HashSet},
    fs,
    path::PathBuf,
};

use solang_parser::pt::{Loc, SourceUnit};

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

#[test]
fn test_import_identifiers() -> eyre::Result<()> {
    let file_contents = fs::read_to_string(PathBuf::from("src/qa/temp.sol")).expect("couldnt read");
    let mut source_unit = solang_parser::parse(&file_contents, 0).unwrap().0;

    let mut source: HashMap<PathBuf, &mut SourceUnit> = HashMap::new();
    source.insert(PathBuf::from("src/qa/temp.sol"), &mut source_unit);

    let qa_locations = ConstructorVarInitialization::find(source)?;

    if let QualityAssuranceOutcome::ConstructorVarInitialization(outcome) = qa_locations {
        assert_eq!(
            outcome
                .iter()
                .map(|(_, outcomes)| outcomes.len())
                .sum::<usize>(),
            1
        );
        assert_eq!(outcome.get(&PathBuf::from("file_0")).unwrap().len(), 1);
        assert!(outcome.get(&PathBuf::from("file_1")).is_none());
    }
    Ok(())
}
