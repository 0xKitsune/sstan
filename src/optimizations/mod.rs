use crate::engine::EngineError;

use super::engine::{Outcome, Report};
use std::collections::HashMap;
use std::path::PathBuf;

use solang_parser::pt::{Loc, SourceUnit};

pub trait OptimizationPattern {
    fn find(source: HashMap<PathBuf, &mut SourceUnit>) -> Result<OptimizationOutcome, EngineError>;
}

#[macro_export]
macro_rules! optimization {
    ($(($name:ident, $gas_savings:expr, $report_title:expr, $description:expr, $gas_report:expr)),+ $(,)?) => {


        $(pub struct $name;)+

        #[allow(non_snake_case)]
        #[derive(Debug)]
        pub enum OptimizationTarget {
            $($name,)+
        }

        #[derive(Debug)]
        pub enum OptimizationOutcome {
            $($name(Outcome),)+
        }


        impl OptimizationOutcome {
            pub fn len(&self) -> usize {
                match self {
                    $(
                        OptimizationOutcome::$name(outcome) => outcome.iter().map(|(_, v)| v.len()).sum(),
                    )+
                }
            }


            pub fn gas_saved(&self) -> usize {
                match self {
                    $(
                        OptimizationOutcome::$name(outcome) => outcome.iter().map(|(_, v)|
                        $gas_savings * v.len()).sum(),
                    )+
                }
            }

            pub fn gas_report(&self) -> &str {
                match self {
                    $(
                        OptimizationOutcome::$name(_) => $gas_report,
                    )+
                }
            }
        }



        //TODO: simplify this so that it isnt implementing this for every single macro, just have | when you are matching
        impl Into<Report> for OptimizationOutcome {
            fn into(self) -> Report {
                match self {
                    $(
                        OptimizationOutcome::$name(outcome) => {
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

// Gas saved is an approximation based on the maximum gas that can be saved from the optimization
optimization!((
    OptName,
    1000,
    "report title",
    "report description",
    "gas report"
));
