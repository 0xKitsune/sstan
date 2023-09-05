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

use super::{DivideBeforeMultiply, VulnerabilityOutcome, VulnerabilityPattern};

impl VulnerabilityPattern for DivideBeforeMultiply {
    fn find(
        source: &mut HashMap<PathBuf, SourceUnit>,
    ) -> Result<VulnerabilityOutcome, EngineError> {
        let mut vulnerability_locations = Outcome::new();

        for (path_buf, source_unit) in source {
            //Extract the target nodes from the source_unit
            let target_nodes = AssignmentExtractor::extract(source_unit)?
                .into_iter()
                .chain(UrnaryOpteratorExtractor::extract(source_unit)?.into_iter())
                .collect::<Vec<Expression>>();

            //For each target node that was extracted, check for the vulnerability patterns
            for node in target_nodes {
                match node.clone() {
                    pt::Expression::Multiply(loc, box_expression, _) => {
                        let mut curr_expression = *box_expression;
                        loop {
                            match curr_expression {
                                pt::Expression::Divide(_, _, _) => {
                                    //Found case where division occurs before multiplication
                                    vulnerability_locations.push_or_insert(
                                        path_buf.clone(),
                                        loc,
                                        node.to_string(),
                                    );
                                    break;
                                }
                                pt::Expression::Multiply(_, next_expression, _)
                                | pt::Expression::Parenthesis(_, next_expression) => {
                                    //Continue to check the next expression for division
                                    curr_expression = *next_expression;
                                }
                                _ => {
                                    break;
                                }
                            }
                        }
                    }
                    pt::Expression::AssignDivide(loc, _, box_expression) => {
                        let mut curr_expression = *box_expression;
                        loop {
                            match curr_expression {
                                pt::Expression::Multiply(_, _, _) => {
                                    //Found case where multiplication occurs before division
                                    vulnerability_locations.push_or_insert(
                                        path_buf.clone(),
                                        loc,
                                        node.to_string(),
                                    );
                                    break;
                                }
                                pt::Expression::Divide(_, next_expression, _)
                                | pt::Expression::Add(_, next_expression, _)
                                | pt::Expression::Subtract(_, next_expression, _)
                                | pt::Expression::Modulo(_, next_expression, _)
                                | pt::Expression::BitwiseAnd(_, next_expression, _)
                                | pt::Expression::BitwiseOr(_, next_expression, _)
                                | pt::Expression::BitwiseXor(_, next_expression, _)
                                | pt::Expression::ShiftLeft(_, next_expression, _)
                                | pt::Expression::ShiftRight(_, next_expression, _)
                                | pt::Expression::Parenthesis(_, next_expression) => {
                                    //Continue to check the next expression for multiplication
                                    curr_expression = *next_expression;
                                }
                                _ => {
                                    break;
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(VulnerabilityOutcome::DivideBeforeMultiply(
            vulnerability_locations,
        ))
    }
}

#[test]
fn test_divide_before_multiply_vulnerability() -> eyre::Result<()> {
    let file_contents = r#"

    contract Contract0 {

        function arithmetic_operations() public {
            1 / 2 * 3; // Unsafe
            1 * 2 / 3; // Safe
            (1 / 2) * 3; // Unsafe
            (1 * 2) / 3; // Safe
            (1 / 2 * 3) * 4; // Unsafe (x2)
            (1 * 2 / 3) * 4; // Unsafe
            (1 / 2 / 3) * 4; // Unsafe
            1 / (2 + 3) * 4; // Unsafe
            (1 / 2 + 3) * 4; // Safe
            (1 / 2 - 3) * 4; // Safe
            (1 + 2 / 3) * 4; // Safe
            (1 / 2 - 3) * 4; // Safe
            (1 / 2 % 3) * 4; // Safe
            (1 / 2 | 3) * 4; // Safe
            (1 / 2 & 3) * 4; // Safe
            (1 / 2 ^ 3) * 4; // Safe
            (1 / 2 << 3) * 4; // Safe
            (1 / 2 >> 3) * 4; // Safe
            1 / (2 * 3 + 3); // Safe
            1 / ((2 / 3) * 3); // Unsafe
            1 / ((2 * 3) + 3); // Safe

            uint256 x = 5;
            x /= 2 * 3; // Unsafe
            x /= 2 / 3; // Safe
            x /= (2 * 3); // Unsafe
            x /= (1 / 2) * 3; // Unsafe (x2)
            x /= (1 * 2) * 3; // Unsafe
            x /= (2 * 3) / 4; // Unsafe
            x /= 2 * 3 / 4; // Unsafe
            x /= 2 * 3 - 4; // Unsafe
            x /= 2 * 3 % 4; // Unsafe
            x /= 2 * 3 | 4; // Unsafe
            x /= 2 * 3 & 4; // Unsafe
            x /= 2 * 3 ^ 4; // Unsafe
            x /= 2 * 3 << 4; // Unsafe
            x /= 2 * 3 >> 4; // Unsafe
            x /= 3 / 4; // Safe
            x /= 3 - 4; // Safe
            x /= 3 % 4; // Safe
            x /= 3 | 4; // Safe
            x /= 3 & 4; // Safe
            x /= 3 ^ 4; // Safe
            x /= 3 << 4; // Safe
            x /= 3 >> 4; // Safe
        }

    }
    "#;

    let mut mock_source = MockSource::new().add_source("divde_before_multiply.sol", file_contents);
    let qa_locations = DivideBeforeMultiply::find(&mut mock_source.source)?;
    assert_eq!(qa_locations.len(), 22);

    let report: Option<ReportSectionFragment> = qa_locations.into();
    if let Some(report) = report {
        let mut f = File::options()
            .append(true)
            .open("src/report/mocks/qa_report_sections.md")?;
        writeln!(&mut f, "{}", &String::from(report))?;
    }

    Ok(())
}
