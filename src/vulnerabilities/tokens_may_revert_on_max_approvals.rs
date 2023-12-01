use std::collections::HashMap;
use std::path::PathBuf;

use solang_parser::helpers::CodeLocation;
use solang_parser::pt::{self, Expression};
use solang_parser::{self, pt::SourceUnit};

use crate::engine::{EngineError, Outcome, Pushable};
use crate::extractors::primitive::{ContractDefinitionExtractor, FunctionCallExtractor};
use crate::extractors::Extractor;

use super::{TokensMayRevertOnMaxApprovals, VulnerabilityOutcome, VulnerabilityPattern};

impl VulnerabilityPattern for TokensMayRevertOnMaxApprovals {
    fn find(
        source: &mut HashMap<PathBuf, SourceUnit>,
    ) -> Result<VulnerabilityOutcome, EngineError> {
        let mut vulnerability_locations = Outcome::new();

        for (path_buf, source_unit) in source {
            let mut contracts = ContractDefinitionExtractor::extract(source_unit)?;
            for contract in contracts.iter_mut() {
                let function_calls = FunctionCallExtractor::extract(contract)?;
                for function_call in function_calls {
                    if let pt::Expression::FunctionCall(_, expr, expr_args) = function_call.clone()
                    {
                        if let pt::Expression::MemberAccess(_, _, ident) = *expr {
                            if ident.name == "approve" {
                                if let Some(Expression::MemberAccess(_, _, ident)) =
                                    expr_args.get(1)
                                {
                                    //type(<uint size>).max approval
                                    if ident.name == "max" {
                                        vulnerability_locations.push_or_insert(
                                            path_buf.clone(),
                                            function_call.loc(),
                                            function_call.to_string(),
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(VulnerabilityOutcome::TokensMayRevertOnMaxApprovals(
            vulnerability_locations,
        ))
    }
}
mod test {
    #[allow(unused)]
    use super::*;
    #[allow(unused)]
    use crate::utils::MockSource;
    #[test]
    fn test_tokens_may_revert_on_max_approvals() -> eyre::Result<()> {
        let file_contents = r#"

    pragma solidity ^0.8.16;

    contract Contract0 {
        function safeApproval(address spender, uint256 value, address token) public {
            IERC20(token).approve(spender, value);
        }

        function maxApproval(address spender, address token) public {
            IERC20(token).approve(spender, type(uint256).max);
        }
    }
    "#;

        let mut mock_source = MockSource::new().add_source("tokens_may_revert_on_max_approval.sol", file_contents);
        let vuln_locations = TokensMayRevertOnMaxApprovals::find(&mut mock_source.source)?;
        assert_eq!(vuln_locations.len(), 1);

        Ok(())
    }
}
