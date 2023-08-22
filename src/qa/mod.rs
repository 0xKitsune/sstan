pub mod constructor_order;
pub mod constructor_var_initialization;
pub mod private_vars_leading_underscore;

use crate::engine::EngineError;

use super::engine::{Outcome, Report};
use std::collections::HashMap;
use std::path::PathBuf;

use solang_parser::pt::{Loc, SourceUnit};
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
    ($(($name:ident, $report_title:expr, $description:expr)),+ $(,)?) => {


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
                                r###"### {}\n{}"###,
                                stringify!($report_title),
                                $description
                            );

                            for (path, loc_snippets) in outcome.iter() {
                                for (loc, snippet) in loc_snippets.iter() {
                                    if let Loc::File(_, start, end) = loc{
                                    report.push_str(&format!(
                                        "{}:{}-{}\n{}\n",
                                        path.display(),
                                      start, //TODO: need to call line number function on this
                                       end,
                                        snippet
                                    ));
                                }else{
                                    panic!("handle this TODO:");

                                }
                            }
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
        "Description of the qa pattern goes here"
    ),
    (
        PrivateVariablesLeadingUnderscore,
        "Private variables should contain a leading underscore",
        "Description of the qa pattern goes here"
    ),
    (
        ConstructorVarInitialization,
        "Constructor should initialize all variables",
        "Description of the qa pattern goes here"
    )
);
