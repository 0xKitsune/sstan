pub mod address_balance;
pub mod address_zero;
pub mod assign_update_array_value;
pub mod bool_equals_bool;
pub mod cache_array_length;
pub mod cache_storage_in_memory;
pub mod constant_variable;
pub mod event_indexing;
pub mod immutable_variable;
pub mod increment_decrement;
pub mod memory_to_calldata;
pub mod multiple_require;
pub mod optimal_comparison;
pub mod pack_storage_variables;
pub mod pack_struct_variables;
pub mod payable_functions;
pub mod private_constant;
pub mod read_storage_in_for_loop;
pub mod safe_math;
pub mod shift_math;
pub mod short_revert_string;
pub mod solidity_keccak256;
pub mod solidity_math;
pub mod sstore;
pub mod string_error;
use super::engine::Outcome;
use crate::engine::EngineError;
use crate::report::Classification;
use crate::report::Identifier;
use crate::report::OutcomeReport;
use crate::report::ReportSectionFragment;
use crate::utils;
use solang_parser::pt::{Loc, SourceUnit};
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;
pub trait OptimizationPattern {
    fn find(source: &mut HashMap<PathBuf, SourceUnit>) -> Result<OptimizationOutcome, EngineError>;
}

#[macro_export]
macro_rules! optimization {
    //@0xOsiris: maybe make gas report link instead of title?
    ($(($name:ident, $identifier_str:literal, $gas_savings:expr, $report_title:expr, $description:expr, $classification:expr)),+ $(,)?) => {


        $(pub struct $name;)+

        #[allow(non_snake_case)]
        #[derive(Debug)]
        pub enum OptimizationTarget {
            $($name,)+
        }

        impl OptimizationTarget{
            pub fn find(
                &self,
                source: &mut HashMap<PathBuf, SourceUnit>,
            ) -> Result<OptimizationOutcome, EngineError> {
                match self {
                    $(
                        OptimizationTarget::$name => $name::find(source),
                    )+
                }

            }


            pub fn all_targets() -> Vec<OptimizationTarget> {
                vec![
                    $(OptimizationTarget::$name,)+
                ]
            }


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

            pub fn is_empty(&self) -> bool {
                match self {
                    $(
                        OptimizationOutcome::$name(outcome) => outcome.is_empty(),
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

            pub fn classification(&self) -> Classification {
                match self {
                    $(
                        OptimizationOutcome::$name(_) => $classification,
                    )+
                }

            }
        }

        impl FromStr for OptimizationTarget {
            type Err = EngineError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(
                        $identifier_str => Ok(OptimizationTarget::$name),
                    )+
                    _ => Err(EngineError::UnrecognizedPattern(s.into())),
                }
            }
        }

        //TODO: simplify this so that it isnt implementing this for every single macro, just have | when you are matching
        impl From<OptimizationOutcome> for Option<ReportSectionFragment> {
            fn from(value: OptimizationOutcome) -> Self {
                match &value {
                    $(
                        OptimizationOutcome::$name(outcome) => {
                            if outcome.is_empty() {
                                return None;
                            }

                            let description = format!("\n {} - Savings: ~{} \n", $description, $gas_savings);

                            let mut report_fragment = ReportSectionFragment::new(
                                $report_title.to_string(),
                                Identifier::new(value.classification(), 0),
                                description,
                                outcome.len(),
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

//TODO: update to calc dynamic gas savings
// Gas saved is an approximation based on the maximum gas that can be saved from the optimization
optimization!(
    (
        PackStorageVariables,
        "pack_storage_variables",
        0,
        "Tightly pack storage variables",
        "When defining storage variables, make sure to declare them in ascending order, according to size. When multiple variables are able to fit into one 256 bit slot, this will save storage size and gas during runtime. For example, if you have a `bool`, `uint256` and a `bool`, instead of defining the variables in the previously mentioned order, defining the two boolean variables first will pack them both into one storage slot since they only take up one byte of storage.",
        Classification::OptimizationHigh
    ),
    (
        PackStructVariables,
        "pack_struct_variables",
        0, //TODO: revisit this gas saving value
        "Pack Structs",
        "When creating structs, make sure that the variables are listed in ascending order by data type. The compiler will pack the variables that can fit into one 32 byte slot. If the variables are not listed in ascending order, the compiler may not pack the data into one slot, causing additional `sload` and `sstore` instructions when reading/storing the struct into the contract's storage.",
        Classification::OptimizationHigh
    ),
    (
        ReadStorageInForLoop,
        "read_storage_in_for_loop",
        0, //TODO: //FIXME:
        "Avoid Reading From Storage in a for loop",
        "", //TODO: //FIXME:
        Classification::OptimizationHigh
    ),
    (
        ConstantVariable,
        "constant_variable",
        2103,
        "Mark storage variables as `constant` if they never change.",
        "State variables can be declared as constant or immutable. In both cases, the variables cannot be modified after the contract has been constructed. For constant variables, the value has to be fixed at compile-time, while for immutable, it can still be assigned at construction time. \n The compiler does not reserve a storage slot for these variables, and every occurrence is inlined by the respective value. \n Compared to regular state variables, the gas costs of constant and immutable variables are much lower. For a constant variable, the expression assigned to it is copied to all the places where it is accessed and also re-evaluated each time. This allows for local optimizations. Immutable variables are evaluated once at construction time and their value is copied to all the places in the code where they are accessed. For these values, 32 bytes are reserved, even if they would fit in fewer bytes. Due to this, constant values can sometimes be cheaper than immutable values.",
        Classification::OptimizationHigh
    ),
    (
        ImmutableVariable,
        "immutable_variable",
        2103,
        "Mark storage variables as `immutable` if they never change after contract initialization.",
        "State variables can be declared as constant or immutable. In both cases, the variables cannot be modified after the contract has been constructed. For constant variables, the value has to be fixed at compile-time, while for immutable, it can still be assigned at construction time. \n The compiler does not reserve a storage slot for these variables, and every occurrence is inlined by the respective value. \n Compared to regular state variables, the gas costs of constant and immutable variables are much lower. For a constant variable, the expression assigned to it is copied to all the places where it is accessed and also re-evaluated each time. This allows for local optimizations. Immutable variables are evaluated once at construction time and their value is copied to all the places in the code where they are accessed. For these values, 32 bytes are reserved, even if they would fit in fewer bytes. Due to this, constant values can sometimes be cheaper than immutable values. \n",
        Classification::OptimizationHigh
    ),
    (
        PrivateConstant,
        "private_constant",
        22,
        "Consider marking constants as private",
        "Consider marking constant variables in storage as private to save gas (unless a constant variable should be easily accessible by another protocol or offchain logic).",
        Classification::OptimizationLow
    ),
    (
        IncrementDecrement,
        "increment_decrement",
        342, //TODO: revisit this with dynamic calculations
        "`unchecked{++i}` instead of `i++` (or use assembly when applicable)",
        "Use `++i` instead of `i++`. This is especially useful in for loops but this optimization can be used anywhere in your code. You can also use `unchecked{++i;}` for even more gas savings but this will not check to see if `i` overflows. For extra safety if you are worried about this, you can add a require statement after the loop checking if `i` is equal to the final incremented value. For best gas savings, use inline assembly, however this limits the functionality you can achieve. For example you cant use Solidity syntax to internally call your own contract within an assembly block and external calls must be done with the `call()` or `delegatecall()` instruction. However when applicable, inline assembly will save much more gas.",
        Classification::OptimizationMedium
    ),
    (
        CacheStorageInMemory,
        "cache_storage_in_memory",
        0,
        "Cache Storage Variables in Memory",
        "", //TODO:
        Classification::OptimizationMedium
    ),
    (
        AddressZero,
        "address_zero",
        6,
        "Use assembly to check for address(0)",
        "", //TODO:
        Classification::OptimizationLow
    ),
    (
        OptimalComparison,
        "optimal_comparison",
        3,
        "Optimal Comparison",
        "When comparing integers, it is cheaper to use strict `>` & `<` operators over `>=` & `<=` operators, even if you must increment or decrement one of the operands. \n Note: before using this technique, it's important to consider whether incrementing/decrementing one of the operators could result in an over/underflow. This optimization is applicable when the optimizer is turned off.",
        Classification::OptimizationLow
    ),
    (
        MemoryToCalldata,
        "memory_to_calldata",
        1716,
        "Use `calldata` instead of `memory` for function arguments that do not get mutated.",
        "Mark data types as `calldata` instead of `memory` where possible. This makes it so that the data is not automatically loaded into memory. If the data passed into the function does not need to be changed (like updating values in an array), it can be passed in as `calldata`. The one exception to this is if the argument must later be passed into another function that takes an argument that specifies `memory` storage.",
        Classification::OptimizationMedium
    ),
    (
        SolidityKeccak256,
        "solidity_keccak256",
        82,
        "Use assembly to hash instead of Solidity",
        "Hashing is a safe operation to perform in assembly, and it is cheaper than Solidity's `keccak256` function.",
        Classification::OptimizationMedium
    ),
    (
        SafeMathPost080,
        "safe_math_post_080",
        45,
        "Don't use SafeMath when using solidity >= 0.8.0",
        "Solidity >= 0.8.0 checks for overflow/underflow by default. Using Safemath when using version >= 0.8.0 is redundant and will incur additional gas costs. Instead of safemath, you can simply use Solidity's built in arithmetic. For further gas savings, you can also use assembly and check for overflow/underflow as seen below.",
        Classification::OptimizationMedium
    ),
    (
        AssignUpdateArrayValue,
        "assign_update_array_value",
        38,
        "`array[index] += amount` is cheaper than `array[index] = array[index] + amount` (or related variants)",
        "When updating a value in an array with arithmetic, using `array[index] += amount` is cheaper than `array[index] = array[index] + amount`. This is because you avoid an additonal `mload` when the array is stored in memory, and an `sload` when the array is stored in storage. This can be applied for any arithmetic operation including `+=`, `-=`,`/=`,`*=`,`^=`,`&=`, `%=`, `<<=`,`>>=`, and `>>>=`. This optimization can be particularly significant if the pattern occurs during a loop.",
        Classification::OptimizationMedium
    ),
    (
        StringError,
        "string_error",
        57,
        "Use custom errors instead of string error messages",
        "Using custom errors will save you gas, and can be used to provide more information about the error.", //TODO: add description
        Classification::OptimizationMedium
    ),
    (
        SolidityMath,
        "solidity_math",
        60,
        "Use assembly for math (add, sub, mul, div)",
        "Use assembly for math instead of Solidity. You can check for overflow/underflow in assembly to ensure safety. If using Solidity versions < 0.8.0 and you are using Safemath, you can gain significant gas savings by using assembly to calculate values and checking for overflow/underflow.",
        Classification::OptimizationLow
    ),
    (
        Sstore,
        "sstore",
        66,
        "Use assembly to write storage values",
        "You can save a fair amount of gas by using assembly to write storage values.",
        Classification::OptimizationLow
    ),
    (
        SafeMathPre080,
        "safe_math_pre_080",
        37,
        "Consider using assembly with overflow/undeflow protection for math (add, sub, mul, div) instead of SafeMath",
        "Consider using assembly for math instead of Solidity. You can check for overflow/underflow in assembly to ensure safety. If using Solidity versions < 0.8.0 and you are using Safemath, you can gain significant gas savings by using assembly to calculate values and checking for overflow/underflow.",
        Classification::OptimizationLow
    ),
    (
        EventIndexing,
        "event_indexing",
        0, //TODO:
        "Event is not properly indexed.",
        "When possible, always include a minimum of 3 indexed event topics to save gas",
        Classification::OptimizationLow
    ),
    (
        ShiftMath,
        "shift_math",
        65,
        "Right shift or Left shift instead of dividing or multiplying by powers of two",
        "Right shift or left shift when possible to save gas.", 
        Classification::OptimizationLow
    ),
    (
        MultipleRequire,
        "multiple_require",
        16,
        "Use multiple require() statments insted of require(expression && expression && ...)",
        "You can safe gas by breaking up a require statement with multiple conditions, into multiple require statements with a single condition.",
        Classification::OptimizationLow
    ),
    (
        PayableFunctions,
        "payable_functions",
        24,
        "Mark functions as payable (with discretion)",
        "You can mark public or external functions as payable to save gas. Functions that are not payable have additional logic to check if there was a value sent with a call, however, making a function payable eliminates this check. This optimization should be carefully considered due to potentially unwanted behavior when a function does not need to accept ether.",
        Classification::OptimizationLow
    ),
    (
        AddressBalance,
        "address_balance",
        15,
        "Use assembly when getting a contract's balance of ETH",
        "You can use `selfbalance()` instead of `address(this).balance` when getting your contract's balance of ETH to save gas. Additionally, you can use `balance(address)` instead of `address.balance()` when getting an external contract's balance of ETH.",
        Classification::OptimizationLow
    ),
    (
        BoolEqualsBool,
        "bool_equals_bool",
        9,
        "Instead of `if (x == bool)`, use `if(x)` or when applicable, use assembly with `iszero(iszero(x))`.",
        "It is redundant to check `if(x == true)` or any form of boolean comparison. You can slightly reduce gas consumption by using `if (x)` instead. When applicable, you can also use assembly to save more gas by using `iszeroiszero(x)` instead of `if (x)` and `iszero(x)` for `if (!x)`",
        Classification::OptimizationLow
    ),
    (
        CacheArrayLength,
        "cache_array_length",
        22,
        "Cache array length during for loop definition.",
        "A typical for loop definition may look like: `for (uint256 i; i < arr.length; i++){}`. Instead of using `array.length`, cache the array length before the loop, and use the cached value to safe gas. This will avoid an `MLOAD` every loop for arrays stored in memory and an `SLOAD` for arrays stored in storage. This can have significant gas savings for arrays with a large length, especially if the array is stored in storage.",
        Classification::OptimizationLow
    ),
    (
        ShortRevertString,
        "short_revert_string",
        3,
        "Short Revert Strings",
        "Keeping revert strings under 32-bytes prevents the string from being stored in more than one memory slot.",
        Classification::OptimizationLow
    )
);
