use std::collections::HashSet;

use regex::Regex;
use solang_parser::pt::{Loc, SourceUnit};

use crate::{
    analyzer::extractors::{
        compound::{ConstantStorageVariableExtractor, ImmutableStorageVariableExtractor},
        Extractor,
    },
    report::report_sections::optimizations::immutable_variable,
};

pub fn constant_immutable_namespace(source_unit: &mut SourceUnit) -> eyre::Result<HashSet<Loc>> {
    let re: Regex = Regex::new(r"\b[A-Z][A-Z0-9_]*\b")?;
    let mut qa_locations = HashSet::new();
    let constant_variables = ConstantStorageVariableExtractor::extract(source_unit)?;
    for constant in constant_variables {
        if let Some(name) = constant.name {
            if !re.is_match(name.name.as_str()) {
                qa_locations.insert(constant.loc);
            }
        }
    }

    let immutable_variables = ImmutableStorageVariableExtractor::extract(source_unit)?;
    for immutable in immutable_variables {
        if let Some(name) = immutable.name {
            if !re.is_match(name.name.as_str()) {
                qa_locations.insert(immutable.loc);
            }
        }
    }

    Ok(qa_locations)
}

#[test]
fn test_constant_immutable_namespace() {
    let file_contents_1 = r#"
    contract Contract {

        address immutable IS_FINE;
        address constant is_bad = address(1);
        address immutable Is_Bad;
        address constant ALSO_IS_FINE = address(1);
        constructor(address _isFine, address _isBad) {
            IS_FINE = _isFine;
            Is_Bad = _isBad;
        }
        
    }
    "#;

    let mut source_unit_1 = solang_parser::parse(file_contents_1, 0).unwrap().0;
    let qa_locations_1 = constant_immutable_namespace(&mut source_unit_1);
    assert_eq!(qa_locations_1.unwrap().len(), 2);
}
