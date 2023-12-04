&nbsp;
## 👍 Identified QA
Below are the currently identified QA that sstan identifies. If you would like to check out a list of patterns that are ready to be implemented and you would like to add them to the repo, you can check out the [Contribution.md](https://github.com/0xKitsune/sstan/blob/main/docs/Contributing.md#potential-optimizations-vulnerability-and-qa-additions)!

| Quality Assurance         | Description                                             |
| ------------------------- | ------------------------------------------------------- |
| constructor_order         | Constructor must be placed before any other function |
| private_func_leading_underscore | Use leading underscore for private functions |
| private_vars_leading_underscore | Use leading underscore for private variables |
| constant_immutable_namespace | Constant and immutable variable names should be in SCREAMING_SNAKE_CASE |
| constructor_var_initialization | Constructor should check that all parameters are not 0 |
| contract_name_pascal_case | Contract names should be in PascalCase |
| contracts_should_inherit_interface | Large contracts with many external functions should inherit an interface |
| error_without_parameters | This error has no parameters, the state of the contract when the revert occured will not be available |
| event_name_pascalcase | Event names should be in PascalCase |
| explicit_visibility | Storage variables should not have implicit visibility |
| function_name_camel_case | Function names should be in camelCase |
| import_identifiers | Consider importing specific identifiers instead of the whole file |
| inconsistent_require_error | Require/Revert statements should be consistent across the codebase |
| interface_namespace | Interface names should start with an I |
| large_multiples_of_ten | Consider using scientific notation for large multiples of 10 |
| missing_underscores_for_large_numeric_literals | Misssing underscores on large numeric literals |
| one_contract_per_file | Only define one contract per file |
| private_vars_leading_underscore | Private variables should contain a leading underscore |
| public_functions | Consider marking public function External |
| remove_console | Remove console.log statements |
| require_without_message | Consider adding a message with require and revert statements |
| storage_variable_namespace | Storage variables should be named with camel case |
| unused_functions | Remove any unused functions |
| unused_returns | Remove any unused returns |
| variable_initialized_with_default | This variables default value is the same as the value it is initialized with |







