pub fn report_section_content() -> String {
    String::from(
        r##"
## Mark functions external when possible. 
"##,
    )
}

use std::{collections::HashMap, path::PathBuf};

use solang_parser::{
    helpers::CodeLocation,
    pt::{Expression, FunctionDefinition, SourceUnit},
};

use super::{PublicFunctions, QAPattern, QualityAssuranceOutcome};
use crate::engine::Pushable;
use crate::engine::{EngineError, Outcome};
use crate::extractors::{
    compound::PublicFunctionExtractor,
    primitive::{ContractDefinitionExtractor, FunctionCallExtractor},
    Extractor,
};

impl QAPattern for PublicFunctions {
    fn find(
        source: &mut HashMap<PathBuf, SourceUnit>,
    ) -> Result<QualityAssuranceOutcome, EngineError> {
        let mut outcome = Outcome::new();

        for (path_buf, source_unit) in source {
            let mut contracts = ContractDefinitionExtractor::extract(source_unit)?;
            for contract in contracts.iter_mut() {
                let public_functions = PublicFunctionExtractor::extract(contract)?;
                let mut function_names = extract_names(public_functions)?;
                let function_calls = FunctionCallExtractor::extract(contract)?;
                for function_call in function_calls {
                    if let Expression::FunctionCall(
                        _loc,
                        function_identifier,
                        _function_call_expressions,
                    ) = function_call
                    {
                        if let Expression::Variable(identifier) = *function_identifier {
                            if function_names.contains_key(&identifier.name) {
                                function_names.remove(&identifier.name);
                            }
                        }
                    }
                }

                for (_, function) in function_names {
                    outcome.push_or_insert(path_buf.clone(), function.loc(), function.to_string());
                }
            }
        }

        Ok(QualityAssuranceOutcome::PublicFunctions(outcome))
    }
}

fn extract_names(
    functions: Vec<FunctionDefinition>,
) -> Result<HashMap<String, FunctionDefinition>, EngineError> {
    let mut function_identifiers: HashMap<String, FunctionDefinition> = HashMap::new();

    for function in functions {
        if let Some(name) = &function.name {
            function_identifiers.insert(name.name.clone(), function);
        }
    }

    Ok(function_identifiers)
}
mod test {
    #[allow(unused)]
    use super::*;
    #[allow(unused)]
    use crate::utils::MockSource;

    #[test]
    fn test_public_function_optimization() -> eyre::Result<()> {
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
        let mut mock_source = MockSource::new().add_source("public_functions.sol", file_contents);
        let qa_locations = PublicFunctions::find(&mut mock_source.source)?;
        assert_eq!(qa_locations.len(), 2);
        Ok(())
    }
}
