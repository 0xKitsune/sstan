pub mod constructor_order;
pub mod constructor_var_initialization;
pub mod import_identifiers;
pub mod interface_namespace;
pub mod large_multiples_of_ten;
pub mod private_vars_leading_underscore;
pub mod constant_immutable_name_screaming_snake_case;
pub mod contract_name_pascal_case;
pub mod event_name_pascalcase;
pub mod function_name_camel_case;
pub mod one_contract_per_file;
pub mod remove_console;
pub mod storage_variable_namespace;
pub mod unused_functions;
pub mod unused_returns;
pub mod public_functions;
pub mod require_without_message;

use super::engine::Outcome;
use crate::engine::EngineError;
use crate::report::Identifier;
use crate::report::{Classification, OutcomeReport, ReportSectionFragment};
use crate::utils;
use solang_parser::pt::{Loc, SourceUnit};
use std::collections::HashMap;
use std::path::PathBuf;

pub trait QAPattern {
    fn find(
        source: &mut HashMap<PathBuf, SourceUnit>,
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

        impl QualityAssuranceTarget {
            pub fn find(
                &self,
                source: &mut HashMap<PathBuf, SourceUnit>,
            ) -> Result<QualityAssuranceOutcome, EngineError> {
                match self {
                    $(
                        QualityAssuranceTarget::$name => $name::find(source),
                    )+
                }
            }


            pub fn all_targets() -> Vec<QualityAssuranceTarget> {
                vec![
                    $(QualityAssuranceTarget::$name,)+
                ]
            }
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

            pub fn is_empty(&self) -> bool {
                self.len() == 0
            }

            pub fn classification(&self) -> Classification {
                Classification::NonCritical
            }
        }




        impl From<QualityAssuranceOutcome> for Option<ReportSectionFragment> {
            fn from(value: QualityAssuranceOutcome) -> Self {
                match &value {
                    $(
                        QualityAssuranceOutcome::$name(outcome) => {
                            if outcome.is_empty() {
                                return None;
                            }
                            let length = outcome.iter().map(|(_, v)| v.len()).sum::<usize>();


                            let mut report_fragment = ReportSectionFragment::new(
                                $report_title.to_string(),
                                Identifier::new(value.classification(), 0),
                                $description.to_string(),
                                length,
                            );
                            let mut outcome_reports = vec![];
                            for (path, loc_snippets) in outcome.iter() {
                                let file_name = path.file_name().expect("couldnt get file name")  //TODO: make this a little more descriptive or propagate
                                .to_str()
                                .expect("no filename"); //TODO: make this a little more descriptive or propagate

                                for (loc, snippet) in loc_snippets.iter() {
                                    if let Loc::File(_, start, end) = loc{
                                        let file_contents = std::fs::read_to_string(path).expect("couldnt read file"); //TODO: propagate this or maybe just make more descriptive
                                        let start_line = utils::get_line_number(*start, &file_contents);
                                        let end_line = utils::get_line_number(*end, &file_contents);
                                        outcome_reports.push(OutcomeReport::new(
                                            file_name.to_string(),
                                            (start_line, end_line),
                                            snippet.to_string(),
                                            path.clone(),
                                        ));

                                }else{
                                    panic!("handle this TODO:");

                                }
                            }


                            }
                            report_fragment.outcomes = outcome_reports;
                            Some(report_fragment)

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
    ),
    (
        ImportIdentifiers,
        "Consider importing specific identifiers instead of the whole file",
        "This will minimize compiled code size and help with readability"
    ),
    (
        InterfaceNamespace,
        "Interface names should start with an I",
        "Consider renaming for consistency"
    ),
    (
        LargeMultiplesOfTen,
        "Consider using scientific notation for large multiples of 10",
        "For example 100000 can be written as 1e5"
    ),
    (UnusedFunctions, "Remove any unused functions", "Any functions not used should be removed as best practice."), 
    (
        OneContractPerFile,
        "Only define one contract per file",
        "It is best practice to only define one contract per file."
    ),
    (
        RemoveConsole,
        "Remove console.log statements",
        "Console.log statements should be removed from production code"
    ),
    (
        StorageVariableNamespace,
        "Storage variables should be named with camel case",
        "Consider renaming to follow convention"
    ),
    (
        ContractNamePascalCase,
        "Contract names should be in PascalCase",
        "Ensure that contract definitions are declared using PascalCase"
    ),
    (
        FunctionNameCamelCase,
        "Function names should be in camelCase",
        "Ensure that function definitions are declared using camelCase"
    ),
    (
        ConstantImmutableNameScreamingSnakeCase,
        "Constant and immutable variable names should be in SCREAMING_SNAKE_CASE",
        "Ensure that Constant and immutable variable names are declared using SCREAMING_SNAKE_CASE"
    ),
    (
        EventNamePascalCase,
        "Event names should be in PascalCase",
        "Ensure that event definitions are declared using PascalCase"
    ),
    (UnusedReturns, "Remove any unused returns", "Either remove the return parameter names, or use them as the returns of the function."), 
    (PublicFunctions,"Consider marking public function External", "If a public function is never called internally. It is best practice to mark it as external."
    ),

    (RequireWithoutMessage,"Consider adding a message with require and revert statements", "Adding a message to accompany require statements will provide more context when a transaction fails."
)
);
