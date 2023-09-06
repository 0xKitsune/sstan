
 <details open> 
 <summary> 
 <Strong>Consider importing specific identifiers instead of the whole file</Strong> Instances(1) 
 </summary> 
 This will minimize compiled code size and help with readability 

 <span style="color: green;">File: </span> import_identifiers.sol 2-2 
 ```solidity 
 import "filename.sol"; 
 ``` 
 </details>

 <details open> 
 <summary> 
 <Strong>Interface names should start with an I</Strong> Instances(1) 
 </summary> 
 Consider renaming for consistency 

 <span style="color: green;">File: </span> interface_namespace.sol 4-0 
 ```solidity 
 interface Contract0 {} 
 ``` 
 </details>
 <details open> 
 <summary> 
 <Strong>Consider using scientific notation for large multiples of 10</Strong> Instances(1) 
 </summary> 
 For example 100000 can be written as 1e5 

 <span style="color: green;">File: </span> large_multiples_of_ten.sol 12-12 
 ```solidity 
 uint256 x = 10000000; 
 ``` 
 </details>

 <details open> 
 <summary> 
 <Strong>Constructor should be listed before any other function</Strong> Instances(1) 
 </summary> 
 Description of the qa pattern goes here 

 <span style="color: green;">File: </span> constructor_order_qa.sol 7-7 
 ```solidity 
 constructor() {owner = address(1)} 
 ``` 
 </details>


 <details open> 
 <summary> 
 <Strong>Constructor should initialize all variables</Strong> Instances(1) 
 </summary> 
 Description of the qa pattern goes here 

 <span style="color: green;">File: </span> constructor_var_initialization.sol 5-5 
 ```solidity 
 constructor(address _owner) {owner = _owner} 
 ``` 
 </details>

 <details open> 
 <summary> 
 <Strong>Private variables should contain a leading underscore</Strong> Instances(3) 
 </summary> 
 Description of the qa pattern goes here 

 <span style="color: green;">File: </span> private_vars.sol 7-7 
 ```solidity 
 address private addr4; 
 ```

 <span style="color: green;">File: </span> private_vars.sol 9-9 
 ```solidity 
 address internal addr6; 
 ```

 <span style="color: green;">File: </span> private_vars.sol 5-5 
 ```solidity 
 address public _addr2; 
 ``` 
 </details>

 <details open> 
 <summary> 
 <Strong>Remove any unused functions</Strong> Instances(1) 
 </summary> 
  

 <span style="color: green;">File: </span> unused_functions.sol 4-4 
 ```solidity 
 function isUnused() internal {} 
 ``` 
 </details>

 <details open> 
 <summary> 
 <Strong>Storage variables should be named with camel case</Strong> Instances(2) 
 </summary> 
 Consider renaming to follow convention 

 <span style="color: green;">File: </span> storage_var_namespace.sol 4-4 
 ```solidity 
 address IS_NOT_FINE; 
 ```

 <span style="color: green;">File: </span> storage_var_namespace.sol 7-7 
 ```solidity 
 address ALSO_IS_BAD; 
 ``` 
 </details>

 <details open> 
 <summary> 
 <Strong>Remove any unused returns</Strong> Instances(1) 
 </summary> 
  

 <span style="color: green;">File: </span> unused_returns.sol 5-5 
 ```solidity 
 function foo() public returns (uint256 x) {uint256 y = 0; return y;} 
 ``` 
 </details>

 <details open> 
 <summary> 
 <Strong>Constants & Immutables should be named with screaming snake case</Strong> Instances(2) 
 </summary> 
 Consider renaming to follow convention 

 <span style="color: green;">File: </span> constant_immutable.sol 5-5 
 ```solidity 
 address constant is_bad = address(1); 
 ```

 <span style="color: green;">File: </span> constant_immutable.sol 6-6 
 ```solidity 
 address immutable Is_Bad; 
 ``` 
 </details>

 <details open> 
 <summary> 
 <Strong>title goes here</Strong> Instances(22) 
 </summary> 
 description goes here 

 <span style="color: green;">File: </span> divde_before_multiply.sol 29-29 
 ```solidity 
 x /= 2 * 3 
 ```

 <span style="color: green;">File: </span> divde_before_multiply.sol 31-31 
 ```solidity 
 x /= (2 * 3) 
 ```

 <span style="color: green;">File: </span> divde_before_multiply.sol 32-32 
 ```solidity 
 x /= (1 / 2) * 3 
 ```

 <span style="color: green;">File: </span> divde_before_multiply.sol 33-33 
 ```solidity 
 x /= (1 * 2) * 3 
 ```

 <span style="color: green;">File: </span> divde_before_multiply.sol 34-34 
 ```solidity 
 x /= (2 * 3) / 4 
 ```

 <span style="color: green;">File: </span> divde_before_multiply.sol 35-35 
 ```solidity 
 x /= 2 * 3 / 4 
 ```

 <span style="color: green;">File: </span> divde_before_multiply.sol 36-36 
 ```solidity 
 x /= 2 * 3 - 4 
 ```

 <span style="color: green;">File: </span> divde_before_multiply.sol 37-37 
 ```solidity 
 x /= 2 * 3 % 4 
 ```

 <span style="color: green;">File: </span> divde_before_multiply.sol 38-38 
 ```solidity 
 x /= 2 * 3 | 4 
 ```

 <span style="color: green;">File: </span> divde_before_multiply.sol 39-39 
 ```solidity 
 x /= 2 * 3 & 4 
 ```

 <span style="color: green;">File: </span> divde_before_multiply.sol 40-40 
 ```solidity 
 x /= 2 * 3 ^ 4 
 ```

 <span style="color: green;">File: </span> divde_before_multiply.sol 41-41 
 ```solidity 
 x /= 2 * 3 << 4 
 ```

 <span style="color: green;">File: </span> divde_before_multiply.sol 42-42 
 ```solidity 
 x /= 2 * 3 >> 4 
 ```

 <span style="color: green;">File: </span> divde_before_multiply.sol 6-6 
 ```solidity 
 1 / 2 * 3 
 ```

 <span style="color: green;">File: </span> divde_before_multiply.sol 8-8 
 ```solidity 
 (1 / 2) * 3 
 ```

 <span style="color: green;">File: </span> divde_before_multiply.sol 10-10 
 ```solidity 
 (1 / 2 * 3) * 4 
 ```

 <span style="color: green;">File: </span> divde_before_multiply.sol 10-10 
 ```solidity 
 1 / 2 * 3 
 ```

 <span style="color: green;">File: </span> divde_before_multiply.sol 11-11 
 ```solidity 
 (1 * 2 / 3) * 4 
 ```

 <span style="color: green;">File: </span> divde_before_multiply.sol 12-12 
 ```solidity 
 (1 / 2 / 3) * 4 
 ```

 <span style="color: green;">File: </span> divde_before_multiply.sol 13-13 
 ```solidity 
 1 / (2 + 3) * 4 
 ```

 <span style="color: green;">File: </span> divde_before_multiply.sol 25-25 
 ```solidity 
 (2 / 3) * 3 
 ```

 <span style="color: green;">File: </span> divde_before_multiply.sol 32-32 
 ```solidity 
 (1 / 2) * 3 
 ``` 
 </details>
