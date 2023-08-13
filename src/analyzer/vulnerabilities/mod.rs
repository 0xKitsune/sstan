pub mod divide_before_multiply;
pub mod floating_pragma;
pub mod template;
pub mod unprotected_self_destruct;
pub mod unsafe_erc20_operation;

use std::{
    collections::{BTreeSet, HashMap},
    fs,
};

use super::utils::{self, LineNumber};

use self::{
    divide_before_multiply::divide_before_multiply_vulnerability,
    floating_pragma::floating_pragma_vulnerability,
    unprotected_self_destruct::unprotected_self_destruct_vulnerability,
    unsafe_erc20_operation::unsafe_erc20_operation_vulnerability,
};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]

pub enum Vulnerability {
    FloatingPragma,
    UnsafeERC20Operation,
    UnprotectedSelfdestruct,
    DivideBeforeMultiply,
}

pub fn get_all_vulnerabilities() -> Vec<Vulnerability> {
    vec![
        Vulnerability::UnsafeERC20Operation,
        Vulnerability::UnprotectedSelfdestruct,
        Vulnerability::DivideBeforeMultiply,
        Vulnerability::FloatingPragma,
    ]
}

pub fn str_to_vulnerability(vuln: &str) -> Vulnerability {
    match vuln.to_lowercase().as_str() {
        "floating_pragma" => Vulnerability::FloatingPragma,
        "unsafe_erc20_operation" => Vulnerability::UnsafeERC20Operation,
        "unprotected_selfdestruct" => Vulnerability::UnprotectedSelfdestruct,
        "divide_before_multiply" => Vulnerability::DivideBeforeMultiply,
        other => {
            panic!("Unrecgonized vulnerability: {}", other)
        }
    }
}

pub fn analyze_dir(
    target_dir: &str,
    vulnerabilities: Vec<Vulnerability>,
) -> eyre::Result<HashMap<Vulnerability, Vec<(String, BTreeSet<LineNumber>)>>> {
    //Initialize a new hashmap to keep track of all the optimizations across the target dir
    let mut vulnerability_locations: HashMap<Vulnerability, Vec<(String, BTreeSet<LineNumber>)>> =
        HashMap::new();

    //For each file in the target dir
    for (i, path) in fs::read_dir(target_dir)
        .unwrap_or_else(|_| panic!("Could not read contracts from directory: {:?}", target_dir))
        .enumerate()
    {
        //Get the file path, name and contents
        let file_path = path
            .unwrap_or_else(|_| panic!("{}", "Could not file unwrap path".to_string()))
            .path();

        if file_path.is_dir() {
            vulnerability_locations.extend(analyze_dir(
                file_path
                    .as_os_str()
                    .to_str()
                    .expect("Could not get nested dir"),
                vulnerabilities.clone(),
            )?)
        } else {
            let file_name = file_path
                .file_name()
                .expect("Could not unwrap file name to OsStr")
                .to_str()
                .expect("Could not convert file name from OsStr to &str")
                .to_string();

            if file_name.ends_with(".sol") && !file_name.to_lowercase().contains(".t.sol") {
                let file_contents = fs::read_to_string(&file_path).expect("Unable to read file");

                //For each active optimization
                for vulnerability in &vulnerabilities {
                    let line_numbers =
                        analyze_for_vulnerability(&file_contents, i, *vulnerability)?;

                    if !line_numbers.is_empty() {
                        let file_optimizations = vulnerability_locations
                            .entry(*vulnerability)
                            .or_insert(vec![]);

                        file_optimizations.push((file_name.clone(), line_numbers));
                    }
                }
            }
        }
    }

    Ok(vulnerability_locations)
}

pub fn analyze_for_vulnerability(
    file_contents: &str,
    file_number: usize,
    vulnerability: Vulnerability,
) -> eyre::Result<BTreeSet<LineNumber>> {
    let mut line_numbers: BTreeSet<LineNumber> = BTreeSet::new();

    //Parse the file into a the ast
    let mut source_unit = solang_parser::parse(file_contents, file_number).unwrap().0;

    let locations = match vulnerability {
        Vulnerability::FloatingPragma => floating_pragma_vulnerability(&mut source_unit)?,
        Vulnerability::UnsafeERC20Operation => unsafe_erc20_operation_vulnerability(source_unit),
        Vulnerability::UnprotectedSelfdestruct => {
            unprotected_self_destruct_vulnerability(&mut source_unit)?
        }
        Vulnerability::DivideBeforeMultiply => {
            divide_before_multiply_vulnerability(&mut source_unit)?
        }
    };

    for loc in locations {
        line_numbers.insert(utils::get_line_number(loc.start(), file_contents));
    }

    Ok(line_numbers)
}
