
<details open>
                                <summary><a name=N-5>[N-5]</a> Interface names should start with an I Instances(1)</summary>
<p>Consider renaming for consistency</p>                       
                                
<h4>
                                                    File: test.sol 3-5: 
<h4>
                                            
```solidity
    interface Contract0 {}
``` 
                                                
</details>
                            

<details open>
                                <summary><a name=N-4>[N-4]</a> Consider importing specific identifiers instead of the whole file Instances(1)</summary>
<p>This will minimize compiled code size and help with readability</p>                       
                                
<h4>
                                                    File: test.sol 2-2: 
<h4>
                                            
```solidity
    import "filename.sol";
``` 
                                                
</details>
                            
<details open>
                                <summary><a name=N-3>[N-3]</a> Constructor should initialize all variables Instances(1)</summary>
<p>Description of the qa pattern goes here</p>                       
                                
<h4>
                                                    File: test.sol 5-5: 
<h4>
                                            
```solidity
    constructor(address _owner) {owner = _owner}
``` 
                                                
</details>
                            

<details open>
                                <summary><a name=N-2>[N-2]</a> Private variables should contain a leading underscore Instances(3)</summary>
<p>Description of the qa pattern goes here</p>                       
                                
<h4>
                                                    File: test.sol 9-9: 
<h4>
                                            
```solidity
    address internal addr6;
``` 
                                                
<h4>
                                                    File: test.sol 6-6: 
<h4>
                                            
```solidity
    address private addr4;
``` 
                                                
<h4>
                                                    File: test.sol 5-5: 
<h4>
                                            
```solidity
    address public _addr2;
``` 
                                                
</details>
                            

<details open>
                                <summary><a name=N-7>[N-7]</a> Consider using scientific notation for large multiples of 10 Instances(1)</summary>
<p>For example 100000 can be written as 1e5</p>                       
                                
<h4>
                                                    File: test.sol 9-10: 
<h4>
                                            
```solidity
    uint256 x = 10000000;
``` 
                                                
</details>
                            


<details open>
                                <summary><a name=N-1>[N-1]</a> Constructor should be listed before any other function Instances(1)</summary>
<p>Description of the qa pattern goes here</p>                       
                                
<h4>
                                                    File: test.sol 6-7: 
<h4>
                                            
```solidity
    constructor() {owner = address(1)}
``` 
                                                
</details>
                            

<details open>
                                <summary><a name=N-6>[N-6]</a> Constants & Immutables should be named with screaming snake case Instances(2)</summary>
<p>Consider renaming to follow convention</p>                       
                                
<h4>
                                                    File: test.sol 5-5: 
<h4>
                                            
```solidity
    address constant is_bad = address(1);
``` 
                                                
<h4>
                                                    File: test.sol 5-6: 
<h4>
                                            
```solidity
    address immutable Is_Bad;
``` 
                                                
</details>
                            
<details open>
                                <summary><a name=N-9>[N-9]</a> Storage variables should be named with camel case Instances(2)</summary>
<p>Consider renaming to follow convention</p>                       
                                
<h4>
                                                    File: test.sol 3-3: 
<h4>
                                            
```solidity
    address IS_NOT_FINE;
``` 
                                                
<h4>
                                                    File: test.sol 5-6: 
<h4>
                                            
```solidity
    address ALSO_IS_BAD;
``` 
                                                
</details>
                            

