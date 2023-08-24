use std::collections::HashSet;

use regex::Regex;
use solang_parser::pt::{Loc, SourceUnit};

use crate::analyzer::extractors::{compound::MutableStorageVariableExtractor, Extractor};

pub fn variable_namespace(source_unit: &mut SourceUnit) -> eyre::Result<HashSet<Loc>> {
    let re: Regex = Regex::new(r"\b[A-Z][A-Z0-9_]*\b")?;

    let mut qa_locations = HashSet::new();
    let variables = MutableStorageVariableExtractor::extract(source_unit)?;
    for variable in variables {
        if let Some(name) = variable.name {
            if re.is_match(name.name.as_str()) {
                qa_locations.insert(variable.loc);
            }
        }
    }

    Ok(qa_locations)
}

#[test]
fn test_storage_variable_namespace() {
    let file_contents_1 = r#"
    contract Contract {

        address IS_NOT_FINE;
        address isFine;
        address alsoIsFine;
        address ALSO_IS_BAD;
        constructor() {

        }
        
    }
    "#;

    let mut source_unit_1 = solang_parser::parse(file_contents_1, 0).unwrap().0;
    let qa_locations_1 = variable_namespace(&mut source_unit_1);
    assert_eq!(qa_locations_1.unwrap().len(), 2);
}
