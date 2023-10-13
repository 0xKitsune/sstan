use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use solang_parser::pt::{Loc, SourceUnit};

use crate::{
    engine::{EngineError, Outcome, Pushable},
    extractors::{
        compound::ContractExtractor,
        primitive::{FunctionExtractor, VariableExtractor},
        Extractor,
    },
};

use super::{QAPattern, QualityAssuranceOutcome, UnusedReturns};

impl QAPattern for UnusedReturns {
    fn find(
        source: &mut HashMap<PathBuf, SourceUnit>,
    ) -> Result<QualityAssuranceOutcome, EngineError> {
        let mut outcome: HashMap<PathBuf, Vec<(Loc, String)>> = Outcome::new();

        for (path_buf, source_unit) in source {
            //Extract all contracts from the source unit.
            let mut contracts = ContractExtractor::extract(source_unit)?;
            //For each contract, extract all functions.
            for contract in contracts.iter_mut() {
                let mut functions = FunctionExtractor::extract(contract)?;
                for function in functions.iter_mut() {
                    let mut return_names = HashSet::new();
                    //Extract all the return variable names into a HashSet.
                    let returns = function.returns.clone();
                    for return_type in returns.iter() {
                        if let Some(parameter) = &return_type.1 {
                            if let Some(name) = &parameter.name {
                                return_names.insert(name.to_string());
                            }
                        }
                    }

                    //Extract all variable names in the function body, and make sure each of the return names exists.
                    if let Some(body) = &mut function.body {
                        let body_variables =
                            VariableExtractor::extract_names(VariableExtractor::extract(body)?);
                        for return_name in return_names.iter() {
                            if !body_variables.contains(return_name) {
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
        }

        Ok(QualityAssuranceOutcome::UnusedReturns(outcome))
    }
}
#[cfg(test)]
mod test {

    use crate::utils::MockSource;

    use super::*;
    #[test]
    fn test_unused_returns() -> eyre::Result<()> {
        let file_contents_1 = r#"
    contract Contract0 {
        address public owner;
        
        function foo() public returns (uint256 x) {
            uint256 y = 0;
            return y;
        }
    }
  
    contract Contract0 {
        address public owner;
        
        function foo() public returns (uint256 x) {
            x = 0;
        }
    }
    "#;

        let mut mock_source = MockSource::new().add_source("unused_returns.sol", file_contents_1);
        let qa_locations = UnusedReturns::find(&mut mock_source.source).unwrap();

        assert_eq!(qa_locations.len(), 1);

        Ok(())
    }
}
