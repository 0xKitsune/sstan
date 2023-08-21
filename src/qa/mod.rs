pub mod constructor_order;
pub mod private_vars_leading_underscore;

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
    fn find(source: HashMap<PathBuf, &mut SourceUnit>) -> QualityAssuranceOutcome;
}

#[macro_export]
macro_rules! quality_assurance {
    ($(($name:ident, $description:expr)),+ $(,)?) => {


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


        impl Into<Report> for QualityAssuranceOutcome {
            fn into(self) -> Report {
                match self {
                    $(
                        QualityAssuranceOutcome::$name(outcome) => {
                            let mut report = format!(
                                r###"### {}\n{}"###,
                                stringify!($name),
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
    (ConstructorOrder, "Description of the qa pattern goes here"),
    (
        PrivateVariablesLeadingUnderscore,
        "Description of the qa pattern goes here"
    ),
);
