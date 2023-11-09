# Sstan - v0.1.0 

 --- 
 ## Authors: 0x00face, 0xOsiris 
 --- 
 TODO: add description

# Summary




## Vulnerabilities 

 | Classification | Title | Instances | 
 |:-------:|:---------|:-------:| 
 | [[L-0]](#[L-0]) | Double Casting | 2 |
## Optimizations 

 | Classification | Title | Instances | 
 |:-------:|:---------|:-------:| 
 | [[G-0]](#[G-0]) | `unchecked{++i}` instead of `i++` (or use assembly when applicable) | 2 |
 | [[G-1]](#[G-1]) | Cache Storage Variables in Memory | 3 |
 | [[G-2]](#[G-2]) | Use `calldata` instead of `memory` for function arguments that do not get mutated. | 10 |
 | [[G-3]](#[G-3]) | Use assembly to hash instead of Solidity | 4 |
 | [[G-4]](#[G-4]) | Use custom errors instead of string error messages | 2 |
 | [[G-5]](#[G-5]) | Use assembly for math (add, sub, mul, div) | 2 |
 | [[G-6]](#[G-6]) | Use assembly to write storage values | 1 |
 | [[G-7]](#[G-7]) | Event is not properly indexed. | 3 |
 | [[G-8]](#[G-8]) | Use multiple require() statments insted of require(expression && expression && ...) | 1 |
 | [[G-9]](#[G-9]) | Mark functions as payable (with discretion) | 14 |
 | [[G-10]](#[G-10]) | Consider marking constants as private | 7 |
 | [[G-11]](#[G-11]) | Use assembly to check for address(0) | 5 |
## Quality Assurance 

 | Classification | Title | Instances | 
 |:-------:|:---------|:-------:| 
 | [[NC-0]](#[NC-0]) | Constructor should check that all parameters are not 0 | 13 |
 | [[NC-1]](#[NC-1]) | Consider importing specific identifiers instead of the whole file | 3 |
 | [[NC-2]](#[NC-2]) | Remove any unused functions | 12 |
 | [[NC-3]](#[NC-3]) | Function names should be in camelCase | 31 |
 | [[NC-4]](#[NC-4]) | Constant and immutable variable names should be in SCREAMING_SNAKE_CASE | 32 |
 | [[NC-5]](#[NC-5]) | Remove any unused returns | 5 |
 | [[NC-6]](#[NC-6]) | Consider marking public function External | 4 |
 | [[NC-7]](#[NC-7]) | This error has no parameters, the state of the contract when the revert occured will not be available | 30 |
 | [[NC-8]](#[NC-8]) | This variables default value is the same as the value it is initialized with | 1 |

## Vulnerabilities - Total: 2 

<a name=[L-0]></a>
### [L-0] Double Casting - Instances: 2 

 
> Avoid double casting as it may introduce unexpected truncations/rounding errors among other issues.
         

 --- 

[File:SafeHelper.sol#L144](https://github.com/code-423n4/2023-10-brahma/blob/main/src/libraries/SafeHelper.sol#L144) 
```solidity
143:        return address(uint160(uint256(bytes32(guardAddress))));
``` 



[File:SafeHelper.sol#L154](https://github.com/code-423n4/2023-10-brahma/blob/main/src/libraries/SafeHelper.sol#L154) 
```solidity
153:        return address(uint160(uint256(bytes32(fallbackHandlerAddress))));
``` 



 --- 



## Optimizations - Total: 54 

<a name=[G-0]></a>
### [G-0] `unchecked{++i}` instead of `i++` (or use assembly when applicable) - Instances: 2 

 
 
> Use `++i` instead of `i++`. This is especially useful in for loops but this optimization can be used anywhere in your code. You can also use `unchecked{++i;}` for even more gas savings but this will not check to see if `i` overflows. For extra safety if you are worried about this, you can add a require statement after the loop checking if `i` is equal to the final incremented value. For best gas savings, use inline assembly, however this limits the functionality you can achieve. For example you cant use Solidity syntax to internally call your own contract within an assembly block and external calls must be done with the `call()` or `delegatecall()` instruction. However when applicable, inline assembly will save much more gas. 
 
#### Gas Report - Savings: ~342 
 <details>  
 <summary>  
  </summary> 
 
```solidity

contract GasTest is DSTest {
    Contract0 c0;
    Contract1 c1;
    Contract2 c2;
    Contract3 c3;
    Contract4 c4;

    function setUp() public {
        c0 = new Contract0();
        c1 = new Contract1();
        c2 = new Contract2();
        c3 = new Contract3();
        c4 = new Contract4();
    }

    function testGas() public {
        c0.iPlusPlus();
        c1.plusPlusI();
        c2.uncheckedPlusPlusI();
        c3.safeUncheckedPlusPlusI();
        c4.inlineAssemblyLoop();
    }
}

contract Contract0 {
    //loop with i++
    function iPlusPlus() public pure {
        uint256 j = 0;
        for (uint256 i; i < 10; i++) {
            j++;
        }
    }
}

contract Contract1 {
    //loop with ++i
    function plusPlusI() public pure {
        uint256 j = 0;
        for (uint256 i; i < 10; ++i) {
            j++;
        }
    }
}

contract Contract2 {
    //loop with unchecked{++i}
    function uncheckedPlusPlusI() public pure {
        uint256 j = 0;
        for (uint256 i; i < 10; ) {
            j++;

            unchecked {
                ++i;
            }
        }
    }
}

contract Contract3 {
    //loop with unchecked{++i} with additional overflow check
    function safeUncheckedPlusPlusI() public pure {
        uint256 j = 0;
        uint256 i = 0;
        for (i; i < 10; ) {
            j++;

            unchecked {
                ++i;
            }
        }

        //check for overflow
        assembly {
            if lt(i, 10) {
                mstore(0x00, "loop overflow")
                revert(0x00, 0x20)
            }
        }
    }
}

contract Contract4 {
    //loop with inline assembly
    function inlineAssemblyLoop() public pure {
        assembly {
            let j := 0

            for {
                let i := 0
            } lt(i, 10) {
                i := add(i, 0x01)
            } {
                j := add(j, 0x01)
            }
        }
    }
}

```


```solidity

╭────────────────────┬─────────────────┬──────┬────────┬──────┬─────────╮
│ Contract0 contract ┆                 ┆      ┆        ┆      ┆         │
╞════════════════════╪═════════════════╪══════╪════════╪══════╪═════════╡
│ Deployment Cost    ┆ Deployment Size ┆      ┆        ┆      ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 37687              ┆ 219             ┆      ┆        ┆      ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name      ┆ min             ┆ avg  ┆ median ┆ max  ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ iPlusPlus          ┆ 2039            ┆ 2039 ┆ 2039   ┆ 2039 ┆ 1       │
╰────────────────────┴─────────────────┴──────┴────────┴──────┴─────────╯
╭────────────────────┬─────────────────┬──────┬────────┬──────┬─────────╮
│ Contract1 contract ┆                 ┆      ┆        ┆      ┆         │
╞════════════════════╪═════════════════╪══════╪════════╪══════╪═════════╡
│ Deployment Cost    ┆ Deployment Size ┆      ┆        ┆      ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 37287              ┆ 217             ┆      ┆        ┆      ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name      ┆ min             ┆ avg  ┆ median ┆ max  ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ plusPlusI          ┆ 1989            ┆ 1989 ┆ 1989   ┆ 1989 ┆ 1       │
╰────────────────────┴─────────────────┴──────┴────────┴──────┴─────────╯
╭────────────────────────┬─────────────────┬──────┬────────┬──────┬─────────╮
│ Contract3 contract     ┆                 ┆      ┆        ┆      ┆         │
╞════════════════════════╪═════════════════╪══════╪════════╪══════╪═════════╡
│ Deployment Cost        ┆ Deployment Size ┆      ┆        ┆      ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 42693                  ┆ 244             ┆      ┆        ┆      ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name          ┆ min             ┆ avg  ┆ median ┆ max  ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ safeUncheckedPlusPlusI ┆ 1355            ┆ 1355 ┆ 1355   ┆ 1355 ┆ 1       │
╰────────────────────────┴─────────────────┴──────┴────────┴──────┴─────────╯
╭────────────────────┬─────────────────┬──────┬────────┬──────┬─────────╮
│ Contract2 contract ┆                 ┆      ┆        ┆      ┆         │
╞════════════════════╪═════════════════╪══════╪════════╪══════╪═════════╡
│ Deployment Cost    ┆ Deployment Size ┆      ┆        ┆      ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 35887              ┆ 210             ┆      ┆        ┆      ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name      ┆ min             ┆ avg  ┆ median ┆ max  ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ uncheckedPlusPlusI ┆ 1329            ┆ 1329 ┆ 1329   ┆ 1329 ┆ 1       │
╰────────────────────┴─────────────────┴──────┴────────┴──────┴─────────╯
╭────────────────────┬─────────────────┬─────┬────────┬─────┬─────────╮
│ Contract4 contract ┆                 ┆     ┆        ┆     ┆         │
╞════════════════════╪═════════════════╪═════╪════════╪═════╪═════════╡
│ Deployment Cost    ┆ Deployment Size ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 26881              ┆ 164             ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name      ┆ min             ┆ avg ┆ median ┆ max ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ inlineAssemblyLoop ┆ 709             ┆ 709 ┆ 709    ┆ 709 ┆ 1       │
╰────────────────────┴─────────────────┴─────┴────────┴─────┴─────────╯

```
 
 </details> 
 

 --- 

[File:SafeDeployer.sol#L254](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeDeployer.sol#L254) 
```solidity
253:        return uint256(keccak256(abi.encodePacked(_ownersHash, ownerSafeCount[_ownersHash]++, _salt, VERSION)));
``` 



[File:ExecutorPlugin.sol#L131](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ExecutorPlugin.sol#L131) 
```solidity
130:                nonce: executorNonce[execRequest.account][execRequest.executor]++
131:            })
``` 



 --- 

<a name=[G-1]></a>
### [G-1] Cache Storage Variables in Memory - Instances: 3 

 
  
 Cache Array Length - Gas Report - Savings: ~0 
 <details>  
 <summary>  
  </summary> 
  
 </details> 
 

 --- 

[File:AddressProvider.sol#L67](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/AddressProvider.sol#L67) 
```solidity
66:        governance = msg.sender;
``` 



[File:AddressProvider.sol#L68](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/AddressProvider.sol#L68) 
```solidity
67:        delete pendingGovernance;
``` 



[File:AddressProvider.sol#L102](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/AddressProvider.sol#L102) 
```solidity
101:        registries[_key] = _registry;
``` 



[File:WalletRegistry.sol#L38](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/registries/WalletRegistry.sol#L38) 
```solidity
37:        isWallet[msg.sender] = true;
``` 



[File:WalletRegistry.sol#L52](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/registries/WalletRegistry.sol#L52) 
```solidity
51:        subAccountToWallet[_subAccount] = _wallet;
``` 



[File:PolicyRegistry.sol#L68](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/registries/PolicyRegistry.sol#L68) 
```solidity
67:        commitments[account] = policyCommit;
``` 



 --- 

<a name=[G-2]></a>
### [G-2] Use `calldata` instead of `memory` for function arguments that do not get mutated. - Instances: 10 

 
 
> Mark data types as `calldata` instead of `memory` where possible. This makes it so that the data is not automatically loaded into memory. If the data passed into the function does not need to be changed (like updating values in an array), it can be passed in as `calldata`. The one exception to this is if the argument must later be passed into another function that takes an argument that specifies `memory` storage. 
 
#### Gas Report - Savings: ~1716 
 <details>  
 <summary>  
  </summary> 
 

```solidity

contract GasTest is DSTest {
    Contract0 c0;
    Contract1 c1;
    Contract2 c2;
    Contract3 c3;

    function setUp() public {
        c0 = new Contract0();
        c1 = new Contract1();
        c2 = new Contract2();
        c3 = new Contract3();
    }

    function testGas() public {
        uint256[] memory arr = new uint256[](10);
        c0.calldataArray(arr);
        c1.memoryArray(arr);

        bytes memory data = abi.encode("someText");
        c2.calldataBytes(data);
        c3.memoryBytes(data);
    }
}

contract Contract0 {
    function calldataArray(uint256[] calldata arr) public {
        uint256 j;
        for (uint256 i; i < arr.length; i++) {
            j = arr[i] + 10;
        }
    }
}

contract Contract1 {
    function memoryArray(uint256[] memory arr) public {
        uint256 j;
        for (uint256 i; i < arr.length; i++) {
            j = arr[i] + 10;
        }
    }
}

contract Contract2 {
    function calldataBytes(bytes calldata data) public {
        bytes32 val;
        for (uint256 i; i < 10; i++) {
            val = keccak256(abi.encode(data, i));
        }
    }
}

contract Contract3 {
    function memoryBytes(bytes memory data) public {
        bytes32 val;
        for (uint256 i; i < 10; i++) {
            val = keccak256(abi.encode(data, i));
        }
    }
}
```

### Gas Report
```solidity
╭───────────────────────────────────────────┬─────────────────┬──────┬────────┬──────┬─────────╮
│ src/test/GasTest.t.sol:Contract0 contract ┆                 ┆      ┆        ┆      ┆         │
╞═══════════════════════════════════════════╪═════════════════╪══════╪════════╪══════╪═════════╡
│ Deployment Cost                           ┆ Deployment Size ┆      ┆        ┆      ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 97947                                     ┆ 521             ┆      ┆        ┆      ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name                             ┆ min             ┆ avg  ┆ median ┆ max  ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ calldataArray                             ┆ 2824            ┆ 2824 ┆ 2824   ┆ 2824 ┆ 1       │
╰───────────────────────────────────────────┴─────────────────┴──────┴────────┴──────┴─────────╯
╭───────────────────────────────────────────┬─────────────────┬──────┬────────┬──────┬─────────╮
│ src/test/GasTest.t.sol:Contract1 contract ┆                 ┆      ┆        ┆      ┆         │
╞═══════════════════════════════════════════╪═════════════════╪══════╪════════╪══════╪═════════╡
│ Deployment Cost                           ┆ Deployment Size ┆      ┆        ┆      ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 128171                                    ┆ 672             ┆      ┆        ┆      ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name                             ┆ min             ┆ avg  ┆ median ┆ max  ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ memoryArray                               ┆ 3755            ┆ 3755 ┆ 3755   ┆ 3755 ┆ 1       │
╰───────────────────────────────────────────┴─────────────────┴──────┴────────┴──────┴─────────╯
╭───────────────────────────────────────────┬─────────────────┬──────┬────────┬──────┬─────────╮
│ src/test/GasTest.t.sol:Contract2 contract ┆                 ┆      ┆        ┆      ┆         │
╞═══════════════════════════════════════════╪═════════════════╪══════╪════════╪══════╪═════════╡
│ Deployment Cost                           ┆ Deployment Size ┆      ┆        ┆      ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 100547                                    ┆ 534             ┆      ┆        ┆      ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name                             ┆ min             ┆ avg  ┆ median ┆ max  ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ calldataBytes                             ┆ 4934            ┆ 4934 ┆ 4934   ┆ 4934 ┆ 1       │
╰───────────────────────────────────────────┴─────────────────┴──────┴────────┴──────┴─────────╯
╭───────────────────────────────────────────┬─────────────────┬──────┬────────┬──────┬─────────╮
│ src/test/GasTest.t.sol:Contract3 contract ┆                 ┆      ┆        ┆      ┆         │
╞═══════════════════════════════════════════╪═════════════════╪══════╪════════╪══════╪═════════╡
│ Deployment Cost                           ┆ Deployment Size ┆      ┆        ┆      ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 135183                                    ┆ 707             ┆      ┆        ┆      ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name                             ┆ min             ┆ avg  ┆ median ┆ max  ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ memoryBytes                               ┆ 7551            ┆ 7551 ┆ 7551   ┆ 7551 ┆ 1       │
╰───────────────────────────────────────────┴─────────────────┴──────┴────────┴──────┴─────────╯

```
         
 </details> 
 

 --- 

[File:SafeModeratorOverridable.sol#L42](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeModeratorOverridable.sol#L42) 
```solidity
41:        bytes memory data,
``` 



[File:SafeModeratorOverridable.sol#L49](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeModeratorOverridable.sol#L49) 
```solidity
48:        bytes memory signatures,
``` 



[File:ExecutorPlugin.sol#L86](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ExecutorPlugin.sol#L86) 
```solidity
85:    function _executeTxnAsModule(address _account, Types.Executable memory _executable)
``` 



[File:SafeModerator.sol#L36](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeModerator.sol#L36) 
```solidity
35:        bytes memory data,
``` 



[File:SafeModerator.sol#L43](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeModerator.sol#L43) 
```solidity
42:        bytes memory signatures,
``` 



[File:ConsoleOpBuilder.sol#L95](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ConsoleOpBuilder.sol#L95) 
```solidity
94:    function enableExecutorPluginOnSubAccount(address subAccount, address[] memory executors)
``` 



[File:SafeHelper.sol#L63](https://github.com/code-423n4/2023-10-brahma/blob/main/src/libraries/SafeHelper.sol#L63) 
```solidity
62:    function _executeOnSafe(address safe, address target, Enum.Operation op, bytes memory data) internal {
``` 



[File:SafeHelper.sol#L103](https://github.com/code-423n4/2023-10-brahma/blob/main/src/libraries/SafeHelper.sol#L103) 
```solidity
102:    function _packMultisendTxns(Types.Executable[] memory _txns) internal pure returns (bytes memory packedTxns) {
``` 



[File:ConsoleFallbackHandler.sol#L39](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ConsoleFallbackHandler.sol#L39) 
```solidity
38:    function isValidSignature(bytes memory _data, bytes memory _signature) public view override returns (bytes4) {
``` 



[File:ConsoleFallbackHandler.sol#L39](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ConsoleFallbackHandler.sol#L39) 
```solidity
38:    function isValidSignature(bytes memory _data, bytes memory _signature) public view override returns (bytes4) {
``` 



[File:ConsoleFallbackHandler.sol#L60](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ConsoleFallbackHandler.sol#L60) 
```solidity
59:    function getMessageHash(bytes memory message) public view returns (bytes32) {
``` 



[File:ConsoleFallbackHandler.sol#L68](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ConsoleFallbackHandler.sol#L68) 
```solidity
67:    function getMessageHashForSafe(GnosisSafe safe, bytes memory message) public view returns (bytes32) {
``` 



[File:SafeDeployer.sol#L110](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeDeployer.sol#L110) 
```solidity
109:    function _setupConsoleAccount(address[] memory _owners, uint256 _threshold, bool _policyHashValid)
``` 



[File:SafeDeployer.sol#L168](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeDeployer.sol#L168) 
```solidity
167:    function _setupSubAccount(address[] memory _owners, uint256 _threshold, address _consoleAccount)
``` 



[File:SafeDeployer.sol#L219](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeDeployer.sol#L219) 
```solidity
218:    function _createSafe(address[] calldata _owners, bytes memory _initializer, bytes32 _salt)
``` 



[File:PolicyValidator.sol#L58](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/PolicyValidator.sol#L58) 
```solidity
57:        bytes memory data,
``` 



[File:TransactionValidator.sol#L64](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/TransactionValidator.sol#L64) 
```solidity
63:    function validatePreTransactionOverridable(SafeTransactionParams memory txParams) external view {
``` 



[File:TransactionValidator.sol#L95](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/TransactionValidator.sol#L95) 
```solidity
94:    function validatePreTransaction(SafeTransactionParams memory txParams) external view {
``` 



[File:TransactionValidator.sol#L123](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/TransactionValidator.sol#L123) 
```solidity
122:        bytes memory signatures
123:    ) external view {
``` 



[File:TransactionValidator.sol#L153](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/TransactionValidator.sol#L153) 
```solidity
152:        bytes memory _data,
``` 



[File:TransactionValidator.sol#L214](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/TransactionValidator.sol#L214) 
```solidity
213:        bytes memory _data,
``` 



[File:TransactionValidator.sol#L216](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/TransactionValidator.sol#L216) 
```solidity
215:        bytes memory _signatures
216:    ) internal view {
``` 



[File:TransactionValidator.sol#L234](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/TransactionValidator.sol#L234) 
```solidity
233:    function _validatePolicySignature(address _from, bytes32 _transactionStructHash, bytes memory _signatures)
``` 



[File:TypeHashHelper.sol#L64](https://github.com/code-423n4/2023-10-brahma/blob/main/src/libraries/TypeHashHelper.sol#L64) 
```solidity
63:    function _buildTransactionStructHash(Transaction memory txn) internal pure returns (bytes32) {
``` 



[File:TypeHashHelper.sol#L84](https://github.com/code-423n4/2023-10-brahma/blob/main/src/libraries/TypeHashHelper.sol#L84) 
```solidity
83:    function _buildValidationStructHash(Validation memory validation) internal pure returns (bytes32) {
``` 



 --- 

<a name=[G-3]></a>
### [G-3] Use assembly to hash instead of Solidity - Instances: 4 

 
 
> Hashing is a safe operation to perform in assembly, and it is cheaper than Solidity's `keccak256` function.
         
 
#### Gas Report - Savings: ~82 
 <details>  
 <summary>  
  </summary> 
 
        
```solidity

contract GasTest is DSTest {
    Contract0 c0;
    Contract1 c1;

    function setUp() public {
        c0 = new Contract0();
        c1 = new Contract1();
    }

    function testGas() public view {
        c0.solidityHash(2309349, 2304923409);
        c1.assemblyHash(2309349, 2304923409);
    }
}

contract Contract0 {
    function solidityHash(uint256 a, uint256 b) public view {
        //unoptimized
        keccak256(abi.encodePacked(a, b));
    }
}

contract Contract1 {
    function assemblyHash(uint256 a, uint256 b) public view {
        //optimized
        assembly {
            mstore(0x00, a)
            mstore(0x20, b)
            let hashedVal := keccak256(0x00, 0x40)
        }
    }
}

```

```solidity

╭────────────────────┬─────────────────┬─────┬────────┬─────┬─────────╮
│ Contract0 contract ┆                 ┆     ┆        ┆     ┆         │
╞════════════════════╪═════════════════╪═════╪════════╪═════╪═════════╡
│ Deployment Cost    ┆ Deployment Size ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 36687              ┆ 214             ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name      ┆ min             ┆ avg ┆ median ┆ max ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ solidityHash       ┆ 313             ┆ 313 ┆ 313    ┆ 313 ┆ 1       │
╰────────────────────┴─────────────────┴─────┴────────┴─────┴─────────╯
╭────────────────────┬─────────────────┬─────┬────────┬─────┬─────────╮
│ Contract1 contract ┆                 ┆     ┆        ┆     ┆         │
╞════════════════════╪═════════════════╪═════╪════════╪═════╪═════════╡
│ Deployment Cost    ┆ Deployment Size ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 31281              ┆ 186             ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name      ┆ min             ┆ avg ┆ median ┆ max ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ assemblyHash       ┆ 231             ┆ 231 ┆ 231    ┆ 231 ┆ 1       │
╰────────────────────┴─────────────────┴─────┴────────┴─────┴─────────╯

```
         
 </details> 
 

 --- 

[File:TransactionValidator.sol#L166](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/TransactionValidator.sol#L166) 
```solidity
165:            if (SafeHelper._GUARD_REMOVAL_CALLDATA_HASH == keccak256(_data)) {
``` 



[File:TransactionValidator.sol#L168](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/TransactionValidator.sol#L168) 
```solidity
167:            } else if (SafeHelper._FALLBACK_REMOVAL_CALLDATA_HASH == keccak256(_data)) {
``` 



[File:SafeDeployer.sol#L225](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeDeployer.sol#L225) 
```solidity
224:        bytes32 ownersHash = keccak256(abi.encode(_owners));
``` 



[File:SafeDeployer.sol#L254](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeDeployer.sol#L254) 
```solidity
253:        return uint256(keccak256(abi.encodePacked(_ownersHash, ownerSafeCount[_ownersHash]++, _salt, VERSION)));
``` 



[File:ConsoleFallbackHandler.sol#L24](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ConsoleFallbackHandler.sol#L24) 
```solidity
23:    bytes4 internal constant SIMULATE_SELECTOR = bytes4(keccak256("simulate(address,bytes)"));
``` 



[File:ConsoleFallbackHandler.sol#L69](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ConsoleFallbackHandler.sol#L69) 
```solidity
68:        bytes32 safeMessageHash = keccak256(abi.encode(SAFE_MSG_TYPEHASH, keccak256(message)));
``` 



[File:ConsoleFallbackHandler.sol#L69](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ConsoleFallbackHandler.sol#L69) 
```solidity
68:        bytes32 safeMessageHash = keccak256(abi.encode(SAFE_MSG_TYPEHASH, keccak256(message)));
``` 



[File:ConsoleFallbackHandler.sol#L70](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ConsoleFallbackHandler.sol#L70) 
```solidity
69:        return keccak256(abi.encodePacked(bytes1(0x19), bytes1(0x01), safe.domainSeparator(), safeMessageHash));
``` 



[File:TypeHashHelper.sol#L65](https://github.com/code-423n4/2023-10-brahma/blob/main/src/libraries/TypeHashHelper.sol#L65) 
```solidity
64:        return keccak256(
65:            abi.encode(
66:                TRANSACTION_PARAMS_TYPEHASH,
67:                txn.to,
68:                txn.value,
69:                keccak256(txn.data),
70:                txn.operation,
71:                txn.account,
72:                txn.executor,
73:                txn.nonce
74:            )
75:        );
``` 



[File:TypeHashHelper.sol#L70](https://github.com/code-423n4/2023-10-brahma/blob/main/src/libraries/TypeHashHelper.sol#L70) 
```solidity
69:                keccak256(txn.data),
``` 



[File:TypeHashHelper.sol#L85](https://github.com/code-423n4/2023-10-brahma/blob/main/src/libraries/TypeHashHelper.sol#L85) 
```solidity
84:        return keccak256(
85:            abi.encode(
86:                VALIDATION_PARAMS_TYPEHASH,
87:                validation.transactionStructHash,
88:                validation.policyHash,
89:                validation.expiryEpoch
90:            )
91:        );
``` 



 --- 

<a name=[G-4]></a>
### [G-4] Use custom errors instead of string error messages - Instances: 2 

 
 
> Using custom errors will save you gas, and can be used to provide more information about the error.
        
         
 
#### Gas Report - Savings: ~57 
 <details>  
 <summary>  
  </summary> 
 
        
```solidity

contract GasTest is DSTest {
    Contract0 c0;
    Contract1 c1;

    function setUp() public {
        c0 = new Contract0();
        c1 = new Contract1();
    }

    function testFailGas() public {
        c0.stringErrorMessage();
        c1.customErrorMessage();
    }
}

contract Contract0 {
    function stringErrorMessage() public {
        bool check = false;
        require(check, "error message");
    }
}

contract Contract1 {
    error CustomError();

    function customErrorMessage() public {
        bool check = false;
        if (!check) {
            revert CustomError();
        }
    }
}

```


```solidity
╭────────────────────┬─────────────────┬─────┬────────┬─────┬─────────╮
│ Contract0 contract ┆                 ┆     ┆        ┆     ┆         │
╞════════════════════╪═════════════════╪═════╪════════╪═════╪═════════╡
│ Deployment Cost    ┆ Deployment Size ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 34087              ┆ 200             ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name      ┆ min             ┆ avg ┆ median ┆ max ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ stringErrorMessage ┆ 218             ┆ 218 ┆ 218    ┆ 218 ┆ 1       │
╰────────────────────┴─────────────────┴─────┴────────┴─────┴─────────╯
╭────────────────────┬─────────────────┬─────┬────────┬─────┬─────────╮
│ Contract1 contract ┆                 ┆     ┆        ┆     ┆         │
╞════════════════════╪═════════════════╪═════╪════════╪═════╪═════════╡
│ Deployment Cost    ┆ Deployment Size ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 26881              ┆ 164             ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name      ┆ min             ┆ avg ┆ median ┆ max ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ customErrorMessage ┆ 161             ┆ 161 ┆ 161    ┆ 161 ┆ 1       │
╰────────────────────┴─────────────────┴─────┴────────┴─────┴─────────╯
```

 
 </details> 
 

 --- 

[File:SafeEnabler.sol#L48](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeEnabler.sol#L48) 
```solidity
47:        require(module != address(0) && module != _SENTINEL_MODULES, "GS101");
``` 



[File:SafeEnabler.sol#L52](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeEnabler.sol#L52) 
```solidity
51:        require(modules[module] == address(0), "GS102");
``` 



[File:ConsoleFallbackHandler.sol#L44](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ConsoleFallbackHandler.sol#L44) 
```solidity
43:            require(safe.signedMessages(messageHash) != 0, "Hash not approved");
``` 



[File:ConsoleFallbackHandler.sol#L51](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ConsoleFallbackHandler.sol#L51) 
```solidity
50:            require(policyValidator.isPolicySignatureValid(msg.sender, messageHash, _signature), "Policy not approved");
``` 



 --- 

<a name=[G-5]></a>
### [G-5] Use assembly for math (add, sub, mul, div) - Instances: 2 

 
 
> Use assembly for math instead of Solidity. You can check for overflow/underflow in assembly to ensure safety. If using Solidity versions < 0.8.0 and you are using Safemath, you can gain significant gas savings by using assembly to calculate values and checking for overflow/underflow. 
 
#### Gas Report - Savings: ~60 
 <details>  
 <summary>  
  </summary> 
 
        
```solidity

contract GasTest is DSTest {
    Contract0 c0;
    Contract1 c1;
    Contract2 c2;
    Contract3 c3;
    Contract4 c4;
    Contract5 c5;
    Contract6 c6;
    Contract7 c7;

    function setUp() public {
        c0 = new Contract0();
        c1 = new Contract1();
        c2 = new Contract2();
        c3 = new Contract3();
        c4 = new Contract4();
        c5 = new Contract5();
        c6 = new Contract6();
        c7 = new Contract7();
    }

    function testGas() public {
        c0.addTest(34598345, 100);
        c1.addAssemblyTest(34598345, 100);
        c2.subTest(34598345, 100);
        c3.subAssemblyTest(34598345, 100);
        c4.mulTest(34598345, 100);
        c5.mulAssemblyTest(34598345, 100);
        c6.divTest(34598345, 100);
        c7.divAssemblyTest(34598345, 100);
    }
}

contract Contract0 {
    //addition in Solidity
    function addTest(uint256 a, uint256 b) public pure {
        uint256 c = a + b;
    }
}

contract Contract1 {
    //addition in assembly
    function addAssemblyTest(uint256 a, uint256 b) public pure {
        assembly {
            let c := add(a, b)

            if lt(c, a) {
                mstore(0x00, "overflow")
                revert(0x00, 0x20)
            }
        }
    }
}

contract Contract2 {
    //subtraction in Solidity
    function subTest(uint256 a, uint256 b) public pure {
        uint256 c = a - b;
    }
}

contract Contract3 {
    //subtraction in assembly
    function subAssemblyTest(uint256 a, uint256 b) public pure {
        assembly {
            let c := sub(a, b)

            if gt(c, a) {
                mstore(0x00, "underflow")
                revert(0x00, 0x20)
            }
        }
    }
}

contract Contract4 {
    //multiplication in Solidity
    function mulTest(uint256 a, uint256 b) public pure {
        uint256 c = a * b;
    }
}

contract Contract5 {
    //multiplication in assembly
    function mulAssemblyTest(uint256 a, uint256 b) public pure {
        assembly {
            let c := mul(a, b)

            if lt(c, a) {
                mstore(0x00, "overflow")
                revert(0x00, 0x20)
            }
        }
    }
}

contract Contract6 {
    //division in Solidity
    function divTest(uint256 a, uint256 b) public pure {
        uint256 c = a * b;
    }
}

contract Contract7 {
    //division in assembly
    function divAssemblyTest(uint256 a, uint256 b) public pure {
        assembly {
            let c := div(a, b)

            if gt(c, a) {
                mstore(0x00, "underflow")
                revert(0x00, 0x20)
            }
        }
    }
}


```


```solidity

╭────────────────────┬─────────────────┬─────┬────────┬─────┬─────────╮
│ Contract0 contract ┆                 ┆     ┆        ┆     ┆         │
╞════════════════════╪═════════════════╪═════╪════════╪═════╪═════════╡
│ Deployment Cost    ┆ Deployment Size ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 40493              ┆ 233             ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name      ┆ min             ┆ avg ┆ median ┆ max ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ addTest            ┆ 303             ┆ 303 ┆ 303    ┆ 303 ┆ 1       │
╰────────────────────┴─────────────────┴─────┴────────┴─────┴─────────╯
╭────────────────────┬─────────────────┬─────┬────────┬─────┬─────────╮
│ Contract1 contract ┆                 ┆     ┆        ┆     ┆         │
╞════════════════════╪═════════════════╪═════╪════════╪═════╪═════════╡
│ Deployment Cost    ┆ Deployment Size ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 37087              ┆ 216             ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name      ┆ min             ┆ avg ┆ median ┆ max ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ addAssemblyTest    ┆ 263             ┆ 263 ┆ 263    ┆ 263 ┆ 1       │
╰────────────────────┴─────────────────┴─────┴────────┴─────┴─────────╯
╭────────────────────┬─────────────────┬─────┬────────┬─────┬─────────╮
│ Contract2 contract ┆                 ┆     ┆        ┆     ┆         │
╞════════════════════╪═════════════════╪═════╪════════╪═════╪═════════╡
│ Deployment Cost    ┆ Deployment Size ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 40293              ┆ 232             ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name      ┆ min             ┆ avg ┆ median ┆ max ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ subTest            ┆ 300             ┆ 300 ┆ 300    ┆ 300 ┆ 1       │
╰────────────────────┴─────────────────┴─────┴────────┴─────┴─────────╯
╭────────────────────┬─────────────────┬─────┬────────┬─────┬─────────╮
│ Contract3 contract ┆                 ┆     ┆        ┆     ┆         │
╞════════════════════╪═════════════════╪═════╪════════╪═════╪═════════╡
│ Deployment Cost    ┆ Deployment Size ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 37287              ┆ 217             ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name      ┆ min             ┆ avg ┆ median ┆ max ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ subAssemblyTest    ┆ 263             ┆ 263 ┆ 263    ┆ 263 ┆ 1       │
╰────────────────────┴─────────────────┴─────┴────────┴─────┴─────────╯
╭────────────────────┬─────────────────┬─────┬────────┬─────┬─────────╮
│ Contract4 contract ┆                 ┆     ┆        ┆     ┆         │
╞════════════════════╪═════════════════╪═════╪════════╪═════╪═════════╡
│ Deployment Cost    ┆ Deployment Size ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 41893              ┆ 240             ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name      ┆ min             ┆ avg ┆ median ┆ max ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ mulTest            ┆ 325             ┆ 325 ┆ 325    ┆ 325 ┆ 1       │
╰────────────────────┴─────────────────┴─────┴────────┴─────┴─────────╯
╭────────────────────┬─────────────────┬─────┬────────┬─────┬─────────╮
│ Contract5 contract ┆                 ┆     ┆        ┆     ┆         │
╞════════════════════╪═════════════════╪═════╪════════╪═════╪═════════╡
│ Deployment Cost    ┆ Deployment Size ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 37087              ┆ 216             ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name      ┆ min             ┆ avg ┆ median ┆ max ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ mulAssemblyTest    ┆ 265             ┆ 265 ┆ 265    ┆ 265 ┆ 1       │
╰────────────────────┴─────────────────┴─────┴────────┴─────┴─────────╯
╭────────────────────┬─────────────────┬─────┬────────┬─────┬─────────╮
│ Contract6 contract ┆                 ┆     ┆        ┆     ┆         │
╞════════════════════╪═════════════════╪═════╪════════╪═════╪═════════╡
│ Deployment Cost    ┆ Deployment Size ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 41893              ┆ 240             ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name      ┆ min             ┆ avg ┆ median ┆ max ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ divTest            ┆ 325             ┆ 325 ┆ 325    ┆ 325 ┆ 1       │
╰────────────────────┴─────────────────┴─────┴────────┴─────┴─────────╯
╭────────────────────┬─────────────────┬─────┬────────┬─────┬─────────╮
│ Contract7 contract ┆                 ┆     ┆        ┆     ┆         │
╞════════════════════╪═════════════════╪═════╪════════╪═════╪═════════╡
│ Deployment Cost    ┆ Deployment Size ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 37287              ┆ 217             ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name      ┆ min             ┆ avg ┆ median ┆ max ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ divAssemblyTest    ┆ 265             ┆ 265 ┆ 265    ┆ 265 ┆ 1       │
╰────────────────────┴─────────────────┴─────┴────────┴─────┴─────────╯

```
         
 </details> 
 

 --- 

[File:PolicyValidator.sol#L164](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/PolicyValidator.sol#L164) 
```solidity
163:        uint32 sigLength = uint32(bytes4(_signatures[length - 8:length - 4]));
``` 



[File:PolicyValidator.sol#L164](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/PolicyValidator.sol#L164) 
```solidity
163:        uint32 sigLength = uint32(bytes4(_signatures[length - 8:length - 4]));
``` 



[File:PolicyValidator.sol#L165](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/PolicyValidator.sol#L165) 
```solidity
164:        expiryEpoch = uint32(bytes4(_signatures[length - 4:length]));
``` 



[File:PolicyValidator.sol#L166](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/PolicyValidator.sol#L166) 
```solidity
165:        validatorSignature = _signatures[length - 8 - sigLength:length - 8];
``` 



[File:PolicyValidator.sol#L166](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/PolicyValidator.sol#L166) 
```solidity
165:        validatorSignature = _signatures[length - 8 - sigLength:length - 8];
``` 



[File:PolicyValidator.sol#L166](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/PolicyValidator.sol#L166) 
```solidity
165:        validatorSignature = _signatures[length - 8 - sigLength:length - 8];
``` 



[File:ConsoleOpBuilder.sol#L101](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ConsoleOpBuilder.sol#L101) 
```solidity
100:        uint256 _numberOfTransactions = _numberOfExecutors + 1;
``` 



[File:ConsoleOpBuilder.sol#L127](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ConsoleOpBuilder.sol#L127) 
```solidity
126:            txns[i + 1] = Types.Executable({
``` 



[File:ConsoleOpBuilder.sol#L159](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ConsoleOpBuilder.sol#L159) 
```solidity
158:        Types.Executable[] memory txns = new Types.Executable[](_numberOfExecutors+1);
``` 



[File:ConsoleOpBuilder.sol#L183](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ConsoleOpBuilder.sol#L183) 
```solidity
182:            txns[i + 1] = Types.Executable({
``` 



 --- 

<a name=[G-6]></a>
### [G-6] Use assembly to write storage values - Instances: 1 

 
 
> You can save a fair amount of gas by using assembly to write storage values.
     
 
#### Gas Report - Savings: ~66 
 <details>  
 <summary>  
  </summary> 
 
```solidity

contract GasTest is DSTest {
    Contract0 c0;
    Contract1 c1;

    function setUp() public {
        c0 = new Contract0();
        c1 = new Contract1();
    }

    function testGas() public {
        c0.updateOwner(0x158B28A1b1CB1BE12C6bD8f5a646a0e3B2024734);
        c1.assemblyUpdateOwner(0x158B28A1b1CB1BE12C6bD8f5a646a0e3B2024734);
    }
}

contract Contract0 {
    address owner = 0xb4c79daB8f259C7Aee6E5b2Aa729821864227e84;

    function updateOwner(address newOwner) public {
        owner = newOwner;
    }
}

contract Contract1 {
    address owner = 0xb4c79daB8f259C7Aee6E5b2Aa729821864227e84;

    function assemblyUpdateOwner(address newOwner) public {
        assembly {
            sstore(owner.slot, newOwner)
        }
    }
}

```

```solidity

╭────────────────────┬─────────────────┬──────┬────────┬──────┬─────────╮
│ Contract0 contract ┆                 ┆      ┆        ┆      ┆         │
╞════════════════════╪═════════════════╪══════╪════════╪══════╪═════════╡
│ Deployment Cost    ┆ Deployment Size ┆      ┆        ┆      ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 60623              ┆ 261             ┆      ┆        ┆      ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name      ┆ min             ┆ avg  ┆ median ┆ max  ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ updateOwner        ┆ 5302            ┆ 5302 ┆ 5302   ┆ 5302 ┆ 1       │
╰────────────────────┴─────────────────┴──────┴────────┴──────┴─────────╯
╭────────────────────┬─────────────────┬──────┬────────┬──────┬─────────╮
│ Contract1 contract ┆                 ┆      ┆        ┆      ┆         │
╞════════════════════╪═════════════════╪══════╪════════╪══════╪═════════╡
│ Deployment Cost    ┆ Deployment Size ┆      ┆        ┆      ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 54823              ┆ 232             ┆      ┆        ┆      ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name      ┆ min             ┆ avg  ┆ median ┆ max  ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ assemblyUpdateOwner┆ 5236            ┆ 5236 ┆ 5236   ┆ 5236 ┆ 1       │
╰────────────────────┴─────────────────┴──────┴────────┴──────┴─────────╯

```
         
 </details> 
 

 --- 

[File:AddressProvider.sol#L45](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/AddressProvider.sol#L45) 
```solidity
44:        governance = _governance;
``` 



[File:AddressProvider.sol#L56](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/AddressProvider.sol#L56) 
```solidity
55:        pendingGovernance = _newGovernance;
``` 



[File:AddressProvider.sol#L67](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/AddressProvider.sol#L67) 
```solidity
66:        governance = msg.sender;
``` 



 --- 

<a name=[G-7]></a>
### [G-7] Event is not properly indexed. - Instances: 3 

 
 
> When possible, always include a minimum of 3 indexed event topics to save gas 
 
#### Gas Report - Savings: ~0 
 <details>  
 <summary>  
  </summary> 
  
 </details> 
 

 --- 

[File:SafeDeployer.sol#L32](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeDeployer.sol#L32) 
```solidity
31:    event SafeProxyCreationFailure(address indexed singleton, uint256 indexed nonce, bytes initializer);
``` 



[File:SafeDeployer.sol#L35](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeDeployer.sol#L35) 
```solidity
34:    event PreComputeAccount(address[] indexed owners, uint256 indexed threshold);
``` 



[File:PolicyRegistry.sol#L19](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/registries/PolicyRegistry.sol#L19) 
```solidity
18:    event UpdatedPolicyCommit(address indexed account, bytes32 policyCommit, bytes32 oldPolicyCommit);
``` 



[File:SafeEnabler.sol#L19](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeEnabler.sol#L19) 
```solidity
18:    event EnabledModule(address module);
``` 



[File:SafeEnabler.sol#L20](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeEnabler.sol#L20) 
```solidity
19:    event ChangedGuard(address guard);
``` 



 --- 

<a name=[G-8]></a>
### [G-8] Use multiple require() statments insted of require(expression && expression && ...) - Instances: 1 

 
 
> You can safe gas by breaking up a require statement with multiple conditions, into multiple require statements with a single condition. 
 
#### Gas Report - Savings: ~16 
 <details>  
 <summary>  
  </summary> 
 
        
```solidity

contract GasTest is DSTest {
    Contract0 c0;
    Contract1 c1;

    function setUp() public {
        c0 = new Contract0();
        c1 = new Contract1();
    }

    function testGas() public {
        c0.singleRequire(3);
        c1.multipleRequire(3);
    }
}

contract Contract0 {
    function singleRequire(uint256 num) public {
        require(num > 1 && num < 10 && num == 3);
    }
}

contract Contract1 {
    function multipleRequire(uint256 num) public {
        require(num > 1);
        require(num < 10);
        require(num == 3);
    }
}

```


```solidity
╭────────────────────┬─────────────────┬─────┬────────┬─────┬─────────╮
│ Contract0 contract ┆                 ┆     ┆        ┆     ┆         │
╞════════════════════╪═════════════════╪═════╪════════╪═════╪═════════╡
│ Deployment Cost    ┆ Deployment Size ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 35487              ┆ 208             ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name      ┆ min             ┆ avg ┆ median ┆ max ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ singleRequire      ┆ 286             ┆ 286 ┆ 286    ┆ 286 ┆ 1       │
╰────────────────────┴─────────────────┴─────┴────────┴─────┴─────────╯
╭────────────────────┬─────────────────┬─────┬────────┬─────┬─────────╮
│ Contract1 contract ┆                 ┆     ┆        ┆     ┆         │
╞════════════════════╪═════════════════╪═════╪════════╪═════╪═════════╡
│ Deployment Cost    ┆ Deployment Size ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 35887              ┆ 210             ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name      ┆ min             ┆ avg ┆ median ┆ max ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ multipleRequire    ┆ 270             ┆ 270 ┆ 270    ┆ 270 ┆ 1       │
╰────────────────────┴─────────────────┴─────┴────────┴─────┴─────────╯

```

 
 </details> 
 

 --- 

[File:SafeEnabler.sol#L48](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeEnabler.sol#L48) 
```solidity
47:        require(module != address(0) && module != _SENTINEL_MODULES, "GS101");
``` 



 --- 

<a name=[G-9]></a>
### [G-9] Mark functions as payable (with discretion) - Instances: 14 

 
 
> You can mark public or external functions as payable to save gas. Functions that are not payable have additional logic to check if there was a value sent with a call, however, making a function payable eliminates this check. This optimization should be carefully considered due to potentially unwanted behavior when a function does not need to accept ether. 
 
#### Gas Report - Savings: ~24 
 <details>  
 <summary>  
  </summary> 
 

```solidity

contract GasTest is DSTest {
    Contract0 c0;
    Contract1 c1;

    function setUp() public {
        c0 = new Contract0();
        c1 = new Contract1();
    }

    function testGas() public {
        c0.isNotPayable();
        c1.isPayable();
    }
}

contract Contract0 {
    function isNotPayable() public view {
        uint256 val = 0;
        val++;
    }
}

contract Contract1 {
    function isPayable() public payable {
        uint256 val = 0;
        val++;
    }
}
```

```solidity

╭────────────────────┬─────────────────┬─────┬────────┬─────┬─────────╮
│ Contract0 contract ┆                 ┆     ┆        ┆     ┆         │
╞════════════════════╪═════════════════╪═════╪════════╪═════╪═════════╡
│ Deployment Cost    ┆ Deployment Size ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 32081              ┆ 190             ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name      ┆ min             ┆ avg ┆ median ┆ max ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ isNotPayable       ┆ 198             ┆ 198 ┆ 198    ┆ 198 ┆ 1       │
╰────────────────────┴─────────────────┴─────┴────────┴─────┴─────────╯
╭────────────────────┬─────────────────┬─────┬────────┬─────┬─────────╮
│ Contract1 contract ┆                 ┆     ┆        ┆     ┆         │
╞════════════════════╪═════════════════╪═════╪════════╪═════╪═════════╡
│ Deployment Cost    ┆ Deployment Size ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 29681              ┆ 178             ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name      ┆ min             ┆ avg ┆ median ┆ max ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ isPayable          ┆ 174             ┆ 174 ┆ 174    ┆ 174 ┆ 1       │
╰────────────────────┴─────────────────┴─────┴────────┴─────┴─────────╯

```

 
 </details> 
 

 --- 

[File:ConsoleFallbackHandler.sol#L39](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ConsoleFallbackHandler.sol#L39) 
```solidity
38:    function isValidSignature(bytes memory _data, bytes memory _signature) public view override returns (bytes4) {
``` 



[File:ConsoleFallbackHandler.sol#L60](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ConsoleFallbackHandler.sol#L60) 
```solidity
59:    function getMessageHash(bytes memory message) public view returns (bytes32) {
``` 



[File:ConsoleFallbackHandler.sol#L68](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ConsoleFallbackHandler.sol#L68) 
```solidity
67:    function getMessageHashForSafe(GnosisSafe safe, bytes memory message) public view returns (bytes32) {
``` 



[File:ConsoleFallbackHandler.sol#L83](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ConsoleFallbackHandler.sol#L83) 
```solidity
82:    function isValidSignature(bytes32 _dataHash, bytes calldata _signature) external view returns (bytes4) {
``` 



[File:ConsoleFallbackHandler.sol#L91](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ConsoleFallbackHandler.sol#L91) 
```solidity
90:    function getModules() external view returns (address[] memory) {
``` 



[File:ConsoleFallbackHandler.sol#L104](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ConsoleFallbackHandler.sol#L104) 
```solidity
103:    function simulate(address targetContract, bytes calldata calldataPayload)
104:        external
105:        returns (bytes memory response)
106:    {
``` 



[File:PolicyRegistry.sol#L35](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/registries/PolicyRegistry.sol#L35) 
```solidity
34:    function updatePolicy(address account, bytes32 policyCommit) external {
``` 



[File:AddressProvider.sol#L52](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/AddressProvider.sol#L52) 
```solidity
51:    function setGovernance(address _newGovernance) external {
``` 



[File:AddressProvider.sol#L62](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/AddressProvider.sol#L62) 
```solidity
61:    function acceptGovernance() external {
``` 



[File:AddressProvider.sol#L77](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/AddressProvider.sol#L77) 
```solidity
76:    function setAuthorizedAddress(bytes32 _key, address _authorizedAddress, bool _overrideCheck) external {
``` 



[File:AddressProvider.sol#L97](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/AddressProvider.sol#L97) 
```solidity
96:    function setRegistry(bytes32 _key, address _registry) external {
``` 



[File:AddressProvider.sol#L112](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/AddressProvider.sol#L112) 
```solidity
111:    function getAuthorizedAddress(bytes32 _key) external view returns (address) {
``` 



[File:AddressProvider.sol#L121](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/AddressProvider.sol#L121) 
```solidity
120:    function getRegistry(bytes32 _key) external view returns (address) {
``` 



[File:AddressProviderService.sol#L35](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/AddressProviderService.sol#L35) 
```solidity
34:    function addressProviderTarget() external view override returns (address) {
``` 



[File:SafeModeratorOverridable.sol#L39](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeModeratorOverridable.sol#L39) 
```solidity
38:    function checkTransaction(
39:        address to,
40:        uint256 value,
41:        bytes memory data,
42:        Enum.Operation operation,
43:        uint256 safeTxGas,
44:        uint256 baseGas,
45:        uint256 gasPrice,
46:        address gasToken,
47:        address payable refundReceiver,
48:        bytes memory signatures,
49:        address msgSender
50:    ) external view override {
``` 



[File:SafeModeratorOverridable.sol#L76](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeModeratorOverridable.sol#L76) 
```solidity
75:    function checkAfterExecution(bytes32 txHash, bool success) external view override {
``` 



[File:SafeModeratorOverridable.sol#L86](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeModeratorOverridable.sol#L86) 
```solidity
85:    function checkModuleTransaction(
86:        address, /* to */
87:        uint256, /* value */
88:        bytes memory, /* data */
89:        Enum.Operation, /* operation */
90:        address /* module */
91:    ) external override {}
``` 



[File:WalletRegistry.sol#L35](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/registries/WalletRegistry.sol#L35) 
```solidity
34:    function registerWallet() external {
``` 



[File:WalletRegistry.sol#L49](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/registries/WalletRegistry.sol#L49) 
```solidity
48:    function registerSubAccount(address _wallet, address _subAccount) external {
``` 



[File:WalletRegistry.sol#L63](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/registries/WalletRegistry.sol#L63) 
```solidity
62:    function getSubAccountsForWallet(address _wallet) external view returns (address[] memory) {
``` 



[File:WalletRegistry.sol#L73](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/registries/WalletRegistry.sol#L73) 
```solidity
72:    function isOwner(address _wallet, address _subAccount) external view returns (bool) {
``` 



[File:PolicyValidator.sol#L54](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/PolicyValidator.sol#L54) 
```solidity
53:    function isPolicySignatureValid(
54:        address account,
55:        address to,
56:        uint256 value,
57:        bytes memory data,
58:        Enum.Operation operation,
59:        bytes calldata signatures
60:    ) external view returns (bool) {
``` 



[File:PolicyValidator.sol#L100](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/PolicyValidator.sol#L100) 
```solidity
99:    function isPolicySignatureValid(address account, bytes32 transactionStructHash, bytes calldata signatures)
100:        public
101:        view
102:        returns (bool)
103:    {
``` 



[File:SafeDeployer.sol#L56](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeDeployer.sol#L56) 
```solidity
55:    function deployConsoleAccount(address[] calldata _owners, uint256 _threshold, bytes32 _policyCommit, bytes32 _salt)
56:        external
57:        nonReentrant
58:        returns (address _safe)
59:    {
``` 



[File:SafeDeployer.sol#L82](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeDeployer.sol#L82) 
```solidity
81:    function deploySubAccount(address[] calldata _owners, uint256 _threshold, bytes32 _policyCommit, bytes32 _salt)
82:        external
83:        nonReentrant
84:        returns (address _subAcc)
85:    {
``` 



[File:ExecutorRegistry.sol#L38](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/registries/ExecutorRegistry.sol#L38) 
```solidity
37:    function registerExecutor(address _subAccount, address _executor) external {
``` 



[File:ExecutorRegistry.sol#L53](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/registries/ExecutorRegistry.sol#L53) 
```solidity
52:    function deRegisterExecutor(address _subAccount, address _executor) external {
``` 



[File:ExecutorRegistry.sol#L67](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/registries/ExecutorRegistry.sol#L67) 
```solidity
66:    function isExecutor(address _subAccount, address _executor) external view returns (bool) {
``` 



[File:ExecutorRegistry.sol#L75](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/registries/ExecutorRegistry.sol#L75) 
```solidity
74:    function getExecutorsForSubAccount(address _subAccount) external view returns (address[] memory) {
``` 



[File:SafeModerator.sol#L33](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeModerator.sol#L33) 
```solidity
32:    function checkTransaction(
33:        address to,
34:        uint256 value,
35:        bytes memory data,
36:        Enum.Operation operation,
37:        uint256 safeTxGas,
38:        uint256 baseGas,
39:        uint256 gasPrice,
40:        address gasToken,
41:        address payable refundReceiver,
42:        bytes memory signatures,
43:        address msgSender
44:    ) external view override {
``` 



[File:SafeModerator.sol#L70](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeModerator.sol#L70) 
```solidity
69:    function checkAfterExecution(bytes32 txHash, bool success) external view override {
``` 



[File:SafeModerator.sol#L80](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeModerator.sol#L80) 
```solidity
79:    function checkModuleTransaction(
80:        address, /* to */
81:        uint256, /* value */
82:        bytes memory, /* data */
83:        Enum.Operation, /* operation */
84:        address /* module */
85:    ) external override {}
``` 



[File:SafeEnabler.sol#L43](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeEnabler.sol#L43) 
```solidity
42:    function enableModule(address module) public {
``` 



[File:SafeEnabler.sol#L66](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeEnabler.sol#L66) 
```solidity
65:    function setGuard(address guard) public {
``` 



[File:TransactionValidator.sol#L64](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/TransactionValidator.sol#L64) 
```solidity
63:    function validatePreTransactionOverridable(SafeTransactionParams memory txParams) external view {
``` 



[File:TransactionValidator.sol#L81](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/TransactionValidator.sol#L81) 
```solidity
80:    function validatePostTransactionOverridable(bytes32, /*txHash */ bool, /*success */ address /*console */ )
81:        external
82:        view
83:    {}
``` 



[File:TransactionValidator.sol#L95](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/TransactionValidator.sol#L95) 
```solidity
94:    function validatePreTransaction(SafeTransactionParams memory txParams) external view {
``` 



[File:TransactionValidator.sol#L105](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/TransactionValidator.sol#L105) 
```solidity
104:    function validatePostTransaction(bytes32, /*txHash */ bool, /*success */ address subAccount) external view {
``` 



[File:TransactionValidator.sol#L119](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/TransactionValidator.sol#L119) 
```solidity
118:    function validatePreExecutorTransaction(
119:        address, /*msgSender */
120:        address from,
121:        bytes32 transactionStructHash,
122:        bytes memory signatures
123:    ) external view {
``` 



[File:TransactionValidator.sol#L132](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/TransactionValidator.sol#L132) 
```solidity
131:    function validatePostExecutorTransaction(address, /*msgSender */ address subAccount) external view {
``` 



[File:ExecutorPlugin.sol#L68](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ExecutorPlugin.sol#L68) 
```solidity
67:    function executeTransaction(ExecutionRequest calldata execRequest) external nonReentrant returns (bytes memory) {
``` 



[File:ConsoleOpBuilder.sol#L30](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ConsoleOpBuilder.sol#L30) 
```solidity
29:    function enablePolicyOnConsole(address account, bytes32 policyCommit) external view returns (bytes memory) {
``` 



[File:ConsoleOpBuilder.sol#L68](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ConsoleOpBuilder.sol#L68) 
```solidity
67:    function disablePolicyOnConsole(address account) external pure returns (bytes memory) {
``` 



[File:ConsoleOpBuilder.sol#L95](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ConsoleOpBuilder.sol#L95) 
```solidity
94:    function enableExecutorPluginOnSubAccount(address subAccount, address[] memory executors)
95:        external
96:        view
97:        returns (bytes memory)
98:    {
``` 



[File:ConsoleOpBuilder.sol#L149](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ConsoleOpBuilder.sol#L149) 
```solidity
148:    function disableExecutorPluginOnSubAccount(address subAccount, address previousModule)
149:        external
150:        view
151:        returns (bytes memory)
152:    {
``` 



 --- 

<a name=[G-10]></a>
### [G-10] Consider marking constants as private - Instances: 7 

 
 
> Consider marking constant variables in storage as private to save gas (unless a constant variable should be easily accessible by another protocol or offchain logic). 
 #### Gas Report - Savings: ~22 
 <details>  
 <summary>  
  </summary> 
 
```solidity

contract GasTest is DSTest {
    Contract0 c0;
    Contract1 c1;
    
    function setUp() public {
        c0 = new Contract0();
        c1 = new Contract1();
        
    }
    function testGas() public view {
        uint256 a = 100;
        c0.addPublicConstant(a);
        c1.addPrivateConstant(a);
        
    }
}
contract Contract0 {

    uint256 constant public x = 100;

    function addPublicConstant(uint256 a) external pure returns (uint256) {
        return a + x;
    }
}

contract Contract1 {

        uint256 constant private x = 100;

    function addPrivateConstant(uint256 a) external pure returns (uint256) {
        return a +x;
    }
}
```


```solidity

╭───────────────────────────────────────────┬─────────────────┬─────┬────────┬─────┬─────────╮
│ src/test/GasTest.t.sol:Contract0 contract ┆                 ┆     ┆        ┆     ┆         │
╞═══════════════════════════════════════════╪═════════════════╪═════╪════════╪═════╪═════════╡
│ Deployment Cost                           ┆ Deployment Size ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 92741                                     ┆ 495             ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name                             ┆ min             ┆ avg ┆ median ┆ max ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ addPublicConstant                         ┆ 790             ┆ 790 ┆ 790    ┆ 790 ┆ 1       │
╰───────────────────────────────────────────┴─────────────────┴─────┴────────┴─────┴─────────╯
╭───────────────────────────────────────────┬─────────────────┬─────┬────────┬─────┬─────────╮
│ src/test/GasTest.t.sol:Contract1 contract ┆                 ┆     ┆        ┆     ┆         │
╞═══════════════════════════════════════════╪═════════════════╪═════╪════════╪═════╪═════════╡
│ Deployment Cost                           ┆ Deployment Size ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 83535                                     ┆ 449             ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name                             ┆ min             ┆ avg ┆ median ┆ max ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ addPrivateConstant                        ┆ 768             ┆ 768 ┆ 768    ┆ 768 ┆ 1       │
╰───────────────────────────────────────────┴─────────────────┴─────┴────────┴─────┴─────────╯

```
             
 </details> 
 

 --- 

[File:ConsoleFallbackHandler.sol#L24](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ConsoleFallbackHandler.sol#L24) 
```solidity
23:    bytes4 internal constant SIMULATE_SELECTOR = bytes4(keccak256("simulate(address,bytes)"));
``` 



[File:ConsoleFallbackHandler.sol#L26](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ConsoleFallbackHandler.sol#L26) 
```solidity
25:    address internal constant SENTINEL_MODULES = address(0x1);
``` 



[File:ConsoleFallbackHandler.sol#L27](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ConsoleFallbackHandler.sol#L27) 
```solidity
26:    bytes4 internal constant UPDATED_MAGIC_VALUE = 0x1626ba7e;
``` 



[File:SafeEnabler.sol#L26](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeEnabler.sol#L26) 
```solidity
25:    address internal constant _SENTINEL_MODULES = address(0x1);
``` 



[File:SafeEnabler.sol#L30](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeEnabler.sol#L30) 
```solidity
29:    bytes32 internal constant _GUARD_STORAGE_SLOT = 0x4a204f620c8c5ccdca3fd54d003badd85ba500436a431f0cbda4f558c93c34c8;
``` 



[File:SafeDeployer.sol#L23](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeDeployer.sol#L23) 
```solidity
22:    string public constant VERSION = "1";
``` 



[File:SafeDeployer.sol#L29](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeDeployer.sol#L29) 
```solidity
28:    bytes32 internal constant _SAFE_CREATION_FAILURE_REASON =
29:        0xd7c71a0bdd2eb2834ad042153c811dd478e4ee2324e3003b9522e03e7b3735dc;
``` 



[File:SafeModeratorOverridable.sol#L21](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeModeratorOverridable.sol#L21) 
```solidity
20:    uint8 public constant DIFFER_SAFE_MOD = 0;
``` 



[File:Constants.sol#L18](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/Constants.sol#L18) 
```solidity
17:    bytes32 internal constant _EXECUTOR_REGISTRY_HASH =
18:        0x165eedff3947ccfbe9739de5f67209b9935e684faef9ce859fb3dc46d33317f1;
``` 



[File:Constants.sol#L22](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/Constants.sol#L22) 
```solidity
21:    bytes32 internal constant _WALLET_REGISTRY_HASH = 0x75559f593e25c44cbf7547c1b715821f42da95df7f98cc735242e5e68111f75b;
``` 



[File:Constants.sol#L25](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/Constants.sol#L25) 
```solidity
24:    bytes32 internal constant _POLICY_REGISTRY_HASH = 0xbcf81d40f2af7491686907859b412c7908faa6fb0c2baa84d3a2f4ee8bddcac9;
``` 



[File:Constants.sol#L31](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/Constants.sol#L31) 
```solidity
30:    bytes32 internal constant _EXECUTOR_PLUGIN_HASH = 0x93ae17d63c11c26435c52b81ff53503650df80d35c62972110e64a0454badbec;
``` 



[File:Constants.sol#L34](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/Constants.sol#L34) 
```solidity
33:    bytes32 internal constant _CONSOLE_FALLBACK_HANDLER_HASH =
34:        0xae7411971cec68831bcab624e30a4a0a5f5439f59e63328ddc36cb3fd7a6e7c1;
``` 



[File:Constants.sol#L38](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/Constants.sol#L38) 
```solidity
37:    bytes32 internal constant _GNOSIS_FALLBACK_HANDLER_HASH =
38:        0x1d0e7baa67448e6f8f5d2d87a7a735e5169126f41c10a38f620af92e1330b40d;
``` 



[File:Constants.sol#L42](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/Constants.sol#L42) 
```solidity
41:    bytes32 internal constant _GNOSIS_MULTI_SEND_HASH =
42:        0x715c5882bd95d4620322df75c5ab7f02e94c69337dbd853ac1ee6f46017c2e70;
``` 



[File:Constants.sol#L46](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/Constants.sol#L46) 
```solidity
45:    bytes32 internal constant _GNOSIS_PROXY_FACTORY_HASH =
46:        0x52941112416a4a1e2ba89836a1489d24ce80a72d85f6aac86c51a27cc3ad5aa1;
``` 



[File:Constants.sol#L50](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/Constants.sol#L50) 
```solidity
49:    bytes32 internal constant _GNOSIS_SINGLETON_HASH =
50:        0x2bfdce65c4c0ca0f23ed8aa5f1391eae3ebd542786781824b0cf2af1ee477849;
``` 



[File:Constants.sol#L54](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/Constants.sol#L54) 
```solidity
53:    bytes32 internal constant _POLICY_VALIDATOR_HASH =
54:        0x64f265888225e50b07bac47655ac1813ae55b484fcd2987f60cc44f47dd2bc62;
``` 



[File:Constants.sol#L58](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/Constants.sol#L58) 
```solidity
57:    bytes32 internal constant _SAFE_DEPLOYER_HASH = 0x6a361f18b7bdd4971167a9db369c1357f3ebc95ef7fca81cd1aebfaeb988666d;
``` 



[File:Constants.sol#L61](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/Constants.sol#L61) 
```solidity
60:    bytes32 internal constant _SAFE_ENABLER_HASH = 0xcfef2b3eaf4a23b5f44a9550982acf18e82082f6a9756b60ca375f7b497918ca;
``` 



[File:Constants.sol#L64](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/Constants.sol#L64) 
```solidity
63:    bytes32 internal constant _SAFE_MODERATOR_HASH = 0x1ba997699674cf2f5c633ee918b93d66618d3081db3e508721439eeef694efc7;
``` 



[File:Constants.sol#L67](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/Constants.sol#L67) 
```solidity
66:    bytes32 internal constant _SAFE_MODERATOR_OVERRIDABLE_HASH =
67:        0xc70d2b4d45b7a19323a1c8274160190d132d8d675cb539a992b330caa33f7faa;
``` 



[File:Constants.sol#L71](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/Constants.sol#L71) 
```solidity
70:    bytes32 internal constant _TRANSACTION_VALIDATOR_HASH =
71:        0xafb0a433e8688b3129b23c7a328676ad1689d2ad64b49e0ed21edcd0d36ec73d;
``` 



[File:Constants.sol#L75](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/Constants.sol#L75) 
```solidity
74:    bytes32 internal constant _CONSOLE_OP_BUILDER_HASH =
75:        0x2fa1de21ef473c3db44f5bbcf769de9538366142cab32c7df7c0fb598c034a5b;
``` 



[File:Constants.sol#L82](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/Constants.sol#L82) 
```solidity
81:    bytes32 internal constant _GUARDIAN_HASH = 0x424560fc12b0242dae8bb63e27dad69d2589059728e8daf9b2ff8557998f3402;
``` 



[File:Constants.sol#L85](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/Constants.sol#L85) 
```solidity
84:    bytes32 internal constant _TRUSTED_VALIDATOR_HASH =
85:        0xc8fab2cea6e498c7d5e11e57566a1bd0376d100edae95e1256aeb6072ee66f89;
``` 



[File:SafeHelper.sol#L23](https://github.com/code-423n4/2023-10-brahma/blob/main/src/libraries/SafeHelper.sol#L23) 
```solidity
22:    uint256 internal constant _GUARD_STORAGE_SLOT =
23:        33528237782592280163068556224972516439282563014722366175641814928123294921928;
``` 



[File:SafeHelper.sol#L27](https://github.com/code-423n4/2023-10-brahma/blob/main/src/libraries/SafeHelper.sol#L27) 
```solidity
26:    uint256 internal constant _FALLBACK_HANDLER_STORAGE_SLOT =
27:        49122629484629529244014240937346711770925847994644146912111677022347558721749;
``` 



[File:SafeHelper.sol#L37](https://github.com/code-423n4/2023-10-brahma/blob/main/src/libraries/SafeHelper.sol#L37) 
```solidity
36:    bytes32 internal constant _GUARD_REMOVAL_CALLDATA_HASH =
37:        0xc0e2c16ecb99419a40dd8b9c0b339b27acebd27c481a28cd606927aeb86f5079;
``` 



[File:SafeHelper.sol#L47](https://github.com/code-423n4/2023-10-brahma/blob/main/src/libraries/SafeHelper.sol#L47) 
```solidity
46:    bytes32 internal constant _FALLBACK_REMOVAL_CALLDATA_HASH =
47:        0x5bdf8c44c012c1347b2b15694dc5cc39b899eb99e32614676b7661001be925b7;
``` 



[File:TypeHashHelper.sol#L49](https://github.com/code-423n4/2023-10-brahma/blob/main/src/libraries/TypeHashHelper.sol#L49) 
```solidity
48:    bytes32 public constant TRANSACTION_PARAMS_TYPEHASH =
49:        0xfd4628b53a91b366f1977138e2eda53b93c8f5cc74bda8440f108d7da1e99290;
``` 



[File:TypeHashHelper.sol#L56](https://github.com/code-423n4/2023-10-brahma/blob/main/src/libraries/TypeHashHelper.sol#L56) 
```solidity
55:    bytes32 public constant VALIDATION_PARAMS_TYPEHASH =
56:        0x0c7f653e0f641e41fbb4ed1440c7d0b08b8d2a19e1c35cfc98de2d47519e15b1;
``` 



 --- 

<a name=[G-11]></a>
### [G-11] Use assembly to check for address(0) - Instances: 5 

 
  
 
#### Gas Report - Savings: ~6 
 <details>  
 <summary>  
  </summary> 
 
```solidity


contract GasTest is DSTest {
    Contract0 c0;
    Contract1 c1;

    function setUp() public {
        c0 = new Contract0();
        c1 = new Contract1();
    }

    function testGas() public view {
        c0.ownerNotZero(address(this));
        c1.assemblyOwnerNotZero(address(this));
    }
}

contract Contract0 {
    function ownerNotZero(address _addr) public pure {
        require(_addr != address(0), "zero address)");
    }
}

contract Contract1 {
    function assemblyOwnerNotZero(address _addr) public pure {
        assembly {
            if iszero(_addr) {
                mstore(0x00, "zero address")
                revert(0x00, 0x20)
            }
        }
    }
}


```

```solidity
╭────────────────────┬─────────────────┬─────┬────────┬─────┬─────────╮
│ Contract0 contract ┆                 ┆     ┆        ┆     ┆         │
╞════════════════════╪═════════════════╪═════╪════════╪═════╪═════════╡
│ Deployment Cost    ┆ Deployment Size ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 61311              ┆ 338             ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name      ┆ min             ┆ avg ┆ median ┆ max ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ ownerNotZero       ┆ 258             ┆ 258 ┆ 258    ┆ 258 ┆ 1       │
╰────────────────────┴─────────────────┴─────┴────────┴─────┴─────────╯
╭──────────────────────┬─────────────────┬─────┬────────┬─────┬─────────╮
│ Contract1 contract   ┆                 ┆     ┆        ┆     ┆         │
╞══════════════════════╪═════════════════╪═════╪════════╪═════╪═════════╡
│ Deployment Cost      ┆ Deployment Size ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 44893                ┆ 255             ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name        ┆ min             ┆ avg ┆ median ┆ max ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ assemblyOwnerNotZero ┆ 252             ┆ 252 ┆ 252    ┆ 252 ┆ 1       │
╰──────────────────────┴─────────────────┴─────┴────────┴─────┴─────────╯
```
 
 </details> 
 

 --- 

[File:SafeEnabler.sol#L48](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeEnabler.sol#L48) 
```solidity
47:        require(module != address(0) && module != _SENTINEL_MODULES, "GS101");
``` 



[File:SafeEnabler.sol#L52](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeEnabler.sol#L52) 
```solidity
51:        require(modules[module] == address(0), "GS102");
``` 



[File:AddressProvider.sol#L101](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/AddressProvider.sol#L101) 
```solidity
100:        if (registries[_key] != address(0)) revert RegistryAlreadyExists();
``` 



[File:AddressProvider.sol#L148](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/AddressProvider.sol#L148) 
```solidity
147:        if (addr == address(0)) revert NullAddress();
``` 



[File:AddressProviderService.sol#L28](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/AddressProviderService.sol#L28) 
```solidity
27:        if (_addressProvider == address(0)) revert InvalidAddressProvider();
``` 



[File:AddressProviderService.sol#L73](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/AddressProviderService.sol#L73) 
```solidity
72:        if (_addr == address(0)) revert InvalidAddress();
``` 



[File:WalletRegistry.sol#L37](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/registries/WalletRegistry.sol#L37) 
```solidity
36:        if (subAccountToWallet[msg.sender] != address(0)) revert IsSubAccount();
``` 



[File:WalletRegistry.sol#L51](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/registries/WalletRegistry.sol#L51) 
```solidity
50:        if (subAccountToWallet[_subAccount] != address(0)) revert AlreadyRegistered();
``` 



[File:SafeDeployer.sol#L244](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeDeployer.sol#L244) 
```solidity
243:        } while (_safe == address(0));
``` 



 --- 



## Quality Assurance - Total: 131 

<a name=[NC-0]></a>
### [NC-0] Constructor should check that all parameters are not 0 - Instances: 13 

 > Consider adding a require statement to check that all parameters are not 0 in the constructor 

 --- 

[File:PolicyRegistry.sol#L24](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/registries/PolicyRegistry.sol#L24) 
```solidity
23:    constructor(address _addressProvider) AddressProviderService(_addressProvider) {}
``` 



[File:WalletRegistry.sol#L29](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/registries/WalletRegistry.sol#L29) 
```solidity
28:    constructor(address _addressProvider) AddressProviderService(_addressProvider) {}
``` 



[File:SafeDeployer.sol#L42](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeDeployer.sol#L42) 
```solidity
41:    constructor(address _addressProvider) AddressProviderService(_addressProvider) {}
``` 



[File:SafeModeratorOverridable.sol#L23](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeModeratorOverridable.sol#L23) 
```solidity
22:    constructor(address _addressProvider) AddressProviderService(_addressProvider) {}
``` 



[File:PolicyValidator.sol#L30](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/PolicyValidator.sol#L30) 
```solidity
29:    constructor(address _addressProvider) AddressProviderService(_addressProvider) {}
``` 



[File:TransactionValidator.sol#L54](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/TransactionValidator.sol#L54) 
```solidity
53:    constructor(address _addressProvider) AddressProviderService(_addressProvider) {}
``` 



[File:AddressProvider.sol#L43](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/AddressProvider.sol#L43) 
```solidity
42:    constructor(address _governance) {
``` 



[File:ConsoleFallbackHandler.sol#L29](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ConsoleFallbackHandler.sol#L29) 
```solidity
28:    constructor(address _addressProvider) AddressProviderService(_addressProvider) {}
``` 



[File:ConsoleOpBuilder.sol#L21](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ConsoleOpBuilder.sol#L21) 
```solidity
20:    constructor(address _addressProvider) AddressProviderService(_addressProvider) {}
``` 



[File:ExecutorPlugin.sol#L60](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ExecutorPlugin.sol#L60) 
```solidity
59:    constructor(address _addressProvider) AddressProviderService(_addressProvider) {}
``` 



[File:ExecutorRegistry.sol#L29](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/registries/ExecutorRegistry.sol#L29) 
```solidity
28:    constructor(address _addressProvider) AddressProviderService(_addressProvider) {}
``` 



[File:SafeModerator.sol#L17](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeModerator.sol#L17) 
```solidity
16:    constructor(address _addressProvider) AddressProviderService(_addressProvider) {}
``` 



[File:AddressProviderService.sol#L27](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/AddressProviderService.sol#L27) 
```solidity
26:    constructor(address _addressProvider) {
``` 



 --- 

<a name=[NC-1]></a>
### [NC-1] Consider importing specific identifiers instead of the whole file - Instances: 3 

 
> This will minimize compiled code size and help with readability 

 --- 

[File:ConsoleFallbackHandler.sol#L7](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ConsoleFallbackHandler.sol#L7) 
```solidity
6:import "safe-contracts/handler/DefaultCallbackHandler.sol";
``` 



[File:ConsoleFallbackHandler.sol#L8](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ConsoleFallbackHandler.sol#L8) 
```solidity
7:import "safe-contracts/interfaces/ISignatureValidator.sol";
``` 



[File:ConsoleFallbackHandler.sol#L9](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ConsoleFallbackHandler.sol#L9) 
```solidity
8:import "safe-contracts/GnosisSafe.sol";
``` 



 --- 

<a name=[NC-2]></a>
### [NC-2] Remove any unused functions - Instances: 12 

 
> Any functions not used should be removed as best practice. 

 --- 

[File:AddressProviderService.sol#L54](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/AddressProviderService.sol#L54) 
```solidity
53:    function _getAuthorizedAddress(bytes32 _key) internal view returns (address authorizedAddress) {
``` 



[File:AddressProviderService.sol#L62](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/AddressProviderService.sol#L62) 
```solidity
61:    function _onlyGov() internal view {
``` 



[File:AddressProviderService.sol#L44](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/AddressProviderService.sol#L44) 
```solidity
43:    function _getRegistry(bytes32 _key) internal view returns (address registry) {
``` 



[File:PolicyValidator.sol#L174](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/PolicyValidator.sol#L174) 
```solidity
173:    function _domainNameAndVersion() internal pure override returns (string memory name, string memory version) {
``` 



[File:SafeHelper.sol#L63](https://github.com/code-423n4/2023-10-brahma/blob/main/src/libraries/SafeHelper.sol#L63) 
```solidity
62:    function _executeOnSafe(address safe, address target, Enum.Operation op, bytes memory data) internal {
``` 



[File:SafeHelper.sol#L142](https://github.com/code-423n4/2023-10-brahma/blob/main/src/libraries/SafeHelper.sol#L142) 
```solidity
141:    function _getGuard(address safe) internal view returns (address) {
``` 



[File:SafeHelper.sol#L152](https://github.com/code-423n4/2023-10-brahma/blob/main/src/libraries/SafeHelper.sol#L152) 
```solidity
151:    function _getFallbackHandler(address safe) internal view returns (address) {
``` 



[File:SafeHelper.sol#L163](https://github.com/code-423n4/2023-10-brahma/blob/main/src/libraries/SafeHelper.sol#L163) 
```solidity
162:    function _parseOperationEnum(Types.CallType callType) internal pure returns (Enum.Operation operation) {
``` 



[File:SafeHelper.sol#L103](https://github.com/code-423n4/2023-10-brahma/blob/main/src/libraries/SafeHelper.sol#L103) 
```solidity
102:    function _packMultisendTxns(Types.Executable[] memory _txns) internal pure returns (bytes memory packedTxns) {
``` 



[File:ExecutorPlugin.sol#L159](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ExecutorPlugin.sol#L159) 
```solidity
158:    function _domainNameAndVersion() internal pure override returns (string memory name, string memory version) {
``` 



[File:TypeHashHelper.sol#L64](https://github.com/code-423n4/2023-10-brahma/blob/main/src/libraries/TypeHashHelper.sol#L64) 
```solidity
63:    function _buildTransactionStructHash(Transaction memory txn) internal pure returns (bytes32) {
``` 



[File:TypeHashHelper.sol#L84](https://github.com/code-423n4/2023-10-brahma/blob/main/src/libraries/TypeHashHelper.sol#L84) 
```solidity
83:    function _buildValidationStructHash(Validation memory validation) internal pure returns (bytes32) {
``` 



 --- 

<a name=[NC-3]></a>
### [NC-3] Function names should be in camelCase - Instances: 31 

 
> Ensure that function definitions are declared using camelCase 

 --- 

[File:TypeHashHelper.sol#L64](https://github.com/code-423n4/2023-10-brahma/blob/main/src/libraries/TypeHashHelper.sol#L64) 
```solidity
63:    function _buildTransactionStructHash(Transaction memory txn) internal pure returns (bytes32) {
``` 



[File:TypeHashHelper.sol#L84](https://github.com/code-423n4/2023-10-brahma/blob/main/src/libraries/TypeHashHelper.sol#L84) 
```solidity
83:    function _buildValidationStructHash(Validation memory validation) internal pure returns (bytes32) {
``` 



[File:PolicyValidator.sol#L156](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/PolicyValidator.sol#L156) 
```solidity
155:    function _decompileSignatures(bytes calldata _signatures)
156:        internal
157:        pure
158:        returns (uint32 expiryEpoch, bytes memory validatorSignature)
159:    {
``` 



[File:PolicyValidator.sol#L174](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/PolicyValidator.sol#L174) 
```solidity
173:    function _domainNameAndVersion() internal pure override returns (string memory name, string memory version) {
``` 



[File:PolicyRegistry.sol#L66](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/registries/PolicyRegistry.sol#L66) 
```solidity
65:    function _updatePolicy(address account, bytes32 policyCommit) internal {
``` 



[File:SafeDeployer.sol#L110](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeDeployer.sol#L110) 
```solidity
109:    function _setupConsoleAccount(address[] memory _owners, uint256 _threshold, bool _policyHashValid)
110:        private
111:        view
112:        returns (bytes memory)
113:    {
``` 



[File:SafeDeployer.sol#L168](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeDeployer.sol#L168) 
```solidity
167:    function _setupSubAccount(address[] memory _owners, uint256 _threshold, address _consoleAccount)
168:        private
169:        view
170:        returns (bytes memory)
171:    {
``` 



[File:SafeDeployer.sol#L219](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeDeployer.sol#L219) 
```solidity
218:    function _createSafe(address[] calldata _owners, bytes memory _initializer, bytes32 _salt)
219:        private
220:        returns (address _safe)
221:    {
``` 



[File:SafeDeployer.sol#L253](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeDeployer.sol#L253) 
```solidity
252:    function _genNonce(bytes32 _ownersHash, bytes32 _salt) private returns (uint256) {
``` 



[File:AddressProvider.sol#L130](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/AddressProvider.sol#L130) 
```solidity
129:    function _ensureAddressProvider(address _newAddress) internal view {
``` 



[File:AddressProvider.sol#L139](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/AddressProvider.sol#L139) 
```solidity
138:    function _onlyGov() internal view {
``` 



[File:AddressProvider.sol#L147](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/AddressProvider.sol#L147) 
```solidity
146:    function _notNull(address addr) internal pure {
``` 



[File:AddressProviderService.sol#L44](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/AddressProviderService.sol#L44) 
```solidity
43:    function _getRegistry(bytes32 _key) internal view returns (address registry) {
``` 



[File:AddressProviderService.sol#L54](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/AddressProviderService.sol#L54) 
```solidity
53:    function _getAuthorizedAddress(bytes32 _key) internal view returns (address authorizedAddress) {
``` 



[File:AddressProviderService.sol#L62](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/AddressProviderService.sol#L62) 
```solidity
61:    function _onlyGov() internal view {
``` 



[File:AddressProviderService.sol#L72](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/AddressProviderService.sol#L72) 
```solidity
71:    function _notNull(address _addr) internal pure {
``` 



[File:ConsoleFallbackHandler.sol#L104](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ConsoleFallbackHandler.sol#L104) 
```solidity
103:    function simulate(address targetContract, bytes calldata calldataPayload)
104:        external
105:        returns (bytes memory response)
106:    {
``` 



[File:SafeEnabler.sol#L81](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeEnabler.sol#L81) 
```solidity
80:    function _onlyDelegateCall() private view {
``` 



[File:TransactionValidator.sol#L149](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/TransactionValidator.sol#L149) 
```solidity
148:    function _isConsoleBeingOverriden(
149:        address _from,
150:        address _to,
151:        uint256 _value,
152:        bytes memory _data,
153:        Enum.Operation _operation
154:    ) internal pure returns (bool) {
``` 



[File:TransactionValidator.sol#L181](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/TransactionValidator.sol#L181) 
```solidity
180:    function _checkSubAccountSecurityConfig(address _subAccount) internal view {
``` 



[File:TransactionValidator.sol#L210](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/TransactionValidator.sol#L210) 
```solidity
209:    function _validatePolicySignature(
210:        address _from,
211:        address _to,
212:        uint256 _value,
213:        bytes memory _data,
214:        Enum.Operation _operation,
215:        bytes memory _signatures
216:    ) internal view {
``` 



[File:TransactionValidator.sol#L234](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/TransactionValidator.sol#L234) 
```solidity
233:    function _validatePolicySignature(address _from, bytes32 _transactionStructHash, bytes memory _signatures)
234:        internal
235:        view
236:    {
``` 



[File:ExecutorPlugin.sol#L86](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ExecutorPlugin.sol#L86) 
```solidity
85:    function _executeTxnAsModule(address _account, Types.Executable memory _executable)
86:        internal
87:        returns (bytes memory)
88:    {
``` 



[File:ExecutorPlugin.sol#L106](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ExecutorPlugin.sol#L106) 
```solidity
105:    function _validateExecutionRequest(ExecutionRequest calldata execRequest) internal {
``` 



[File:ExecutorPlugin.sol#L159](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ExecutorPlugin.sol#L159) 
```solidity
158:    function _domainNameAndVersion() internal pure override returns (string memory name, string memory version) {
``` 



[File:SafeHelper.sol#L63](https://github.com/code-423n4/2023-10-brahma/blob/main/src/libraries/SafeHelper.sol#L63) 
```solidity
62:    function _executeOnSafe(address safe, address target, Enum.Operation op, bytes memory data) internal {
``` 



[File:SafeHelper.sol#L87](https://github.com/code-423n4/2023-10-brahma/blob/main/src/libraries/SafeHelper.sol#L87) 
```solidity
86:    function _generateSingleThresholdSignature(address owner) internal pure returns (bytes memory) {
``` 



[File:SafeHelper.sol#L103](https://github.com/code-423n4/2023-10-brahma/blob/main/src/libraries/SafeHelper.sol#L103) 
```solidity
102:    function _packMultisendTxns(Types.Executable[] memory _txns) internal pure returns (bytes memory packedTxns) {
``` 



[File:SafeHelper.sol#L142](https://github.com/code-423n4/2023-10-brahma/blob/main/src/libraries/SafeHelper.sol#L142) 
```solidity
141:    function _getGuard(address safe) internal view returns (address) {
``` 



[File:SafeHelper.sol#L152](https://github.com/code-423n4/2023-10-brahma/blob/main/src/libraries/SafeHelper.sol#L152) 
```solidity
151:    function _getFallbackHandler(address safe) internal view returns (address) {
``` 



[File:SafeHelper.sol#L163](https://github.com/code-423n4/2023-10-brahma/blob/main/src/libraries/SafeHelper.sol#L163) 
```solidity
162:    function _parseOperationEnum(Types.CallType callType) internal pure returns (Enum.Operation operation) {
``` 



 --- 

<a name=[NC-4]></a>
### [NC-4] Constant and immutable variable names should be in SCREAMING_SNAKE_CASE - Instances: 32 

 
> Ensure that Constant and immutable variable names are declared using SCREAMING_SNAKE_CASE 

 --- 

[File:SafeDeployer.sol#L23](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeDeployer.sol#L23) 
```solidity
22:    string public constant VERSION = "1";
``` 



[File:SafeDeployer.sol#L29](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeDeployer.sol#L29) 
```solidity
28:    bytes32 internal constant _SAFE_CREATION_FAILURE_REASON =
29:        0xd7c71a0bdd2eb2834ad042153c811dd478e4ee2324e3003b9522e03e7b3735dc;
``` 



[File:PolicyValidator.sol#L26](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/PolicyValidator.sol#L26) 
```solidity
25:    string private constant _NAME = "PolicyValidator";
``` 



[File:PolicyValidator.sol#L28](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/PolicyValidator.sol#L28) 
```solidity
27:    string private constant _VERSION = "1.0";
``` 



[File:Constants.sol#L18](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/Constants.sol#L18) 
```solidity
17:    bytes32 internal constant _EXECUTOR_REGISTRY_HASH =
18:        0x165eedff3947ccfbe9739de5f67209b9935e684faef9ce859fb3dc46d33317f1;
``` 



[File:Constants.sol#L22](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/Constants.sol#L22) 
```solidity
21:    bytes32 internal constant _WALLET_REGISTRY_HASH = 0x75559f593e25c44cbf7547c1b715821f42da95df7f98cc735242e5e68111f75b;
``` 



[File:Constants.sol#L25](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/Constants.sol#L25) 
```solidity
24:    bytes32 internal constant _POLICY_REGISTRY_HASH = 0xbcf81d40f2af7491686907859b412c7908faa6fb0c2baa84d3a2f4ee8bddcac9;
``` 



[File:Constants.sol#L31](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/Constants.sol#L31) 
```solidity
30:    bytes32 internal constant _EXECUTOR_PLUGIN_HASH = 0x93ae17d63c11c26435c52b81ff53503650df80d35c62972110e64a0454badbec;
``` 



[File:Constants.sol#L34](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/Constants.sol#L34) 
```solidity
33:    bytes32 internal constant _CONSOLE_FALLBACK_HANDLER_HASH =
34:        0xae7411971cec68831bcab624e30a4a0a5f5439f59e63328ddc36cb3fd7a6e7c1;
``` 



[File:Constants.sol#L38](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/Constants.sol#L38) 
```solidity
37:    bytes32 internal constant _GNOSIS_FALLBACK_HANDLER_HASH =
38:        0x1d0e7baa67448e6f8f5d2d87a7a735e5169126f41c10a38f620af92e1330b40d;
``` 



[File:Constants.sol#L42](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/Constants.sol#L42) 
```solidity
41:    bytes32 internal constant _GNOSIS_MULTI_SEND_HASH =
42:        0x715c5882bd95d4620322df75c5ab7f02e94c69337dbd853ac1ee6f46017c2e70;
``` 



[File:Constants.sol#L46](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/Constants.sol#L46) 
```solidity
45:    bytes32 internal constant _GNOSIS_PROXY_FACTORY_HASH =
46:        0x52941112416a4a1e2ba89836a1489d24ce80a72d85f6aac86c51a27cc3ad5aa1;
``` 



[File:Constants.sol#L50](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/Constants.sol#L50) 
```solidity
49:    bytes32 internal constant _GNOSIS_SINGLETON_HASH =
50:        0x2bfdce65c4c0ca0f23ed8aa5f1391eae3ebd542786781824b0cf2af1ee477849;
``` 



[File:Constants.sol#L54](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/Constants.sol#L54) 
```solidity
53:    bytes32 internal constant _POLICY_VALIDATOR_HASH =
54:        0x64f265888225e50b07bac47655ac1813ae55b484fcd2987f60cc44f47dd2bc62;
``` 



[File:Constants.sol#L58](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/Constants.sol#L58) 
```solidity
57:    bytes32 internal constant _SAFE_DEPLOYER_HASH = 0x6a361f18b7bdd4971167a9db369c1357f3ebc95ef7fca81cd1aebfaeb988666d;
``` 



[File:Constants.sol#L61](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/Constants.sol#L61) 
```solidity
60:    bytes32 internal constant _SAFE_ENABLER_HASH = 0xcfef2b3eaf4a23b5f44a9550982acf18e82082f6a9756b60ca375f7b497918ca;
``` 



[File:Constants.sol#L64](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/Constants.sol#L64) 
```solidity
63:    bytes32 internal constant _SAFE_MODERATOR_HASH = 0x1ba997699674cf2f5c633ee918b93d66618d3081db3e508721439eeef694efc7;
``` 



[File:Constants.sol#L67](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/Constants.sol#L67) 
```solidity
66:    bytes32 internal constant _SAFE_MODERATOR_OVERRIDABLE_HASH =
67:        0xc70d2b4d45b7a19323a1c8274160190d132d8d675cb539a992b330caa33f7faa;
``` 



[File:Constants.sol#L71](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/Constants.sol#L71) 
```solidity
70:    bytes32 internal constant _TRANSACTION_VALIDATOR_HASH =
71:        0xafb0a433e8688b3129b23c7a328676ad1689d2ad64b49e0ed21edcd0d36ec73d;
``` 



[File:Constants.sol#L75](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/Constants.sol#L75) 
```solidity
74:    bytes32 internal constant _CONSOLE_OP_BUILDER_HASH =
75:        0x2fa1de21ef473c3db44f5bbcf769de9538366142cab32c7df7c0fb598c034a5b;
``` 



[File:Constants.sol#L82](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/Constants.sol#L82) 
```solidity
81:    bytes32 internal constant _GUARDIAN_HASH = 0x424560fc12b0242dae8bb63e27dad69d2589059728e8daf9b2ff8557998f3402;
``` 



[File:Constants.sol#L85](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/Constants.sol#L85) 
```solidity
84:    bytes32 internal constant _TRUSTED_VALIDATOR_HASH =
85:        0xc8fab2cea6e498c7d5e11e57566a1bd0376d100edae95e1256aeb6072ee66f89;
``` 



[File:AddressProviderService.sol#L25](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/AddressProviderService.sol#L25) 
```solidity
24:    AddressProvider public immutable addressProvider;
``` 



[File:SafeEnabler.sol#L26](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeEnabler.sol#L26) 
```solidity
25:    address internal constant _SENTINEL_MODULES = address(0x1);
``` 



[File:SafeEnabler.sol#L30](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeEnabler.sol#L30) 
```solidity
29:    bytes32 internal constant _GUARD_STORAGE_SLOT = 0x4a204f620c8c5ccdca3fd54d003badd85ba500436a431f0cbda4f558c93c34c8;
``` 



[File:SafeEnabler.sol#L23](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeEnabler.sol#L23) 
```solidity
22:    address internal immutable _self;
``` 



[File:ExecutorPlugin.sol#L53](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ExecutorPlugin.sol#L53) 
```solidity
52:    string private constant _NAME = "ExecutorPlugin";
``` 



[File:ExecutorPlugin.sol#L55](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ExecutorPlugin.sol#L55) 
```solidity
54:    string private constant _VERSION = "1.0";
``` 



[File:SafeHelper.sol#L23](https://github.com/code-423n4/2023-10-brahma/blob/main/src/libraries/SafeHelper.sol#L23) 
```solidity
22:    uint256 internal constant _GUARD_STORAGE_SLOT =
23:        33528237782592280163068556224972516439282563014722366175641814928123294921928;
``` 



[File:SafeHelper.sol#L27](https://github.com/code-423n4/2023-10-brahma/blob/main/src/libraries/SafeHelper.sol#L27) 
```solidity
26:    uint256 internal constant _FALLBACK_HANDLER_STORAGE_SLOT =
27:        49122629484629529244014240937346711770925847994644146912111677022347558721749;
``` 



[File:SafeHelper.sol#L37](https://github.com/code-423n4/2023-10-brahma/blob/main/src/libraries/SafeHelper.sol#L37) 
```solidity
36:    bytes32 internal constant _GUARD_REMOVAL_CALLDATA_HASH =
37:        0xc0e2c16ecb99419a40dd8b9c0b339b27acebd27c481a28cd606927aeb86f5079;
``` 



[File:SafeHelper.sol#L47](https://github.com/code-423n4/2023-10-brahma/blob/main/src/libraries/SafeHelper.sol#L47) 
```solidity
46:    bytes32 internal constant _FALLBACK_REMOVAL_CALLDATA_HASH =
47:        0x5bdf8c44c012c1347b2b15694dc5cc39b899eb99e32614676b7661001be925b7;
``` 



 --- 

<a name=[NC-5]></a>
### [NC-5] Remove any unused returns - Instances: 5 

 
> Either remove the return parameter names, or use them as the returns of the function. 

 --- 

[File:ConsoleFallbackHandler.sol#L104](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ConsoleFallbackHandler.sol#L104) 
```solidity
103:    function simulate(address targetContract, bytes calldata calldataPayload)
104:        external
105:        returns (bytes memory response)
106:    {
``` 



[File:PolicyValidator.sol#L174](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/PolicyValidator.sol#L174) 
```solidity
173:    function _domainNameAndVersion() internal pure override returns (string memory name, string memory version) {
``` 



[File:PolicyValidator.sol#L174](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/PolicyValidator.sol#L174) 
```solidity
173:    function _domainNameAndVersion() internal pure override returns (string memory name, string memory version) {
``` 



[File:ExecutorPlugin.sol#L159](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ExecutorPlugin.sol#L159) 
```solidity
158:    function _domainNameAndVersion() internal pure override returns (string memory name, string memory version) {
``` 



[File:ExecutorPlugin.sol#L159](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ExecutorPlugin.sol#L159) 
```solidity
158:    function _domainNameAndVersion() internal pure override returns (string memory name, string memory version) {
``` 



 --- 

<a name=[NC-6]></a>
### [NC-6] Consider marking public function External - Instances: 4 

 
> If a public function is never called internally. It is best practice to mark it as external. 

 --- 

[File:SafeEnabler.sol#L43](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeEnabler.sol#L43) 
```solidity
42:    function enableModule(address module) public {
``` 



[File:SafeEnabler.sol#L66](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeEnabler.sol#L66) 
```solidity
65:    function setGuard(address guard) public {
``` 



[File:ConsoleFallbackHandler.sol#L39](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ConsoleFallbackHandler.sol#L39) 
```solidity
38:    function isValidSignature(bytes memory _data, bytes memory _signature) public view override returns (bytes4) {
``` 



[File:ConsoleFallbackHandler.sol#L60](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ConsoleFallbackHandler.sol#L60) 
```solidity
59:    function getMessageHash(bytes memory message) public view returns (bytes32) {
``` 



 --- 

<a name=[NC-7]></a>
### [NC-7] This error has no parameters, the state of the contract when the revert occured will not be available - Instances: 30 

 
> Consider adding parameters to the error to provide more context when a transaction fails
 

 --- 

[File:TransactionValidator.sol#L19](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/TransactionValidator.sol#L19) 
```solidity
18:    error TxnUnAuthorized();
``` 



[File:TransactionValidator.sol#L20](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/TransactionValidator.sol#L20) 
```solidity
19:    error InvalidGuard();
``` 



[File:TransactionValidator.sol#L21](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/TransactionValidator.sol#L21) 
```solidity
20:    error InvalidFallbackHandler();
``` 



[File:TransactionValidator.sol#L22](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/TransactionValidator.sol#L22) 
```solidity
21:    error InvalidModule();
``` 



[File:WalletRegistry.sol#L15](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/registries/WalletRegistry.sol#L15) 
```solidity
14:    error AlreadyRegistered();
``` 



[File:WalletRegistry.sol#L16](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/registries/WalletRegistry.sol#L16) 
```solidity
15:    error InvalidSender();
``` 



[File:WalletRegistry.sol#L17](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/registries/WalletRegistry.sol#L17) 
```solidity
16:    error IsSubAccount();
``` 



[File:SafeHelper.sol#L17](https://github.com/code-423n4/2023-10-brahma/blob/main/src/libraries/SafeHelper.sol#L17) 
```solidity
16:    error InvalidMultiSendInput();
``` 



[File:SafeHelper.sol#L18](https://github.com/code-423n4/2023-10-brahma/blob/main/src/libraries/SafeHelper.sol#L18) 
```solidity
17:    error SafeExecTransactionFailed();
``` 



[File:SafeHelper.sol#L19](https://github.com/code-423n4/2023-10-brahma/blob/main/src/libraries/SafeHelper.sol#L19) 
```solidity
18:    error UnableToParseOperation();
``` 



[File:AddressProviderService.sol#L19](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/AddressProviderService.sol#L19) 
```solidity
18:    error InvalidAddressProvider();
``` 



[File:AddressProviderService.sol#L21](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/AddressProviderService.sol#L21) 
```solidity
20:    error InvalidAddress();
``` 



[File:PolicyRegistry.sol#L16](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/registries/PolicyRegistry.sol#L16) 
```solidity
15:    error PolicyCommitInvalid();
``` 



[File:PolicyRegistry.sol#L17](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/registries/PolicyRegistry.sol#L17) 
```solidity
16:    error UnauthorizedPolicyUpdate();
``` 



[File:ExecutorPlugin.sol#L25](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ExecutorPlugin.sol#L25) 
```solidity
24:    error InvalidExecutor();
``` 



[File:ExecutorPlugin.sol#L26](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ExecutorPlugin.sol#L26) 
```solidity
25:    error InvalidSignature();
``` 



[File:ExecutorPlugin.sol#L27](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/ExecutorPlugin.sol#L27) 
```solidity
26:    error ModuleExecutionFailed();
``` 



[File:SafeEnabler.sol#L17](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeEnabler.sol#L17) 
```solidity
16:    error OnlyDelegateCall();
``` 



[File:SafeDeployer.sol#L37](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeDeployer.sol#L37) 
```solidity
36:    error InvalidCommitment();
``` 



[File:SafeDeployer.sol#L38](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeDeployer.sol#L38) 
```solidity
37:    error NotWallet();
``` 



[File:SafeDeployer.sol#L40](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeDeployer.sol#L40) 
```solidity
39:    error SafeProxyCreationFailed();
``` 



[File:ExecutorRegistry.sol#L19](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/registries/ExecutorRegistry.sol#L19) 
```solidity
18:    error NotOwnerWallet();
``` 



[File:ExecutorRegistry.sol#L20](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/registries/ExecutorRegistry.sol#L20) 
```solidity
19:    error AlreadyExists();
``` 



[File:ExecutorRegistry.sol#L21](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/registries/ExecutorRegistry.sol#L21) 
```solidity
20:    error DoesNotExist();
``` 



[File:AddressProvider.sol#L15](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/AddressProvider.sol#L15) 
```solidity
14:    error RegistryAlreadyExists();
``` 



[File:AddressProvider.sol#L16](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/AddressProvider.sol#L16) 
```solidity
15:    error AddressProviderUnsupported();
``` 



[File:AddressProvider.sol#L19](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/AddressProvider.sol#L19) 
```solidity
18:    error NullAddress();
``` 



[File:PolicyValidator.sol#L20](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/PolicyValidator.sol#L20) 
```solidity
19:    error InvalidSignature();
``` 



[File:PolicyValidator.sol#L21](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/PolicyValidator.sol#L21) 
```solidity
20:    error NoPolicyCommit();
``` 



[File:PolicyValidator.sol#L23](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/PolicyValidator.sol#L23) 
```solidity
22:    error InvalidSignatures();
``` 



 --- 

<a name=[NC-8]></a>
### [NC-8] This variables default value is the same as the value it is initialized with - Instances: 1 

 
> This is unnecessary and will have some overhead on Gas
     

 --- 

[File:SafeModeratorOverridable.sol#L21](https://github.com/code-423n4/2023-10-brahma/blob/main/src/core/SafeModeratorOverridable.sol#L21) 
```solidity
20:    uint8 public constant DIFFER_SAFE_MOD = 0;
``` 



 --- 


