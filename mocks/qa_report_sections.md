
 <details open> 
 <summary> 
 <h3> Constants & Immutables should be named with screaming snake case - Instances: 2 </h3>
 </summary> 
 &nbsp; Consider renaming to follow convention 

 &nbsp;
 <span style="color: green;">File: </span> constant_immutable.sol 5-5 
 ```solidity 
 address constant is_bad = address(1); 
 ```
 &nbsp;
 <span style="color: green;">File: </span> constant_immutable.sol 6-6 
 ```solidity 
 address immutable Is_Bad; 
 ``` 
 </details>

 <details open> 
 <summary> 
 <h3> Interface names should start with an I - Instances: 1 </h3>
 </summary> 
 &nbsp; Consider renaming for consistency 

 &nbsp;
 <span style="color: green;">File: </span> interface_namespace.sol 4-0 
 ```solidity 
 interface Contract0 {} 
 ``` 
 </details>

 <details open> 
 <summary> 
 <h3> Constructor should initialize all variables - Instances: 1 </h3>
 </summary> 
 &nbsp; Description of the qa pattern goes here 

 &nbsp;
 <span style="color: green;">File: </span> constructor_var_initialization.sol 5-5 
 ```solidity 
 constructor(address _owner) {owner = _owner} 
 ``` 
 </details>

 <details open> 
 <summary> 
 <h3> Consider importing specific identifiers instead of the whole file - Instances: 1 </h3>
 </summary> 
 &nbsp; This will minimize compiled code size and help with readability 

 &nbsp;
 <span style="color: green;">File: </span> import_identifiers.sol 2-2 
 ```solidity 
 import "filename.sol"; 
 ``` 
 </details>

 <details open> 
 <summary> 
 <h3> Consider using scientific notation for large multiples of 10 - Instances: 1 </h3>
 </summary> 
 &nbsp; For example 100000 can be written as 1e5 

 &nbsp;
 <span style="color: green;">File: </span> large_multiples_of_ten.sol 12-12 
 ```solidity 
 uint256 x = 10000000; 
 ``` 
 </details>

 <details open> 
 <summary> 
 <h3> Remove any unused functions - Instances: 1 </h3>
 </summary> 
 &nbsp;  

 &nbsp;
 <span style="color: green;">File: </span> unused_functions.sol 4-4 
 ```solidity 
 function isUnused() internal {} 
 ``` 
 </details>

 <details open> 
 <summary> 
 <h3> Private variables should contain a leading underscore - Instances: 3 </h3>
 </summary> 
 &nbsp; Description of the qa pattern goes here 

 &nbsp;
 <span style="color: green;">File: </span> private_vars.sol 9-9 
 ```solidity 
 address internal addr6; 
 ```
 &nbsp;
 <span style="color: green;">File: </span> private_vars.sol 7-7 
 ```solidity 
 address private addr4; 
 ```
 &nbsp;
 <span style="color: green;">File: </span> private_vars.sol 5-5 
 ```solidity 
 address public _addr2; 
 ``` 
 </details>

 <details open> 
 <summary> 
 <h3> Constructor should be listed before any other function - Instances: 1 </h3>
 </summary> 
 &nbsp; Description of the qa pattern goes here 

 &nbsp;
 <span style="color: green;">File: </span> constructor_order_qa.sol 7-7 
 ```solidity 
 constructor() {owner = address(1)} 
 ``` 
 </details>

 <details open> 
 <summary> 
 <h3> Remove any unused returns - Instances: 1 </h3>
 </summary> 
 &nbsp;  

 &nbsp;
 <span style="color: green;">File: </span> unused_returns.sol 5-5 
 ```solidity 
 function foo() public returns (uint256 x) {uint256 y = 0; return y;} 
 ``` 
 </details>

 <details open> 
 <summary> 
 <h3> Storage variables should be named with camel case - Instances: 2 </h3>
 </summary> 
 &nbsp; Consider renaming to follow convention 

 &nbsp;
 <span style="color: green;">File: </span> storage_var_namespace.sol 4-4 
 ```solidity 
 address IS_NOT_FINE; 
 ```
 &nbsp;
 <span style="color: green;">File: </span> storage_var_namespace.sol 7-7 
 ```solidity 
 address ALSO_IS_BAD; 
 ``` 
 </details>
