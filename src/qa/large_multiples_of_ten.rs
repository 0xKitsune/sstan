use std::{collections::HashMap, path::PathBuf};

use solang_parser::pt::{self, Loc, SourceUnit};

use crate::{
    create_test_source,
    engine::{EngineError, Outcome, Pushable},
    extractors::{primitive::NumberLiteralExtractor, Extractor},
};

use super::{LargeMultiplesOfTen, QAPattern, QualityAssuranceOutcome};
impl QAPattern for LargeMultiplesOfTen {
    fn find(
        source: HashMap<PathBuf, &mut SourceUnit>,
    ) -> Result<QualityAssuranceOutcome, EngineError> {
        let mut outcome: HashMap<PathBuf, Vec<(Loc, String)>> = Outcome::new();

        for (path_buf, source_unit) in source {
            let number_literals = NumberLiteralExtractor::extract(source_unit)?;
            //Get all variables that are larger than 1000000, and divisible by 10.
            for number_literal in number_literals.iter() {
                if let pt::Expression::NumberLiteral(loc, str_number, _, _) = number_literal {
                    //TODO: Use u256's here to prevent potential overflow. Or alternatively rug Integer.
                    let number = str_number.to_string().parse::<u128>()?;
                    if number % 10 == 0 && number > 1000000 {
                        outcome.push_or_insert(
                            path_buf.clone(),
                            loc.clone(),
                            number_literal.to_string(),
                        );
                    }
                }
            }
        }

        Ok(QualityAssuranceOutcome::LargeMultiplesOfTen(outcome))
    }
}

#[test]
fn test_large_multiples_of_ten() -> eyre::Result<()> {
    let file_contents_1 = r#"
    contract Contract0 {
        address public owner;
        uint x = 1e7;
        constructor(address _owner) {
            owner = _owner;
        }
    }
  
    contract Contract0 {
        address public owner;
        uint x = 10000000;
        constructor(address _owner) {
            require(_owner != address(0), "Owner cannot be zero address");
            owner = _owner;
        }
    }
    "#;

    let source = create_test_source!(file_contents_1);
    let qa_locations = LargeMultiplesOfTen::find(source)?;
    assert_eq!(qa_locations.len(), 1);
    Ok(())
}
