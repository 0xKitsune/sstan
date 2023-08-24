pub mod constructor_order;
pub mod constructor_var_initialization;
pub mod private_vars_leading_underscore;
pub mod import_identifiers;
use super::engine::{Outcome, Report};
use crate::engine::EngineError;
use crate::utils;
use solang_parser::pt::{Loc, SourceUnit};
use std::collections::HashMap;
use std::path::PathBuf;
//TODO: we could have something here that creates the OptimizationOutcome enum

//optimizations!(
// (ConstructorOrder, "This is the description that will be used when into_report, maybe add an example or something")
// )

//TODO: this is what we would use for each individual pattern and then we just implement the find method instead of the function
pub trait QAPattern {
    fn find(
        source: HashMap<PathBuf, &mut SourceUnit>,
    ) -> Result<QualityAssuranceOutcome, EngineError>;
}

#[macro_export]
macro_rules! quality_assurance {
    ($(($name:ident, $report_title:expr, $description:expr, $issue_type:expr)),+ $(,)?) => {


        $(pub struct $name;)+

        #[allow(non_snake_case)]
        #[derive(Debug)]
        pub enum QualityAssuranceTarget {
            $($name,)+
        }

        #[derive(Debug)]
        pub enum QualityAssuranceOutcome {
            $($name(Outcome),)+
        }


        impl QualityAssuranceOutcome {
            pub fn len(&self) -> usize {
                match self {
                    $(
                        QualityAssuranceOutcome::$name(outcome) => outcome.iter().map(|(_, v)| v.len()).sum(),
                    )+
                }
            }
        }



            //TODO: simplify this so that it isnt implementing this for every single macro, just have | when you are matching
        impl Into<Report> for QualityAssuranceOutcome {
            fn into(self) -> Report {
                match self {
                    $(
                        QualityAssuranceOutcome::$name(outcome) => {
                            let mut report = format!(
                                "### {}  \n {}
                                \n
                                 ",
                                $report_title,
                                $description
                            );
                            if !outcome.is_empty() {
                                report.push_str(&format!("\n <a name=\"{}\"></a>[{}] {}\n", $issue_type, $issue_type, stringify!($report_title)));
                            }
                            let length = outcome.iter().map(|(_, v)| v.len()).sum::<usize>();
                            report.push_str(&format!("\n*Instances ({})*:\n", length.to_string()));
                            for (path, loc_snippets) in outcome.iter() {
                                let file_name = path.file_name().expect("couldnt get file name")
                                .to_str()
                                .expect("no filename");
                             //TODO: update this to propagate
                             report.push_str(&format!(
                                "\n File: {} \n ```solidity", file_name
                            ));
                                for (loc, snippet) in loc_snippets.iter() {
                                    if let Loc::File(_, start, end) = loc{
                                    let file_contents = std::fs::read_to_string(path).expect("couldnt read file"); //TODO: propagate this
                                    let start_line = utils::get_line_number(*start, &file_contents);
                                    let end_line = utils::get_line_number(*end, &file_contents);
                                    report.push_str(&format!(
                                        "\n {}-{}: {}",
                                        start_line, end_line, snippet
                                    ));
                                }else{
                                    panic!("handle this TODO:");

                                }
                            }
                            report.push_str("```\n");
                            }

                            report
                        }
                    )+
                }
            }
        }
    };
}

//TODO: add section name
quality_assurance!(
    (
        ConstructorOrder,
        "Constructor should be listed before any other function",
        "Description of the qa pattern goes here",
        "N-1"
    ),
    (
        PrivateVariablesLeadingUnderscore,
        "Private variables should contain a leading underscore",
        "Description of the qa pattern goes here",
        "N-2"
    ),
    (
        ConstructorVarInitialization,
        "Constructor should initialize all variables",
        "Description of the qa pattern goes here",
        "N-3"
    ),
    (
        ImportIdentifiers,
        "Consider importing specific identifiers instead of the whole file",
        "This will minimize compiled code size and help with readability",
        "N-4"
    )
);
