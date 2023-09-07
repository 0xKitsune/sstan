use std::{collections::HashMap, path::PathBuf};

use crate::{
    engine::{EngineError, Outcome, Pushable},
    extractors::{
        primitive::{NumberLiteralExtractor, VariableDefinitionExtractor},
        Extractor,
    },
};
use solang_parser::pt::{self, CodeLocation, Loc, SourceUnit};

use super::{LargeMultiplesOfTen, QAPattern, QualityAssuranceOutcome};
impl QAPattern for LargeMultiplesOfTen {
    fn find(
        source: &mut HashMap<PathBuf, SourceUnit>,
    ) -> Result<QualityAssuranceOutcome, EngineError> {
        let mut outcome: HashMap<PathBuf, Vec<(Loc, String)>> = Outcome::new();

        for (path_buf, source_unit) in source {
            let mut variable_defs = VariableDefinitionExtractor::extract(source_unit)?;
            for mut variable in variable_defs.iter_mut() {
                let mut number_literals = NumberLiteralExtractor::extract(&mut variable)?;
                for number_literal in number_literals.iter_mut() {
                    if let pt::Expression::NumberLiteral(_loc, number, _value, _ident) =
                        number_literal
                    {
                        let number = number.parse::<u128>().unwrap();
                        if number % 10 == 0 && number > 1000000 {
                            outcome.push_or_insert(
                                path_buf.clone(),
                                variable.loc(),
                                variable.to_string(),
                            );
                        }
                    }
                }
            }
        }

        Ok(QualityAssuranceOutcome::LargeMultiplesOfTen(outcome))
    }
}
#[cfg(test)]
mod test {
    use crate::{report::ReportSectionFragment, utils::MockSource};

    use super::*;
    use std::{fs::File, io::Write};

    #[test]
    fn test_large_multiples_of_ten() -> eyre::Result<()> {
        let file_contents = r#"
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

        let mut mock_source =
            MockSource::new().add_source("large_multiples_of_ten.sol", file_contents);
        let qa_locations = LargeMultiplesOfTen::find(&mut mock_source.source)?;
        assert_eq!(qa_locations.len(), 1);

        let report: Option<ReportSectionFragment> = qa_locations.into();
        if let Some(report) = report {
            let mut f = File::options().append(true).open("qa_report_sections.md")?;
            writeln!(&mut f, "{}", &String::from(report))?;
        }
        Ok(())
    }
}
