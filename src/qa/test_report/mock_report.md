
 <details open> 
 <summary> 
 <a name=N-1>[<span style="color: blue;">N-1</span>]</a> <Strong>Constructor should be listed before any other function</Strong> Instances(1) 
 </summary> 
 Description of the qa pattern goes here 

 <span style="color: green;">File: </span> test.sol 10-12 
 
 ```solidity 
 constructor() {owner = address(1)} 
 ``` 
 </details>
 <details open> 
 <summary> 
 <a name=N-5>[<span style="color: blue;">N-5</span>]</a> <Strong>Interface names should start with an I</Strong> Instances(1) 
 </summary> 
 Consider renaming for consistency 

 <span style="color: green;">File: </span> test.sol 3-8 
 
 ```solidity 
 interface Contract0 {} 
 ``` 
 </details>


 <details open> 
 <summary> 
 <a name=N-3>[<span style="color: blue;">N-3</span>]</a> <Strong>Constructor should initialize all variables</Strong> Instances(1) 
 </summary> 
 Description of the qa pattern goes here 

 <span style="color: green;">File: </span> test.sol 5-8 
 
 ```solidity 
 constructor(address _owner) {owner = _owner} 
 ``` 
 </details>

 <details open> 
 <summary> 
 <a name=N-2>[<span style="color: blue;">N-2</span>]</a> <Strong>Private variables should contain a leading underscore</Strong> Instances(3) 
 </summary> 
 Description of the qa pattern goes here 

 <span style="color: green;">File: </span> test.sol 9-12 
 
 ```solidity 
 address private addr4; 
 ```
 <span style="color: green;">File: </span> test.sol 11-12 
 
 ```solidity 
 address internal addr6; 
 ```
 <span style="color: green;">File: </span> test.sol 5-5 
 
 ```solidity 
 address public _addr2; 
 ``` 
 </details>

 <details open> 
 <summary> 
 <a name=N-4>[<span style="color: blue;">N-4</span>]</a> <Strong>Consider importing specific identifiers instead of the whole file</Strong> Instances(1) 
 </summary> 
 This will minimize compiled code size and help with readability 

 <span style="color: green;">File: </span> test.sol 2-3 
 
 ```solidity 
 import "filename.sol"; 
 ``` 
 </details>
 <details open> 
 <summary> 
 <a name=N-7>[<span style="color: blue;">N-7</span>]</a> <Strong>Consider using scientific notation for large multiples of 10</Strong> Instances(1) 
 </summary> 
 For example 100000 can be written as 1e5 

 <span style="color: green;">File: </span> test.sol 12-13 
 
 ```solidity 
 uint256 x = 10000000; 
 ``` 
 </details>


 <details open> 
 <summary> 
 <a name=N-9>[<span style="color: blue;">N-9</span>]</a> <Strong>Storage variables should be named with camel case</Strong> Instances(2) 
 </summary> 
 Consider renaming to follow convention 

 <span style="color: green;">File: </span> test.sol 3-3 
 
 ```solidity 
 address IS_NOT_FINE; 
 ```
 <span style="color: green;">File: </span> test.sol 5-6 
 
 ```solidity 
 address ALSO_IS_BAD; 
 ``` 
 </details>

 <details open> 
 <summary> 
 <a name=N-6>[<span style="color: blue;">N-6</span>]</a> <Strong>Constants & Immutables should be named with screaming snake case</Strong> Instances(2) 
 </summary> 
 Consider renaming to follow convention 

 <span style="color: green;">File: </span> test.sol 5-5 
 
 ```solidity 
 address constant is_bad = address(1); 
 ```
 <span style="color: green;">File: </span> test.sol 5-6 
 
 ```solidity 
 address immutable Is_Bad; 
 ``` 
 </details>
