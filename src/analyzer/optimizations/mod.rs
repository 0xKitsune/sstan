pub mod address_balance;
pub mod address_zero;
pub mod assign_update_array_value;
pub mod bool_equals_bool;
pub mod cache_array_length;
pub mod cache_storage_in_memory;
pub mod constant_variables;
pub mod event_indexing;
pub mod immutable_variables;
pub mod increment_decrement;
pub mod memory_to_calldata;
pub mod multiple_require;
pub mod optimal_comparison;
pub mod pack_storage_variables;
pub mod pack_struct_variables;
pub mod payable_function;
pub mod private_constant;
pub mod public_functions;
pub mod read_storage_in_for_loop;
pub mod safe_math;
pub mod shift_math;
pub mod short_revert_string;
pub mod solidity_keccak256;
pub mod solidity_math;
pub mod sstore;
pub mod string_errors;
mod template;

use std::{
    collections::{BTreeSet, HashMap},
    fs, vec,
};

use self::{
    address_balance::address_balance_optimization,
    address_zero::address_zero_optimization,
    assign_update_array_value::assign_update_array_optimization,
    bool_equals_bool::bool_equals_bool_optimization,
    cache_array_length::cache_array_length_optimization,
    cache_storage_in_memory::cache_storage_in_memory_optimization,
    constant_variables::constant_variable_optimization,
    immutable_variables::immutable_variables_optimization,
    increment_decrement::increment_decrement_optimization,
    memory_to_calldata::memory_to_calldata_optimization,
    multiple_require::multiple_require_optimization,
    optimal_comparison::optimal_comparison_optimization,
    pack_storage_variables::pack_storage_variables_optimization,
    pack_struct_variables::pack_struct_variables_optimization,
    payable_function::payable_function_optimization,
    private_constant::private_constant_optimization,
    public_functions::public_function_optimization,
    read_storage_in_for_loop::read_storage_in_for_loop_optimization,
    safe_math::{safe_math_post_080_optimization, safe_math_pre_080_optimization},
    shift_math::shift_math_optimization,
    short_revert_string::short_revert_string_optimization,
    solidity_keccak256::solidity_keccak256_optimization,
    solidity_math::solidity_math_optimization,
    sstore::sstore_optimization,
    string_errors::string_error_optimization,
};

use super::utils::{self, LineNumber};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Optimization {
    PublicFunctions,
    AddressBalance,
    AddressZero,
    AssignUpdateArrayValue,
    CacheArrayLength,
    ConstantVariables,
    BoolEqualsBool,
    ImmutableVarialbes,
    IncrementDecrement,
    MemoryToCalldata,
    MultipleRequire,
    PackStorageVariables,
    PackStructVariables,
    PayableFunction,
    PrivateConstant,
    SafeMathPre080,
    SafeMathPost080,
    ShiftMath,
    SolidityKeccak256,
    SolidityMath,
    Sstore,
    StringErrors,
    OptimalComparison,
    ShortRevertString,
    CacheStorageInMemory,
    ReadFromStorageInForLoop,
}

pub fn get_all_optimizations() -> Vec<Optimization> {
    vec![
        Optimization::PublicFunctions,
        Optimization::AddressBalance,
        Optimization::AddressZero,
        Optimization::AssignUpdateArrayValue,
        Optimization::CacheArrayLength,
        Optimization::ConstantVariables,
        Optimization::BoolEqualsBool,
        Optimization::ImmutableVarialbes,
        Optimization::IncrementDecrement,
        Optimization::MemoryToCalldata,
        Optimization::MultipleRequire,
        Optimization::PackStorageVariables,
        Optimization::PackStructVariables,
        Optimization::PrivateConstant,
        Optimization::PayableFunction,
        Optimization::SafeMathPre080,
        Optimization::SafeMathPost080,
        Optimization::ShiftMath,
        Optimization::SolidityKeccak256,
        Optimization::SolidityMath,
        Optimization::Sstore,
        Optimization::StringErrors,
        Optimization::OptimalComparison,
        Optimization::ShortRevertString,
        Optimization::CacheStorageInMemory,
        Optimization::ReadFromStorageInForLoop,
    ]
}

