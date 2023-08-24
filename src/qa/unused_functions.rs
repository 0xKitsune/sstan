use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use solang_parser::pt::{Expression, Loc, SourceUnit};

use crate::{
    create_test_source,
    engine::{EngineError, Outcome, Pushable},
    extractors::{
        compound::InternalFunctionExtractor,
        primitive::{ContractDefinitionExtractor, FunctionCallExtractor},
        Extractor,
    },
};

use super::{QAPattern, QualityAssuranceOutcome, UnusedFunctions};
impl QAPattern for UnusedFunctions {
    fn find(
        source: HashMap<PathBuf, &mut SourceUnit>,
    ) -> Result<QualityAssuranceOutcome, EngineError> {
        let mut outcome: HashMap<PathBuf, Vec<(Loc, String)>> = Outcome::new();

        for (path_buf, source_unit) in source {
            let mut contracts = ContractDefinitionExtractor::extract(source_unit)?;
            for contract in contracts.iter_mut() {
                let mut internal_function_names = HashMap::new();
                let functions = InternalFunctionExtractor::extract(contract)?;
                for function in functions.iter() {
                    if let Some(name) = function.name.clone() {
                        internal_function_names.insert(name.name, function);
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
                        let function = internal_function_names.get(name);
                        if let Some(function) = function {
                            outcome.push_or_insert(
                                path_buf.clone(),
                                function.loc,
                                function.to_string(),
                            );
                        }
                    }
                }
            }
        }

        Ok(QualityAssuranceOutcome::UnusedFunctions(outcome))
    }
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

    let source = create_test_source!(file_contents_1);
    let qa_locations_1 = UnusedFunctions::find(source).unwrap();
    assert_eq!(qa_locations_1.len(), 1);
}
