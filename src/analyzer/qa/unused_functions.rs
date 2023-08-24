use std::collections::{HashMap, HashSet};

use solang_parser::pt::{Expression, Loc, SourceUnit};

use crate::analyzer::extractors::{
    compound::InternalFunctionExtractor,
    primitive::{ContractDefinitionExtractor, FunctionCallExtractor},
    Extractor,
};

pub fn unused_functions(source_unit: &mut SourceUnit) -> eyre::Result<HashSet<Loc>> {
    let mut qa_locations = HashSet::new();

    let mut contracts = ContractDefinitionExtractor::extract(source_unit)?;
    for contract in contracts.iter_mut() {
        let mut internal_function_names = HashMap::new();
        let functions = InternalFunctionExtractor::extract(contract)?;
        for function in functions.iter() {
            if let Some(name) = function.name.clone() {
                internal_function_names.insert(name.name, function.loc.clone());
            }
        }
        let mut function_call_names = HashSet::new();
        let function_calls = FunctionCallExtractor::extract(contract)?;
        for function_call in function_calls.iter() {
            if let Expression::FunctionCall(_, var, _) = function_call {
                if let Expression::Variable(ident) = var.as_ref() {
                    function_call_names.insert(ident.name.clone());
                }
            }
        }

        for name in internal_function_names.keys() {
            if !function_call_names.contains(name) {
                let loc = internal_function_names.get(name);
                if let Some(loc) = loc {
                    qa_locations.insert(*loc);
                }
            }
        }
    }

    Ok(qa_locations)
}

#[test]
fn test_unused_functions() {
    let file_contents_1 = r#"
    contract Contract0 {
        
        function isUnused() internal {

        }

        function isUsed() internal {
            
        }

        function useIsUsed() public {
            isUsed();
        }
        
    }
    "#;

    let mut source_unit_1 = solang_parser::parse(file_contents_1, 0).unwrap().0;
    let qa_locations_1 = unused_functions(&mut source_unit_1);
    assert_eq!(qa_locations_1.unwrap().len(), 1);
}
