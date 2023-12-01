pub mod blindly_calling_decimals;
pub mod divide_before_multiply;
pub mod double_casting;
pub mod floating_pragma;
pub mod incorrect_shift_math;
pub mod uninitialized_storage_variable;
pub mod unprotected_self_destruct;
pub mod unsafe_erc20_operation;

use super::engine::Outcome;
use crate::engine::EngineError;
use crate::report::Identifier;
use crate::report::{Classification, OutcomeReport, ReportSectionFragment};
use crate::utils;
use solang_parser::pt::{Loc, SourceUnit};
use std::collections::HashMap;
use std::path::PathBuf;

pub trait VulnerabilityPattern {
    fn find(source: &mut HashMap<PathBuf, SourceUnit>)
        -> Result<VulnerabilityOutcome, EngineError>;
}

#[macro_export]
macro_rules! vulnerability {
    ($(($name:ident, $report_title:expr, $description:expr, $classification:expr)),+ $(,)?) => {


        $(pub struct $name;)+

        #[allow(non_snake_case)]
        #[derive(Debug)]
        pub enum VulnerabilityTarget {
            $($name,)+
        }


        impl VulnerabilityTarget{
            pub fn find(
                &self,
                source: &mut HashMap<PathBuf, SourceUnit>,
            ) -> Result<VulnerabilityOutcome, EngineError> {
                match self {
                    $(
                        VulnerabilityTarget::$name => $name::find(source),
                    )+
                }

            }


            pub fn all_targets() -> Vec<VulnerabilityTarget> {
                vec![
                    $(VulnerabilityTarget::$name,)+
                ]
            }

        }


        #[derive(Debug)]
        pub enum VulnerabilityOutcome {
            $($name(Outcome),)+
        }


        impl VulnerabilityOutcome {
            pub fn len(&self) -> usize {
                match self {
                    $(
                        VulnerabilityOutcome::$name(outcome) => outcome.iter().map(|(_, v)| v.len()).sum(),
                    )+
                }
            }

            pub fn is_empty(&self) -> bool {
                self.len() == 0
            }

            pub fn classification(&self) -> Classification {
                match self {
                    $(
                        VulnerabilityOutcome::$name(_) => $classification,
                    )+
                }
            }
        }

        impl From<VulnerabilityOutcome> for Option<ReportSectionFragment> {
            fn from(value: VulnerabilityOutcome) -> Self {
                match &value {
                    $(
                        VulnerabilityOutcome::$name(outcome) => {
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

        //TODO: into tablefragment, propagate an eror if identifier nonce is not populated?
    };



}

//TODO: in the future maybe make this generated by gpt4
//TODO: adjust these classifications
vulnerability!(
    (
        UnprotectedSelfDestruct,
        "Unprotected selfdestruct",
        "
> Unprotected call to a function executing `selfdestruct` or `suicide`.

<details>
<summary>Expand Example</summary>

```js
#### Exploit scenario

```js
contract Suicidal {
    function kill() public {
        selfdestruct(msg.sender);
    }
}
```

Anyone can call kill() and destroy the contract.
        
> Recommendations
 > Protect access to all affected functions. Consider one of the following solutions:
 > 1. Restrict the visibility of the function to `internal` or `private`. 
 > 2. If the function must be public, either:
  >  2.1. Add a modifier to allow only shortlisted EOAs to call this function (such as `onlyOwner`).
  >  2.2. Add a check on the `msg.sender` directly inside the affected function.

```js
// restrict visibility to internal or private
function kill() internal {
    selfdestruct(msg.sender);
}

// add a modifier to allow only shortlisted EOAs to call this function
function kill() public onlyOwner {
    selfdestruct(msg.sender);
}

// add a check on the msg.sender directly inside the affected function
function kill() public {
    require(msg.sender == owner);
    selfdestruct(msg.sender);
}
</details>
```
        ",
        Classification::VulnerabilityHigh
    ),
    (
        UninitializedStorageVariable,
        "Uninitialized storage variables",
        "
> A storage variable that is declared but not initialized will have a default value of zero (or the equivalent, such as an empty array for array types or zero-address for address types). Failing to initialize a storage variable can pose risks if the contract logic assumes that the variable has been explicitly set to a particular value.",
        Classification::VulnerabilityHigh
    ),
    (
        DivideBeforeMultiply,
        "Division before multiplication",
        "
> Consider ordering multiplication before division to avoid loss of precision because integer division might truncate. Loss of precision in Solidity can lead to vulnerabilities because it can result in unexpected behavior in smart contracts. This can be particularly problematic in financial applications, where even small errors in calculations can have significant consequences. For example, if a contract uses integer division to calculate a result and the division operation truncates the fractional part of the result, it could lead to incorrect pricing or loss of funds due to miscalculated balances.
<details>
<summary>Expand Example</summary>

#### Unsafe Division

```js
    n = 5 / 2 * 4; // n = 8 because 5 / 2 == 2 since division truncates.
```

#### Safe Division
```js
    n = 5 * 4 / 2; // n = 10
```

</details>
        ",
        Classification::VulnerabilityMedium
    ),
    (
        IncorrectShiftMath, // TODO: we should change this to be more descriptive since its only in assembly blocks, something like IncorrectShiftOperation
        "Incorrect order of operations when using `shl` or `shr` in an assembly block",
        "

> Incorrect assembly shift math

<details>
<summary>Expand Example</summary>

#### Ex. x << 5.
#### Incorrect
        
```js
    assembly {
        shl(x, 5)
    }
```
       
#### Correct
        
```js
    assembly {
        shl(5,x)
    }
```
</details>  
",
        Classification::VulnerabilityMedium
    ),
    (
        FloatingPragma,
        "Use a locked pragma version instead of a floating pragma version",
        "
> Floating pragma is a vulnerability in smart contract code that can cause unexpected behavior by allowing the compiler to use a specified range of versions. This can lead to issues such as using an older compiler version with known vulnerabilities, using a newer compiler version with undiscovered vulnerabilities, inconsistency across files using different versions, or unpredictable behavior because the compiler can use any version within the specified range. It is recommended to use a locked pragma version in order to avoid these potential vulnerabilities. In some cases it may be acceptable to use a floating pragma, such as when a contract is intended for consumption by other developers and needs to be compatible with a range of compiler versions.
<details>
<summary>Expand Example</summary>

#### Bad

```js
    pragma solidity ^0.8.0;
```

#### Good

```js
    pragma solidity 0.8.15;
```
</details>
",
        Classification::VulnerabilityLow
    ),
    (
        UnsafeErc20Operation,
        "Unsafe ERC20 Operation",
        "
> ERC20 operations can be unsafe due to different implementations and vulnerabilities in the standard. To account for this, either use OpenZeppelin's SafeERC20 library or wrap each operation in a require statement.
> Additionally, ERC20's approve functions have a known race-condition vulnerability. To account for this, use OpenZeppelin's SafeERC20 library's `safeIncrease` or `safeDecrease` Allowance functions.
<details>
<summary>Expand Example</summary>

#### Unsafe Transfer

```js
IERC20(token).transfer(msg.sender, amount);
```

#### OpenZeppelin SafeTransfer

```js
import {SafeERC20} from \"openzeppelin/token/utils/SafeERC20.sol\";
//--snip--

IERC20(token).safeTransfer(msg.sender, address(this), amount);
```
        
#### Safe Transfer with require statement.

```js
bool success = IERC20(token).transfer(msg.sender, amount);
require(success, \"ERC20 transfer failed\");
```
        
#### Unsafe TransferFrom

```js
IERC20(token).transferFrom(msg.sender, address(this), amount);
```

#### OpenZeppelin SafeTransferFrom

```js
import {SafeERC20} from \"openzeppelin/token/utils/SafeERC20.sol\";
//--snip--

IERC20(token).safeTransferFrom(msg.sender, address(this), amount);
```
        
#### Safe TransferFrom with require statement.

```js
bool success = IERC20(token).transferFrom(msg.sender, address(this), amount);
require(success, \"ERC20 transfer failed\");
```

</details>
        
        ",
        Classification::VulnerabilityLow
    ),
    (
        DoubleCasting,
        "Double Casting",
        "
> Avoid double casting as it may introduce unexpected truncations/rounding errors among other issues.
        ",
        Classification::VulnerabilityLow
    ),
//     (
//         VulnerableLibraries,
//         "This project is using a vulnerable version of the openzeppelin libraries",
//         "
// > Mitigation Steps: Update to the latest version of the openzeppelin libraries. Ref: https://security.snyk.io/package/npm/@openzeppelin%2Fcontracts
//         ",
//         Classification::VulnerabilityLow
//     )
(
    BlindDecimalsCall,
    "`decimals()` is not part of the ERC20 standard",
    "
> `decimals()` is not part of the ERC20 standard, and as such there exists valid ERC20 tokens that do not conform to this interface. Therefore it is unsafe to assume that all ERC20 tokens have a `decimals()` function.
    ",
    Classification::VulnerabilityLow
),
);