pub fn str_to_optimization(opt: &str) -> Optimization {
    match opt.to_lowercase().as_str() {
        "address_balance" => Optimization::AddressBalance,
        "address_zero" => Optimization::AddressZero,
        "assign_update_array_value" => Optimization::AssignUpdateArrayValue,
        "cache_array_length" => Optimization::CacheArrayLength,
        "constant_variables" => Optimization::ConstantVariables,
        "bool_equals_bool" => Optimization::BoolEqualsBool,
        "immutable_variables" => Optimization::ImmutableVarialbes,
        "increment_decrement" => Optimization::IncrementDecrement,
        "memory_to_calldata" => Optimization::MemoryToCalldata,
        "multiple_require" => Optimization::MultipleRequire,
        "pack_storage_variables" => Optimization::PackStorageVariables,
        "pack_struct_variables" => Optimization::PackStructVariables,
        "payable_function" => Optimization::PayableFunction,
        "private_constant" => Optimization::PrivateConstant,
        "safe_math_pre_080" => Optimization::SafeMathPre080,
        "safe_math_post_080" => Optimization::SafeMathPost080,
        "shift_math" => Optimization::ShiftMath,
        "solidity_keccak256" => Optimization::SolidityKeccak256,
        "solidity_math" => Optimization::SolidityMath,
        "sstore" => Optimization::Sstore,
        "string_errors" => Optimization::StringErrors,
        "optimal_comparison" => Optimization::OptimalComparison,
        "short_revert_string" => Optimization::ShortRevertString,
        "cache_storage_in_memory" => Optimization::CacheStorageInMemory,
        "public_functions" => Optimization::PublicFunctions,
        "read_from_storage_in_for_loop" => Optimization::ReadFromStorageInForLoop,
        other => {
            panic!("Unrecgonized optimization: {}", other)
        }
    }
}

pub fn analyze_dir(
    target_dir: &str,
    optimizations: Vec<Optimization>,
) -> eyre::Result<HashMap<Optimization, Vec<(String, BTreeSet<LineNumber>)>>> {
    //Initialize a new hashmap to keep track of all the optimizations across the target dir
    let mut optimization_locations: HashMap<Optimization, Vec<(String, BTreeSet<LineNumber>)>> =
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
            optimization_locations.extend(analyze_dir(
                file_path
                    .as_os_str()
                    .to_str()
                    .expect("Could not get nested dir"),
                optimizations.clone(),
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
                for optimization in &optimizations {
                    let line_numbers = analyze_for_optimization(&file_contents, i, *optimization)?;

                    if !line_numbers.is_empty() {
                        let file_optimizations = optimization_locations
                            .entry(*optimization)
                            .or_insert(vec![]);

                        file_optimizations.push((file_name.clone(), line_numbers));
                    }
                }
            }
        }
    }

    Ok(optimization_locations)
}

pub fn analyze_for_optimization(
    file_contents: &str,
    file_number: usize,
    optimization: Optimization,
) -> eyre::Result<BTreeSet<LineNumber>> {
    let mut line_numbers: BTreeSet<LineNumber> = BTreeSet::new();

    //Parse the file into a the ast
    let mut source_unit = solang_parser::parse(file_contents, file_number).unwrap().0;

    let locations = match optimization {
        Optimization::ReadFromStorageInForLoop => {
            read_storage_in_for_loop_optimization(&mut source_unit)?
        }
        Optimization::CacheStorageInMemory => {
            cache_storage_in_memory_optimization(&mut source_unit)?
        }
        Optimization::PublicFunctions => public_function_optimization(&mut source_unit)?,
        Optimization::AddressBalance => address_balance_optimization(&mut source_unit)?,
        Optimization::AddressZero => address_zero_optimization(&mut source_unit)?,
        Optimization::AssignUpdateArrayValue => assign_update_array_optimization(&mut source_unit)?,
        Optimization::CacheArrayLength => cache_array_length_optimization(&mut source_unit)?,
        Optimization::ConstantVariables => constant_variable_optimization(&mut source_unit)?,
        Optimization::BoolEqualsBool => bool_equals_bool_optimization(&mut source_unit)?,
        Optimization::ImmutableVarialbes => immutable_variables_optimization(&mut source_unit)?,
        Optimization::IncrementDecrement => increment_decrement_optimization(&mut source_unit)?,
        Optimization::MemoryToCalldata => memory_to_calldata_optimization(&mut source_unit)?,
        Optimization::MultipleRequire => multiple_require_optimization(&mut source_unit)?,
        Optimization::PackStorageVariables => {
            pack_storage_variables_optimization(&mut source_unit)?
        }
        Optimization::PackStructVariables => pack_struct_variables_optimization(&mut source_unit)?,
        Optimization::PayableFunction => payable_function_optimization(&mut source_unit)?,
        Optimization::PrivateConstant => private_constant_optimization(&mut source_unit)?,
        Optimization::SafeMathPre080 => safe_math_pre_080_optimization(&mut source_unit)?,
        Optimization::SafeMathPost080 => safe_math_post_080_optimization(&mut source_unit)?,
        Optimization::ShiftMath => shift_math_optimization(&mut source_unit)?,
        Optimization::SolidityKeccak256 => solidity_keccak256_optimization(&mut source_unit)?,
        Optimization::SolidityMath => solidity_math_optimization(&mut source_unit)?,
        Optimization::Sstore => sstore_optimization(&mut source_unit)?,
        Optimization::StringErrors => string_error_optimization(&mut source_unit)?,
        Optimization::OptimalComparison => optimal_comparison_optimization(&mut source_unit)?,
        Optimization::ShortRevertString => short_revert_string_optimization(&mut source_unit)?,
    };

    for loc in locations {
        line_numbers.insert(utils::get_line_number(loc.start(), file_contents));
    }

    Ok(line_numbers)
}
