use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufRead, BufReader},
    path::PathBuf,
    str::FromStr,
};

use crate::{
    engine::{EngineError, Outcome, Pushable},
    extractors::{
        primitive::{NumberLiteralExtractor, VariableDefinitionExtractor},
        Extractor,
    },
    utils,
};
use ruint::Uint;
use solang_parser::pt::{self, CodeLocation, Loc, SourceUnit};

use super::{MissingUnderscoresForLargeNumericLiterals, QAPattern, QualityAssuranceOutcome};
impl QAPattern for MissingUnderscoresForLargeNumericLiterals {
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
                        //If anyone looks at this code .. I'm sorry
                        let val = Uint::<256, 4>::from_str(number).unwrap();
                        if val.gt(&Uint::<256, 4>::from_str("1000000000000").unwrap()) {
                            let path = path_buf.as_path().to_str().unwrap();
                            let file_contents = fs::read_to_string(path).unwrap();
                            let line = utils::get_line_number(variable.loc().end(), &file_contents);
                            let line_contents = BufReader::new(File::open(path).unwrap())
                                .lines()
                                .nth(line - 1)
                                .unwrap()
                                .unwrap();
                            if let Some(ident) = &variable.name {
                                let node = line_contents.split(&ident.name).collect::<Vec<&str>>()
                                    [1]
                                .split('=')
                                .collect::<Vec<&str>>()[1];
                                if !node.contains('_') {
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
            }
        }

        Ok(QualityAssuranceOutcome::MissingUnderscoresForLargeNumericLiterals(outcome))
    }
}
#[cfg(test)]
mod test {
    use crate::utils::MockSource;

    use super::*;

    #[test]
    fn test_missing_underscores_for_large_numeric_literals() -> eyre::Result<()> {
        let file_contents = r#"
    contract Contract0 {
        uint x = 1000000000001;
        uint y = 10000000000000000;
        uint z = 1000;
        uint a = 100_000_000_000_000_000_000;
    }
  

    "#;

        let mut mock_source =
            MockSource::new().add_source("missing_underscores.sol", file_contents);
        let qa_locations =
            MissingUnderscoresForLargeNumericLiterals::find(&mut mock_source.source)?;
        assert_eq!(qa_locations.len(), 2);

        Ok(())
    }
}
