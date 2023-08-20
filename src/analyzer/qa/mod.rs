pub mod patterns;
use solang_parser::pt::Loc;
use std::{
    collections::{BTreeSet, HashMap},
    fs,
    path::PathBuf,
};

use patterns::{
    constructor_order::constructor_order, import_identifiers::import_identifiers,
    private_func_leading_underscore::private_func_leading_underscore,
    private_vars_leading_underscore::private_vars_leading_underscore,
};

use super::utils::{self, LineNumber, Outcome};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum QualityAssuranceTarget {
    ConstructorOrder,
    PrivateVarsLeadingUnderscore,
    PrivateFuncLeadingUnderscore,
    ImportIdentifiers,
}

//TODO: make a macro to define the type and the report description. the macro should just take many of these, so tha it can create the enum. 
There should be a file for it in the qa, vuln or optimizations dir and then a file for each pattern with tests.
quality_assurance_outcome!(
    ConstructorOrderOutcome,
    r#"""The constructor should be the first function in the contract"""#
);





//TODO: it will generate all this
pub enum QualityAssuranceOutcome {
    ConstructorOrder(ConstructorOrderOutcome),
    //     PrivateVarsLeadingUnderscore(PrivateVarsLeadingUnderscoreOutcome),
    //     PrivateFuncLeadingUnderscore(PrivateFuncLeadingUnderscoreOutcome),
    //     ImportIdentifiers(ImportIdentifiersOutcome),
}
pub struct ConstructorOrderOutcome {
    pub outcomes: Vec<Outcome>,
}


pub const CONSTRUCTOR_ORDER_DESCRIPTION: &str =
    "The constructor should be the first function in the contract";
impl Reporting for ConstructorOrderOutcome {
    fn into_report(self) -> Report {
        let report = CONSTRUCTOR_ORDER_DESCRIPTION;
        //TODO: maybe do gpt generation flag here, make a note

        for outcome in self.outcomes {
            // Add the outcome to the report and break the line
            format!("{}{}",report, outcome.into_report();)
        }
    }
    //TODO: return the report
}

//TODO: this should be stand alone, just implemented but not a part of the macro
// impl Reporting for Outcome {
//     fn into_report(self) -> Report {
//         //TODO: get the line number, get the code block, get the file name, generate into markdown
//     }
// }

pub enum QualityAssuranceOutcome {
    ConstructorOrder(Outcome),
    PrivateVarsLeadingUnderscore(Outcome),
    PrivateFuncLeadingUnderscore(Outcome),
    ImportIdentifiers(Outcome),
}

// -------------------- move the rest of this into some engine component trait

pub fn get_all_qa() -> Vec<QualityAssuranceTarget> {
    vec![
        QualityAssuranceTarget::ConstructorOrder,
        QualityAssuranceTarget::PrivateVarsLeadingUnderscore,
        QualityAssuranceTarget::PrivateFuncLeadingUnderscore,
        QualityAssuranceTarget::ImportIdentifiers,
    ]
}

//TODO: change this to impl from str
pub fn str_to_qa(qa: &str) -> QualityAssuranceTarget {
    match qa.to_lowercase().as_str() {
        "constructor_order" => QualityAssuranceTarget::ConstructorOrder,
        "private_vars_leading_underscore" => QualityAssuranceTarget::PrivateVarsLeadingUnderscore,
        "private_func_leading_underscore" => QualityAssuranceTarget::PrivateFuncLeadingUnderscore,
        other => {
            panic!("Unrecgonized qa: {}", other)
        }
    }
}

//TODO: this should be the run function for the QAModule, it should pass in the Vec<SourceUnit> though or whatever the IR looks like
pub fn analyze_dir(
    target_dir: &str,
    qa: Vec<QualityAssuranceTarget>,
) -> eyre::Result<HashMap<QualityAssuranceTarget, Vec<(String, BTreeSet<LineNumber>)>>> {
    //Initialize a new hashmap to keep track of all the optimizations across the target dir
    let mut qa_locations: HashMap<QualityAssuranceTarget, Vec<(String, BTreeSet<LineNumber>)>> =
        HashMap::new();

    //For each file in the target dir
    for (i, path) in fs::read_dir(target_dir)
        .unwrap_or_else(|_| panic!("Could not read contracts from directory: {:?}", target_dir))
        .enumerate()
    {
        //Get the file path, name and contents
        let file_path = path
            .unwrap_or_else(|_| panic!("Could not unwrap file path: {}", i))
            .path();

        if file_path.is_dir() {
            qa_locations.extend(analyze_dir(
                file_path
                    .as_os_str()
                    .to_str()
                    .expect("Could not get nested dir"),
                qa.clone(),
            )?)
        } else {
            let file_name = file_path
                .file_name()
                .unwrap_or_else(|| panic!("Could not unwrap file name to OsStr: {}", i))
                .to_str()
                .expect("Could not convert file name from OsStr to &str")
                .to_string();

            if file_name.ends_with(".sol") && !file_name.to_lowercase().contains(".t.sol") {
                let file_contents = fs::read_to_string(&file_path).expect("Unable to read file");

                //For each active optimization
                for target in &qa {
                    let line_numbers = analyze_for_qa(&file_contents, i, *target)?;

                    if !line_numbers.is_empty() {
                        let file_optimizations = qa_locations.entry(*target).or_insert(vec![]);

                        file_optimizations.push((file_name.clone(), line_numbers));
                    }
                }
            }
        }
    }

    Ok(qa_locations)
}

//TODO: update this to be some sort of impl on the enum, return a BtreeSet of Outcome instead of line number
pub fn analyze_for_qa(
    file_contents: &str,
    file_number: usize,
    qa: QualityAssuranceTarget,
) -> eyre::Result<BTreeSet<LineNumber>> {
    let mut line_numbers: BTreeSet<LineNumber> = BTreeSet::new();

    //Parse the file into a the ast
    let mut source_unit = solang_parser::parse(file_contents, file_number).unwrap().0;

    let locations = match qa {
        QualityAssuranceTarget::ConstructorOrder => constructor_order(&mut source_unit)?,
        QualityAssuranceTarget::PrivateVarsLeadingUnderscore => {
            private_vars_leading_underscore(&mut source_unit)?
        }
        QualityAssuranceTarget::PrivateFuncLeadingUnderscore => {
            private_func_leading_underscore(&mut source_unit)?
        }
        QualityAssuranceTarget::ImportIdentifiers => import_identifiers(&mut source_unit)?,
    };

    for loc in locations {
        line_numbers.insert(utils::get_line_number(loc.start(), file_contents));
    }

    Ok(line_numbers)
}
