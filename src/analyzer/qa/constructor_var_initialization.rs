use std::{collections::HashSet, hash::Hash};

use solang_parser::pt::{Loc, SourceUnit};

use crate::analyzer::{
    extractors::{
        compound::ConstructorExtractor,
        primitive::{
            ContractDefinitionExtractor, EqualityExtractor, ParameterExtractor, VariableExtractor,
        },
        Extractor,
    },
    qa,
};

pub fn constructor_var_initialization(source_unit: &mut SourceUnit) -> eyre::Result<HashSet<Loc>> {
    let mut qa_locations: HashSet<Loc> = HashSet::new();

    let mut contracts = ContractDefinitionExtractor::extract(source_unit)?;

    for contract in contracts.iter_mut() {
        let mut constructors = ConstructorExtractor::extract(contract)?;
        for constructor in constructors.iter_mut() {
            let parameter_names =
                ParameterExtractor::extract_names(ParameterExtractor::extract(constructor)?);
            let mut not_equal_checks =
                EqualityExtractor::extract_not_equal(EqualityExtractor::extract(constructor)?);
            let mut not_equal_check_variable_names: HashSet<String> = HashSet::new();
            for not_equal_check in not_equal_checks.iter_mut() {
                not_equal_check_variable_names.extend(VariableExtractor::extract_names(
                    VariableExtractor::extract(not_equal_check)?,
                ));
            }
            for parameter in parameter_names {
                if !not_equal_check_variable_names.contains(&parameter) {
                    qa_locations.insert(constructor.loc);
                }
            }
        }
    }

    Ok(qa_locations)
}

#[test]
fn test_import_identifiers() {
    let file_contents_1 = r#"
    contract Contract0 {
        address public owner;
        
        constructor(address _owner) {
            owner = _owner;
        }
    }
    "#;

    let file_contents_2 = r#"
    contract Contract0 {
        address public owner;
        
        constructor(address _owner) {
            require(_owner != address(0), "Owner cannot be zero address");
            owner = _owner;
        }
    }
    "#;

    let mut source_unit_1 = solang_parser::parse(file_contents_1, 0).unwrap().0;
    let mut source_unit_2 = solang_parser::parse(file_contents_2, 0).unwrap().0;
    let qa_locations_1 = constructor_var_initialization(&mut source_unit_1);
    let qa_locations_2 = constructor_var_initialization(&mut source_unit_2);
    assert_eq!(qa_locations_1.unwrap().len(), 1);
    assert_eq!(qa_locations_2.unwrap().len(), 0);
}
