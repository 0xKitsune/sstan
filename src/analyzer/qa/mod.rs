pub mod addresses_hardcoded;
pub mod constant_immutable_namespace;
pub mod constructor_order;
pub mod constructor_var_initialization;
pub mod import_identifiers;
pub mod interface_namespace;
pub mod large_multiples_of_ten;
pub mod private_func_leading_underscore;
pub mod private_vars_leading_underscore;
pub mod storage_variable_namespace;
pub mod template;
pub mod unused_functions;
pub mod unused_returns;

use std::{
    collections::{BTreeSet, HashMap},
    fs,
};

use self::{
    constructor_order::constructor_order_qa, import_identifiers::import_identifiers,
    private_func_leading_underscore::private_func_leading_underscore,
    private_vars_leading_underscore::private_vars_leading_underscore,
};

use super::utils::{self, LineNumber};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum QualityAssurance {
    ConstructorOrder,
    PrivateVarsLeadingUnderscore,
    PrivateFuncLeadingUnderscore,
    ImportIdentifiers,
    AddressHardcoded,
    ConstantImmutableNamespace,
    InterfaceNamespace,
    LargeMultiplesOfTen,
    StorageVariableNamespace,
    UnusedFunctions,
    UnusedReturns,
    ConstructorVarInitialization,
}

pub fn get_all_qa() -> Vec<QualityAssurance> {
    vec![
        QualityAssurance::ConstructorOrder,
        QualityAssurance::PrivateVarsLeadingUnderscore,
        QualityAssurance::PrivateFuncLeadingUnderscore,
        QualityAssurance::ImportIdentifiers,
        QualityAssurance::AddressHardcoded,
        QualityAssurance::ConstantImmutableNamespace,
        QualityAssurance::InterfaceNamespace,
        QualityAssurance::LargeMultiplesOfTen,
        QualityAssurance::StorageVariableNamespace,
        QualityAssurance::UnusedFunctions,
        QualityAssurance::UnusedReturns,
        QualityAssurance::ConstructorVarInitialization,
    ]
}

pub fn str_to_qa(qa: &str) -> QualityAssurance {
    match qa.to_lowercase().as_str() {
        "constructor_order" => QualityAssurance::ConstructorOrder,
        "private_vars_leading_underscore" => QualityAssurance::PrivateVarsLeadingUnderscore,
        "private_func_leading_underscore" => QualityAssurance::PrivateFuncLeadingUnderscore,
        "import_identifiers" => QualityAssurance::ImportIdentifiers,
        "address_hardcoded" => QualityAssurance::AddressHardcoded,
        "constant_immutable_namespace" => QualityAssurance::ConstantImmutableNamespace,
        "interface_namespace" => QualityAssurance::InterfaceNamespace,
        "large_multiples_of_ten" => QualityAssurance::LargeMultiplesOfTen,
        "storage_variable_namespace" => QualityAssurance::StorageVariableNamespace,
        "unused_functions" => QualityAssurance::UnusedFunctions,
        "unused_returns" => QualityAssurance::UnusedReturns,
        "constructor_var_initialization" => QualityAssurance::ConstructorVarInitialization,

        other => {
            panic!("Unrecgonized qa: {}", other)
        }
    }
}

pub fn analyze_dir(
    target_dir: &str,
    qa: Vec<QualityAssurance>,
) -> eyre::Result<HashMap<QualityAssurance, Vec<(String, BTreeSet<LineNumber>)>>> {
    //Initialize a new hashmap to keep track of all the optimizations across the target dir
    let mut qa_locations: HashMap<QualityAssurance, Vec<(String, BTreeSet<LineNumber>)>> =
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
            if file_path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .contains("test")
                || file_path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .contains("mock")
            {
                continue;
            }

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

pub fn analyze_for_qa(
    file_contents: &str,
    file_number: usize,
    qa: QualityAssurance,
) -> eyre::Result<BTreeSet<LineNumber>> {
    let mut line_numbers: BTreeSet<LineNumber> = BTreeSet::new();

    //Parse the file into a the ast
    let mut source_unit = solang_parser::parse(file_contents, file_number).unwrap().0;

    let locations = match qa {
        QualityAssurance::ConstructorOrder => constructor_order_qa(&mut source_unit)?,
        QualityAssurance::PrivateVarsLeadingUnderscore => {
            private_vars_leading_underscore(&mut source_unit)?
        }
        QualityAssurance::PrivateFuncLeadingUnderscore => {
            private_func_leading_underscore(&mut source_unit)?
        }
        QualityAssurance::ImportIdentifiers => import_identifiers(&mut source_unit)?,
        QualityAssurance::AddressHardcoded => {
            addresses_hardcoded::addresses_hardcoded(&mut source_unit)?
        }
        QualityAssurance::ConstantImmutableNamespace => {
            constant_immutable_namespace::constant_immutable_namespace(&mut source_unit)?
        }
        QualityAssurance::InterfaceNamespace => {
            interface_namespace::interfaces_namespace(&mut source_unit)?
        }
        QualityAssurance::LargeMultiplesOfTen => {
            large_multiples_of_ten::large_multiples_of_ten(&mut source_unit)?
        }
        QualityAssurance::StorageVariableNamespace => {
            storage_variable_namespace::variable_namespace(&mut source_unit)?
        }
        QualityAssurance::UnusedFunctions => unused_functions::unused_functions(&mut source_unit)?,
        QualityAssurance::UnusedReturns => unused_returns::unused_returns(&mut source_unit)?,
        QualityAssurance::ConstructorVarInitialization => {
            constructor_var_initialization::constructor_var_initialization(&mut source_unit)?
        }
    };

    for loc in locations {
        line_numbers.insert(utils::get_line_number(loc.start(), file_contents));
    }

    Ok(line_numbers)
}
