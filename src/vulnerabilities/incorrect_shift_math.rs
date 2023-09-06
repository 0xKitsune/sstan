use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::path::PathBuf;

use solang_parser::pt::{self, Expression, Loc};
use solang_parser::{self, pt::SourceUnit};

use crate::engine::{EngineError, Outcome, Pushable};
use crate::extractors::{
    primitive::{AssignmentExtractor, UrnaryOpteratorExtractor},
    Extractor,
};
use crate::report::ReportSectionFragment;
use crate::utils::MockSource;
use std::io::Write;

use super::{
    DivideBeforeMultiply, FloatingPragma, IncorrectShiftMath, VulnerabilityOutcome,
    VulnerabilityPattern,
};

impl VulnerabilityPattern for IncorrectShiftMath {
    fn find(
        source: &mut HashMap<PathBuf, SourceUnit>,
    ) -> Result<VulnerabilityOutcome, EngineError> {
        let mut vulnerability_locations = Outcome::new();

        for (path_buf, source_unit) in source {
            let yul_shift_function_calls = YulShiftExtractor::extract(source_unit)?;
            //Iterate through each shift. If the first element of the shift is a Variable, and the second is a NumberLiteral | HexNumberLiteral this is likely a vulnerability.
            for shift_function_call in yul_shift_function_calls {
                let shift_amount = shift_function_call.arguments.first();
                let shifted_variable = shift_function_call.arguments.last();

                if let Some(shift_amount) = shift_amount {
                    if let Some(shifted_variable) = shifted_variable {
                        if let pt::YulExpression::Variable(_) = shift_amount {
                            if let pt::YulExpression::NumberLiteral(_, _, _, _) = shifted_variable {
                                vulnerability_locations.push_or_insert(
                                    path_buf.clone(),
                                    shift_function_call.loc,
                                    shift_function_call.to_string(),
                                )
                            }
                            if let pt::YulExpression::HexNumberLiteral(_, _, _) = shifted_variable {
                                vulnerability_locations.push_or_insert(
                                    path_buf.clone(),
                                    shift_function_call.loc,
                                    shift_function_call.to_string(),
                                )
                            }
                        }
                    }
                }
            }
            Ok(vulnerability_locations)
        }

        Ok(VulnerabilityOutcome::DivideBeforeMultiply(
            vulnerability_locations,
        ))
    }
}

#[test]
fn test_floating_pragma_vulnerability() -> eyre::Result<()> {
    let file_contents = r#"
    
    contract Contract0 {
        function incorrectShiftMath() public returns (uint256 x) {
            assembly {
                let x := shl(x, 1)
                let x := shr(x, 1)
            }
        }

        function correctShiftMath() public returns (uint256 x) {
            assembly {
                let x := shl(1, x)
                let x := shr(1, x)
            }
        }
    }
    "#;
    let mut mock_source = MockSource::new().add_source("incorrect_shift_math.sol", file_contents);
    let qa_locations = DivideBeforeMultiply::find(&mut mock_source.source)?;
    assert_eq!(qa_locations.len(), 2);

    let report: Option<ReportSectionFragment> = qa_locations.into();
    if let Some(report) = report {
        let mut f = File::options()
            .append(true)
            .open("src/report/mocks/qa_report_sections.md")?;
        writeln!(&mut f, "{}", &String::from(report))?;
    }

    Ok(())
}
