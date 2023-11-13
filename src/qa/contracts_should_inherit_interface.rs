use std::{collections::HashMap, path::PathBuf};

use solang_parser::pt::{self, Loc, SourceUnit};

use crate::{
    engine::{EngineError, Outcome, Pushable},
    extractors::{
        compound::ExternalFunctionExtractor, primitive::ContractDefinitionExtractor, Extractor,
    },
};

use super::{ContractsShouldInheritInterface, QAPattern, QualityAssuranceOutcome};
impl QAPattern for ContractsShouldInheritInterface {
    fn find(
        source: &mut HashMap<PathBuf, SourceUnit>,
    ) -> Result<QualityAssuranceOutcome, EngineError> {
        let mut outcome: HashMap<PathBuf, Vec<(Loc, String)>> = Outcome::new();

        for (path_buf, source_unit) in source {
            let contracts = ContractDefinitionExtractor::extract(source_unit)?;

            for mut contract in contracts {
                //Dont care about interfaces here
                if !matches!(contract.ty, pt::ContractTy::Interface(_)) {
                    let external_functions = ExternalFunctionExtractor::extract(&mut contract)?;
                    if external_functions.len() > 10 {
                        let inherits_interface = contract.base.iter().any(|base| {
                            base.name
                                .identifiers
                                .iter()
                                .any(|identifier| identifier.name.contains('I'))
                        });
                        if !inherits_interface {
                            if let Some(ident) = &contract.name {
                                outcome.push_or_insert(
                                    path_buf.clone(),
                                    ident.loc,
                                    ident.to_string(),
                                );
                            }
                        }
                    }
                }
            }
        }

        Ok(QualityAssuranceOutcome::ContractsShouldInheritInterface(
            outcome,
        ))
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused)]
    use super::*;
    use crate::qa::ContractsShouldInheritInterface;
    #[allow(unused)]
    use crate::utils::MockSource;
    #[test]
    fn test_contract_interface_inherits() -> eyre::Result<()> {
        let file_contents = r#"
    import "IContract.sol";
    import "IContract2.sol";

    contract contract0 is IContract {}

    contract contract1 is Contract0 {
        function foo() external returns (uint256 x);
        function bar() external returns (uint256 x);

        function foo1() external returns (uint256 x);
        function bar1() external returns (uint256 x);
        function foo2() external returns (uint256 x);

        function foo3() external returns (uint256 x);
        function bar3() external returns (uint256 x);
        function foo4() external returns (uint256 x);
        function bar4() external returns (uint256 x);
        function foo5() external returns (uint256 x);
        function bar5() external returns (uint256 x);
        function foo6() external returns (uint256 x);
    }

    contract contract2 is contract1, IContract2 {
        function foo() external returns (uint256 x);
        function bar() external returns (uint256 x);

        function foo1() external returns (uint256 x);
        function bar1() external returns (uint256 x);
        function foo2() external returns (uint256 x);

        function foo3() external returns (uint256 x);
        function bar3() external returns (uint256 x);
        function foo4() external returns (uint256 x);
        function bar4() external returns (uint256 x);
        function foo5() external returns (uint256 x);
        function bar5() external returns (uint256 x);
        function foo6() external returns (uint256 x);
    }
    "#;

        let mut mock_source =
            MockSource::new().add_source("contract_interface_inheritance.sol", file_contents);
        let qa_locations = ContractsShouldInheritInterface::find(&mut mock_source.source)?;

        assert_eq!(qa_locations.len(), 1);

        Ok(())
    }
}
