use std::collections::{HashMap, HashSet};

use solang_parser::pt::{Expression, FunctionDefinition, Loc, SourceUnit};

use crate::analyzer::extractors::{
    compound::PublicFunctionExtractor,
    primitive::{ContractDefinitionExtractor, FunctionCallExtractor},
    Extractor,
};
//TODO: Make this a QA since there is supposedly no gas saved. 
pub fn public_function_optimization(source_unit: &mut SourceUnit) -> eyre::Result<HashSet<Loc>> {
    let mut optimization_locations: HashSet<Loc> = HashSet::new();
    let mut contracts = ContractDefinitionExtractor::extract(source_unit)?;
    for contract in contracts.iter_mut() {
        let public_functions = PublicFunctionExtractor::extract(contract)?;
        let mut function_names = extract_names(public_functions)?;
        let function_calls = FunctionCallExtractor::extract(contract)?;
        for function_call in function_calls {
            if let Expression::FunctionCall(_loc, function_identifier, _function_call_expressions) =
                function_call
            {
                if let Expression::Variable(identifier) = *function_identifier {
                    if function_names.contains_key(&identifier.name) {
                        function_names.remove(&identifier.name);
                    }
                }
            }
        }

        for (_, loc) in function_names {
            optimization_locations.insert(loc);
        }
    }

    Ok(optimization_locations)
}

fn extract_names(functions: Vec<FunctionDefinition>) -> eyre::Result<HashMap<String, Loc>> {
    let mut function_identifiers: HashMap<String, Loc> = HashMap::new();

    for function in functions {
        if let Some(name) = function.name {
            function_identifiers.insert(name.name, name.loc);
        }
    }

    Ok(function_identifiers)
}

#[test]
fn test_public_function_optimization() {
    let file_contents = r#"

    contract Contract0 {

        function shouldBeExternal1() public {}
        function shouldBeExternal2() public {}
        function shouldBePublic() public {}

        function test1() internal {
            shouldBePublic();
        }
    }
    "#;

    let mut source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    let optimization_locations = public_function_optimization(&mut source_unit);

    assert_eq!(optimization_locations.unwrap().len(), 2)
}
