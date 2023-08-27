
 <details open> 
 <summary> 
 <Strong>Consider importing specific identifiers instead of the whole file</Strong> Instances(1) 
 </summary> 
 This will minimize compiled code size and help with readability 

 <span style="color: green;">File: </span> test0.sol 2-2 
 ```solidity 
 import "filename.sol"; 
 ``` 
 </details>
 <details open> 
 <summary> 
 <Strong>Interface names should start with an I</Strong> Instances(1) 
 </summary> 
 Consider renaming for consistency 

 <span style="color: green;">File: </span> test0.sol 3-5 
 ```solidity 
 interface Contract0 {} 
 ``` 
 </details>
 <details open> 
 <summary> 
 <Strong>Consider using scientific notation for large multiples of 10</Strong> Instances(1) 
 </summary> 
 For example 100000 can be written as 1e5 

 <span style="color: green;">File: </span> test0.sol 9-10 
 ```solidity 
 uint256 x = 10000000; 
 ``` 
 </details>



 <details open> 
 <summary> 
 <Strong>Constructor should initialize all variables</Strong> Instances(1) 
 </summary> 
 Description of the qa pattern goes here 

 <span style="color: green;">File: </span> test0.sol 5-5 
 ```solidity 
 constructor(address _owner) {owner = _owner} 
 ``` 
 </details>

 <details open> 
 <summary> 
 <Strong>Private variables should contain a leading underscore</Strong> Instances(3) 
 </summary> 
 Description of the qa pattern goes here 

 <span style="color: green;">File: </span> test0.sol 5-5 
 ```solidity 
 address public _addr2; 
 ```

 <span style="color: green;">File: </span> test0.sol 9-9 
 ```solidity 
 address internal addr6; 
 ```

 <span style="color: green;">File: </span> test0.sol 6-6 
 ```solidity 
 address private addr4; 
 ``` 
 </details>

 <details open> 
 <summary> 
 <Strong>Constructor should be listed before any other function</Strong> Instances(1) 
 </summary> 
 Description of the qa pattern goes here 

 <span style="color: green;">File: </span> test0.sol 0-0 
 ```solidity 
 constructor() {owner = address(1)} 
 ``` 
 </details>

 <details open> 
 <summary> 
 <Strong>Constants & Immutables should be named with screaming snake case</Strong> Instances(2) 
 </summary> 
 Consider renaming to follow convention 

 <span style="color: green;">File: </span> test0.sol 5-6 
 ```solidity 
 address constant is_bad = address(1); 
 ```

 <span style="color: green;">File: </span> test0.sol 7-8 
 ```solidity 
 address immutable Is_Bad; 
 ``` 
 </details>

 <details open> 
 <summary> 
 <Strong>Storage variables should be named with camel case</Strong> Instances(2) 
 </summary> 
 Consider renaming to follow convention 

 <span style="color: green;">File: </span> test0.sol 4-4 
 ```solidity 
 address IS_NOT_FINE; 
 ```

 <span style="color: green;">File: </span> test0.sol 7-7 
 ```solidity 
 address ALSO_IS_BAD; 
 ``` 
 </details>
