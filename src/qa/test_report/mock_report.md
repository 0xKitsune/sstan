
 <details open> 
 <summary> 
 <a name=N-4>[N-4]</a> <Strong>Consider importing specific identifiers instead of the whole file</Strong> Instances(1) 
 </summary> 
 This will minimize compiled code size and help with readability 

 File: test.sol 2-3 

 ```solidity 
 import "filename.sol"; 
 ```
</details>
                            

 <details open> 
 <summary> 
 <a name=N-5>[N-5]</a> <Strong>Interface names should start with an I</Strong> Instances(1) 
 </summary> 
 Consider renaming for consistency 

 File: test.sol 4-7 

 ```solidity 
 interface Contract0 {} 
 ```
</details>
                            
 <details open> 
 <summary> 
 <a name=N-1>[N-1]</a> <Strong>Constructor should be listed before any other function</Strong> Instances(1) 
 </summary> 
 Description of the qa pattern goes here 

 File: test.sol 8-8 

 ```solidity 
 constructor() {owner = address(1)} 
 ```
</details>
                            


 <details open> 
 <summary> 
 <a name=N-7>[N-7]</a> <Strong>Consider using scientific notation for large multiples of 10</Strong> Instances(1) 
 </summary> 
 For example 100000 can be written as 1e5 

 File: test.sol 0-0 

 ```solidity 
 uint256 x = 10000000; 
 ```
</details>
                            
 <details open> 
 <summary> 
 <a name=N-3>[N-3]</a> <Strong>Constructor should initialize all variables</Strong> Instances(1) 
 </summary> 
 Description of the qa pattern goes here 

 File: test.sol 5-6 

 ```solidity 
 constructor(address _owner) {owner = _owner} 
 ```
</details>
                            


 <details open> 
 <summary> 
 <a name=N-2>[N-2]</a> <Strong>Private variables should contain a leading underscore</Strong> Instances(3) 
 </summary> 
 Description of the qa pattern goes here 

 File: test.sol 7-8 

 ```solidity 
 address private addr4; 
 ```
 File: test.sol 11-12 

 ```solidity 
 address internal addr6; 
 ```
 File: test.sol 5-5 

 ```solidity 
 address public _addr2; 
 ```
</details>
                            

 <details open> 
 <summary> 
 <a name=N-9>[N-9]</a> <Strong>Storage variables should be named with camel case</Strong> Instances(2) 
 </summary> 
 Consider renaming to follow convention 

 File: test.sol 3-3 

 ```solidity 
 address IS_NOT_FINE; 
 ```
 File: test.sol 5-6 

 ```solidity 
 address ALSO_IS_BAD; 
 ```
</details>
                            

 <details open> 
 <summary> 
 <a name=N-6>[N-6]</a> <Strong>Constants & Immutables should be named with screaming snake case</Strong> Instances(2) 
 </summary> 
 Consider renaming to follow convention 

 File: test.sol 5-5 

 ```solidity 
 address constant is_bad = address(1); 
 ```
 File: test.sol 5-6 

 ```solidity 
 address immutable Is_Bad; 
 ```
</details>
                            
