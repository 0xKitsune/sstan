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
    pt::{Expression, SourceUnit},
};

use super::{QAPattern, QualityAssuranceOutcome, RequireWithoutMessage};
use crate::engine::Pushable;
use crate::engine::{EngineError, Outcome};
use crate::extractors::{primitive::FunctionCallExtractor, Extractor};

impl QAPattern for RequireWithoutMessage {
    fn find(
        source: &mut HashMap<PathBuf, SourceUnit>,
    ) -> Result<QualityAssuranceOutcome, EngineError> {
        let mut outcome = Outcome::new();
        for (path_buf, source_unit) in source {
            let target_nodes = FunctionCallExtractor::extract(source_unit)?;

            for node in target_nodes {
                if let Expression::FunctionCall(
                    _loc,
                    function_identifier,
                    function_call_expressions,
                ) = node.clone()
                {
                    //if the function call identifier is a variable
                    if let Expression::Variable(identifier) = *function_identifier {
                        dbg!(identifier.name.clone());
                        //if the identifier name is "require"
                        if identifier.name == "require" || identifier.name == "revert" {
                            //if the last expression is not a string literal
                            match function_call_expressions.last() {
                                Some(Expression::StringLiteral(_)) => {}
                                _ => {
                                    outcome.push_or_insert(
                                        path_buf.clone(),
                                        node.loc(),
                                        node.to_string(),
                                    );
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(QualityAssuranceOutcome::RequireWithoutMessage(outcome))
    }
}

mod test {
    #[allow(unused)]
    use super::*;
    #[allow(unused)]
    use crate::utils::MockSource;

    #[test]
    fn test_require_without_message() -> eyre::Result<()> {
        let file_contents = r#"
    contract Contract0 {
        function addressInternalBalance() public returns (uint256) {

            uint256 a = 100;
            uint256 b = 100;
            uint256 c = 100;

            require(a == 101, "some message");
            require(a == 101);

            require(true && a==b, "some message");
            require(true && a==b && b==c);

            return address(this).balance;


        }
    }
    "#;

        let mut source = MockSource::new().add_source("require_without_message.sol", file_contents);
        let optimization_locations = RequireWithoutMessage::find(&mut source.source)?;

        assert_eq!(optimization_locations.len(), 2);

        Ok(())
    }
}
