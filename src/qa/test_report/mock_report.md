### Private variables should contain a leading underscore  
 Description of the qa pattern goes here
                                                           
 <a name="N-2"></a>[N-2] "Private variables should contain a leading underscore"

*Instances (3)*:

 File: test.sol 5-6: 
 ```solidity
 address public _addr2; 
 ```
 File: test.sol 6-7: 
 ```solidity
 address private addr4; 
 ```
 File: test.sol 11-12: 
 ```solidity
 address internal addr6; 
 ```

### Consider importing specific identifiers instead of the whole file  
 This will minimize compiled code size and help with readability
                                

                                 
 <a name="N-4"></a>[N-4] "Consider importing specific identifiers instead of the whole file"

*Instances (1)*:

 File: test.sol 2-3: 
 ```solidity
 import "filename.sol"; 
 ```


### Interface names should start with an I  
 Consider renaming for consistency
                                

                                 
 <a name="N-5"></a>[N-5] "Interface names should start with an I"

*Instances (1)*:

 File: test.sol 3-3: 
 ```solidity
 Contract0 
 ```


### Consider using scientific notation for large multiples of 10  
 For example 100000 can be written as 1e5
                                

                                 
 <a name="N-7"></a>[N-7] "Consider using scientific notation for large multiples of 10"

*Instances (1)*:

 File: test.sol 12-13: 
 ```solidity
 10000000 
 ```

### Constructor should initialize all variables  
 Description of the qa pattern goes here
                                

                                 
 <a name="N-3"></a>[N-3] "Constructor should initialize all variables"

*Instances (1)*:

 File: test.sol 5-5: 
 ```solidity
 constructor(address _owner) {owner = _owner} 
 ```



### Private variables should contain a leading underscore  
 Description of the qa pattern goes here
                                

                                 
 <a name="N-2"></a>[N-2] "Private variables should contain a leading underscore"

*Instances (3)*:

 File: test.sol 7-9: 
 ```solidity
 address private addr4; 
 ```
 File: test.sol 11-12: 
 ```solidity
 address internal addr6; 
 ```
 File: test.sol 5-5: 
 ```solidity
 address public _addr2; 
 ```


### Interface names should start with an I  
 Consider renaming for consistency
                                

                                 
 <a name="N-5"></a>[N-5] "Interface names should start with an I"

*Instances (1)*:

 File: test.sol 4-4: 
 ```solidity
 Contract0 
 ```


### Constructor should initialize all variables  
 Description of the qa pattern goes here
                                

                                 
 <a name="N-3"></a>[N-3] "Constructor should initialize all variables"

*Instances (1)*:

 File: test.sol 5-8: 
 ```solidity
 constructor(address _owner) {owner = _owner} 
 ```

### Constructor should be listed before any other function  
 Description of the qa pattern goes here
                                

                                 
 <a name="N-1"></a>[N-1] "Constructor should be listed before any other function"

*Instances (1)*:

 File: test.sol 10-12: 
 ```solidity
 constructor() {owner = address(1)} 
 ```


### Consider using scientific notation for large multiples of 10  
 For example 100000 can be written as 1e5
                                

                                 
 <a name="N-7"></a>[N-7] "Consider using scientific notation for large multiples of 10"

*Instances (1)*:

 File: test.sol 0-0: 
 ```solidity
 10000000 
 ```



### Private variables should contain a leading underscore  
 Description of the qa pattern goes here
                                

                                 
 <a name="N-2"></a>[N-2] "Private variables should contain a leading underscore"

*Instances (3)*:

 File: test.sol 9-12: 
 ```solidity
 address private addr4; 
 ```
 File: test.sol 11-12: 
 ```solidity
 address internal addr6; 
 ```
 File: test.sol 5-5: 
 ```solidity
 address public _addr2; 
 ```


### Consider importing specific identifiers instead of the whole file  
 This will minimize compiled code size and help with readability
                                

                                 
 <a name="N-4"></a>[N-4] "Consider importing specific identifiers instead of the whole file"

*Instances (1)*:

 File: test.sol 2-3: 
 ```solidity
 import "filename.sol"; 
 ```


### Storage variables should be named with camel case  
 Consider renaming to follow convention
                                

                                 
 <a name="N-9"></a>[N-9] "Storage variables should be named with camel case"

*Instances (2)*:

 File: test.sol 3-3: 
 ```solidity
 address IS_NOT_FINE; 
 ```
 File: test.sol 5-6: 
 ```solidity
 address ALSO_IS_BAD; 
 ```


### Constants & Immutables should be named with screaming snake case  
 Consider renaming to follow convention
                                

                                 
 <a name="N-6"></a>[N-6] "Constants & Immutables should be named with screaming snake case"

*Instances (2)*:

 File: test.sol 5-5: 
 ```solidity
 address constant is_bad = address(1); 
 ```
 File: test.sol 5-6: 
 ```solidity
 address immutable Is_Bad; 
 ```


### Consider using scientific notation for large multiples of 10  
 For example 100000 can be written as 1e5
                                

                                 
 <a name="N-7"></a>[N-7] "Consider using scientific notation for large multiples of 10"

*Instances (1)*:

 File: test.sol 8-8: 
 ```solidity
 10000000 
 ```

### Constructor should initialize all variables  
 Description of the qa pattern goes here
                                

                                 
 <a name="N-3"></a>[N-3] "Constructor should initialize all variables"

*Instances (1)*:

 File: test.sol 5-5: 
 ```solidity
 constructor(address _owner) {owner = _owner} 
 ```

### Consider importing specific identifiers instead of the whole file  
 This will minimize compiled code size and help with readability
                                

                                 
 <a name="N-4"></a>[N-4] "Consider importing specific identifiers instead of the whole file"

*Instances (1)*:

 File: test.sol 2-4: 
 ```solidity
 import "filename.sol"; 
 ```

### Constructor should be listed before any other function  
 Description of the qa pattern goes here
                                

                                 
 <a name="N-1"></a>[N-1] "Constructor should be listed before any other function"

*Instances (1)*:

 File: test.sol 6-7: 
 ```solidity
 constructor() {owner = address(1)} 
 ```


### Interface names should start with an I  
 Consider renaming for consistency
                                

                                 
 <a name="N-5"></a>[N-5] "Interface names should start with an I"

*Instances (1)*:

 File: test.sol 4-4: 
 ```solidity
 Contract0 
 ```

### Private variables should contain a leading underscore  
 Description of the qa pattern goes here
                                

                                 
 <a name="N-2"></a>[N-2] "Private variables should contain a leading underscore"

*Instances (3)*:

 File: test.sol 6-7: 
 ```solidity
 address private addr4; 
 ```
 File: test.sol 5-5: 
 ```solidity
 address public _addr2; 
 ```
 File: test.sol 8-8: 
 ```solidity
 address internal addr6; 
 ```






### Constants & Immutables should be named with screaming snake case  
 Consider renaming to follow convention
                                

                                 
 <a name="N-6"></a>[N-6] "Constants & Immutables should be named with screaming snake case"

*Instances (2)*:

 File: test.sol 5-5: 
 ```solidity
 address constant is_bad = address(1); 
 ```
 File: test.sol 5-6: 
 ```solidity
 address immutable Is_Bad; 
 ```


### Storage variables should be named with camel case  
 Consider renaming to follow convention
                                

                                 
 <a name="N-9"></a>[N-9] "Storage variables should be named with camel case"

*Instances (2)*:

 File: test.sol 3-3: 
 ```solidity
 address IS_NOT_FINE; 
 ```
 File: test.sol 5-6: 
 ```solidity
 address ALSO_IS_BAD; 
 ```


