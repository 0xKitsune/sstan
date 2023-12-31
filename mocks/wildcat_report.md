# Sstan - v0.1.0 

 --- 
 TODO: add description

# Summary




## Vulnerabilities 

 | Classification | Title | Instances | 
 |:-------:|:---------|:-------:| 
 | [[Low-0]](#[Low-0]) | Use a locked pragma version instead of a floating pragma version | 2 |
 | [[Low-1]](#[Low-1]) | Unsafe ERC20 Operation | 1 |
 | [[Low-2]](#[Low-2]) | Double Casting | 1 |
## Optimizations 

 | Classification | Title | Instances | 
 |:-------:|:---------|:-------:| 
 | [[Gas-0]](#[Gas-0]) | Tightly pack storage variables | 1 |
 | [[Gas-1]](#[Gas-1]) | Avoid Reading From Storage in a for loop | 3 |
 | [[Gas-2]](#[Gas-2]) | Mark storage variables as `constant` if they never change. | 3 |
 | [[Gas-3]](#[Gas-3]) | Mark storage variables as `immutable` if they never change after contract initialization. | 1 |
 | [[Gas-4]](#[Gas-4]) | Consider marking constants as private | 2 |
 | [[Gas-5]](#[Gas-5]) | `unchecked{++i}` instead of `i++` (or use assembly when applicable) | 4 |
 | [[Gas-6]](#[Gas-6]) | Cache Storage Variables in Memory | 4 |
 | [[Gas-7]](#[Gas-7]) | Use assembly to check for address(0) | 3 |
 | [[Gas-8]](#[Gas-8]) | Optimal Comparison | 1 |
 | [[Gas-9]](#[Gas-9]) | Use `calldata` instead of `memory` for function arguments that do not get mutated. | 7 |
 | [[Gas-10]](#[Gas-10]) | Use assembly to hash instead of Solidity | 2 |
 | [[Gas-11]](#[Gas-11]) | Use assembly for math (add, sub, mul, div) | 13 |
 | [[Gas-12]](#[Gas-12]) | Use assembly to write storage values | 3 |
 | [[Gas-13]](#[Gas-13]) | Event is not properly indexed. | 8 |
 | [[Gas-14]](#[Gas-14]) | Right shift or Left shift instead of dividing or multiplying by powers of two | 1 |
 | [[Gas-15]](#[Gas-15]) | Mark functions as payable (with discretion) | 10 |
 | [[Gas-16]](#[Gas-16]) | Cache array length during for loop definition. | 1 |
## Quality Assurance 

 | Classification | Title | Instances | 
 |:-------:|:---------|:-------:| 
 | [[NonCritical-0]](#[NonCritical-0]) | Private variables should contain a leading underscore | 23 |
 | [[NonCritical-1]](#[NonCritical-1]) | Constructor should check that all parameters are not 0 | 5 |
 | [[NonCritical-2]](#[NonCritical-2]) | Large contracts with many external functions should inherit an interface | 1 |
 | [[NonCritical-3]](#[NonCritical-3]) | This error has no parameters, the state of the contract when the revert occured will not be available | 72 |
 | [[NonCritical-4]](#[NonCritical-4]) | Function names should be in camelCase | 34 |
 | [[NonCritical-5]](#[NonCritical-5]) | Consider importing specific identifiers instead of the whole file | 54 |
 | [[NonCritical-6]](#[NonCritical-6]) | Require/Revert statements should be consistent across the codebase | 54 |
 | [[NonCritical-7]](#[NonCritical-7]) | Constant and immutable variable names should be in SCREAMING_SNAKE_CASE | 54 |
 | [[NonCritical-8]](#[NonCritical-8]) | Consider marking public function External | 9 |
 | [[NonCritical-9]](#[NonCritical-9]) | Remove any unused functions | 75 |
 | [[NonCritical-10]](#[NonCritical-10]) | Remove any unused returns | 6 |
 | [[NonCritical-11]](#[NonCritical-11]) | Function parameters should be in camelCase | 342 |
 | [[NonCritical-12]](#[NonCritical-12]) | Consider explicitly naming mapping parameters | 4 |

## Vulnerabilities - Total: 4 

<a name=[Low-0]></a>
### [Low-0] Use a locked pragma version instead of a floating pragma version - Instances: 2 

 > ""
        Floating pragma is a vulnerability in smart contract code that can cause unexpected behavior by allowing the compiler to use a specified range of versions. \n This can lead to issues such as using an older compiler version with known vulnerabilities, using a newer compiler version with undiscovered vulnerabilities, inconsistency across files using different versions, or unpredictable behavior because the compiler can use any version within the specified range. It is recommended to use a locked pragma version in order to avoid these potential vulnerabilities. In some cases it may be acceptable to use a floating pragma, such as when a contract is intended for consumption by other developers and needs to be compatible with a range of compiler versions.
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
        "" 

 --- 

[File:IWildcatArchController.sol#L2](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatArchController.sol#L2) 
```solidity
1:pragma solidity ^0.8.20;
``` 



[File:Chainalysis.sol#L2](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/Chainalysis.sol#L2) 
```solidity
1:pragma solidity ^0.8.20;
``` 



 --- 

<a name=[Low-1]></a>
### [Low-1] Unsafe ERC20 Operation - Instances: 1 

 > ""
        ERC20 operations can be unsafe due to different implementations and vulnerabilities in the standard. To account for this, either use OpenZeppelin's SafeERC20 library or wrap each operation in a require statement. \n
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
        "" 

 --- 

[File:WildcatSanctionsEscrow.sol#L38](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsEscrow.sol#L38) 
```solidity
37:    IERC20(asset).transfer(account, amount);
``` 



 --- 

<a name=[Low-2]></a>
### [Low-2] Double Casting - Instances: 1 

 > Avoid double casting as it may introduce unexpected truncations/rounding errors among other issues. 

 --- 

[File:WildcatSanctionsSentinel.sol#L72](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsSentinel.sol#L72) 
```solidity
71:        uint160(
72:          uint256(
73:            keccak256(
74:              abi.encodePacked(
75:                bytes1(0xff),
76:                address(this),
77:                keccak256(abi.encode(borrower, account, asset)),
78:                WildcatSanctionsEscrowInitcodeHash
79:              )
80:            )
81:          )
82:        )
83:      );
``` 



 --- 



## Optimizations - Total: 67 

<a name=[Gas-0]></a>
### [Gas-0] Tightly pack storage variables - Instances: 1 

 > 
 When defining storage variables, make sure to declare them in ascending order, according to size. When multiple variables are able to fit into one 256 bit slot, this will save storage size and gas during runtime. For example, if you have a `bool`, `uint256` and a `bool`, instead of defining the variables in the previously mentioned order, defining the two boolean variables first will pack them both into one storage slot since they only take up one byte of storage. - Savings: ~0 
 

 --- 

[File:WildcatMarketBase.sol#L24](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L24) 
```solidity
23:  string public constant version = '1.0';
``` 



 --- 

<a name=[Gas-1]></a>
### [Gas-1] Avoid Reading From Storage in a for loop - Instances: 3 

 > 
  - Savings: ~0 
 

 --- 

[File:WildcatArchController.sol#L93](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L93) 
```solidity
92:    for (uint256 i = 0; i < count; i++) {
93:      arr[i] = _borrowers.at(start + i);
94:    }
95:  }
``` 



[File:WildcatArchController.sol#L136](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L136) 
```solidity
135:    for (uint256 i = 0; i < count; i++) {
136:      arr[i] = _controllerFactories.at(start + i);
137:    }
138:  }
``` 



[File:WildcatArchController.sol#L179](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L179) 
```solidity
178:    for (uint256 i = 0; i < count; i++) {
179:      arr[i] = _controllers.at(start + i);
180:    }
181:  }
``` 



[File:WildcatArchController.sol#L222](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L222) 
```solidity
221:    for (uint256 i = 0; i < count; i++) {
222:      arr[i] = _markets.at(start + i);
223:    }
224:  }
``` 



[File:WildcatMarketControllerFactory.sol#L146](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L146) 
```solidity
145:    for (uint256 i = 0; i < count; i++) {
146:      arr[i] = _deployedControllers.at(start + i);
147:    }
148:  }
``` 



[File:WildcatMarketController.sol#L133](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L133) 
```solidity
132:    for (uint256 i = 0; i < count; i++) {
133:      arr[i] = _authorizedLenders.at(start + i);
134:    }
135:  }
``` 



[File:WildcatMarketController.sol#L154](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L154) 
```solidity
153:    for (uint256 i = 0; i < lenders.length; i++) {
154:      address lender = lenders[i];
155:      if (_authorizedLenders.add(lender)) {
156:        emit LenderAuthorized(lender);
157:      }
158:    }
159:  }
``` 



[File:WildcatMarketController.sol#L154](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L154) 
```solidity
153:    for (uint256 i = 0; i < lenders.length; i++) {
154:      address lender = lenders[i];
155:      if (_authorizedLenders.add(lender)) {
156:        emit LenderAuthorized(lender);
157:      }
158:    }
159:  }
``` 



[File:WildcatMarketController.sol#L170](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L170) 
```solidity
169:    for (uint256 i = 0; i < lenders.length; i++) {
170:      address lender = lenders[i];
171:      if (_authorizedLenders.remove(lender)) {
172:        emit LenderDeauthorized(lender);
173:      }
174:    }
175:  }
``` 



[File:WildcatMarketController.sol#L170](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L170) 
```solidity
169:    for (uint256 i = 0; i < lenders.length; i++) {
170:      address lender = lenders[i];
171:      if (_authorizedLenders.remove(lender)) {
172:        emit LenderDeauthorized(lender);
173:      }
174:    }
175:  }
``` 



[File:WildcatMarketController.sol#L183](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L183) 
```solidity
182:    for (uint256 i; i < markets.length; i++) {
183:      address market = markets[i];
184:      if (!_controlledMarkets.contains(market)) {
185:        revert NotControlledMarket();
186:      }
187:      WildcatMarket(market).updateAccountAuthorization(lender, _authorizedLenders.contains(lender));
188:    }
189:  }
``` 



[File:WildcatMarketController.sol#L183](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L183) 
```solidity
182:    for (uint256 i; i < markets.length; i++) {
183:      address market = markets[i];
184:      if (!_controlledMarkets.contains(market)) {
185:        revert NotControlledMarket();
186:      }
187:      WildcatMarket(market).updateAccountAuthorization(lender, _authorizedLenders.contains(lender));
188:    }
189:  }
``` 



[File:WildcatMarketController.sol#L212](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L212) 
```solidity
211:    for (uint256 i = 0; i < count; i++) {
212:      arr[i] = _controlledMarkets.at(start + i);
213:    }
214:  }
``` 



 --- 

<a name=[Gas-2]></a>
### [Gas-2] Mark storage variables as `constant` if they never change. - Instances: 3 

 > 
 State variables can be declared as constant or immutable. In both cases, the variables cannot be modified after the contract has been constructed. For constant variables, the value has to be fixed at compile-time, while for immutable, it can still be assigned at construction time. 
 The compiler does not reserve a storage slot for these variables, and every occurrence is inlined by the respective value. 
 Compared to regular state variables, the gas costs of constant and immutable variables are much lower. For a constant variable, the expression assigned to it is copied to all the places where it is accessed and also re-evaluated each time. This allows for local optimizations. Immutable variables are evaluated once at construction time and their value is copied to all the places in the code where they are accessed. For these values, 32 bytes are reserved, even if they would fit in fewer bytes. Due to this, constant values can sometimes be cheaper than immutable values. - Savings: ~2103 
 

 --- 

[File:WildcatMarketControllerFactory.sol#L36](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L36) 
```solidity
35:  address public immutable marketInitCodeStorage;
``` 



[File:WildcatMarketControllerFactory.sol#L38](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L38) 
```solidity
37:  uint256 public immutable marketInitCodeHash;
``` 



[File:WildcatMarketControllerFactory.sol#L44](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L44) 
```solidity
43:  uint256 internal immutable ownCreate2Prefix = LibStoredInitCode.getCreate2Prefix(address(this));
``` 



[File:WildcatMarketControllerFactory.sol#L40](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L40) 
```solidity
39:  address public immutable controllerInitCodeStorage;
``` 



[File:WildcatMarketControllerFactory.sol#L42](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L42) 
```solidity
41:  uint256 public immutable controllerInitCodeHash;
``` 



[File:WildcatMarketController.sol#L53](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L53) 
```solidity
52:  uint256 internal immutable ownCreate2Prefix = LibStoredInitCode.getCreate2Prefix(address(this));
``` 



[File:WildcatSanctionsEscrow.sol#L13](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsEscrow.sol#L13) 
```solidity
12:  address public immutable override account;
``` 



[File:WildcatSanctionsEscrow.sol#L12](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsEscrow.sol#L12) 
```solidity
11:  address public immutable override borrower;
``` 



[File:WildcatSanctionsEscrow.sol#L14](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsEscrow.sol#L14) 
```solidity
13:  address internal immutable asset;
``` 



 --- 

<a name=[Gas-3]></a>
### [Gas-3] Mark storage variables as `immutable` if they never change after contract initialization. - Instances: 1 

 > 
 State variables can be declared as constant or immutable. In both cases, the variables cannot be modified after the contract has been constructed. For constant variables, the value has to be fixed at compile-time, while for immutable, it can still be assigned at construction time. 
 The compiler does not reserve a storage slot for these variables, and every occurrence is inlined by the respective value. 
 Compared to regular state variables, the gas costs of constant and immutable variables are much lower. For a constant variable, the expression assigned to it is copied to all the places where it is accessed and also re-evaluated each time. This allows for local optimizations. Immutable variables are evaluated once at construction time and their value is copied to all the places in the code where they are accessed. For these values, 32 bytes are reserved, even if they would fit in fewer bytes. Due to this, constant values can sometimes be cheaper than immutable values. 
 - Savings: ~2103 
 

 --- 

[File:WildcatMarketBase.sol#L60](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L60) 
```solidity
59:  string public symbol;
``` 



[File:WildcatMarketBase.sol#L57](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L57) 
```solidity
56:  string public name;
``` 



 --- 

<a name=[Gas-4]></a>
### [Gas-4] Consider marking constants as private - Instances: 2 

 > 
 Consider marking constant variables in storage as private to save gas (unless a constant variable should be easily accessible by another protocol or offchain logic). - Savings: ~22 
 

 --- 

[File:WildcatMarketBase.sol#L24](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L24) 
```solidity
23:  string public constant version = '1.0';
``` 



[File:WildcatSanctionsSentinel.sol#L11](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsSentinel.sol#L11) 
```solidity
10:  bytes32 public constant override WildcatSanctionsEscrowInitcodeHash =
11:    keccak256(type(WildcatSanctionsEscrow).creationCode);
``` 



 --- 

<a name=[Gas-5]></a>
### [Gas-5] `unchecked{++i}` instead of `i++` (or use assembly when applicable) - Instances: 4 

 > 
 Use `++i` instead of `i++`. This is especially useful in for loops but this optimization can be used anywhere in your code. You can also use `unchecked{++i;}` for even more gas savings but this will not check to see if `i` overflows. For extra safety if you are worried about this, you can add a require statement after the loop checking if `i` is equal to the final incremented value. For best gas savings, use inline assembly, however this limits the functionality you can achieve. For example you cant use Solidity syntax to internally call your own contract within an assembly block and external calls must be done with the `call()` or `delegatecall()` instruction. However when applicable, inline assembly will save much more gas. - Savings: ~342 
 

 --- 

[File:WildcatMarketControllerFactory.sol#L146](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L146) 
```solidity
145:    for (uint256 i = 0; i < count; i++) {
``` 



[File:WildcatMarketController.sol#L183](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L183) 
```solidity
182:    for (uint256 i; i < markets.length; i++) {
``` 



[File:WildcatMarketController.sol#L212](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L212) 
```solidity
211:    for (uint256 i = 0; i < count; i++) {
``` 



[File:WildcatMarketController.sol#L133](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L133) 
```solidity
132:    for (uint256 i = 0; i < count; i++) {
``` 



[File:WildcatMarketController.sol#L170](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L170) 
```solidity
169:    for (uint256 i = 0; i < lenders.length; i++) {
``` 



[File:WildcatMarketController.sol#L154](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L154) 
```solidity
153:    for (uint256 i = 0; i < lenders.length; i++) {
``` 



[File:FIFOQueue.sol#L48](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FIFOQueue.sol#L48) 
```solidity
47:    for (uint256 i = 0; i < len; i++) {
``` 



[File:FIFOQueue.sol#L75](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FIFOQueue.sol#L75) 
```solidity
74:    for (uint256 i = 0; i < n; i++) {
``` 



[File:WildcatArchController.sol#L222](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L222) 
```solidity
221:    for (uint256 i = 0; i < count; i++) {
``` 



[File:WildcatArchController.sol#L179](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L179) 
```solidity
178:    for (uint256 i = 0; i < count; i++) {
``` 



[File:WildcatArchController.sol#L136](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L136) 
```solidity
135:    for (uint256 i = 0; i < count; i++) {
``` 



[File:WildcatArchController.sol#L93](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L93) 
```solidity
92:    for (uint256 i = 0; i < count; i++) {
``` 



 --- 

<a name=[Gas-6]></a>
### [Gas-6] Cache Storage Variables in Memory - Instances: 4 

 > 
  - Savings: ~0 
 

 --- 

[File:WildcatMarketController.sol#L134](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L134) 
```solidity
133:      arr[i] = _authorizedLenders.at(start + i);
``` 



[File:WildcatMarketController.sol#L213](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L213) 
```solidity
212:      arr[i] = _controlledMarkets.at(start + i);
``` 



[File:WildcatMarketController.sol#L240](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L240) 
```solidity
239:    parameters.namePrefix = _tmpMarketParameters.namePrefix;
``` 



[File:WildcatMarketController.sol#L241](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L241) 
```solidity
240:    parameters.symbolPrefix = _tmpMarketParameters.symbolPrefix;
``` 



[File:WildcatMarketController.sol#L244](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L244) 
```solidity
243:    parameters.feeRecipient = _tmpMarketParameters.feeRecipient;
``` 



[File:WildcatMarketController.sol#L246](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L246) 
```solidity
245:    parameters.maxTotalSupply = _tmpMarketParameters.maxTotalSupply;
``` 



[File:WildcatMarketController.sol#L247](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L247) 
```solidity
246:    parameters.protocolFeeBips = _tmpMarketParameters.protocolFeeBips;
``` 



[File:WildcatMarketController.sol#L248](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L248) 
```solidity
247:    parameters.annualInterestBips = _tmpMarketParameters.annualInterestBips;
``` 



[File:WildcatMarketController.sol#L249](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L249) 
```solidity
248:    parameters.delinquencyFeeBips = _tmpMarketParameters.delinquencyFeeBips;
``` 



[File:WildcatMarketController.sol#L250](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L250) 
```solidity
249:    parameters.withdrawalBatchDuration = _tmpMarketParameters.withdrawalBatchDuration;
``` 



[File:WildcatMarketController.sol#L251](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L251) 
```solidity
250:    parameters.reserveRatioBips = _tmpMarketParameters.reserveRatioBips;
``` 



[File:WildcatMarketController.sol#L252](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L252) 
```solidity
251:    parameters.delinquencyGracePeriod = _tmpMarketParameters.delinquencyGracePeriod;
``` 



[File:WildcatMarketController.sol#L257](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L257) 
```solidity
256:    _tmpMarketParameters.namePrefix = '_';
``` 



[File:WildcatMarketController.sol#L258](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L258) 
```solidity
257:    _tmpMarketParameters.symbolPrefix = '_';
``` 



[File:WildcatMarketController.sol#L259](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L259) 
```solidity
258:    _tmpMarketParameters.feeRecipient = address(1);
``` 



[File:WildcatMarketController.sol#L260](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L260) 
```solidity
259:    _tmpMarketParameters.protocolFeeBips = 1;
``` 



[File:WildcatMarketController.sol#L261](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L261) 
```solidity
260:    _tmpMarketParameters.maxTotalSupply = 1;
``` 



[File:WildcatMarketController.sol#L262](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L262) 
```solidity
261:    _tmpMarketParameters.annualInterestBips = 1;
``` 



[File:WildcatMarketController.sol#L263](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L263) 
```solidity
262:    _tmpMarketParameters.delinquencyFeeBips = 1;
``` 



[File:WildcatMarketController.sol#L264](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L264) 
```solidity
263:    _tmpMarketParameters.withdrawalBatchDuration = 1;
``` 



[File:WildcatMarketController.sol#L265](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L265) 
```solidity
264:    _tmpMarketParameters.reserveRatioBips = 1;
``` 



[File:WildcatMarketController.sol#L266](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L266) 
```solidity
265:    _tmpMarketParameters.delinquencyGracePeriod = 1;
``` 



[File:WildcatMarketBase.sol#L178](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L178) 
```solidity
177:        _accounts[escrow].scaledBalance += scaledBalance;
``` 



[File:WildcatMarketBase.sol#L185](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L185) 
```solidity
184:      _accounts[accountAddress] = account;
``` 



[File:WildcatMarketBase.sol#L484](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L484) 
```solidity
483:      _withdrawalData.unpaidBatches.push(expiry);
``` 



[File:WildcatMarketBase.sol#L491](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L491) 
```solidity
490:    _withdrawalData.batches[expiry] = batch;
``` 



[File:WildcatMarketControllerFactory.sol#L147](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L147) 
```solidity
146:      arr[i] = _deployedControllers.at(start + i);
``` 



[File:WildcatMarketControllerFactory.sol#L177](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L177) 
```solidity
176:      _protocolFeeConfiguration.originationFeeAsset,
``` 



[File:WildcatMarketControllerFactory.sol#L178](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L178) 
```solidity
177:      _protocolFeeConfiguration.originationFeeAmount,
``` 



[File:WildcatMarketControllerFactory.sol#L179](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L179) 
```solidity
178:      _protocolFeeConfiguration.protocolFeeBips
``` 



[File:WildcatMarketControllerFactory.sol#L298](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L298) 
```solidity
297:    _tmpMarketBorrowerParameter = address(1);
``` 



[File:WildcatArchController.sol#L94](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L94) 
```solidity
93:      arr[i] = _borrowers.at(start + i);
``` 



[File:WildcatArchController.sol#L137](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L137) 
```solidity
136:      arr[i] = _controllerFactories.at(start + i);
``` 



[File:WildcatArchController.sol#L180](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L180) 
```solidity
179:      arr[i] = _controllers.at(start + i);
``` 



[File:WildcatArchController.sol#L223](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L223) 
```solidity
222:      arr[i] = _markets.at(start + i);
``` 



 --- 

<a name=[Gas-7]></a>
### [Gas-7] Use assembly to check for address(0) - Instances: 3 

 > 
  - Savings: ~6 
 

 --- 

[File:WildcatMarketBase.sol#L79](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L79) 
```solidity
78:    if ((parameters.protocolFeeBips > 0).and(parameters.feeRecipient == address(0))) {
``` 



[File:WildcatMarketController.sol#L345](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L345) 
```solidity
344:    if (originationFeeAsset != address(0)) {
``` 



[File:WildcatMarketControllerFactory.sol#L202](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L202) 
```solidity
201:    bool nullFeeRecipient = feeRecipient == address(0);
``` 



[File:WildcatMarketControllerFactory.sol#L203](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L203) 
```solidity
202:    bool nullOriginationFeeAsset = originationFeeAsset == address(0);
``` 



 --- 

<a name=[Gas-8]></a>
### [Gas-8] Optimal Comparison - Instances: 1 

 > 
 When comparing integers, it is cheaper to use strict `>` & `<` operators over `>=` & `<=` operators, even if you must increment or decrement one of the operands. 
 Note: before using this technique, it's important to consider whether incrementing/decrementing one of the operators could result in an over/underflow. This optimization is applicable when the optimizer is turned off. - Savings: ~3 
 

 --- 

[File:FIFOQueue.sol#L32](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FIFOQueue.sol#L32) 
```solidity
31:    if (index >= arr.nextIndex) {
``` 



 --- 

<a name=[Gas-9]></a>
### [Gas-9] Use `calldata` instead of `memory` for function arguments that do not get mutated. - Instances: 7 

 > 
 Mark data types as `calldata` instead of `memory` where possible. This makes it so that the data is not automatically loaded into memory. If the data passed into the function does not need to be changed (like updating values in an array), it can be passed in as `calldata`. The one exception to this is if the argument must later be passed into another function that takes an argument that specifies `memory` storage. - Savings: ~1716 
 

 --- 

[File:WildcatMarketControllerFactory.sol#L318](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L318) 
```solidity
317:    string memory namePrefix,
``` 



[File:WildcatMarketControllerFactory.sol#L319](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L319) 
```solidity
318:    string memory symbolPrefix,
``` 



[File:WildcatMarketBase.sol#L163](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L163) 
```solidity
162:  function _blockAccount(MarketState memory state, address accountAddress) internal {
``` 



[File:WildcatMarketBase.sol#L448](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L448) 
```solidity
447:  function _writeState(MarketState memory state) internal {
``` 



[File:WildcatMarketBase.sol#L466](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L466) 
```solidity
465:  function _processExpiredWithdrawalBatch(MarketState memory state) internal {
``` 



[File:WildcatMarketBase.sol#L499](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L499) 
```solidity
498:    WithdrawalBatch memory batch,
``` 



[File:WildcatMarketBase.sol#L500](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L500) 
```solidity
499:    MarketState memory state,
``` 



[File:WildcatMarketBase.sol#L529](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L529) 
```solidity
528:    WithdrawalBatch memory batch,
``` 



[File:WildcatMarketBase.sol#L530](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L530) 
```solidity
529:    MarketState memory state,
``` 



[File:Withdrawal.sol#L38](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/Withdrawal.sol#L38) 
```solidity
37:  function scaledOwedAmount(WithdrawalBatch memory batch) internal pure returns (uint104) {
``` 



[File:Withdrawal.sol#L49](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/Withdrawal.sol#L49) 
```solidity
48:    MarketState memory state,
``` 



[File:Withdrawal.sol#L48](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/Withdrawal.sol#L48) 
```solidity
47:    WithdrawalBatch memory batch,
``` 



[File:MarketState.sol#L51](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MarketState.sol#L51) 
```solidity
50:  function totalSupply(MarketState memory state) internal pure returns (uint256) {
``` 



[File:MarketState.sol#L59](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MarketState.sol#L59) 
```solidity
58:  function maximumDeposit(MarketState memory state) internal pure returns (uint256) {
``` 



[File:MarketState.sol#L67](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MarketState.sol#L67) 
```solidity
66:    MarketState memory state,
``` 



[File:MarketState.sol#L76](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MarketState.sol#L76) 
```solidity
75:  function scaleAmount(MarketState memory state, uint256 amount) internal pure returns (uint256) {
``` 



[File:MarketState.sol#L88](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MarketState.sol#L88) 
```solidity
87:    MarketState memory state
88:  ) internal pure returns (uint256 _liquidityRequired) {
``` 



[File:MarketState.sol#L106](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MarketState.sol#L106) 
```solidity
105:    MarketState memory state,
``` 



[File:MarketState.sol#L124](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MarketState.sol#L124) 
```solidity
123:    MarketState memory state,
``` 



[File:MarketState.sol#L130](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MarketState.sol#L130) 
```solidity
129:  function hasPendingExpiredBatch(MarketState memory state) internal view returns (bool result) {
``` 



[File:MarketState.sol#L138](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MarketState.sol#L138) 
```solidity
137:  function totalDebts(MarketState memory state) internal pure returns (uint256) {
``` 



[File:WildcatMarketController.sol#L153](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L153) 
```solidity
152:  function authorizeLenders(address[] memory lenders) external onlyBorrower {
``` 



[File:WildcatMarketController.sol#L169](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L169) 
```solidity
168:  function deauthorizeLenders(address[] memory lenders) external onlyBorrower {
``` 



[File:WildcatMarketController.sol#L182](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L182) 
```solidity
181:  function updateLenderAuthorization(address lender, address[] memory markets) external {
``` 



[File:WildcatMarketController.sol#L224](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L224) 
```solidity
223:    string memory symbolPrefix
224:  ) external view returns (address) {
``` 



[File:WildcatMarketController.sol#L223](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L223) 
```solidity
222:    string memory namePrefix,
``` 



[File:WildcatMarketController.sol#L293](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L293) 
```solidity
292:    string memory namePrefix,
``` 



[File:WildcatMarketController.sol#L294](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L294) 
```solidity
293:    string memory symbolPrefix,
``` 



[File:WildcatMarketController.sol#L372](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L372) 
```solidity
371:    string memory namePrefix,
``` 



[File:WildcatMarketController.sol#L373](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L373) 
```solidity
372:    string memory symbolPrefix
373:  ) internal pure returns (bytes32 salt) {
``` 



[File:WildcatMarketController.sol#L395](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L395) 
```solidity
394:    string memory namePrefix,
``` 



[File:WildcatMarketController.sol#L396](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L396) 
```solidity
395:    string memory symbolPrefix,
``` 



[File:FeeMath.sol#L31](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FeeMath.sol#L31) 
```solidity
30:    MarketState memory state,
``` 



[File:FeeMath.sol#L41](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FeeMath.sol#L41) 
```solidity
40:    MarketState memory state,
``` 



[File:FeeMath.sol#L54](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FeeMath.sol#L54) 
```solidity
53:    MarketState memory state,
``` 



[File:FeeMath.sol#L90](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FeeMath.sol#L90) 
```solidity
89:    MarketState memory state,
``` 



[File:FeeMath.sol#L143](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FeeMath.sol#L143) 
```solidity
142:    MarketState memory state,
``` 



[File:LibStoredInitCode.sol#L7](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/LibStoredInitCode.sol#L7) 
```solidity
6:  function deployInitCode(bytes memory data) internal returns (address initCodeStorage) {
``` 



 --- 

<a name=[Gas-10]></a>
### [Gas-10] Use assembly to hash instead of Solidity - Instances: 2 

 > 
 Hashing is a safe operation to perform in assembly, and it is cheaper than Solidity's `keccak256` function. - Savings: ~82 
 

 --- 

[File:WildcatMarketControllerFactory.sol#L112](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L112) 
```solidity
111:    initCodeHash = uint256(keccak256(controllerInitCode));
``` 



[File:WildcatMarketControllerFactory.sol#L122](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L122) 
```solidity
121:    initCodeHash = uint256(keccak256(marketInitCode));
``` 



[File:WildcatSanctionsSentinel.sol#L12](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsSentinel.sol#L12) 
```solidity
11:    keccak256(type(WildcatSanctionsEscrow).creationCode);
``` 



[File:WildcatSanctionsSentinel.sol#L74](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsSentinel.sol#L74) 
```solidity
73:            keccak256(
74:              abi.encodePacked(
75:                bytes1(0xff),
76:                address(this),
77:                keccak256(abi.encode(borrower, account, asset)),
78:                WildcatSanctionsEscrowInitcodeHash
79:              )
80:            )
81:          )
``` 



[File:WildcatSanctionsSentinel.sol#L78](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsSentinel.sol#L78) 
```solidity
77:                keccak256(abi.encode(borrower, account, asset)),
``` 



[File:WildcatSanctionsSentinel.sol#L110](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsSentinel.sol#L110) 
```solidity
109:    new WildcatSanctionsEscrow{ salt: keccak256(abi.encode(borrower, account, asset)) }();
``` 



 --- 

<a name=[Gas-11]></a>
### [Gas-11] Use assembly for math (add, sub, mul, div) - Instances: 13 

 > 
 Use assembly for math instead of Solidity. You can check for overflow/underflow in assembly to ensure safety. If using Solidity versions < 0.8.0 and you are using Safemath, you can gain significant gas savings by using assembly to calculate values and checking for overflow/underflow. - Savings: ~60 
 

 --- 

[File:WildcatMarketToken.sol#L50](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketToken.sol#L50) 
```solidity
49:      uint256 newAllowance = allowed - amount;
``` 



[File:MarketState.sol#L91](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MarketState.sol#L91) 
```solidity
90:    uint256 scaledRequiredReserves = (state.scaledTotalSupply - scaledWithdrawals).bipMul(
91:      state.reserveRatioBips
92:    ) + scaledWithdrawals;
``` 



[File:MarketState.sol#L91](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MarketState.sol#L91) 
```solidity
90:    uint256 scaledRequiredReserves = (state.scaledTotalSupply - scaledWithdrawals).bipMul(
``` 



[File:MarketState.sol#L95](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MarketState.sol#L95) 
```solidity
94:      state.normalizeAmount(scaledRequiredReserves) +
95:      state.accruedProtocolFees +
96:      state.normalizedUnclaimedWithdrawals;
``` 



[File:MarketState.sol#L95](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MarketState.sol#L95) 
```solidity
94:      state.normalizeAmount(scaledRequiredReserves) +
95:      state.accruedProtocolFees +
``` 



[File:MarketState.sol#L109](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MarketState.sol#L109) 
```solidity
108:    uint256 totalAvailableAssets = totalAssets - state.normalizedUnclaimedWithdrawals;
``` 



[File:MarketState.sol#L140](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MarketState.sol#L140) 
```solidity
139:      state.normalizeAmount(state.scaledTotalSupply) +
140:      state.normalizedUnclaimedWithdrawals +
141:      state.accruedProtocolFees;
``` 



[File:MarketState.sol#L140](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MarketState.sol#L140) 
```solidity
139:      state.normalizeAmount(state.scaledTotalSupply) +
140:      state.normalizedUnclaimedWithdrawals +
``` 



[File:WildcatMarketController.sol#L131](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L131) 
```solidity
130:    uint256 count = end - start;
``` 



[File:WildcatMarketController.sol#L134](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L134) 
```solidity
133:      arr[i] = _authorizedLenders.at(start + i);
``` 



[File:WildcatMarketController.sol#L210](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L210) 
```solidity
209:    uint256 count = end - start;
``` 



[File:WildcatMarketController.sol#L213](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L213) 
```solidity
212:      arr[i] = _controlledMarkets.at(start + i);
``` 



[File:WildcatMarketController.sol#L484](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L484) 
```solidity
483:      tmp.expiry = uint128(block.timestamp + 2 weeks);
``` 



[File:WildcatMarket.sol#L154](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarket.sol#L154) 
```solidity
153:      asset.safeTransferFrom(borrower, address(this), totalDebts - currentlyHeld);
``` 



[File:WildcatMarket.sol#L157](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarket.sol#L157) 
```solidity
156:      asset.safeTransfer(borrower, currentlyHeld - totalDebts);
``` 



[File:Withdrawal.sol#L39](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/Withdrawal.sol#L39) 
```solidity
38:    return batch.scaledTotalAmount - batch.scaledAmountBurned;
``` 



[File:Withdrawal.sol#L54](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/Withdrawal.sol#L54) 
```solidity
53:    uint256 priorScaledAmountPending = (state.scaledPendingWithdrawals - batch.scaledOwedAmount());
``` 



[File:Withdrawal.sol#L55](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/Withdrawal.sol#L55) 
```solidity
54:    uint256 unavailableAssets = state.normalizedUnclaimedWithdrawals +
55:      state.normalizeAmount(priorScaledAmountPending) +
56:      state.accruedProtocolFees;
``` 



[File:Withdrawal.sol#L55](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/Withdrawal.sol#L55) 
```solidity
54:    uint256 unavailableAssets = state.normalizedUnclaimedWithdrawals +
55:      state.normalizeAmount(priorScaledAmountPending) +
``` 



[File:MathUtils.sol#L35](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L35) 
```solidity
34:    uint256 accumulatedInterestRay = rate * timeDelta;
``` 



[File:MathUtils.sol#L37](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L37) 
```solidity
36:      return accumulatedInterestRay / SECONDS_IN_365_DAYS;
``` 



[File:FeeMath.sol#L24](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FeeMath.sol#L24) 
```solidity
23:    uint256 accumulatedInterestRay = rate * timeDelta;
``` 



[File:FeeMath.sol#L26](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FeeMath.sol#L26) 
```solidity
25:      return accumulatedInterestRay / SECONDS_IN_365_DAYS;
``` 



[File:FeeMath.sol#L36](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FeeMath.sol#L36) 
```solidity
35:      timestamp - state.lastInterestAccruedTimestamp
36:    );
``` 



[File:FeeMath.sol#L50](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FeeMath.sol#L50) 
```solidity
49:    state.accruedProtocolFees = (state.accruedProtocolFees + protocolFee).toUint128();
``` 



[File:FeeMath.sol#L64](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FeeMath.sol#L64) 
```solidity
63:      timestamp - state.lastInterestAccruedTimestamp
64:    );
``` 



[File:FeeMath.sol#L100](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FeeMath.sol#L100) 
```solidity
99:      state.timeDelinquent = (previousTimeDelinquent + timeDelta).toUint32();
``` 



[File:FeeMath.sol#L169](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FeeMath.sol#L169) 
```solidity
168:    uint256 scaleFactorDelta = prevScaleFactor.rayMul(baseInterestRay + delinquencyFeeRay);
``` 



[File:FeeMath.sol#L171](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FeeMath.sol#L171) 
```solidity
170:    state.scaleFactor = (prevScaleFactor + scaleFactorDelta).toUint112();
``` 



[File:WildcatMarketControllerFactory.sol#L144](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L144) 
```solidity
143:    uint256 count = end - start;
``` 



[File:WildcatMarketControllerFactory.sol#L147](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L147) 
```solidity
146:      arr[i] = _deployedControllers.at(start + i);
``` 



[File:WildcatArchController.sol#L91](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L91) 
```solidity
90:    uint256 count = end - start;
``` 



[File:WildcatArchController.sol#L94](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L94) 
```solidity
93:      arr[i] = _borrowers.at(start + i);
``` 



[File:WildcatArchController.sol#L134](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L134) 
```solidity
133:    uint256 count = end - start;
``` 



[File:WildcatArchController.sol#L137](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L137) 
```solidity
136:      arr[i] = _controllerFactories.at(start + i);
``` 



[File:WildcatArchController.sol#L177](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L177) 
```solidity
176:    uint256 count = end - start;
``` 



[File:WildcatArchController.sol#L180](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L180) 
```solidity
179:      arr[i] = _controllers.at(start + i);
``` 



[File:WildcatArchController.sol#L220](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L220) 
```solidity
219:    uint256 count = end - start;
``` 



[File:WildcatArchController.sol#L223](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L223) 
```solidity
222:      arr[i] = _markets.at(start + i);
``` 



[File:WildcatMarketWithdrawals.sol#L67](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketWithdrawals.sol#L67) 
```solidity
66:    return newTotalWithdrawn - previousTotalWithdrawn;
``` 



[File:WildcatMarketWithdrawals.sol#L95](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketWithdrawals.sol#L95) 
```solidity
94:      state.pendingWithdrawalExpiry = uint32(block.timestamp + withdrawalBatchDuration);
``` 



[File:WildcatMarketWithdrawals.sol#L155](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketWithdrawals.sol#L155) 
```solidity
154:    uint128 normalizedAmountWithdrawn = newTotalWithdrawn - status.normalizedAmountWithdrawn;
``` 



[File:WildcatMarketWithdrawals.sol#L200](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketWithdrawals.sol#L200) 
```solidity
199:    uint256 availableLiquidity = totalAssets() -
200:      (state.normalizedUnclaimedWithdrawals + state.accruedProtocolFees);
``` 



[File:WildcatMarketWithdrawals.sol#L201](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketWithdrawals.sol#L201) 
```solidity
200:      (state.normalizedUnclaimedWithdrawals + state.accruedProtocolFees);
``` 



[File:StringQuery.sol#L22](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/StringQuery.sol#L22) 
```solidity
21:    uint256 sizeInBits = 255 - uint256(value).ffs();
``` 



[File:StringQuery.sol#L23](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/StringQuery.sol#L23) 
```solidity
22:    size = (sizeInBits + 7) / 8;
``` 



[File:StringQuery.sol#L23](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/StringQuery.sol#L23) 
```solidity
22:    size = (sizeInBits + 7) / 8;
``` 



[File:StringQuery.sol#L72](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/StringQuery.sol#L72) 
```solidity
71:      uint256 sizeInBits = 255 - value.ffs();
``` 



[File:StringQuery.sol#L73](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/StringQuery.sol#L73) 
```solidity
72:      size = (sizeInBits + 7) / 8;
``` 



[File:StringQuery.sol#L73](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/StringQuery.sol#L73) 
```solidity
72:      size = (sizeInBits + 7) / 8;
``` 



[File:FIFOQueue.sol#L39](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FIFOQueue.sol#L39) 
```solidity
38:    return arr.nextIndex - arr.startIndex;
``` 



[File:FIFOQueue.sol#L45](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FIFOQueue.sol#L45) 
```solidity
44:    uint256 len = nextIndex - startIndex;
``` 



[File:FIFOQueue.sol#L49](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FIFOQueue.sol#L49) 
```solidity
48:      _values[i] = arr.data[startIndex + i];
``` 



[File:FIFOQueue.sol#L58](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FIFOQueue.sol#L58) 
```solidity
57:    arr.nextIndex = nextIndex + 1;
``` 



[File:FIFOQueue.sol#L67](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FIFOQueue.sol#L67) 
```solidity
66:    arr.startIndex = startIndex + 1;
``` 



[File:FIFOQueue.sol#L72](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FIFOQueue.sol#L72) 
```solidity
71:    if (startIndex + n > arr.nextIndex) {
``` 



[File:FIFOQueue.sol#L76](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FIFOQueue.sol#L76) 
```solidity
75:      delete arr.data[startIndex + i];
``` 



[File:FIFOQueue.sol#L78](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FIFOQueue.sol#L78) 
```solidity
77:    arr.startIndex = startIndex + n;
``` 



[File:WildcatMarketBase.sol#L321](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L321) 
```solidity
320:    uint256 apr = MathUtils.bipToRay(state.annualInterestBips).bipMul(BIP + protocolFeeBips);
``` 



[File:WildcatMarketBase.sol#L505](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L505) 
```solidity
504:    uint104 scaledAmountOwed = batch.scaledTotalAmount - batch.scaledAmountBurned;
``` 



[File:WildcatMarketBase.sol#L534](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L534) 
```solidity
533:    uint104 scaledAmountOwed = batch.scaledTotalAmount - batch.scaledAmountBurned;
``` 



 --- 

<a name=[Gas-12]></a>
### [Gas-12] Use assembly to write storage values - Instances: 3 

 > 
 You can save a fair amount of gas by using assembly to write storage values. - Savings: ~66 
 

 --- 

[File:ReentrancyGuard.sol#L51](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/ReentrancyGuard.sol#L51) 
```solidity
50:    _reentrancyGuard = _NOT_ENTERED;
``` 



[File:ReentrancyGuard.sol#L65](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/ReentrancyGuard.sol#L65) 
```solidity
64:      _reentrancyGuard = _ENTERED;
``` 



[File:ReentrancyGuard.sol#L74](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/ReentrancyGuard.sol#L74) 
```solidity
73:    _reentrancyGuard = _NOT_ENTERED;
``` 



[File:WildcatMarketBase.sol#L97](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L97) 
```solidity
96:    name = string.concat(parameters.namePrefix, queryName(parameters.asset));
``` 



[File:WildcatMarketBase.sol#L98](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L98) 
```solidity
97:    symbol = string.concat(parameters.symbolPrefix, querySymbol(parameters.asset));
``` 



[File:WildcatMarketControllerFactory.sol#L286](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L286) 
```solidity
285:    _tmpMarketBorrowerParameter = msg.sender;
``` 



[File:WildcatMarketControllerFactory.sol#L298](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L298) 
```solidity
297:    _tmpMarketBorrowerParameter = address(1);
``` 



 --- 

<a name=[Gas-13]></a>
### [Gas-13] Event is not properly indexed. - Instances: 8 

 > 
 When possible, always include a minimum of 3 indexed event topics to save gas - Savings: ~0 
 

 --- 

[File:IWildcatMarketControllerFactory.sol#L7](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketControllerFactory.sol#L7) 
```solidity
6:  event NewController(address borrower, address controller, string namePrefix, string symbolPrefix);
``` 



[File:IWildcatMarketControllerFactory.sol#L8](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketControllerFactory.sol#L8) 
```solidity
7:  event UpdateProtocolFeeConfiguration(
8:    address feeRecipient,
9:    uint16 protocolFeeBips,
10:    address originationFeeAsset,
11:    uint256 originationFeeAmount
12:  );
``` 



[File:IWildcatMarketControllerEventsAndErrors.sol#L39](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketControllerEventsAndErrors.sol#L39) 
```solidity
38:  event LenderAuthorized(address);
``` 



[File:IWildcatMarketControllerEventsAndErrors.sol#L41](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketControllerEventsAndErrors.sol#L41) 
```solidity
40:  event LenderDeauthorized(address);
``` 



[File:IERC20.sol#L5](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IERC20.sol#L5) 
```solidity
4:  event Transfer(address indexed from, address indexed to, uint256 value);
``` 



[File:IERC20.sol#L6](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IERC20.sol#L6) 
```solidity
5:  event Approval(address indexed owner, address indexed spender, uint256 value);
``` 



[File:IMarketEventsAndErrors.sol#L79](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L79) 
```solidity
78:  event Transfer(address indexed from, address indexed to, uint256 value);
``` 



[File:IMarketEventsAndErrors.sol#L81](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L81) 
```solidity
80:  event Approval(address indexed owner, address indexed spender, uint256 value);
``` 



[File:IMarketEventsAndErrors.sol#L83](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L83) 
```solidity
82:  event MaxTotalSupplyUpdated(uint256 assets);
``` 



[File:IMarketEventsAndErrors.sol#L85](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L85) 
```solidity
84:  event AnnualInterestBipsUpdated(uint256 annualInterestBipsUpdated);
``` 



[File:IMarketEventsAndErrors.sol#L87](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L87) 
```solidity
86:  event ReserveRatioBipsUpdated(uint256 reserveRatioBipsUpdated);
``` 



[File:IMarketEventsAndErrors.sol#L89](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L89) 
```solidity
88:  event SanctionedAccountAssetsSentToEscrow(address account, address escrow, uint256 amount);
``` 



[File:IMarketEventsAndErrors.sol#L91](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L91) 
```solidity
90:  event Deposit(address indexed account, uint256 assetAmount, uint256 scaledAmount);
``` 



[File:IMarketEventsAndErrors.sol#L93](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L93) 
```solidity
92:  event Borrow(uint256 assetAmount);
``` 



[File:IMarketEventsAndErrors.sol#L95](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L95) 
```solidity
94:  event MarketClosed(uint256 timestamp);
``` 



[File:IMarketEventsAndErrors.sol#L97](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L97) 
```solidity
96:  event FeesCollected(uint256 assets);
``` 



[File:IMarketEventsAndErrors.sol#L99](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L99) 
```solidity
98:  event StateUpdated(uint256 scaleFactor, bool isDelinquent);
``` 



[File:IMarketEventsAndErrors.sol#L101](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L101) 
```solidity
100:  event ScaleFactorUpdated(
101:    uint256 scaleFactor,
102:    uint256 baseInterestRay,
103:    uint256 delinquencyFeeRay,
104:    uint256 protocolFee
105:  );
``` 



[File:IMarketEventsAndErrors.sol#L108](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L108) 
```solidity
107:  event AuthorizationStatusUpdated(address indexed account, AuthRole role);
``` 



[File:IMarketEventsAndErrors.sol#L114](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L114) 
```solidity
113:  event WithdrawalBatchExpired(
114:    uint256 expiry,
115:    uint256 scaledTotalAmount,
116:    uint256 scaledAmountBurned,
117:    uint256 normalizedAmountPaid
118:  );
``` 



[File:IMarketEventsAndErrors.sol#L124](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L124) 
```solidity
123:  event WithdrawalBatchCreated(uint256 expiry);
``` 



[File:IMarketEventsAndErrors.sol#L129](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L129) 
```solidity
128:  event WithdrawalBatchClosed(uint256 expiry);
``` 



[File:IMarketEventsAndErrors.sol#L131](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L131) 
```solidity
130:  event WithdrawalBatchPayment(
131:    uint256 expiry,
132:    uint256 scaledAmountBurned,
133:    uint256 normalizedAmountPaid
134:  );
``` 



[File:IMarketEventsAndErrors.sol#L137](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L137) 
```solidity
136:  event WithdrawalQueued(uint256 expiry, address account, uint256 scaledAmount);
``` 



[File:IMarketEventsAndErrors.sol#L139](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L139) 
```solidity
138:  event WithdrawalExecuted(uint256 expiry, address account, uint256 normalizedAmount);
``` 



[File:IMarketEventsAndErrors.sol#L141](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L141) 
```solidity
140:  event Withdrawal(address indexed account, uint256 assetAmount, uint256 scaledAmount);
``` 



[File:IMarketEventsAndErrors.sol#L143](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L143) 
```solidity
142:  event SanctionedAccountWithdrawalSentToEscrow(
143:    address account,
144:    address escrow,
145:    uint32 expiry,
146:    uint256 amount
147:  );
``` 



[File:WildcatMarketControllerFactory.sol#L16](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L16) 
```solidity
15:  event NewController(address borrower, address controller, string namePrefix, string symbolPrefix);
``` 



[File:WildcatMarketControllerFactory.sol#L17](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L17) 
```solidity
16:  event UpdateProtocolFeeConfiguration(
17:    address feeRecipient,
18:    uint16 protocolFeeBips,
19:    address originationFeeAsset,
20:    uint256 originationFeeAmount
21:  );
``` 



[File:IWildcatArchController.sol#L15](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatArchController.sol#L15) 
```solidity
14:  event ControllerFactoryAdded(address);
``` 



[File:IWildcatArchController.sol#L17](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatArchController.sol#L17) 
```solidity
16:  event ControllerFactoryRemoved(address);
``` 



[File:IWildcatArchController.sol#L38](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatArchController.sol#L38) 
```solidity
37:  event ControllerAdded(address, address);
``` 



[File:IWildcatArchController.sol#L40](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatArchController.sol#L40) 
```solidity
39:  event ControllerRemoved(address);
``` 



[File:IWildcatArchController.sol#L61](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatArchController.sol#L61) 
```solidity
60:  event BorrowerAdded(address);
``` 



[File:IWildcatArchController.sol#L63](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatArchController.sol#L63) 
```solidity
62:  event BorrowerRemoved(address);
``` 



[File:IWildcatArchController.sol#L84](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatArchController.sol#L84) 
```solidity
83:  event MarketAdded(address, address);
``` 



[File:IWildcatArchController.sol#L86](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatArchController.sol#L86) 
```solidity
85:  event MarketRemoved(address);
``` 



[File:WildcatArchController.sol#L29](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L29) 
```solidity
28:  event MarketAdded(address indexed controller, address market);
``` 



[File:WildcatArchController.sol#L30](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L30) 
```solidity
29:  event MarketRemoved(address market);
``` 



[File:WildcatArchController.sol#L32](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L32) 
```solidity
31:  event ControllerFactoryAdded(address controllerFactory);
``` 



[File:WildcatArchController.sol#L33](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L33) 
```solidity
32:  event ControllerFactoryRemoved(address controllerFactory);
``` 



[File:WildcatArchController.sol#L35](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L35) 
```solidity
34:  event BorrowerAdded(address borrower);
``` 



[File:WildcatArchController.sol#L36](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L36) 
```solidity
35:  event BorrowerRemoved(address borrower);
``` 



[File:WildcatArchController.sol#L38](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L38) 
```solidity
37:  event ControllerAdded(address indexed controllerFactory, address controller);
``` 



[File:WildcatArchController.sol#L39](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L39) 
```solidity
38:  event ControllerRemoved(address controller);
``` 



[File:IWildcatSanctionsEscrow.sol#L5](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatSanctionsEscrow.sol#L5) 
```solidity
4:  event EscrowReleased(address indexed account, address indexed asset, uint256 amount);
``` 



 --- 

<a name=[Gas-14]></a>
### [Gas-14] Right shift or Left shift instead of dividing or multiplying by powers of two - Instances: 1 

 > 
 Right shift or left shift when possible to save gas. - Savings: ~65 
 

 --- 

[File:StringQuery.sol#L23](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/StringQuery.sol#L23) 
```solidity
22:    size = (sizeInBits + 7) / 8;
``` 



[File:StringQuery.sol#L73](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/StringQuery.sol#L73) 
```solidity
72:      size = (sizeInBits + 7) / 8;
``` 



 --- 

<a name=[Gas-15]></a>
### [Gas-15] Mark functions as payable (with discretion) - Instances: 10 

 > 
 You can mark public or external functions as payable to save gas. Functions that are not payable have additional logic to check if there was a value sent with a call, however, making a function payable eliminates this check. This optimization should be carefully considered due to potentially unwanted behavior when a function does not need to accept ether. - Savings: ~24 
 

 --- 

[File:WildcatMarketToken.sol#L16](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketToken.sol#L16) 
```solidity
15:  function balanceOf(address account) public view virtual nonReentrantView returns (uint256) {
``` 



[File:WildcatMarketToken.sol#L22](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketToken.sol#L22) 
```solidity
21:  function totalSupply() external view virtual nonReentrantView returns (uint256) {
``` 



[File:WildcatMarketToken.sol#L31](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketToken.sol#L31) 
```solidity
30:  function approve(address spender, uint256 amount) external virtual nonReentrant returns (bool) {
``` 



[File:WildcatMarketToken.sol#L36](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketToken.sol#L36) 
```solidity
35:  function transfer(address to, uint256 amount) external virtual nonReentrant returns (bool) {
``` 



[File:WildcatMarketToken.sol#L41](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketToken.sol#L41) 
```solidity
40:  function transferFrom(
41:    address from,
42:    address to,
43:    uint256 amount
44:  ) external virtual nonReentrant returns (bool) {
``` 



[File:WildcatMarket.sol#L26](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarket.sol#L26) 
```solidity
25:  function updateState() external nonReentrant {
``` 



[File:WildcatMarket.sol#L42](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarket.sol#L42) 
```solidity
41:  function depositUpTo(
42:    uint256 amount
43:  ) public virtual nonReentrant returns (uint256 /* actualAmount */) {
``` 



[File:WildcatMarket.sol#L86](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarket.sol#L86) 
```solidity
85:  function deposit(uint256 amount) external virtual {
``` 



[File:WildcatMarket.sol#L96](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarket.sol#L96) 
```solidity
95:  function collectFees() external nonReentrant {
``` 



[File:WildcatMarket.sol#L119](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarket.sol#L119) 
```solidity
118:  function borrow(uint256 amount) external onlyBorrower nonReentrant {
``` 



[File:WildcatMarket.sol#L142](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarket.sol#L142) 
```solidity
141:  function closeMarket() external onlyController nonReentrant {
``` 



[File:WildcatMarketConfig.sol#L21](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketConfig.sol#L21) 
```solidity
20:  function maximumDeposit() external view returns (uint256) {
``` 



[File:WildcatMarketConfig.sol#L30](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketConfig.sol#L30) 
```solidity
29:  function maxTotalSupply() external view returns (uint256) {
``` 



[File:WildcatMarketConfig.sol#L38](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketConfig.sol#L38) 
```solidity
37:  function annualInterestBips() external view returns (uint256) {
``` 



[File:WildcatMarketConfig.sol#L42](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketConfig.sol#L42) 
```solidity
41:  function reserveRatioBips() external view returns (uint256) {
``` 



[File:WildcatMarketConfig.sol#L74](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketConfig.sol#L74) 
```solidity
73:  function nukeFromOrbit(address accountAddress) external nonReentrant {
``` 



[File:WildcatMarketConfig.sol#L88](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketConfig.sol#L88) 
```solidity
87:  function stunningReversal(address accountAddress) external nonReentrant {
``` 



[File:WildcatMarketConfig.sol#L112](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketConfig.sol#L112) 
```solidity
111:  function updateAccountAuthorization(
112:    address _account,
113:    bool _isAuthorized
114:  ) external onlyController nonReentrant {
``` 



[File:WildcatMarketConfig.sol#L134](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketConfig.sol#L134) 
```solidity
133:  function setMaxTotalSupply(uint256 _maxTotalSupply) external onlyController nonReentrant {
``` 



[File:WildcatMarketConfig.sol#L149](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketConfig.sol#L149) 
```solidity
148:  function setAnnualInterestBips(uint16 _annualInterestBips) public onlyController nonReentrant {
``` 



[File:WildcatMarketConfig.sol#L171](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketConfig.sol#L171) 
```solidity
170:  function setReserveRatioBips(uint16 _reserveRatioBips) public onlyController nonReentrant {
``` 



[File:WildcatMarketWithdrawals.sol#L24](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketWithdrawals.sol#L24) 
```solidity
23:  function getUnpaidBatchExpiries() external view nonReentrantView returns (uint32[] memory) {
``` 



[File:WildcatMarketWithdrawals.sol#L28](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketWithdrawals.sol#L28) 
```solidity
27:  function getWithdrawalBatch(
28:    uint32 expiry
29:  ) external view nonReentrantView returns (WithdrawalBatch memory) {
``` 



[File:WildcatMarketWithdrawals.sol#L38](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketWithdrawals.sol#L38) 
```solidity
37:  function getAccountWithdrawalStatus(
38:    address accountAddress,
39:    uint32 expiry
40:  ) external view nonReentrantView returns (AccountWithdrawalStatus memory) {
``` 



[File:WildcatMarketWithdrawals.sol#L45](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketWithdrawals.sol#L45) 
```solidity
44:  function getAvailableWithdrawalAmount(
45:    address accountAddress,
46:    uint32 expiry
47:  ) external view nonReentrantView returns (uint256) {
``` 



[File:WildcatMarketWithdrawals.sol#L77](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketWithdrawals.sol#L77) 
```solidity
76:  function queueWithdrawal(uint256 amount) external nonReentrant {
``` 



[File:WildcatMarketWithdrawals.sol#L137](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketWithdrawals.sol#L137) 
```solidity
136:  function executeWithdrawal(
137:    address accountAddress,
138:    uint32 expiry
139:  ) external nonReentrant returns (uint256) {
``` 



[File:WildcatMarketWithdrawals.sol#L190](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketWithdrawals.sol#L190) 
```solidity
189:  function processUnpaidWithdrawalBatch() external nonReentrant {
``` 



[File:WildcatSanctionsSentinel.sol#L39](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsSentinel.sol#L39) 
```solidity
38:  function isSanctioned(address borrower, address account) public view override returns (bool) {
``` 



[File:WildcatSanctionsSentinel.sol#L48](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsSentinel.sol#L48) 
```solidity
47:  function overrideSanction(address account) public override {
``` 



[File:WildcatSanctionsSentinel.sol#L56](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsSentinel.sol#L56) 
```solidity
55:  function removeSanctionOverride(address account) public override {
``` 



[File:WildcatSanctionsSentinel.sol#L65](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsSentinel.sol#L65) 
```solidity
64:  function getEscrowAddress(
65:    address borrower,
66:    address account,
67:    address asset
68:  ) public view override returns (address escrowAddress) {
``` 



[File:WildcatSanctionsSentinel.sol#L95](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsSentinel.sol#L95) 
```solidity
94:  function createEscrow(
95:    address borrower,
96:    address account,
97:    address asset
98:  ) public override returns (address escrowContract) {
``` 



[File:WildcatMarketControllerFactory.sol#L126](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L126) 
```solidity
125:  function isDeployedController(address controller) external view returns (bool) {
``` 



[File:WildcatMarketControllerFactory.sol#L130](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L130) 
```solidity
129:  function getDeployedControllersCount() external view returns (uint256) {
``` 



[File:WildcatMarketControllerFactory.sol#L134](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L134) 
```solidity
133:  function getDeployedControllers() external view returns (address[] memory) {
``` 



[File:WildcatMarketControllerFactory.sol#L138](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L138) 
```solidity
137:  function getDeployedControllers(
138:    uint256 start,
139:    uint256 end
140:  ) external view returns (address[] memory arr) {
``` 



[File:WildcatMarketControllerFactory.sol#L165](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L165) 
```solidity
164:  function getProtocolFeeConfiguration()
165:    external
166:    view
167:    returns (
168:      address feeRecipient,
169:      address originationFeeAsset,
170:      uint80 originationFeeAmount,
171:      uint16 protocolFeeBips
172:    )
173:  {
``` 



[File:WildcatMarketControllerFactory.sol#L195](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L195) 
```solidity
194:  function setProtocolFeeConfiguration(
195:    address feeRecipient,
196:    address originationFeeAsset,
197:    uint80 originationFeeAmount,
198:    uint16 protocolFeeBips
199:  ) external onlyArchControllerOwner {
``` 



[File:WildcatMarketControllerFactory.sol#L223](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L223) 
```solidity
222:  function getParameterConstraints()
223:    external
224:    view
225:    returns (MarketParameterConstraints memory constraints)
226:  {
``` 



[File:WildcatMarketControllerFactory.sol#L246](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L246) 
```solidity
245:  function getMarketControllerParameters()
246:    external
247:    view
248:    virtual
249:    returns (MarketControllerParameters memory parameters)
250:  {
``` 



[File:WildcatMarketControllerFactory.sol#L282](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L282) 
```solidity
281:  function deployController() public returns (address controller) {
``` 



[File:WildcatMarketControllerFactory.sol#L317](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L317) 
```solidity
316:  function deployControllerAndMarket(
317:    string memory namePrefix,
318:    string memory symbolPrefix,
319:    address asset,
320:    uint128 maxTotalSupply,
321:    uint16 annualInterestBips,
322:    uint16 delinquencyFeeBips,
323:    uint32 withdrawalBatchDuration,
324:    uint16 reserveRatioBips,
325:    uint32 delinquencyGracePeriod
326:  ) external returns (address controller, address market) {
``` 



[File:WildcatMarketControllerFactory.sol#L342](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L342) 
```solidity
341:  function computeControllerAddress(address borrower) external view returns (address) {
``` 



[File:WildcatMarketController.sol#L121](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L121) 
```solidity
120:  function getAuthorizedLenders() external view returns (address[] memory) {
``` 



[File:WildcatMarketController.sol#L125](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L125) 
```solidity
124:  function getAuthorizedLenders(
125:    uint256 start,
126:    uint256 end
127:  ) external view returns (address[] memory arr) {
``` 



[File:WildcatMarketController.sol#L138](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L138) 
```solidity
137:  function getAuthorizedLendersCount() external view returns (uint256) {
``` 



[File:WildcatMarketController.sol#L142](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L142) 
```solidity
141:  function isAuthorizedLender(address lender) external view virtual returns (bool) {
``` 



[File:WildcatMarketController.sol#L153](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L153) 
```solidity
152:  function authorizeLenders(address[] memory lenders) external onlyBorrower {
``` 



[File:WildcatMarketController.sol#L169](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L169) 
```solidity
168:  function deauthorizeLenders(address[] memory lenders) external onlyBorrower {
``` 



[File:WildcatMarketController.sol#L182](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L182) 
```solidity
181:  function updateLenderAuthorization(address lender, address[] memory markets) external {
``` 



[File:WildcatMarketController.sol#L196](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L196) 
```solidity
195:  function isControlledMarket(address market) external view returns (bool) {
``` 



[File:WildcatMarketController.sol#L200](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L200) 
```solidity
199:  function getControlledMarkets() external view returns (address[] memory) {
``` 



[File:WildcatMarketController.sol#L204](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L204) 
```solidity
203:  function getControlledMarkets(
204:    uint256 start,
205:    uint256 end
206:  ) external view returns (address[] memory arr) {
``` 



[File:WildcatMarketController.sol#L217](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L217) 
```solidity
216:  function getControlledMarketsCount() external view returns (uint256) {
``` 



[File:WildcatMarketController.sol#L221](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L221) 
```solidity
220:  function computeMarketAddress(
221:    address asset,
222:    string memory namePrefix,
223:    string memory symbolPrefix
224:  ) external view returns (address) {
``` 



[File:WildcatMarketController.sol#L238](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L238) 
```solidity
237:  function getMarketParameters() external view returns (MarketParameters memory parameters) {
``` 



[File:WildcatMarketController.sol#L291](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L291) 
```solidity
290:  function deployMarket(
291:    address asset,
292:    string memory namePrefix,
293:    string memory symbolPrefix,
294:    uint128 maxTotalSupply,
295:    uint16 annualInterestBips,
296:    uint16 delinquencyFeeBips,
297:    uint32 withdrawalBatchDuration,
298:    uint16 reserveRatioBips,
299:    uint32 delinquencyGracePeriod
300:  ) external returns (address market) {
``` 



[File:WildcatMarketController.sol#L446](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L446) 
```solidity
445:  function getParameterConstraints()
446:    external
447:    view
448:    returns (MarketParameterConstraints memory constraints)
449:  {
``` 



[File:WildcatMarketController.sol#L468](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L468) 
```solidity
467:  function setAnnualInterestBips(
468:    address market,
469:    uint16 annualInterestBips
470:  ) external virtual onlyBorrower onlyControlledMarket(market) {
``` 



[File:WildcatMarketController.sol#L490](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L490) 
```solidity
489:  function resetReserveRatio(address market) external virtual {
``` 



[File:WildcatSanctionsEscrow.sol#L21](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsEscrow.sol#L21) 
```solidity
20:  function balance() public view override returns (uint256) {
``` 



[File:WildcatSanctionsEscrow.sol#L25](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsEscrow.sol#L25) 
```solidity
24:  function canReleaseEscrow() public view override returns (bool) {
``` 



[File:WildcatSanctionsEscrow.sol#L29](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsEscrow.sol#L29) 
```solidity
28:  function escrowedAsset() public view override returns (address, uint256) {
``` 



[File:WildcatSanctionsEscrow.sol#L33](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsEscrow.sol#L33) 
```solidity
32:  function releaseEscrow() public override {
``` 



[File:WildcatArchController.sol#L63](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L63) 
```solidity
62:  function registerBorrower(address borrower) external onlyOwner {
``` 



[File:WildcatArchController.sol#L70](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L70) 
```solidity
69:  function removeBorrower(address borrower) external onlyOwner {
``` 



[File:WildcatArchController.sol#L77](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L77) 
```solidity
76:  function isRegisteredBorrower(address borrower) external view returns (bool) {
``` 



[File:WildcatArchController.sol#L81](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L81) 
```solidity
80:  function getRegisteredBorrowers() external view returns (address[] memory) {
``` 



[File:WildcatArchController.sol#L85](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L85) 
```solidity
84:  function getRegisteredBorrowers(
85:    uint256 start,
86:    uint256 end
87:  ) external view returns (address[] memory arr) {
``` 



[File:WildcatArchController.sol#L98](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L98) 
```solidity
97:  function getRegisteredBorrowersCount() external view returns (uint256) {
``` 



[File:WildcatArchController.sol#L106](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L106) 
```solidity
105:  function registerControllerFactory(address factory) external onlyOwner {
``` 



[File:WildcatArchController.sol#L113](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L113) 
```solidity
112:  function removeControllerFactory(address factory) external onlyOwner {
``` 



[File:WildcatArchController.sol#L120](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L120) 
```solidity
119:  function isRegisteredControllerFactory(address factory) external view returns (bool) {
``` 



[File:WildcatArchController.sol#L124](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L124) 
```solidity
123:  function getRegisteredControllerFactories() external view returns (address[] memory) {
``` 



[File:WildcatArchController.sol#L128](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L128) 
```solidity
127:  function getRegisteredControllerFactories(
128:    uint256 start,
129:    uint256 end
130:  ) external view returns (address[] memory arr) {
``` 



[File:WildcatArchController.sol#L141](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L141) 
```solidity
140:  function getRegisteredControllerFactoriesCount() external view returns (uint256) {
``` 



[File:WildcatArchController.sol#L149](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L149) 
```solidity
148:  function registerController(address controller) external onlyControllerFactory {
``` 



[File:WildcatArchController.sol#L156](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L156) 
```solidity
155:  function removeController(address controller) external onlyOwner {
``` 



[File:WildcatArchController.sol#L163](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L163) 
```solidity
162:  function isRegisteredController(address controller) external view returns (bool) {
``` 



[File:WildcatArchController.sol#L167](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L167) 
```solidity
166:  function getRegisteredControllers() external view returns (address[] memory) {
``` 



[File:WildcatArchController.sol#L171](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L171) 
```solidity
170:  function getRegisteredControllers(
171:    uint256 start,
172:    uint256 end
173:  ) external view returns (address[] memory arr) {
``` 



[File:WildcatArchController.sol#L184](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L184) 
```solidity
183:  function getRegisteredControllersCount() external view returns (uint256) {
``` 



[File:WildcatArchController.sol#L192](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L192) 
```solidity
191:  function registerMarket(address market) external onlyController {
``` 



[File:WildcatArchController.sol#L199](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L199) 
```solidity
198:  function removeMarket(address market) external onlyOwner {
``` 



[File:WildcatArchController.sol#L206](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L206) 
```solidity
205:  function isRegisteredMarket(address market) external view returns (bool) {
``` 



[File:WildcatArchController.sol#L210](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L210) 
```solidity
209:  function getRegisteredMarkets() external view returns (address[] memory) {
``` 



[File:WildcatArchController.sol#L214](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L214) 
```solidity
213:  function getRegisteredMarkets(
214:    uint256 start,
215:    uint256 end
216:  ) external view returns (address[] memory arr) {
``` 



[File:WildcatArchController.sol#L227](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L227) 
```solidity
226:  function getRegisteredMarketsCount() external view returns (uint256) {
``` 



[File:WildcatMarketBase.sol#L223](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L223) 
```solidity
222:  function coverageLiquidity() external view nonReentrantView returns (uint256) {
``` 



[File:WildcatMarketBase.sol#L231](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L231) 
```solidity
230:  function scaleFactor() external view nonReentrantView returns (uint256) {
``` 



[File:WildcatMarketBase.sol#L238](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L238) 
```solidity
237:  function totalAssets() public view returns (uint256) {
``` 



[File:WildcatMarketBase.sol#L252](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L252) 
```solidity
251:  function borrowableAssets() external view nonReentrantView returns (uint256) {
``` 



[File:WildcatMarketBase.sol#L260](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L260) 
```solidity
259:  function accruedProtocolFees() external view nonReentrantView returns (uint256) {
``` 



[File:WildcatMarketBase.sol#L267](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L267) 
```solidity
266:  function previousState() external view returns (MarketState memory) {
``` 



[File:WildcatMarketBase.sol#L276](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L276) 
```solidity
275:  function currentState() public view nonReentrantView returns (MarketState memory state) {
``` 



[File:WildcatMarketBase.sol#L285](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L285) 
```solidity
284:  function scaledTotalSupply() external view nonReentrantView returns (uint256) {
``` 



[File:WildcatMarketBase.sol#L292](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L292) 
```solidity
291:  function scaledBalanceOf(address account) external view nonReentrantView returns (uint256) {
``` 



[File:WildcatMarketBase.sol#L299](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L299) 
```solidity
298:  function getAccountRole(address account) external view nonReentrantView returns (AuthRole) {
``` 



[File:WildcatMarketBase.sol#L307](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L307) 
```solidity
306:  function withdrawableProtocolFees() external view returns (uint128) {
``` 



[File:WildcatMarketBase.sol#L318](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L318) 
```solidity
317:  function effectiveBorrowerAPR() external view returns (uint256) {
``` 



[File:WildcatMarketBase.sol#L334](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L334) 
```solidity
333:  function effectiveLenderAPR() external view returns (uint256) {
``` 



 --- 

<a name=[Gas-16]></a>
### [Gas-16] Cache array length during for loop definition. - Instances: 1 

 > 
 A typical for loop definition may look like: `for (uint256 i; i < arr.length; i++){}`. Instead of using `array.length`, cache the array length before the loop, and use the cached value to safe gas. This will avoid an `MLOAD` every loop for arrays stored in memory and an `SLOAD` for arrays stored in storage. This can have significant gas savings for arrays with a large length, especially if the array is stored in storage. - Savings: ~22 
 

 --- 

[File:WildcatMarketController.sol#L154](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L154) 
```solidity
153:    for (uint256 i = 0; i < lenders.length; i++) {
``` 



[File:WildcatMarketController.sol#L170](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L170) 
```solidity
169:    for (uint256 i = 0; i < lenders.length; i++) {
``` 



[File:WildcatMarketController.sol#L183](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L183) 
```solidity
182:    for (uint256 i; i < markets.length; i++) {
``` 



 --- 



## Quality Assurance - Total: 733 

<a name=[NonCritical-0]></a>
### [NonCritical-0] Private variables should contain a leading underscore - Instances: 23 

 > Consider adding an underscore to the beginning of the variable name 

 --- 

[File:WildcatMarketControllerFactory.sol#L56](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L56) 
```solidity
55:  uint32 internal immutable MaximumWithdrawalBatchDuration;
``` 



[File:WildcatMarketControllerFactory.sol#L47](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L47) 
```solidity
46:  uint32 internal immutable MaximumDelinquencyGracePeriod;
``` 



[File:WildcatMarketControllerFactory.sol#L46](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L46) 
```solidity
45:  uint32 internal immutable MinimumDelinquencyGracePeriod;
``` 



[File:WildcatMarketControllerFactory.sol#L58](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L58) 
```solidity
57:  uint16 internal immutable MinimumAnnualInterestBips;
``` 



[File:WildcatMarketControllerFactory.sol#L59](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L59) 
```solidity
58:  uint16 internal immutable MaximumAnnualInterestBips;
``` 



[File:WildcatMarketControllerFactory.sol#L53](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L53) 
```solidity
52:  uint16 internal immutable MaximumDelinquencyFeeBips;
``` 



[File:WildcatMarketControllerFactory.sol#L52](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L52) 
```solidity
51:  uint16 internal immutable MinimumDelinquencyFeeBips;
``` 



[File:WildcatMarketControllerFactory.sol#L44](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L44) 
```solidity
43:  uint256 internal immutable ownCreate2Prefix = LibStoredInitCode.getCreate2Prefix(address(this));
``` 



[File:WildcatMarketControllerFactory.sol#L55](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L55) 
```solidity
54:  uint32 internal immutable MinimumWithdrawalBatchDuration;
``` 



[File:WildcatMarketControllerFactory.sol#L50](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L50) 
```solidity
49:  uint16 internal immutable MaximumReserveRatioBips;
``` 



[File:WildcatMarketControllerFactory.sol#L49](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L49) 
```solidity
48:  uint16 internal immutable MinimumReserveRatioBips;
``` 



[File:WildcatMarketController.sol#L68](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L68) 
```solidity
67:  uint16 internal immutable MaximumAnnualInterestBips;
``` 



[File:WildcatMarketController.sol#L58](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L58) 
```solidity
57:  uint16 internal immutable MinimumReserveRatioBips;
``` 



[File:WildcatMarketController.sol#L67](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L67) 
```solidity
66:  uint16 internal immutable MinimumAnnualInterestBips;
``` 



[File:WildcatMarketController.sol#L61](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L61) 
```solidity
60:  uint16 internal immutable MinimumDelinquencyFeeBips;
``` 



[File:WildcatMarketController.sol#L59](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L59) 
```solidity
58:  uint16 internal immutable MaximumReserveRatioBips;
``` 



[File:WildcatMarketController.sol#L55](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L55) 
```solidity
54:  uint32 internal immutable MinimumDelinquencyGracePeriod;
``` 



[File:WildcatMarketController.sol#L62](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L62) 
```solidity
61:  uint16 internal immutable MaximumDelinquencyFeeBips;
``` 



[File:WildcatMarketController.sol#L65](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L65) 
```solidity
64:  uint32 internal immutable MaximumWithdrawalBatchDuration;
``` 



[File:WildcatMarketController.sol#L56](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L56) 
```solidity
55:  uint32 internal immutable MaximumDelinquencyGracePeriod;
``` 



[File:WildcatMarketController.sol#L53](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L53) 
```solidity
52:  uint256 internal immutable ownCreate2Prefix = LibStoredInitCode.getCreate2Prefix(address(this));
``` 



[File:WildcatMarketController.sol#L64](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L64) 
```solidity
63:  uint32 internal immutable MinimumWithdrawalBatchDuration;
``` 



[File:WildcatSanctionsEscrow.sol#L14](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsEscrow.sol#L14) 
```solidity
13:  address internal immutable asset;
``` 



 --- 

<a name=[NonCritical-1]></a>
### [NonCritical-1] Constructor should check that all parameters are not 0 - Instances: 5 

 > Consider adding a require statement to check that all parameters are not 0 in the constructor 

 --- 

[File:WildcatMarketControllerFactory.sol#L72](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L72) 
```solidity
71:  constructor(
72:    address _archController,
73:    address _sentinel,
74:    MarketParameterConstraints memory constraints
75:  ) {
``` 



[File:WildcatMarketControllerFactory.sol#L72](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L72) 
```solidity
71:  constructor(
72:    address _archController,
73:    address _sentinel,
74:    MarketParameterConstraints memory constraints
75:  ) {
``` 



[File:WildcatMarketControllerFactory.sol#L72](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L72) 
```solidity
71:  constructor(
72:    address _archController,
73:    address _sentinel,
74:    MarketParameterConstraints memory constraints
75:  ) {
``` 



[File:WildcatSanctionsSentinel.sol#L24](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsSentinel.sol#L24) 
```solidity
23:  constructor(address _archController, address _chainalysisSanctionsList) {
``` 



[File:WildcatSanctionsSentinel.sol#L24](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsSentinel.sol#L24) 
```solidity
23:  constructor(address _archController, address _chainalysisSanctionsList) {
``` 



 --- 

<a name=[NonCritical-2]></a>
### [NonCritical-2] Large contracts with many external functions should inherit an interface - Instances: 1 

 > Consider inheriting the interface to ensure the interface matches the contract spec 

 --- 

[File:WildcatArchController.sol#L8](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L8) 
```solidity
7:contract WildcatArchController is Ownable {
``` 



 --- 

<a name=[NonCritical-3]></a>
### [NonCritical-3] This error has no parameters, the state of the contract when the revert occured will not be available - Instances: 72 

 > Consider adding parameters to the error to provide more context when a transaction fails 

 --- 

[File:IWildcatSanctionsSentinel.sol#L15](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatSanctionsSentinel.sol#L15) 
```solidity
14:  error NotRegisteredMarket();
``` 



[File:LibStoredInitCode.sol#L5](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/LibStoredInitCode.sol#L5) 
```solidity
4:  error InitCodeDeploymentFailed();
``` 



[File:StringQuery.sol#L16](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/StringQuery.sol#L16) 
```solidity
15:error InvalidReturnDataString();
``` 



[File:StringQuery.sol#L17](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/StringQuery.sol#L17) 
```solidity
16:error InvalidCompactString();
``` 



[File:WildcatArchController.sol#L16](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L16) 
```solidity
15:  error NotControllerFactory();
``` 



[File:WildcatArchController.sol#L17](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L17) 
```solidity
16:  error NotController();
``` 



[File:WildcatArchController.sol#L19](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L19) 
```solidity
18:  error BorrowerAlreadyExists();
``` 



[File:WildcatArchController.sol#L20](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L20) 
```solidity
19:  error ControllerFactoryAlreadyExists();
``` 



[File:WildcatArchController.sol#L21](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L21) 
```solidity
20:  error ControllerAlreadyExists();
``` 



[File:WildcatArchController.sol#L22](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L22) 
```solidity
21:  error MarketAlreadyExists();
``` 



[File:WildcatArchController.sol#L24](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L24) 
```solidity
23:  error BorrowerDoesNotExist();
``` 



[File:WildcatArchController.sol#L25](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L25) 
```solidity
24:  error ControllerFactoryDoesNotExist();
``` 



[File:WildcatArchController.sol#L26](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L26) 
```solidity
25:  error ControllerDoesNotExist();
``` 



[File:WildcatArchController.sol#L27](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L27) 
```solidity
26:  error MarketDoesNotExist();
``` 



[File:IMarketEventsAndErrors.sol#L9](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L9) 
```solidity
8:  error MaxSupplyExceeded();
``` 



[File:IMarketEventsAndErrors.sol#L12](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L12) 
```solidity
11:  error NotApprovedBorrower();
``` 



[File:IMarketEventsAndErrors.sol#L15](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L15) 
```solidity
14:  error NotApprovedLender();
``` 



[File:IMarketEventsAndErrors.sol#L18](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L18) 
```solidity
17:  error NotController();
``` 



[File:IMarketEventsAndErrors.sol#L21](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L21) 
```solidity
20:  error BadLaunchCode();
``` 



[File:IMarketEventsAndErrors.sol#L24](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L24) 
```solidity
23:  error NewMaxSupplyTooLow();
``` 



[File:IMarketEventsAndErrors.sol#L27](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L27) 
```solidity
26:  error ReserveRatioBipsTooHigh();
``` 



[File:IMarketEventsAndErrors.sol#L30](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L30) 
```solidity
29:  error InterestRateTooHigh();
``` 



[File:IMarketEventsAndErrors.sol#L33](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L33) 
```solidity
32:  error InterestFeeTooHigh();
``` 



[File:IMarketEventsAndErrors.sol#L36](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L36) 
```solidity
35:  error PenaltyFeeTooHigh();
``` 



[File:IMarketEventsAndErrors.sol#L39](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L39) 
```solidity
38:  error AccountBlacklisted();
``` 



[File:IMarketEventsAndErrors.sol#L41](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L41) 
```solidity
40:  error AccountNotBlocked();
``` 



[File:IMarketEventsAndErrors.sol#L43](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L43) 
```solidity
42:  error NotReversedOrStunning();
``` 



[File:IMarketEventsAndErrors.sol#L45](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L45) 
```solidity
44:  error UnknownNameQueryError();
``` 



[File:IMarketEventsAndErrors.sol#L47](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L47) 
```solidity
46:  error UnknownSymbolQueryError();
``` 



[File:IMarketEventsAndErrors.sol#L49](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L49) 
```solidity
48:  error BorrowAmountTooHigh();
``` 



[File:IMarketEventsAndErrors.sol#L51](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L51) 
```solidity
50:  error FeeSetWithoutRecipient();
``` 



[File:IMarketEventsAndErrors.sol#L53](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L53) 
```solidity
52:  error InsufficientReservesForFeeWithdrawal();
``` 



[File:IMarketEventsAndErrors.sol#L55](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L55) 
```solidity
54:  error WithdrawalBatchNotExpired();
``` 



[File:IMarketEventsAndErrors.sol#L57](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L57) 
```solidity
56:  error NullMintAmount();
``` 



[File:IMarketEventsAndErrors.sol#L59](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L59) 
```solidity
58:  error NullBurnAmount();
``` 



[File:IMarketEventsAndErrors.sol#L61](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L61) 
```solidity
60:  error NullFeeAmount();
``` 



[File:IMarketEventsAndErrors.sol#L63](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L63) 
```solidity
62:  error NullTransferAmount();
``` 



[File:IMarketEventsAndErrors.sol#L65](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L65) 
```solidity
64:  error NullWithdrawalAmount();
``` 



[File:IMarketEventsAndErrors.sol#L67](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L67) 
```solidity
66:  error DepositToClosedMarket();
``` 



[File:IMarketEventsAndErrors.sol#L69](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L69) 
```solidity
68:  error BorrowFromClosedMarket();
``` 



[File:IMarketEventsAndErrors.sol#L71](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L71) 
```solidity
70:  error CloseMarketWithUnpaidWithdrawals();
``` 



[File:IMarketEventsAndErrors.sol#L75](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L75) 
```solidity
74:  error InsufficientReservesForNewLiquidityRatio();
``` 



[File:IMarketEventsAndErrors.sol#L77](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IMarketEventsAndErrors.sol#L77) 
```solidity
76:  error InsufficientReservesForOldLiquidityRatio();
``` 



[File:WildcatMarketControllerFactory.sol#L24](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L24) 
```solidity
23:  error NotRegisteredBorrower();
``` 



[File:WildcatMarketControllerFactory.sol#L25](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L25) 
```solidity
24:  error InvalidProtocolFeeConfiguration();
``` 



[File:WildcatMarketControllerFactory.sol#L26](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L26) 
```solidity
25:  error CallerNotArchControllerOwner();
``` 



[File:WildcatMarketControllerFactory.sol#L27](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L27) 
```solidity
26:  error InvalidConstraints();
``` 



[File:WildcatMarketControllerFactory.sol#L28](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L28) 
```solidity
27:  error ControllerAlreadyDeployed();
``` 



[File:IWildcatSanctionsEscrow.sol#L7](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatSanctionsEscrow.sol#L7) 
```solidity
6:  error CanNotReleaseEscrow();
``` 



[File:FIFOQueue.sol#L17](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FIFOQueue.sol#L17) 
```solidity
16:  error FIFOQueueOutOfBounds();
``` 



[File:MathUtils.sol#L19](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L19) 
```solidity
18:  error MulDivFailed();
``` 



[File:ReentrancyGuard.sol#L17](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/ReentrancyGuard.sol#L17) 
```solidity
16:  error NoReentrantCalls();
``` 



[File:IWildcatMarketControllerFactory.sol#L15](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketControllerFactory.sol#L15) 
```solidity
14:  error NotRegisteredBorrower();
``` 



[File:IWildcatMarketControllerFactory.sol#L16](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketControllerFactory.sol#L16) 
```solidity
15:  error InvalidProtocolFeeConfiguration();
``` 



[File:IWildcatMarketControllerFactory.sol#L17](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketControllerFactory.sol#L17) 
```solidity
16:  error CallerNotArchControllerOwner();
``` 



[File:IWildcatMarketControllerFactory.sol#L18](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketControllerFactory.sol#L18) 
```solidity
17:  error InvalidConstraints();
``` 



[File:IWildcatMarketControllerFactory.sol#L19](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketControllerFactory.sol#L19) 
```solidity
18:  error ControllerAlreadyDeployed();
``` 



[File:IWildcatMarketControllerEventsAndErrors.sol#L9](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketControllerEventsAndErrors.sol#L9) 
```solidity
8:  error DelinquencyGracePeriodOutOfBounds();
``` 



[File:IWildcatMarketControllerEventsAndErrors.sol#L10](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketControllerEventsAndErrors.sol#L10) 
```solidity
9:  error ReserveRatioBipsOutOfBounds();
``` 



[File:IWildcatMarketControllerEventsAndErrors.sol#L11](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketControllerEventsAndErrors.sol#L11) 
```solidity
10:  error DelinquencyFeeBipsOutOfBounds();
``` 



[File:IWildcatMarketControllerEventsAndErrors.sol#L12](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketControllerEventsAndErrors.sol#L12) 
```solidity
11:  error WithdrawalBatchDurationOutOfBounds();
``` 



[File:IWildcatMarketControllerEventsAndErrors.sol#L13](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketControllerEventsAndErrors.sol#L13) 
```solidity
12:  error AnnualInterestBipsOutOfBounds();
``` 



[File:IWildcatMarketControllerEventsAndErrors.sol#L16](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketControllerEventsAndErrors.sol#L16) 
```solidity
15:  error CallerNotBorrower();
``` 



[File:IWildcatMarketControllerEventsAndErrors.sol#L20](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketControllerEventsAndErrors.sol#L20) 
```solidity
19:  error CallerNotBorrowerOrControllerFactory();
``` 



[File:IWildcatMarketControllerEventsAndErrors.sol#L24](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketControllerEventsAndErrors.sol#L24) 
```solidity
23:  error NotRegisteredBorrower();
``` 



[File:IWildcatMarketControllerEventsAndErrors.sol#L26](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketControllerEventsAndErrors.sol#L26) 
```solidity
25:  error EmptyString();
``` 



[File:IWildcatMarketControllerEventsAndErrors.sol#L28](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketControllerEventsAndErrors.sol#L28) 
```solidity
27:  error NotControlledMarket();
``` 



[File:IWildcatMarketControllerEventsAndErrors.sol#L30](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketControllerEventsAndErrors.sol#L30) 
```solidity
29:  error MarketAlreadyDeployed();
``` 



[File:IWildcatMarketControllerEventsAndErrors.sol#L32](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketControllerEventsAndErrors.sol#L32) 
```solidity
31:  error ExcessReserveRatioStillActive();
``` 



[File:IWildcatMarketControllerEventsAndErrors.sol#L33](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketControllerEventsAndErrors.sol#L33) 
```solidity
32:  error AprChangeNotPending();
``` 



[File:IWildcatArchController.sol#L5](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatArchController.sol#L5) 
```solidity
4:  error NotMarketFactory();
``` 



[File:IWildcatArchController.sol#L7](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatArchController.sol#L7) 
```solidity
6:  error NotControllerFactory();
``` 



 --- 

<a name=[NonCritical-4]></a>
### [NonCritical-4] Function names should be in camelCase - Instances: 34 

 > Ensure that function definitions are declared using camelCase 

 --- 

[File:IERC20.sol#L12](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IERC20.sol#L12) 
```solidity
11:  function transfer(address recipient, uint256 amount) external returns (bool);
``` 



[File:IERC20.sol#L14](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IERC20.sol#L14) 
```solidity
13:  function allowance(address owner, address spender) external view returns (uint256);
``` 



[File:IERC20.sol#L16](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IERC20.sol#L16) 
```solidity
15:  function approve(address spender, uint256 amount) external returns (bool);
``` 



[File:IWildcatSanctionsEscrow.sol#L9](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatSanctionsEscrow.sol#L9) 
```solidity
8:  function sentinel() external view returns (address);
``` 



[File:IWildcatSanctionsEscrow.sol#L11](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatSanctionsEscrow.sol#L11) 
```solidity
10:  function borrower() external view returns (address);
``` 



[File:IWildcatSanctionsEscrow.sol#L13](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatSanctionsEscrow.sol#L13) 
```solidity
12:  function account() external view returns (address);
``` 



[File:IWildcatSanctionsEscrow.sol#L15](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatSanctionsEscrow.sol#L15) 
```solidity
14:  function balance() external view returns (uint256);
``` 



[File:MathUtils.sol#L44](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L44) 
```solidity
43:  function min(uint256 a, uint256 b) internal pure returns (uint256 c) {
``` 



[File:MathUtils.sol#L51](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L51) 
```solidity
50:  function max(uint256 a, uint256 b) internal pure returns (uint256 c) {
``` 



[File:MathUtils.sol#L71](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L71) 
```solidity
70:  function ternary(
71:    bool condition,
72:    uint256 valueIfTrue,
73:    uint256 valueIfFalse
74:  ) internal pure returns (uint256 c) {
``` 



[File:BoolUtils.sol#L5](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/BoolUtils.sol#L5) 
```solidity
4:  function and(bool a, bool b) internal pure returns (bool c) {
``` 



[File:BoolUtils.sol#L11](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/BoolUtils.sol#L11) 
```solidity
10:  function or(bool a, bool b) internal pure returns (bool c) {
``` 



[File:BoolUtils.sol#L17](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/BoolUtils.sol#L17) 
```solidity
16:  function xor(bool a, bool b) internal pure returns (bool c) {
``` 



[File:IWildcatSanctionsSentinel.sol#L23](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatSanctionsSentinel.sol#L23) 
```solidity
22:  function WildcatSanctionsEscrowInitcodeHash() external pure returns (bytes32);
``` 



[File:WildcatMarketToken.sol#L31](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketToken.sol#L31) 
```solidity
30:  function approve(address spender, uint256 amount) external virtual nonReentrant returns (bool) {
``` 



[File:WildcatMarketToken.sol#L36](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketToken.sol#L36) 
```solidity
35:  function transfer(address to, uint256 amount) external virtual nonReentrant returns (bool) {
``` 



[File:WildcatMarketToken.sol#L59](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketToken.sol#L59) 
```solidity
58:  function _approve(address approver, address spender, uint256 amount) internal virtual {
``` 



[File:WildcatMarketToken.sol#L64](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketToken.sol#L64) 
```solidity
63:  function _transfer(address from, address to, uint256 amount) internal virtual {
``` 



[File:WildcatSanctionsEscrow.sol#L21](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsEscrow.sol#L21) 
```solidity
20:  function balance() public view override returns (uint256) {
``` 



[File:FIFOQueue.sol#L19](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FIFOQueue.sol#L19) 
```solidity
18:  function empty(FIFOQueue storage arr) internal view returns (bool) {
``` 



[File:FIFOQueue.sol#L23](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FIFOQueue.sol#L23) 
```solidity
22:  function first(FIFOQueue storage arr) internal view returns (uint32) {
``` 



[File:FIFOQueue.sol#L30](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FIFOQueue.sol#L30) 
```solidity
29:  function at(FIFOQueue storage arr, uint256 index) internal view returns (uint32) {
``` 



[File:FIFOQueue.sol#L38](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FIFOQueue.sol#L38) 
```solidity
37:  function length(FIFOQueue storage arr) internal view returns (uint128) {
``` 



[File:FIFOQueue.sol#L42](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FIFOQueue.sol#L42) 
```solidity
41:  function values(FIFOQueue storage arr) internal view returns (uint32[] memory _values) {
``` 



[File:FIFOQueue.sol#L55](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FIFOQueue.sol#L55) 
```solidity
54:  function push(FIFOQueue storage arr, uint32 value) internal {
``` 



[File:FIFOQueue.sol#L61](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FIFOQueue.sol#L61) 
```solidity
60:  function shift(FIFOQueue storage arr) internal {
``` 



[File:IWildcatArchController.sol#L9](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatArchController.sol#L9) 
```solidity
8:  function owner() external view returns (address);
``` 



[File:IWildcatMarketController.sol#L18](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketController.sol#L18) 
```solidity
17:  function borrower() external view returns (address);
``` 



[File:IERC20Metadata.sol#L7](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IERC20Metadata.sol#L7) 
```solidity
6:  function name() external view returns (string memory);
``` 



[File:IERC20Metadata.sol#L9](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IERC20Metadata.sol#L9) 
```solidity
8:  function symbol() external view returns (string memory);
``` 



[File:IERC20Metadata.sol#L11](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IERC20Metadata.sol#L11) 
```solidity
10:  function decimals() external view returns (uint8);
``` 



[File:IWildcatMarketControllerFactory.sol#L28](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketControllerFactory.sol#L28) 
```solidity
27:  function sentinel() external view returns (address);
``` 



[File:WildcatMarket.sol#L86](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarket.sol#L86) 
```solidity
85:  function deposit(uint256 amount) external virtual {
``` 



[File:WildcatMarket.sol#L119](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarket.sol#L119) 
```solidity
118:  function borrow(uint256 amount) external onlyBorrower nonReentrant {
``` 



 --- 

<a name=[NonCritical-5]></a>
### [NonCritical-5] Consider importing specific identifiers instead of the whole file - Instances: 54 

 > This will minimize compiled code size and help with readability 

 --- 

[File:IWildcatMarketControllerFactory.sol#L4](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketControllerFactory.sol#L4) 
```solidity
3:import './WildcatStructsAndEnums.sol';
``` 



[File:IWildcatMarketController.sol#L4](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketController.sol#L4) 
```solidity
3:import './WildcatStructsAndEnums.sol';
``` 



[File:IWildcatMarketController.sol#L5](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketController.sol#L5) 
```solidity
4:import './IWildcatMarketControllerEventsAndErrors.sol';
``` 



[File:WildcatArchController.sol#L5](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L5) 
```solidity
4:import 'solady/auth/Ownable.sol';
``` 



[File:WildcatArchController.sol#L6](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L6) 
```solidity
5:import './libraries/MathUtils.sol';
``` 



[File:WildcatMarketConfig.sol#L4](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketConfig.sol#L4) 
```solidity
3:import '../interfaces/IWildcatSanctionsSentinel.sol';
``` 



[File:WildcatMarketConfig.sol#L5](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketConfig.sol#L5) 
```solidity
4:import '../libraries/FeeMath.sol';
``` 



[File:WildcatMarketConfig.sol#L6](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketConfig.sol#L6) 
```solidity
5:import '../libraries/SafeCastLib.sol';
``` 



[File:WildcatMarketConfig.sol#L7](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketConfig.sol#L7) 
```solidity
6:import './WildcatMarketBase.sol';
``` 



[File:SafeCastLib.sol#L4](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L4) 
```solidity
3:import './Errors.sol';
``` 



[File:WildcatMarketBase.sol#L4](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L4) 
```solidity
3:import '../libraries/FeeMath.sol';
``` 



[File:WildcatMarketBase.sol#L5](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L5) 
```solidity
4:import '../libraries/Withdrawal.sol';
``` 



[File:WildcatMarketBase.sol#L7](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L7) 
```solidity
6:import '../interfaces/IMarketEventsAndErrors.sol';
``` 



[File:WildcatMarketBase.sol#L8](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L8) 
```solidity
7:import '../interfaces/IWildcatMarketController.sol';
``` 



[File:WildcatMarketBase.sol#L9](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L9) 
```solidity
8:import '../interfaces/IWildcatSanctionsSentinel.sol';
``` 



[File:WildcatMarketBase.sol#L11](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L11) 
```solidity
10:import '../ReentrancyGuard.sol';
``` 



[File:WildcatMarketBase.sol#L12](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L12) 
```solidity
11:import '../libraries/BoolUtils.sol';
``` 



[File:WildcatMarketToken.sol#L4](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketToken.sol#L4) 
```solidity
3:import './WildcatMarketBase.sol';
``` 



[File:FeeMath.sol#L4](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FeeMath.sol#L4) 
```solidity
3:import './MathUtils.sol';
``` 



[File:FeeMath.sol#L5](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FeeMath.sol#L5) 
```solidity
4:import './SafeCastLib.sol';
``` 



[File:FeeMath.sol#L6](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FeeMath.sol#L6) 
```solidity
5:import './MarketState.sol';
``` 



[File:WildcatMarket.sol#L4](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarket.sol#L4) 
```solidity
3:import '../libraries/FeeMath.sol';
``` 



[File:WildcatMarket.sol#L5](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarket.sol#L5) 
```solidity
4:import './WildcatMarketBase.sol';
``` 



[File:WildcatMarket.sol#L6](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarket.sol#L6) 
```solidity
5:import './WildcatMarketConfig.sol';
``` 



[File:WildcatMarket.sol#L7](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarket.sol#L7) 
```solidity
6:import './WildcatMarketToken.sol';
``` 



[File:WildcatMarket.sol#L8](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarket.sol#L8) 
```solidity
7:import './WildcatMarketWithdrawals.sol';
``` 



[File:MathUtils.sol#L4](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L4) 
```solidity
3:import './Errors.sol';
``` 



[File:WildcatMarketController.sol#L5](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L5) 
```solidity
4:import 'solady/utils/SafeTransferLib.sol';
``` 



[File:WildcatMarketController.sol#L6](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L6) 
```solidity
5:import './market/WildcatMarket.sol';
``` 



[File:WildcatMarketController.sol#L7](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L7) 
```solidity
6:import './interfaces/IWildcatArchController.sol';
``` 



[File:WildcatMarketController.sol#L8](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L8) 
```solidity
7:import './interfaces/IWildcatMarketControllerEventsAndErrors.sol';
``` 



[File:WildcatMarketController.sol#L9](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L9) 
```solidity
8:import './interfaces/IWildcatMarketControllerFactory.sol';
``` 



[File:WildcatMarketController.sol#L10](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L10) 
```solidity
9:import './libraries/LibStoredInitCode.sol';
``` 



[File:WildcatMarketController.sol#L11](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L11) 
```solidity
10:import './libraries/MathUtils.sol';
``` 



[File:Chainalysis.sol#L4](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/Chainalysis.sol#L4) 
```solidity
3:import '../interfaces/IChainalysisSanctionsList.sol';
``` 



[File:IERC20Metadata.sol#L4](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IERC20Metadata.sol#L4) 
```solidity
3:import './IERC20.sol';
``` 



[File:Withdrawal.sol#L4](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/Withdrawal.sol#L4) 
```solidity
3:import './MarketState.sol';
``` 



[File:Withdrawal.sol#L5](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/Withdrawal.sol#L5) 
```solidity
4:import './FIFOQueue.sol';
``` 



[File:WildcatMarketControllerFactory.sol#L5](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L5) 
```solidity
4:import './interfaces/WildcatStructsAndEnums.sol';
``` 



[File:WildcatMarketControllerFactory.sol#L6](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L6) 
```solidity
5:import './interfaces/IWildcatMarketController.sol';
``` 



[File:WildcatMarketControllerFactory.sol#L7](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L7) 
```solidity
6:import './interfaces/IWildcatArchController.sol';
``` 



[File:WildcatMarketControllerFactory.sol#L8](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L8) 
```solidity
7:import './libraries/LibStoredInitCode.sol';
``` 



[File:WildcatMarketControllerFactory.sol#L9](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L9) 
```solidity
8:import './libraries/MathUtils.sol';
``` 



[File:WildcatMarketControllerFactory.sol#L10](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L10) 
```solidity
9:import './market/WildcatMarket.sol';
``` 



[File:WildcatMarketControllerFactory.sol#L11](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L11) 
```solidity
10:import './WildcatMarketController.sol';
``` 



[File:WildcatMarketWithdrawals.sol#L4](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketWithdrawals.sol#L4) 
```solidity
3:import './WildcatMarketBase.sol';
``` 



[File:WildcatMarketWithdrawals.sol#L5](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketWithdrawals.sol#L5) 
```solidity
4:import '../libraries/MarketState.sol';
``` 



[File:WildcatMarketWithdrawals.sol#L6](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketWithdrawals.sol#L6) 
```solidity
5:import '../libraries/FeeMath.sol';
``` 



[File:WildcatMarketWithdrawals.sol#L7](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketWithdrawals.sol#L7) 
```solidity
6:import '../libraries/FIFOQueue.sol';
``` 



[File:WildcatMarketWithdrawals.sol#L8](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketWithdrawals.sol#L8) 
```solidity
7:import '../interfaces/IWildcatSanctionsSentinel.sol';
``` 



[File:WildcatMarketWithdrawals.sol#L9](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketWithdrawals.sol#L9) 
```solidity
8:import 'solady/utils/SafeTransferLib.sol';
``` 



[File:MarketState.sol#L5](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MarketState.sol#L5) 
```solidity
4:import './MathUtils.sol';
``` 



[File:MarketState.sol#L6](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MarketState.sol#L6) 
```solidity
5:import './SafeCastLib.sol';
``` 



[File:MarketState.sol#L7](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MarketState.sol#L7) 
```solidity
6:import './FeeMath.sol';
``` 



 --- 

<a name=[NonCritical-6]></a>
### [NonCritical-6] Require/Revert statements should be consistent across the codebase - Instances: 54 

 > Consider using require/revert statements consistently across the codebase 

 --- 

[File:WildcatMarketController.sol#L41](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L41) 
```solidity
40:  IWildcatArchController public immutable archController;
``` 



[File:WildcatMarketController.sol#L43](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L43) 
```solidity
42:  IWildcatMarketControllerFactory public immutable controllerFactory;
``` 



[File:WildcatMarketController.sol#L45](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L45) 
```solidity
44:  address public immutable borrower;
``` 



[File:WildcatMarketController.sol#L47](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L47) 
```solidity
46:  address public immutable sentinel;
``` 



[File:WildcatMarketController.sol#L49](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L49) 
```solidity
48:  address public immutable marketInitCodeStorage;
``` 



[File:WildcatMarketController.sol#L51](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L51) 
```solidity
50:  uint256 public immutable marketInitCodeHash;
``` 



[File:WildcatMarketController.sol#L53](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L53) 
```solidity
52:  uint256 internal immutable ownCreate2Prefix = LibStoredInitCode.getCreate2Prefix(address(this));
``` 



[File:WildcatMarketController.sol#L55](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L55) 
```solidity
54:  uint32 internal immutable MinimumDelinquencyGracePeriod;
``` 



[File:WildcatMarketController.sol#L56](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L56) 
```solidity
55:  uint32 internal immutable MaximumDelinquencyGracePeriod;
``` 



[File:WildcatMarketController.sol#L58](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L58) 
```solidity
57:  uint16 internal immutable MinimumReserveRatioBips;
``` 



[File:WildcatMarketController.sol#L59](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L59) 
```solidity
58:  uint16 internal immutable MaximumReserveRatioBips;
``` 



[File:WildcatMarketController.sol#L61](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L61) 
```solidity
60:  uint16 internal immutable MinimumDelinquencyFeeBips;
``` 



[File:WildcatMarketController.sol#L62](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L62) 
```solidity
61:  uint16 internal immutable MaximumDelinquencyFeeBips;
``` 



[File:WildcatMarketController.sol#L64](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L64) 
```solidity
63:  uint32 internal immutable MinimumWithdrawalBatchDuration;
``` 



[File:WildcatMarketController.sol#L65](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L65) 
```solidity
64:  uint32 internal immutable MaximumWithdrawalBatchDuration;
``` 



[File:WildcatMarketController.sol#L67](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L67) 
```solidity
66:  uint16 internal immutable MinimumAnnualInterestBips;
``` 



[File:WildcatMarketController.sol#L68](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L68) 
```solidity
67:  uint16 internal immutable MaximumAnnualInterestBips;
``` 



[File:WildcatSanctionsSentinel.sol#L11](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsSentinel.sol#L11) 
```solidity
10:  bytes32 public constant override WildcatSanctionsEscrowInitcodeHash =
11:    keccak256(type(WildcatSanctionsEscrow).creationCode);
``` 



[File:WildcatSanctionsSentinel.sol#L14](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsSentinel.sol#L14) 
```solidity
13:  address public immutable override chainalysisSanctionsList;
``` 



[File:WildcatSanctionsSentinel.sol#L16](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsSentinel.sol#L16) 
```solidity
15:  address public immutable override archController;
``` 



[File:WildcatMarketControllerFactory.sol#L31](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L31) 
```solidity
30:  IWildcatArchController public immutable archController;
``` 



[File:WildcatMarketControllerFactory.sol#L34](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L34) 
```solidity
33:  address public immutable sentinel;
``` 



[File:WildcatMarketControllerFactory.sol#L36](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L36) 
```solidity
35:  address public immutable marketInitCodeStorage;
``` 



[File:WildcatMarketControllerFactory.sol#L38](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L38) 
```solidity
37:  uint256 public immutable marketInitCodeHash;
``` 



[File:WildcatMarketControllerFactory.sol#L40](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L40) 
```solidity
39:  address public immutable controllerInitCodeStorage;
``` 



[File:WildcatMarketControllerFactory.sol#L42](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L42) 
```solidity
41:  uint256 public immutable controllerInitCodeHash;
``` 



[File:WildcatMarketControllerFactory.sol#L44](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L44) 
```solidity
43:  uint256 internal immutable ownCreate2Prefix = LibStoredInitCode.getCreate2Prefix(address(this));
``` 



[File:WildcatMarketControllerFactory.sol#L46](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L46) 
```solidity
45:  uint32 internal immutable MinimumDelinquencyGracePeriod;
``` 



[File:WildcatMarketControllerFactory.sol#L47](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L47) 
```solidity
46:  uint32 internal immutable MaximumDelinquencyGracePeriod;
``` 



[File:WildcatMarketControllerFactory.sol#L49](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L49) 
```solidity
48:  uint16 internal immutable MinimumReserveRatioBips;
``` 



[File:WildcatMarketControllerFactory.sol#L50](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L50) 
```solidity
49:  uint16 internal immutable MaximumReserveRatioBips;
``` 



[File:WildcatMarketControllerFactory.sol#L52](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L52) 
```solidity
51:  uint16 internal immutable MinimumDelinquencyFeeBips;
``` 



[File:WildcatMarketControllerFactory.sol#L53](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L53) 
```solidity
52:  uint16 internal immutable MaximumDelinquencyFeeBips;
``` 



[File:WildcatMarketControllerFactory.sol#L55](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L55) 
```solidity
54:  uint32 internal immutable MinimumWithdrawalBatchDuration;
``` 



[File:WildcatMarketControllerFactory.sol#L56](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L56) 
```solidity
55:  uint32 internal immutable MaximumWithdrawalBatchDuration;
``` 



[File:WildcatMarketControllerFactory.sol#L58](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L58) 
```solidity
57:  uint16 internal immutable MinimumAnnualInterestBips;
``` 



[File:WildcatMarketControllerFactory.sol#L59](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L59) 
```solidity
58:  uint16 internal immutable MaximumAnnualInterestBips;
``` 



[File:WildcatSanctionsEscrow.sol#L11](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsEscrow.sol#L11) 
```solidity
10:  address public immutable override sentinel;
``` 



[File:WildcatSanctionsEscrow.sol#L12](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsEscrow.sol#L12) 
```solidity
11:  address public immutable override borrower;
``` 



[File:WildcatSanctionsEscrow.sol#L13](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsEscrow.sol#L13) 
```solidity
12:  address public immutable override account;
``` 



[File:WildcatSanctionsEscrow.sol#L14](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsEscrow.sol#L14) 
```solidity
13:  address internal immutable asset;
``` 



[File:ReentrancyGuard.sol#L22](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/ReentrancyGuard.sol#L22) 
```solidity
21:  uint256 private constant _NOT_ENTERED = 1;
``` 



[File:ReentrancyGuard.sol#L23](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/ReentrancyGuard.sol#L23) 
```solidity
22:  uint256 private constant _ENTERED = 2;
``` 



[File:WildcatMarketBase.sol#L24](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L24) 
```solidity
23:  string public constant version = '1.0';
``` 



[File:WildcatMarketBase.sol#L27](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L27) 
```solidity
26:  address public immutable sentinel;
``` 



[File:WildcatMarketBase.sol#L30](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L30) 
```solidity
29:  address public immutable borrower;
``` 



[File:WildcatMarketBase.sol#L33](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L33) 
```solidity
32:  address public immutable feeRecipient;
``` 



[File:WildcatMarketBase.sol#L36](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L36) 
```solidity
35:  uint256 public immutable protocolFeeBips;
``` 



[File:WildcatMarketBase.sol#L39](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L39) 
```solidity
38:  uint256 public immutable delinquencyFeeBips;
``` 



[File:WildcatMarketBase.sol#L42](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L42) 
```solidity
41:  uint256 public immutable delinquencyGracePeriod;
``` 



[File:WildcatMarketBase.sol#L45](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L45) 
```solidity
44:  address public immutable controller;
``` 



[File:WildcatMarketBase.sol#L48](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L48) 
```solidity
47:  address public immutable asset;
``` 



[File:WildcatMarketBase.sol#L51](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L51) 
```solidity
50:  uint256 public immutable withdrawalBatchDuration;
``` 



[File:WildcatMarketBase.sol#L54](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L54) 
```solidity
53:  uint8 public immutable decimals;
``` 



 --- 

<a name=[NonCritical-7]></a>
### [NonCritical-7] Constant and immutable variable names should be in SCREAMING_SNAKE_CASE - Instances: 54 

 > Ensure that Constant and immutable variable names are declared using SCREAMING_SNAKE_CASE 

 --- 

[File:WildcatSanctionsEscrow.sol#L11](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsEscrow.sol#L11) 
```solidity
10:  address public immutable override sentinel;
``` 



[File:WildcatSanctionsEscrow.sol#L12](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsEscrow.sol#L12) 
```solidity
11:  address public immutable override borrower;
``` 



[File:WildcatSanctionsEscrow.sol#L13](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsEscrow.sol#L13) 
```solidity
12:  address public immutable override account;
``` 



[File:WildcatSanctionsEscrow.sol#L14](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsEscrow.sol#L14) 
```solidity
13:  address internal immutable asset;
``` 



[File:WildcatMarketBase.sol#L24](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L24) 
```solidity
23:  string public constant version = '1.0';
``` 



[File:WildcatMarketBase.sol#L27](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L27) 
```solidity
26:  address public immutable sentinel;
``` 



[File:WildcatMarketBase.sol#L30](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L30) 
```solidity
29:  address public immutable borrower;
``` 



[File:WildcatMarketBase.sol#L33](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L33) 
```solidity
32:  address public immutable feeRecipient;
``` 



[File:WildcatMarketBase.sol#L36](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L36) 
```solidity
35:  uint256 public immutable protocolFeeBips;
``` 



[File:WildcatMarketBase.sol#L39](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L39) 
```solidity
38:  uint256 public immutable delinquencyFeeBips;
``` 



[File:WildcatMarketBase.sol#L42](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L42) 
```solidity
41:  uint256 public immutable delinquencyGracePeriod;
``` 



[File:WildcatMarketBase.sol#L45](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L45) 
```solidity
44:  address public immutable controller;
``` 



[File:WildcatMarketBase.sol#L48](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L48) 
```solidity
47:  address public immutable asset;
``` 



[File:WildcatMarketBase.sol#L51](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L51) 
```solidity
50:  uint256 public immutable withdrawalBatchDuration;
``` 



[File:WildcatMarketBase.sol#L54](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L54) 
```solidity
53:  uint8 public immutable decimals;
``` 



[File:WildcatSanctionsSentinel.sol#L11](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsSentinel.sol#L11) 
```solidity
10:  bytes32 public constant override WildcatSanctionsEscrowInitcodeHash =
11:    keccak256(type(WildcatSanctionsEscrow).creationCode);
``` 



[File:WildcatSanctionsSentinel.sol#L14](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsSentinel.sol#L14) 
```solidity
13:  address public immutable override chainalysisSanctionsList;
``` 



[File:WildcatSanctionsSentinel.sol#L16](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsSentinel.sol#L16) 
```solidity
15:  address public immutable override archController;
``` 



[File:ReentrancyGuard.sol#L22](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/ReentrancyGuard.sol#L22) 
```solidity
21:  uint256 private constant _NOT_ENTERED = 1;
``` 



[File:ReentrancyGuard.sol#L23](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/ReentrancyGuard.sol#L23) 
```solidity
22:  uint256 private constant _ENTERED = 2;
``` 



[File:WildcatMarketController.sol#L41](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L41) 
```solidity
40:  IWildcatArchController public immutable archController;
``` 



[File:WildcatMarketController.sol#L43](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L43) 
```solidity
42:  IWildcatMarketControllerFactory public immutable controllerFactory;
``` 



[File:WildcatMarketController.sol#L45](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L45) 
```solidity
44:  address public immutable borrower;
``` 



[File:WildcatMarketController.sol#L47](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L47) 
```solidity
46:  address public immutable sentinel;
``` 



[File:WildcatMarketController.sol#L49](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L49) 
```solidity
48:  address public immutable marketInitCodeStorage;
``` 



[File:WildcatMarketController.sol#L51](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L51) 
```solidity
50:  uint256 public immutable marketInitCodeHash;
``` 



[File:WildcatMarketController.sol#L53](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L53) 
```solidity
52:  uint256 internal immutable ownCreate2Prefix = LibStoredInitCode.getCreate2Prefix(address(this));
``` 



[File:WildcatMarketController.sol#L55](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L55) 
```solidity
54:  uint32 internal immutable MinimumDelinquencyGracePeriod;
``` 



[File:WildcatMarketController.sol#L56](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L56) 
```solidity
55:  uint32 internal immutable MaximumDelinquencyGracePeriod;
``` 



[File:WildcatMarketController.sol#L58](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L58) 
```solidity
57:  uint16 internal immutable MinimumReserveRatioBips;
``` 



[File:WildcatMarketController.sol#L59](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L59) 
```solidity
58:  uint16 internal immutable MaximumReserveRatioBips;
``` 



[File:WildcatMarketController.sol#L61](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L61) 
```solidity
60:  uint16 internal immutable MinimumDelinquencyFeeBips;
``` 



[File:WildcatMarketController.sol#L62](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L62) 
```solidity
61:  uint16 internal immutable MaximumDelinquencyFeeBips;
``` 



[File:WildcatMarketController.sol#L64](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L64) 
```solidity
63:  uint32 internal immutable MinimumWithdrawalBatchDuration;
``` 



[File:WildcatMarketController.sol#L65](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L65) 
```solidity
64:  uint32 internal immutable MaximumWithdrawalBatchDuration;
``` 



[File:WildcatMarketController.sol#L67](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L67) 
```solidity
66:  uint16 internal immutable MinimumAnnualInterestBips;
``` 



[File:WildcatMarketController.sol#L68](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L68) 
```solidity
67:  uint16 internal immutable MaximumAnnualInterestBips;
``` 



[File:WildcatMarketControllerFactory.sol#L31](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L31) 
```solidity
30:  IWildcatArchController public immutable archController;
``` 



[File:WildcatMarketControllerFactory.sol#L34](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L34) 
```solidity
33:  address public immutable sentinel;
``` 



[File:WildcatMarketControllerFactory.sol#L36](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L36) 
```solidity
35:  address public immutable marketInitCodeStorage;
``` 



[File:WildcatMarketControllerFactory.sol#L38](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L38) 
```solidity
37:  uint256 public immutable marketInitCodeHash;
``` 



[File:WildcatMarketControllerFactory.sol#L40](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L40) 
```solidity
39:  address public immutable controllerInitCodeStorage;
``` 



[File:WildcatMarketControllerFactory.sol#L42](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L42) 
```solidity
41:  uint256 public immutable controllerInitCodeHash;
``` 



[File:WildcatMarketControllerFactory.sol#L44](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L44) 
```solidity
43:  uint256 internal immutable ownCreate2Prefix = LibStoredInitCode.getCreate2Prefix(address(this));
``` 



[File:WildcatMarketControllerFactory.sol#L46](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L46) 
```solidity
45:  uint32 internal immutable MinimumDelinquencyGracePeriod;
``` 



[File:WildcatMarketControllerFactory.sol#L47](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L47) 
```solidity
46:  uint32 internal immutable MaximumDelinquencyGracePeriod;
``` 



[File:WildcatMarketControllerFactory.sol#L49](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L49) 
```solidity
48:  uint16 internal immutable MinimumReserveRatioBips;
``` 



[File:WildcatMarketControllerFactory.sol#L50](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L50) 
```solidity
49:  uint16 internal immutable MaximumReserveRatioBips;
``` 



[File:WildcatMarketControllerFactory.sol#L52](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L52) 
```solidity
51:  uint16 internal immutable MinimumDelinquencyFeeBips;
``` 



[File:WildcatMarketControllerFactory.sol#L53](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L53) 
```solidity
52:  uint16 internal immutable MaximumDelinquencyFeeBips;
``` 



[File:WildcatMarketControllerFactory.sol#L55](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L55) 
```solidity
54:  uint32 internal immutable MinimumWithdrawalBatchDuration;
``` 



[File:WildcatMarketControllerFactory.sol#L56](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L56) 
```solidity
55:  uint32 internal immutable MaximumWithdrawalBatchDuration;
``` 



[File:WildcatMarketControllerFactory.sol#L58](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L58) 
```solidity
57:  uint16 internal immutable MinimumAnnualInterestBips;
``` 



[File:WildcatMarketControllerFactory.sol#L59](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L59) 
```solidity
58:  uint16 internal immutable MaximumAnnualInterestBips;
``` 



 --- 

<a name=[NonCritical-8]></a>
### [NonCritical-8] Consider marking public function External - Instances: 9 

 > If a public function is never called internally, it is best practice to mark it as external. 

 --- 

[File:WildcatSanctionsEscrow.sol#L29](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsEscrow.sol#L29) 
```solidity
28:  function escrowedAsset() public view override returns (address, uint256) {
``` 



[File:WildcatSanctionsEscrow.sol#L33](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsEscrow.sol#L33) 
```solidity
32:  function releaseEscrow() public override {
``` 



[File:WildcatMarketToken.sol#L16](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketToken.sol#L16) 
```solidity
15:  function balanceOf(address account) public view virtual nonReentrantView returns (uint256) {
``` 



[File:WildcatMarketConfig.sol#L171](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketConfig.sol#L171) 
```solidity
170:  function setReserveRatioBips(uint16 _reserveRatioBips) public onlyController nonReentrant {
``` 



[File:WildcatMarketConfig.sol#L149](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketConfig.sol#L149) 
```solidity
148:  function setAnnualInterestBips(uint16 _annualInterestBips) public onlyController nonReentrant {
``` 



[File:WildcatSanctionsSentinel.sol#L48](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsSentinel.sol#L48) 
```solidity
47:  function overrideSanction(address account) public override {
``` 



[File:WildcatSanctionsSentinel.sol#L39](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsSentinel.sol#L39) 
```solidity
38:  function isSanctioned(address borrower, address account) public view override returns (bool) {
``` 



[File:WildcatSanctionsSentinel.sol#L95](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsSentinel.sol#L95) 
```solidity
94:  function createEscrow(
95:    address borrower,
96:    address account,
97:    address asset
98:  ) public override returns (address escrowContract) {
``` 



[File:WildcatSanctionsSentinel.sol#L56](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsSentinel.sol#L56) 
```solidity
55:  function removeSanctionOverride(address account) public override {
``` 



 --- 

<a name=[NonCritical-9]></a>
### [NonCritical-9] Remove any unused functions - Instances: 75 

 > Any functions not used should be removed as best practice. 

 --- 

[File:WildcatMarketBase.sol#L358](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L358) 
```solidity
357:  function _getUpdatedState() internal returns (MarketState memory state) {
``` 



[File:WildcatMarketBase.sol#L163](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L163) 
```solidity
162:  function _blockAccount(MarketState memory state, address accountAddress) internal {
``` 



[File:WildcatMarketBase.sol#L197](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L197) 
```solidity
196:  function _getAccountWithRole(
197:    address accountAddress,
198:    AuthRole requiredRole
199:  ) internal returns (Account memory account) {
``` 



[File:WildcatMarketBase.sol#L448](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L448) 
```solidity
447:  function _writeState(MarketState memory state) internal {
``` 



[File:Withdrawal.sol#L38](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/Withdrawal.sol#L38) 
```solidity
37:  function scaledOwedAmount(WithdrawalBatch memory batch) internal pure returns (uint104) {
``` 



[File:Withdrawal.sol#L47](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/Withdrawal.sol#L47) 
```solidity
46:  function availableLiquidityForPendingBatch(
47:    WithdrawalBatch memory batch,
48:    MarketState memory state,
49:    uint256 totalAssets
50:  ) internal pure returns (uint256) {
``` 



[File:FeeMath.sol#L30](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FeeMath.sol#L30) 
```solidity
29:  function calculateBaseInterest(
30:    MarketState memory state,
31:    uint256 timestamp
32:  ) internal pure returns (uint256 baseInterestRay) {
``` 



[File:FeeMath.sol#L53](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FeeMath.sol#L53) 
```solidity
52:  function updateDelinquency(
53:    MarketState memory state,
54:    uint256 timestamp,
55:    uint256 delinquencyFeeBips,
56:    uint256 delinquencyGracePeriod
57:  ) internal pure returns (uint256 delinquencyFeeRay) {
``` 



[File:FeeMath.sol#L142](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FeeMath.sol#L142) 
```solidity
141:  function updateScaleFactorAndFees(
142:    MarketState memory state,
143:    uint256 protocolFeeBips,
144:    uint256 delinquencyFeeBips,
145:    uint256 delinquencyGracePeriod,
146:    uint256 timestamp
147:  )
148:    internal
149:    pure
150:    returns (uint256 baseInterestRay, uint256 delinquencyFeeRay, uint256 protocolFee)
151:  {
``` 



[File:FeeMath.sol#L40](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FeeMath.sol#L40) 
```solidity
39:  function applyProtocolFee(
40:    MarketState memory state,
41:    uint256 baseInterestRay,
42:    uint256 protocolFeeBips
43:  ) internal pure returns (uint256 protocolFee) {
``` 



[File:BoolUtils.sol#L17](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/BoolUtils.sol#L17) 
```solidity
16:  function xor(bool a, bool b) internal pure returns (bool c) {
``` 



[File:BoolUtils.sol#L5](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/BoolUtils.sol#L5) 
```solidity
4:  function and(bool a, bool b) internal pure returns (bool c) {
``` 



[File:BoolUtils.sol#L11](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/BoolUtils.sol#L11) 
```solidity
10:  function or(bool a, bool b) internal pure returns (bool c) {
``` 



[File:MarketState.sol#L59](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MarketState.sol#L59) 
```solidity
58:  function maximumDeposit(MarketState memory state) internal pure returns (uint256) {
``` 



[File:MarketState.sol#L66](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MarketState.sol#L66) 
```solidity
65:  function normalizeAmount(
66:    MarketState memory state,
67:    uint256 amount
68:  ) internal pure returns (uint256) {
``` 



[File:MarketState.sol#L123](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MarketState.sol#L123) 
```solidity
122:  function borrowableAssets(
123:    MarketState memory state,
124:    uint256 totalAssets
125:  ) internal pure returns (uint256) {
``` 



[File:MarketState.sol#L51](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MarketState.sol#L51) 
```solidity
50:  function totalSupply(MarketState memory state) internal pure returns (uint256) {
``` 



[File:MarketState.sol#L76](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MarketState.sol#L76) 
```solidity
75:  function scaleAmount(MarketState memory state, uint256 amount) internal pure returns (uint256) {
``` 



[File:MarketState.sol#L130](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MarketState.sol#L130) 
```solidity
129:  function hasPendingExpiredBatch(MarketState memory state) internal view returns (bool result) {
``` 



[File:MarketState.sol#L87](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MarketState.sol#L87) 
```solidity
86:  function liquidityRequired(
87:    MarketState memory state
88:  ) internal pure returns (uint256 _liquidityRequired) {
``` 



[File:MarketState.sol#L138](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MarketState.sol#L138) 
```solidity
137:  function totalDebts(MarketState memory state) internal pure returns (uint256) {
``` 



[File:MarketState.sol#L105](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MarketState.sol#L105) 
```solidity
104:  function withdrawableProtocolFees(
105:    MarketState memory state,
106:    uint256 totalAssets
107:  ) internal pure returns (uint128) {
``` 



[File:FIFOQueue.sol#L70](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FIFOQueue.sol#L70) 
```solidity
69:  function shiftN(FIFOQueue storage arr, uint128 n) internal {
``` 



[File:FIFOQueue.sol#L23](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FIFOQueue.sol#L23) 
```solidity
22:  function first(FIFOQueue storage arr) internal view returns (uint32) {
``` 



[File:FIFOQueue.sol#L42](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FIFOQueue.sol#L42) 
```solidity
41:  function values(FIFOQueue storage arr) internal view returns (uint32[] memory _values) {
``` 



[File:FIFOQueue.sol#L61](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FIFOQueue.sol#L61) 
```solidity
60:  function shift(FIFOQueue storage arr) internal {
``` 



[File:FIFOQueue.sol#L38](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FIFOQueue.sol#L38) 
```solidity
37:  function length(FIFOQueue storage arr) internal view returns (uint128) {
``` 



[File:FIFOQueue.sol#L19](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FIFOQueue.sol#L19) 
```solidity
18:  function empty(FIFOQueue storage arr) internal view returns (bool) {
``` 



[File:FIFOQueue.sol#L30](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FIFOQueue.sol#L30) 
```solidity
29:  function at(FIFOQueue storage arr, uint256 index) internal view returns (uint32) {
``` 



[File:FIFOQueue.sol#L55](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FIFOQueue.sol#L55) 
```solidity
54:  function push(FIFOQueue storage arr, uint32 value) internal {
``` 



[File:SafeCastLib.sol#L41](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L41) 
```solidity
40:  function toUint56(uint256 x) internal pure returns (uint56 y) {
``` 



[File:SafeCastLib.sol#L73](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L73) 
```solidity
72:  function toUint120(uint256 x) internal pure returns (uint120 y) {
``` 



[File:SafeCastLib.sol#L85](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L85) 
```solidity
84:  function toUint144(uint256 x) internal pure returns (uint144 y) {
``` 



[File:SafeCastLib.sol#L49](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L49) 
```solidity
48:  function toUint72(uint256 x) internal pure returns (uint72 y) {
``` 



[File:SafeCastLib.sol#L93](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L93) 
```solidity
92:  function toUint160(uint256 x) internal pure returns (uint160 y) {
``` 



[File:SafeCastLib.sol#L125](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L125) 
```solidity
124:  function toUint224(uint256 x) internal pure returns (uint224 y) {
``` 



[File:SafeCastLib.sol#L61](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L61) 
```solidity
60:  function toUint96(uint256 x) internal pure returns (uint96 y) {
``` 



[File:SafeCastLib.sol#L69](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L69) 
```solidity
68:  function toUint112(uint256 x) internal pure returns (uint112 y) {
``` 



[File:SafeCastLib.sol#L29](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L29) 
```solidity
28:  function toUint32(uint256 x) internal pure returns (uint32 y) {
``` 



[File:SafeCastLib.sol#L37](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L37) 
```solidity
36:  function toUint48(uint256 x) internal pure returns (uint48 y) {
``` 



[File:SafeCastLib.sol#L45](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L45) 
```solidity
44:  function toUint64(uint256 x) internal pure returns (uint64 y) {
``` 



[File:SafeCastLib.sol#L25](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L25) 
```solidity
24:  function toUint24(uint256 x) internal pure returns (uint24 y) {
``` 



[File:SafeCastLib.sol#L105](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L105) 
```solidity
104:  function toUint184(uint256 x) internal pure returns (uint184 y) {
``` 



[File:SafeCastLib.sol#L117](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L117) 
```solidity
116:  function toUint208(uint256 x) internal pure returns (uint208 y) {
``` 



[File:SafeCastLib.sol#L121](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L121) 
```solidity
120:  function toUint216(uint256 x) internal pure returns (uint216 y) {
``` 



[File:SafeCastLib.sol#L57](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L57) 
```solidity
56:  function toUint88(uint256 x) internal pure returns (uint88 y) {
``` 



[File:SafeCastLib.sol#L17](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L17) 
```solidity
16:  function toUint8(uint256 x) internal pure returns (uint8 y) {
``` 



[File:SafeCastLib.sol#L33](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L33) 
```solidity
32:  function toUint40(uint256 x) internal pure returns (uint40 y) {
``` 



[File:SafeCastLib.sol#L81](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L81) 
```solidity
80:  function toUint136(uint256 x) internal pure returns (uint136 y) {
``` 



[File:SafeCastLib.sol#L129](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L129) 
```solidity
128:  function toUint232(uint256 x) internal pure returns (uint232 y) {
``` 



[File:SafeCastLib.sol#L21](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L21) 
```solidity
20:  function toUint16(uint256 x) internal pure returns (uint16 y) {
``` 



[File:SafeCastLib.sol#L89](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L89) 
```solidity
88:  function toUint152(uint256 x) internal pure returns (uint152 y) {
``` 



[File:SafeCastLib.sol#L101](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L101) 
```solidity
100:  function toUint176(uint256 x) internal pure returns (uint176 y) {
``` 



[File:SafeCastLib.sol#L113](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L113) 
```solidity
112:  function toUint200(uint256 x) internal pure returns (uint200 y) {
``` 



[File:SafeCastLib.sol#L137](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L137) 
```solidity
136:  function toUint248(uint256 x) internal pure returns (uint248 y) {
``` 



[File:SafeCastLib.sol#L97](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L97) 
```solidity
96:  function toUint168(uint256 x) internal pure returns (uint168 y) {
``` 



[File:SafeCastLib.sol#L77](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L77) 
```solidity
76:  function toUint128(uint256 x) internal pure returns (uint128 y) {
``` 



[File:SafeCastLib.sol#L65](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L65) 
```solidity
64:  function toUint104(uint256 x) internal pure returns (uint104 y) {
``` 



[File:SafeCastLib.sol#L109](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L109) 
```solidity
108:  function toUint192(uint256 x) internal pure returns (uint192 y) {
``` 



[File:SafeCastLib.sol#L53](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L53) 
```solidity
52:  function toUint80(uint256 x) internal pure returns (uint80 y) {
``` 



[File:SafeCastLib.sol#L133](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L133) 
```solidity
132:  function toUint240(uint256 x) internal pure returns (uint240 y) {
``` 



[File:LibStoredInitCode.sol#L7](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/LibStoredInitCode.sol#L7) 
```solidity
6:  function deployInitCode(bytes memory data) internal returns (address initCodeStorage) {
``` 



[File:LibStoredInitCode.sol#L48](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/LibStoredInitCode.sol#L48) 
```solidity
47:  function getCreate2Prefix(address deployer) internal pure returns (uint256 create2Prefix) {
``` 



[File:LibStoredInitCode.sol#L54](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/LibStoredInitCode.sol#L54) 
```solidity
53:  function calculateCreate2Address(
54:    uint256 create2Prefix,
55:    bytes32 salt,
56:    uint256 initCodeHash
57:  ) internal pure returns (address create2Address) {
``` 



[File:MathUtils.sol#L155](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L155) 
```solidity
154:  function rayDiv(uint256 a, uint256 b) internal pure returns (uint256 c) {
``` 



[File:MathUtils.sol#L173](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L173) 
```solidity
172:  function mulDiv(uint256 x, uint256 y, uint256 d) internal pure returns (uint256 z) {
``` 



[File:MathUtils.sol#L121](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L121) 
```solidity
120:  function bipToRay(uint256 a) internal pure returns (uint256 b) {
``` 



[File:MathUtils.sol#L191](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L191) 
```solidity
190:  function mulDivUp(uint256 x, uint256 y, uint256 d) internal pure returns (uint256 z) {
``` 



[File:MathUtils.sol#L85](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L85) 
```solidity
84:  function bipMul(uint256 a, uint256 b) internal pure returns (uint256 c) {
``` 



[File:MathUtils.sol#L30](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L30) 
```solidity
29:  function calculateLinearInterestFromBips(
30:    uint256 rateBip,
31:    uint256 timeDelta
32:  ) internal pure returns (uint256 result) {
``` 



[File:MathUtils.sol#L44](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L44) 
```solidity
43:  function min(uint256 a, uint256 b) internal pure returns (uint256 c) {
``` 



[File:MathUtils.sol#L59](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L59) 
```solidity
58:  function satSub(uint256 a, uint256 b) internal pure returns (uint256 c) {
``` 



[File:MathUtils.sol#L138](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L138) 
```solidity
137:  function rayMul(uint256 a, uint256 b) internal pure returns (uint256 c) {
``` 



[File:MathUtils.sol#L105](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L105) 
```solidity
104:  function bipDiv(uint256 a, uint256 b) internal pure returns (uint256 c) {
``` 



[File:MathUtils.sol#L51](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L51) 
```solidity
50:  function max(uint256 a, uint256 b) internal pure returns (uint256 c) {
``` 



 --- 

<a name=[NonCritical-10]></a>
### [NonCritical-10] Remove any unused returns - Instances: 6 

 > Either remove the return parameter names, or use them as the returns of the function. 

 --- 

[File:WildcatSanctionsSentinel.sol#L65](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsSentinel.sol#L65) 
```solidity
64:  function getEscrowAddress(
65:    address borrower,
66:    address account,
67:    address asset
68:  ) public view override returns (address escrowAddress) {
``` 



[File:WildcatMarketController.sol#L370](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L370) 
```solidity
369:  function _deriveSalt(
370:    address asset,
371:    string memory namePrefix,
372:    string memory symbolPrefix
373:  ) internal pure returns (bytes32 salt) {
``` 



[File:WildcatMarketControllerFactory.sol#L165](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L165) 
```solidity
164:  function getProtocolFeeConfiguration()
165:    external
166:    view
167:    returns (
168:      address feeRecipient,
169:      address originationFeeAsset,
170:      uint80 originationFeeAmount,
171:      uint16 protocolFeeBips
172:    )
173:  {
``` 



[File:WildcatMarketControllerFactory.sol#L165](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L165) 
```solidity
164:  function getProtocolFeeConfiguration()
165:    external
166:    view
167:    returns (
168:      address feeRecipient,
169:      address originationFeeAsset,
170:      uint80 originationFeeAmount,
171:      uint16 protocolFeeBips
172:    )
173:  {
``` 



[File:WildcatMarketControllerFactory.sol#L165](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L165) 
```solidity
164:  function getProtocolFeeConfiguration()
165:    external
166:    view
167:    returns (
168:      address feeRecipient,
169:      address originationFeeAsset,
170:      uint80 originationFeeAmount,
171:      uint16 protocolFeeBips
172:    )
173:  {
``` 



[File:WildcatMarketControllerFactory.sol#L165](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L165) 
```solidity
164:  function getProtocolFeeConfiguration()
165:    external
166:    view
167:    returns (
168:      address feeRecipient,
169:      address originationFeeAsset,
170:      uint80 originationFeeAmount,
171:      uint16 protocolFeeBips
172:    )
173:  {
``` 



 --- 

<a name=[NonCritical-11]></a>
### [NonCritical-11] Function parameters should be in camelCase - Instances: 342 

 > Ensure that function parameters are declared using camelCase 

 --- 

[File:Errors.sol#L47](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/Errors.sol#L47) 
```solidity
46:function revertWithSelectorAndArgument(bytes4 errorSelector, uint256 argument) pure {
``` 



[File:Errors.sol#L60](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/Errors.sol#L60) 
```solidity
59:function revertWithSelectorAndArgument(uint256 errorSelector, uint256 argument) pure {
``` 



[File:WildcatArchController.sol#L63](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L63) 
```solidity
62:  function registerBorrower(address borrower) external onlyOwner {
``` 



[File:WildcatArchController.sol#L70](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L70) 
```solidity
69:  function removeBorrower(address borrower) external onlyOwner {
``` 



[File:WildcatArchController.sol#L77](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L77) 
```solidity
76:  function isRegisteredBorrower(address borrower) external view returns (bool) {
``` 



[File:WildcatArchController.sol#L86](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L86) 
```solidity
85:    uint256 start,
``` 



[File:WildcatArchController.sol#L87](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L87) 
```solidity
86:    uint256 end
87:  ) external view returns (address[] memory arr) {
``` 



[File:WildcatArchController.sol#L88](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L88) 
```solidity
87:  ) external view returns (address[] memory arr) {
``` 



[File:WildcatArchController.sol#L106](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L106) 
```solidity
105:  function registerControllerFactory(address factory) external onlyOwner {
``` 



[File:WildcatArchController.sol#L113](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L113) 
```solidity
112:  function removeControllerFactory(address factory) external onlyOwner {
``` 



[File:WildcatArchController.sol#L120](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L120) 
```solidity
119:  function isRegisteredControllerFactory(address factory) external view returns (bool) {
``` 



[File:WildcatArchController.sol#L129](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L129) 
```solidity
128:    uint256 start,
``` 



[File:WildcatArchController.sol#L130](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L130) 
```solidity
129:    uint256 end
130:  ) external view returns (address[] memory arr) {
``` 



[File:WildcatArchController.sol#L131](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L131) 
```solidity
130:  ) external view returns (address[] memory arr) {
``` 



[File:WildcatArchController.sol#L149](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L149) 
```solidity
148:  function registerController(address controller) external onlyControllerFactory {
``` 



[File:WildcatArchController.sol#L156](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L156) 
```solidity
155:  function removeController(address controller) external onlyOwner {
``` 



[File:WildcatArchController.sol#L163](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L163) 
```solidity
162:  function isRegisteredController(address controller) external view returns (bool) {
``` 



[File:WildcatArchController.sol#L172](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L172) 
```solidity
171:    uint256 start,
``` 



[File:WildcatArchController.sol#L173](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L173) 
```solidity
172:    uint256 end
173:  ) external view returns (address[] memory arr) {
``` 



[File:WildcatArchController.sol#L174](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L174) 
```solidity
173:  ) external view returns (address[] memory arr) {
``` 



[File:WildcatArchController.sol#L192](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L192) 
```solidity
191:  function registerMarket(address market) external onlyController {
``` 



[File:WildcatArchController.sol#L199](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L199) 
```solidity
198:  function removeMarket(address market) external onlyOwner {
``` 



[File:WildcatArchController.sol#L206](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L206) 
```solidity
205:  function isRegisteredMarket(address market) external view returns (bool) {
``` 



[File:WildcatArchController.sol#L215](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L215) 
```solidity
214:    uint256 start,
``` 



[File:WildcatArchController.sol#L216](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L216) 
```solidity
215:    uint256 end
216:  ) external view returns (address[] memory arr) {
``` 



[File:WildcatArchController.sol#L217](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatArchController.sol#L217) 
```solidity
216:  ) external view returns (address[] memory arr) {
``` 



[File:MathUtils.sol#L33](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L33) 
```solidity
32:  ) internal pure returns (uint256 result) {
``` 



[File:MathUtils.sol#L44](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L44) 
```solidity
43:  function min(uint256 a, uint256 b) internal pure returns (uint256 c) {
``` 



[File:MathUtils.sol#L44](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L44) 
```solidity
43:  function min(uint256 a, uint256 b) internal pure returns (uint256 c) {
``` 



[File:MathUtils.sol#L44](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L44) 
```solidity
43:  function min(uint256 a, uint256 b) internal pure returns (uint256 c) {
``` 



[File:MathUtils.sol#L51](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L51) 
```solidity
50:  function max(uint256 a, uint256 b) internal pure returns (uint256 c) {
``` 



[File:MathUtils.sol#L51](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L51) 
```solidity
50:  function max(uint256 a, uint256 b) internal pure returns (uint256 c) {
``` 



[File:MathUtils.sol#L51](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L51) 
```solidity
50:  function max(uint256 a, uint256 b) internal pure returns (uint256 c) {
``` 



[File:MathUtils.sol#L59](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L59) 
```solidity
58:  function satSub(uint256 a, uint256 b) internal pure returns (uint256 c) {
``` 



[File:MathUtils.sol#L59](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L59) 
```solidity
58:  function satSub(uint256 a, uint256 b) internal pure returns (uint256 c) {
``` 



[File:MathUtils.sol#L59](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L59) 
```solidity
58:  function satSub(uint256 a, uint256 b) internal pure returns (uint256 c) {
``` 



[File:MathUtils.sol#L72](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L72) 
```solidity
71:    bool condition,
``` 



[File:MathUtils.sol#L75](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L75) 
```solidity
74:  ) internal pure returns (uint256 c) {
``` 



[File:MathUtils.sol#L85](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L85) 
```solidity
84:  function bipMul(uint256 a, uint256 b) internal pure returns (uint256 c) {
``` 



[File:MathUtils.sol#L85](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L85) 
```solidity
84:  function bipMul(uint256 a, uint256 b) internal pure returns (uint256 c) {
``` 



[File:MathUtils.sol#L85](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L85) 
```solidity
84:  function bipMul(uint256 a, uint256 b) internal pure returns (uint256 c) {
``` 



[File:MathUtils.sol#L105](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L105) 
```solidity
104:  function bipDiv(uint256 a, uint256 b) internal pure returns (uint256 c) {
``` 



[File:MathUtils.sol#L105](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L105) 
```solidity
104:  function bipDiv(uint256 a, uint256 b) internal pure returns (uint256 c) {
``` 



[File:MathUtils.sol#L105](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L105) 
```solidity
104:  function bipDiv(uint256 a, uint256 b) internal pure returns (uint256 c) {
``` 



[File:MathUtils.sol#L121](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L121) 
```solidity
120:  function bipToRay(uint256 a) internal pure returns (uint256 b) {
``` 



[File:MathUtils.sol#L121](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L121) 
```solidity
120:  function bipToRay(uint256 a) internal pure returns (uint256 b) {
``` 



[File:MathUtils.sol#L138](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L138) 
```solidity
137:  function rayMul(uint256 a, uint256 b) internal pure returns (uint256 c) {
``` 



[File:MathUtils.sol#L138](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L138) 
```solidity
137:  function rayMul(uint256 a, uint256 b) internal pure returns (uint256 c) {
``` 



[File:MathUtils.sol#L138](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L138) 
```solidity
137:  function rayMul(uint256 a, uint256 b) internal pure returns (uint256 c) {
``` 



[File:MathUtils.sol#L155](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L155) 
```solidity
154:  function rayDiv(uint256 a, uint256 b) internal pure returns (uint256 c) {
``` 



[File:MathUtils.sol#L155](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L155) 
```solidity
154:  function rayDiv(uint256 a, uint256 b) internal pure returns (uint256 c) {
``` 



[File:MathUtils.sol#L155](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L155) 
```solidity
154:  function rayDiv(uint256 a, uint256 b) internal pure returns (uint256 c) {
``` 



[File:MathUtils.sol#L173](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L173) 
```solidity
172:  function mulDiv(uint256 x, uint256 y, uint256 d) internal pure returns (uint256 z) {
``` 



[File:MathUtils.sol#L173](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L173) 
```solidity
172:  function mulDiv(uint256 x, uint256 y, uint256 d) internal pure returns (uint256 z) {
``` 



[File:MathUtils.sol#L173](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L173) 
```solidity
172:  function mulDiv(uint256 x, uint256 y, uint256 d) internal pure returns (uint256 z) {
``` 



[File:MathUtils.sol#L173](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L173) 
```solidity
172:  function mulDiv(uint256 x, uint256 y, uint256 d) internal pure returns (uint256 z) {
``` 



[File:MathUtils.sol#L191](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L191) 
```solidity
190:  function mulDivUp(uint256 x, uint256 y, uint256 d) internal pure returns (uint256 z) {
``` 



[File:MathUtils.sol#L191](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L191) 
```solidity
190:  function mulDivUp(uint256 x, uint256 y, uint256 d) internal pure returns (uint256 z) {
``` 



[File:MathUtils.sol#L191](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L191) 
```solidity
190:  function mulDivUp(uint256 x, uint256 y, uint256 d) internal pure returns (uint256 z) {
``` 



[File:MathUtils.sol#L191](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MathUtils.sol#L191) 
```solidity
190:  function mulDivUp(uint256 x, uint256 y, uint256 d) internal pure returns (uint256 z) {
``` 



[File:StringQuery.sol#L19](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/StringQuery.sol#L19) 
```solidity
18:function bytes32ToString(bytes32 value) pure returns (string memory str) {
``` 



[File:StringQuery.sol#L19](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/StringQuery.sol#L19) 
```solidity
18:function bytes32ToString(bytes32 value) pure returns (string memory str) {
``` 



[File:StringQuery.sol#L34](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/StringQuery.sol#L34) 
```solidity
33:  address target,
``` 



[File:StringQuery.sol#L37](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/StringQuery.sol#L37) 
```solidity
36:) view returns (string memory str) {
``` 



[File:StringQuery.sol#L96](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/StringQuery.sol#L96) 
```solidity
95:function queryName(address target) view returns (string memory) {
``` 



[File:StringQuery.sol#L101](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/StringQuery.sol#L101) 
```solidity
100:function querySymbol(address target) view returns (string memory) {
``` 



[File:SafeCastLib.sol#L17](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L17) 
```solidity
16:  function toUint8(uint256 x) internal pure returns (uint8 y) {
``` 



[File:SafeCastLib.sol#L17](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L17) 
```solidity
16:  function toUint8(uint256 x) internal pure returns (uint8 y) {
``` 



[File:SafeCastLib.sol#L21](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L21) 
```solidity
20:  function toUint16(uint256 x) internal pure returns (uint16 y) {
``` 



[File:SafeCastLib.sol#L21](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L21) 
```solidity
20:  function toUint16(uint256 x) internal pure returns (uint16 y) {
``` 



[File:SafeCastLib.sol#L25](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L25) 
```solidity
24:  function toUint24(uint256 x) internal pure returns (uint24 y) {
``` 



[File:SafeCastLib.sol#L25](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L25) 
```solidity
24:  function toUint24(uint256 x) internal pure returns (uint24 y) {
``` 



[File:SafeCastLib.sol#L29](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L29) 
```solidity
28:  function toUint32(uint256 x) internal pure returns (uint32 y) {
``` 



[File:SafeCastLib.sol#L29](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L29) 
```solidity
28:  function toUint32(uint256 x) internal pure returns (uint32 y) {
``` 



[File:SafeCastLib.sol#L33](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L33) 
```solidity
32:  function toUint40(uint256 x) internal pure returns (uint40 y) {
``` 



[File:SafeCastLib.sol#L33](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L33) 
```solidity
32:  function toUint40(uint256 x) internal pure returns (uint40 y) {
``` 



[File:SafeCastLib.sol#L37](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L37) 
```solidity
36:  function toUint48(uint256 x) internal pure returns (uint48 y) {
``` 



[File:SafeCastLib.sol#L37](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L37) 
```solidity
36:  function toUint48(uint256 x) internal pure returns (uint48 y) {
``` 



[File:SafeCastLib.sol#L41](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L41) 
```solidity
40:  function toUint56(uint256 x) internal pure returns (uint56 y) {
``` 



[File:SafeCastLib.sol#L41](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L41) 
```solidity
40:  function toUint56(uint256 x) internal pure returns (uint56 y) {
``` 



[File:SafeCastLib.sol#L45](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L45) 
```solidity
44:  function toUint64(uint256 x) internal pure returns (uint64 y) {
``` 



[File:SafeCastLib.sol#L45](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L45) 
```solidity
44:  function toUint64(uint256 x) internal pure returns (uint64 y) {
``` 



[File:SafeCastLib.sol#L49](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L49) 
```solidity
48:  function toUint72(uint256 x) internal pure returns (uint72 y) {
``` 



[File:SafeCastLib.sol#L49](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L49) 
```solidity
48:  function toUint72(uint256 x) internal pure returns (uint72 y) {
``` 



[File:SafeCastLib.sol#L53](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L53) 
```solidity
52:  function toUint80(uint256 x) internal pure returns (uint80 y) {
``` 



[File:SafeCastLib.sol#L53](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L53) 
```solidity
52:  function toUint80(uint256 x) internal pure returns (uint80 y) {
``` 



[File:SafeCastLib.sol#L57](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L57) 
```solidity
56:  function toUint88(uint256 x) internal pure returns (uint88 y) {
``` 



[File:SafeCastLib.sol#L57](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L57) 
```solidity
56:  function toUint88(uint256 x) internal pure returns (uint88 y) {
``` 



[File:SafeCastLib.sol#L61](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L61) 
```solidity
60:  function toUint96(uint256 x) internal pure returns (uint96 y) {
``` 



[File:SafeCastLib.sol#L61](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L61) 
```solidity
60:  function toUint96(uint256 x) internal pure returns (uint96 y) {
``` 



[File:SafeCastLib.sol#L65](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L65) 
```solidity
64:  function toUint104(uint256 x) internal pure returns (uint104 y) {
``` 



[File:SafeCastLib.sol#L65](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L65) 
```solidity
64:  function toUint104(uint256 x) internal pure returns (uint104 y) {
``` 



[File:SafeCastLib.sol#L69](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L69) 
```solidity
68:  function toUint112(uint256 x) internal pure returns (uint112 y) {
``` 



[File:SafeCastLib.sol#L69](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L69) 
```solidity
68:  function toUint112(uint256 x) internal pure returns (uint112 y) {
``` 



[File:SafeCastLib.sol#L73](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L73) 
```solidity
72:  function toUint120(uint256 x) internal pure returns (uint120 y) {
``` 



[File:SafeCastLib.sol#L73](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L73) 
```solidity
72:  function toUint120(uint256 x) internal pure returns (uint120 y) {
``` 



[File:SafeCastLib.sol#L77](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L77) 
```solidity
76:  function toUint128(uint256 x) internal pure returns (uint128 y) {
``` 



[File:SafeCastLib.sol#L77](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L77) 
```solidity
76:  function toUint128(uint256 x) internal pure returns (uint128 y) {
``` 



[File:SafeCastLib.sol#L81](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L81) 
```solidity
80:  function toUint136(uint256 x) internal pure returns (uint136 y) {
``` 



[File:SafeCastLib.sol#L81](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L81) 
```solidity
80:  function toUint136(uint256 x) internal pure returns (uint136 y) {
``` 



[File:SafeCastLib.sol#L85](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L85) 
```solidity
84:  function toUint144(uint256 x) internal pure returns (uint144 y) {
``` 



[File:SafeCastLib.sol#L85](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L85) 
```solidity
84:  function toUint144(uint256 x) internal pure returns (uint144 y) {
``` 



[File:SafeCastLib.sol#L89](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L89) 
```solidity
88:  function toUint152(uint256 x) internal pure returns (uint152 y) {
``` 



[File:SafeCastLib.sol#L89](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L89) 
```solidity
88:  function toUint152(uint256 x) internal pure returns (uint152 y) {
``` 



[File:SafeCastLib.sol#L93](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L93) 
```solidity
92:  function toUint160(uint256 x) internal pure returns (uint160 y) {
``` 



[File:SafeCastLib.sol#L93](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L93) 
```solidity
92:  function toUint160(uint256 x) internal pure returns (uint160 y) {
``` 



[File:SafeCastLib.sol#L97](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L97) 
```solidity
96:  function toUint168(uint256 x) internal pure returns (uint168 y) {
``` 



[File:SafeCastLib.sol#L97](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L97) 
```solidity
96:  function toUint168(uint256 x) internal pure returns (uint168 y) {
``` 



[File:SafeCastLib.sol#L101](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L101) 
```solidity
100:  function toUint176(uint256 x) internal pure returns (uint176 y) {
``` 



[File:SafeCastLib.sol#L101](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L101) 
```solidity
100:  function toUint176(uint256 x) internal pure returns (uint176 y) {
``` 



[File:SafeCastLib.sol#L105](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L105) 
```solidity
104:  function toUint184(uint256 x) internal pure returns (uint184 y) {
``` 



[File:SafeCastLib.sol#L105](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L105) 
```solidity
104:  function toUint184(uint256 x) internal pure returns (uint184 y) {
``` 



[File:SafeCastLib.sol#L109](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L109) 
```solidity
108:  function toUint192(uint256 x) internal pure returns (uint192 y) {
``` 



[File:SafeCastLib.sol#L109](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L109) 
```solidity
108:  function toUint192(uint256 x) internal pure returns (uint192 y) {
``` 



[File:SafeCastLib.sol#L113](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L113) 
```solidity
112:  function toUint200(uint256 x) internal pure returns (uint200 y) {
``` 



[File:SafeCastLib.sol#L113](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L113) 
```solidity
112:  function toUint200(uint256 x) internal pure returns (uint200 y) {
``` 



[File:SafeCastLib.sol#L117](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L117) 
```solidity
116:  function toUint208(uint256 x) internal pure returns (uint208 y) {
``` 



[File:SafeCastLib.sol#L117](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L117) 
```solidity
116:  function toUint208(uint256 x) internal pure returns (uint208 y) {
``` 



[File:SafeCastLib.sol#L121](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L121) 
```solidity
120:  function toUint216(uint256 x) internal pure returns (uint216 y) {
``` 



[File:SafeCastLib.sol#L121](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L121) 
```solidity
120:  function toUint216(uint256 x) internal pure returns (uint216 y) {
``` 



[File:SafeCastLib.sol#L125](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L125) 
```solidity
124:  function toUint224(uint256 x) internal pure returns (uint224 y) {
``` 



[File:SafeCastLib.sol#L125](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L125) 
```solidity
124:  function toUint224(uint256 x) internal pure returns (uint224 y) {
``` 



[File:SafeCastLib.sol#L129](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L129) 
```solidity
128:  function toUint232(uint256 x) internal pure returns (uint232 y) {
``` 



[File:SafeCastLib.sol#L129](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L129) 
```solidity
128:  function toUint232(uint256 x) internal pure returns (uint232 y) {
``` 



[File:SafeCastLib.sol#L133](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L133) 
```solidity
132:  function toUint240(uint256 x) internal pure returns (uint240 y) {
``` 



[File:SafeCastLib.sol#L133](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L133) 
```solidity
132:  function toUint240(uint256 x) internal pure returns (uint240 y) {
``` 



[File:SafeCastLib.sol#L137](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L137) 
```solidity
136:  function toUint248(uint256 x) internal pure returns (uint248 y) {
``` 



[File:SafeCastLib.sol#L137](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/SafeCastLib.sol#L137) 
```solidity
136:  function toUint248(uint256 x) internal pure returns (uint248 y) {
``` 



[File:WildcatSanctionsSentinel.sol#L39](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsSentinel.sol#L39) 
```solidity
38:  function isSanctioned(address borrower, address account) public view override returns (bool) {
``` 



[File:WildcatSanctionsSentinel.sol#L39](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsSentinel.sol#L39) 
```solidity
38:  function isSanctioned(address borrower, address account) public view override returns (bool) {
``` 



[File:WildcatSanctionsSentinel.sol#L48](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsSentinel.sol#L48) 
```solidity
47:  function overrideSanction(address account) public override {
``` 



[File:WildcatSanctionsSentinel.sol#L56](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsSentinel.sol#L56) 
```solidity
55:  function removeSanctionOverride(address account) public override {
``` 



[File:WildcatSanctionsSentinel.sol#L66](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsSentinel.sol#L66) 
```solidity
65:    address borrower,
``` 



[File:WildcatSanctionsSentinel.sol#L67](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsSentinel.sol#L67) 
```solidity
66:    address account,
``` 



[File:WildcatSanctionsSentinel.sol#L68](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsSentinel.sol#L68) 
```solidity
67:    address asset
68:  ) public view override returns (address escrowAddress) {
``` 



[File:WildcatSanctionsSentinel.sol#L96](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsSentinel.sol#L96) 
```solidity
95:    address borrower,
``` 



[File:WildcatSanctionsSentinel.sol#L97](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsSentinel.sol#L97) 
```solidity
96:    address account,
``` 



[File:WildcatSanctionsSentinel.sol#L98](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsSentinel.sol#L98) 
```solidity
97:    address asset
98:  ) public override returns (address escrowContract) {
``` 



[File:IWildcatArchController.sol#L22](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatArchController.sol#L22) 
```solidity
21:    uint256 start,
``` 



[File:IWildcatArchController.sol#L23](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatArchController.sol#L23) 
```solidity
22:    uint256 end
23:  ) external view returns (address[] memory);
``` 



[File:IWildcatArchController.sol#L28](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatArchController.sol#L28) 
```solidity
27:  function isRegisteredControllerFactory(address factory) external view returns (bool);
``` 



[File:IWildcatArchController.sol#L30](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatArchController.sol#L30) 
```solidity
29:  function registerControllerFactory(address factory) external;
``` 



[File:IWildcatArchController.sol#L32](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatArchController.sol#L32) 
```solidity
31:  function removeControllerFactory(address factory) external;
``` 



[File:IWildcatArchController.sol#L45](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatArchController.sol#L45) 
```solidity
44:    uint256 start,
``` 



[File:IWildcatArchController.sol#L46](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatArchController.sol#L46) 
```solidity
45:    uint256 end
46:  ) external view returns (address[] memory);
``` 



[File:IWildcatArchController.sol#L51](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatArchController.sol#L51) 
```solidity
50:  function isRegisteredController(address controller) external view returns (bool);
``` 



[File:IWildcatArchController.sol#L53](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatArchController.sol#L53) 
```solidity
52:  function registerController(address controller) external;
``` 



[File:IWildcatArchController.sol#L55](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatArchController.sol#L55) 
```solidity
54:  function removeController(address controller) external;
``` 



[File:IWildcatArchController.sol#L68](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatArchController.sol#L68) 
```solidity
67:    uint256 start,
``` 



[File:IWildcatArchController.sol#L69](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatArchController.sol#L69) 
```solidity
68:    uint256 end
69:  ) external view returns (address[] memory);
``` 



[File:IWildcatArchController.sol#L74](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatArchController.sol#L74) 
```solidity
73:  function isRegisteredBorrower(address borrower) external view returns (bool);
``` 



[File:IWildcatArchController.sol#L76](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatArchController.sol#L76) 
```solidity
75:  function registerBorrower(address borrower) external;
``` 



[File:IWildcatArchController.sol#L78](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatArchController.sol#L78) 
```solidity
77:  function removeBorrower(address borrower) external;
``` 



[File:IWildcatArchController.sol#L90](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatArchController.sol#L90) 
```solidity
89:  function getRegisteredMarkets(uint256 start, uint256 end) external view returns (address[] memory);
``` 



[File:IWildcatArchController.sol#L90](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatArchController.sol#L90) 
```solidity
89:  function getRegisteredMarkets(uint256 start, uint256 end) external view returns (address[] memory);
``` 



[File:IWildcatArchController.sol#L94](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatArchController.sol#L94) 
```solidity
93:  function isRegisteredMarket(address market) external view returns (bool);
``` 



[File:IWildcatArchController.sol#L96](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatArchController.sol#L96) 
```solidity
95:  function registerMarket(address market) external;
``` 



[File:IWildcatArchController.sol#L98](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatArchController.sol#L98) 
```solidity
97:  function removeMarket(address market) external;
``` 



[File:WildcatMarketControllerFactory.sol#L74](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L74) 
```solidity
73:    address _sentinel,
``` 



[File:WildcatMarketControllerFactory.sol#L75](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L75) 
```solidity
74:    MarketParameterConstraints memory constraints
75:  ) {
``` 



[File:WildcatMarketControllerFactory.sol#L126](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L126) 
```solidity
125:  function isDeployedController(address controller) external view returns (bool) {
``` 



[File:WildcatMarketControllerFactory.sol#L139](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L139) 
```solidity
138:    uint256 start,
``` 



[File:WildcatMarketControllerFactory.sol#L140](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L140) 
```solidity
139:    uint256 end
140:  ) external view returns (address[] memory arr) {
``` 



[File:WildcatMarketControllerFactory.sol#L141](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L141) 
```solidity
140:  ) external view returns (address[] memory arr) {
``` 



[File:WildcatMarketControllerFactory.sol#L226](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L226) 
```solidity
225:    returns (MarketParameterConstraints memory constraints)
``` 



[File:WildcatMarketControllerFactory.sol#L250](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L250) 
```solidity
249:    returns (MarketControllerParameters memory parameters)
``` 



[File:WildcatMarketControllerFactory.sol#L282](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L282) 
```solidity
281:  function deployController() public returns (address controller) {
``` 



[File:WildcatMarketControllerFactory.sol#L320](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L320) 
```solidity
319:    address asset,
``` 



[File:WildcatMarketControllerFactory.sol#L327](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L327) 
```solidity
326:  ) external returns (address controller, address market) {
``` 



[File:WildcatMarketControllerFactory.sol#L327](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L327) 
```solidity
326:  ) external returns (address controller, address market) {
``` 



[File:WildcatMarketControllerFactory.sol#L342](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketControllerFactory.sol#L342) 
```solidity
341:  function computeControllerAddress(address borrower) external view returns (address) {
``` 



[File:WildcatMarketBase.sol#L150](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L150) 
```solidity
149:  function _getAccount(address accountAddress) internal view returns (Account memory account) {
``` 



[File:WildcatMarketBase.sol#L163](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L163) 
```solidity
162:  function _blockAccount(MarketState memory state, address accountAddress) internal {
``` 



[File:WildcatMarketBase.sol#L200](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L200) 
```solidity
199:  ) internal returns (Account memory account) {
``` 



[File:WildcatMarketBase.sol#L276](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L276) 
```solidity
275:  function currentState() public view nonReentrantView returns (MarketState memory state) {
``` 



[File:WildcatMarketBase.sol#L292](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L292) 
```solidity
291:  function scaledBalanceOf(address account) external view nonReentrantView returns (uint256) {
``` 



[File:WildcatMarketBase.sol#L299](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L299) 
```solidity
298:  function getAccountRole(address account) external view nonReentrantView returns (AuthRole) {
``` 



[File:WildcatMarketBase.sol#L358](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L358) 
```solidity
357:  function _getUpdatedState() internal returns (MarketState memory state) {
``` 



[File:WildcatMarketBase.sol#L403](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L403) 
```solidity
402:      MarketState memory state,
``` 



[File:WildcatMarketBase.sol#L448](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L448) 
```solidity
447:  function _writeState(MarketState memory state) internal {
``` 



[File:WildcatMarketBase.sol#L466](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L466) 
```solidity
465:  function _processExpiredWithdrawalBatch(MarketState memory state) internal {
``` 



[File:WildcatMarketBase.sol#L499](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L499) 
```solidity
498:    WithdrawalBatch memory batch,
``` 



[File:WildcatMarketBase.sol#L500](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L500) 
```solidity
499:    MarketState memory state,
``` 



[File:WildcatMarketBase.sol#L501](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L501) 
```solidity
500:    uint32 expiry,
``` 



[File:WildcatMarketBase.sol#L529](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L529) 
```solidity
528:    WithdrawalBatch memory batch,
``` 



[File:WildcatMarketBase.sol#L530](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L530) 
```solidity
529:    MarketState memory state,
``` 



[File:LibStoredInitCode.sol#L7](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/LibStoredInitCode.sol#L7) 
```solidity
6:  function deployInitCode(bytes memory data) internal returns (address initCodeStorage) {
``` 



[File:LibStoredInitCode.sol#L48](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/LibStoredInitCode.sol#L48) 
```solidity
47:  function getCreate2Prefix(address deployer) internal pure returns (uint256 create2Prefix) {
``` 



[File:LibStoredInitCode.sol#L56](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/LibStoredInitCode.sol#L56) 
```solidity
55:    bytes32 salt,
``` 



[File:LibStoredInitCode.sol#L83](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/LibStoredInitCode.sol#L83) 
```solidity
82:  function createWithStoredInitCode(address initCodeStorage) internal returns (address deployment) {
``` 



[File:LibStoredInitCode.sol#L89](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/LibStoredInitCode.sol#L89) 
```solidity
88:    uint256 value
89:  ) internal returns (address deployment) {
``` 



[File:LibStoredInitCode.sol#L90](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/LibStoredInitCode.sol#L90) 
```solidity
89:  ) internal returns (address deployment) {
``` 



[File:LibStoredInitCode.sol#L101](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/LibStoredInitCode.sol#L101) 
```solidity
100:    bytes32 salt
101:  ) internal returns (address deployment) {
``` 



[File:LibStoredInitCode.sol#L102](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/LibStoredInitCode.sol#L102) 
```solidity
101:  ) internal returns (address deployment) {
``` 



[File:LibStoredInitCode.sol#L108](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/LibStoredInitCode.sol#L108) 
```solidity
107:    bytes32 salt,
``` 



[File:LibStoredInitCode.sol#L109](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/LibStoredInitCode.sol#L109) 
```solidity
108:    uint256 value
109:  ) internal returns (address deployment) {
``` 



[File:LibStoredInitCode.sol#L110](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/LibStoredInitCode.sol#L110) 
```solidity
109:  ) internal returns (address deployment) {
``` 



[File:IWildcatMarketControllerFactory.sol#L30](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketControllerFactory.sol#L30) 
```solidity
29:  function isDeployedController(address controller) external view returns (bool);
``` 



[File:IWildcatMarketControllerFactory.sol#L37](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketControllerFactory.sol#L37) 
```solidity
36:    uint256 start,
``` 



[File:IWildcatMarketControllerFactory.sol#L38](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketControllerFactory.sol#L38) 
```solidity
37:    uint256 count
38:  ) external view returns (address[] memory);
``` 



[File:IWildcatMarketControllerFactory.sol#L91](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketControllerFactory.sol#L91) 
```solidity
90:    returns (MarketParameterConstraints memory constraints);
``` 



[File:IWildcatMarketControllerFactory.sol#L108](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketControllerFactory.sol#L108) 
```solidity
107:  function deployController() external returns (address controller);
``` 



[File:IWildcatMarketControllerFactory.sol#L126](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketControllerFactory.sol#L126) 
```solidity
125:    address asset,
``` 



[File:IWildcatMarketControllerFactory.sol#L133](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketControllerFactory.sol#L133) 
```solidity
132:  ) external returns (address controller, address market);
``` 



[File:IWildcatMarketControllerFactory.sol#L133](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketControllerFactory.sol#L133) 
```solidity
132:  ) external returns (address controller, address market);
``` 



[File:IWildcatSanctionsSentinel.sol#L35](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatSanctionsSentinel.sol#L35) 
```solidity
34:    returns (address borrower, address account, address asset);
``` 



[File:IWildcatSanctionsSentinel.sol#L35](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatSanctionsSentinel.sol#L35) 
```solidity
34:    returns (address borrower, address account, address asset);
``` 



[File:IWildcatSanctionsSentinel.sol#L35](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatSanctionsSentinel.sol#L35) 
```solidity
34:    returns (address borrower, address account, address asset);
``` 



[File:IWildcatSanctionsSentinel.sol#L39](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatSanctionsSentinel.sol#L39) 
```solidity
38:  function isSanctioned(address borrower, address account) external view returns (bool);
``` 



[File:IWildcatSanctionsSentinel.sol#L39](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatSanctionsSentinel.sol#L39) 
```solidity
38:  function isSanctioned(address borrower, address account) external view returns (bool);
``` 



[File:IWildcatSanctionsSentinel.sol#L43](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatSanctionsSentinel.sol#L43) 
```solidity
42:  function sanctionOverrides(address borrower, address account) external view returns (bool);
``` 



[File:IWildcatSanctionsSentinel.sol#L43](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatSanctionsSentinel.sol#L43) 
```solidity
42:  function sanctionOverrides(address borrower, address account) external view returns (bool);
``` 



[File:IWildcatSanctionsSentinel.sol#L45](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatSanctionsSentinel.sol#L45) 
```solidity
44:  function overrideSanction(address account) external;
``` 



[File:IWildcatSanctionsSentinel.sol#L47](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatSanctionsSentinel.sol#L47) 
```solidity
46:  function removeSanctionOverride(address account) external;
``` 



[File:IWildcatSanctionsSentinel.sol#L52](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatSanctionsSentinel.sol#L52) 
```solidity
51:    address account,
``` 



[File:IWildcatSanctionsSentinel.sol#L53](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatSanctionsSentinel.sol#L53) 
```solidity
52:    address borrower,
``` 



[File:IWildcatSanctionsSentinel.sol#L54](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatSanctionsSentinel.sol#L54) 
```solidity
53:    address asset
54:  ) external view returns (address escrowContract);
``` 



[File:IWildcatSanctionsSentinel.sol#L72](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatSanctionsSentinel.sol#L72) 
```solidity
71:    address account,
``` 



[File:IWildcatSanctionsSentinel.sol#L73](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatSanctionsSentinel.sol#L73) 
```solidity
72:    address borrower,
``` 



[File:IWildcatSanctionsSentinel.sol#L74](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatSanctionsSentinel.sol#L74) 
```solidity
73:    address asset
74:  ) external returns (address escrowContract);
``` 



[File:IERC20.sol#L10](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IERC20.sol#L10) 
```solidity
9:  function balanceOf(address account) external view returns (uint256);
``` 



[File:IERC20.sol#L12](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IERC20.sol#L12) 
```solidity
11:  function transfer(address recipient, uint256 amount) external returns (bool);
``` 



[File:IERC20.sol#L12](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IERC20.sol#L12) 
```solidity
11:  function transfer(address recipient, uint256 amount) external returns (bool);
``` 



[File:IERC20.sol#L14](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IERC20.sol#L14) 
```solidity
13:  function allowance(address owner, address spender) external view returns (uint256);
``` 



[File:IERC20.sol#L14](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IERC20.sol#L14) 
```solidity
13:  function allowance(address owner, address spender) external view returns (uint256);
``` 



[File:IERC20.sol#L16](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IERC20.sol#L16) 
```solidity
15:  function approve(address spender, uint256 amount) external returns (bool);
``` 



[File:IERC20.sol#L16](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IERC20.sol#L16) 
```solidity
15:  function approve(address spender, uint256 amount) external returns (bool);
``` 



[File:IERC20.sol#L18](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IERC20.sol#L18) 
```solidity
17:  function increaseAllowance(address spender, uint256 addedValue) external returns (bool);
``` 



[File:IERC20.sol#L20](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IERC20.sol#L20) 
```solidity
19:  function decreaseAllowance(address spender, uint256 subtractedValue) external returns (bool);
``` 



[File:IERC20.sol#L22](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IERC20.sol#L22) 
```solidity
21:  function transferFrom(address sender, address recipient, uint256 amount) external returns (bool);
``` 



[File:IERC20.sol#L22](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IERC20.sol#L22) 
```solidity
21:  function transferFrom(address sender, address recipient, uint256 amount) external returns (bool);
``` 



[File:IERC20.sol#L22](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IERC20.sol#L22) 
```solidity
21:  function transferFrom(address sender, address recipient, uint256 amount) external returns (bool);
``` 



[File:IWildcatSanctionsEscrow.sol#L19](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatSanctionsEscrow.sol#L19) 
```solidity
18:  function escrowedAsset() external view returns (address token, uint256 amount);
``` 



[File:IWildcatSanctionsEscrow.sol#L19](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatSanctionsEscrow.sol#L19) 
```solidity
18:  function escrowedAsset() external view returns (address token, uint256 amount);
``` 



[File:FIFOQueue.sol#L19](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FIFOQueue.sol#L19) 
```solidity
18:  function empty(FIFOQueue storage arr) internal view returns (bool) {
``` 



[File:FIFOQueue.sol#L23](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FIFOQueue.sol#L23) 
```solidity
22:  function first(FIFOQueue storage arr) internal view returns (uint32) {
``` 



[File:FIFOQueue.sol#L30](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FIFOQueue.sol#L30) 
```solidity
29:  function at(FIFOQueue storage arr, uint256 index) internal view returns (uint32) {
``` 



[File:FIFOQueue.sol#L30](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FIFOQueue.sol#L30) 
```solidity
29:  function at(FIFOQueue storage arr, uint256 index) internal view returns (uint32) {
``` 



[File:FIFOQueue.sol#L38](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FIFOQueue.sol#L38) 
```solidity
37:  function length(FIFOQueue storage arr) internal view returns (uint128) {
``` 



[File:FIFOQueue.sol#L42](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FIFOQueue.sol#L42) 
```solidity
41:  function values(FIFOQueue storage arr) internal view returns (uint32[] memory _values) {
``` 



[File:FIFOQueue.sol#L42](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FIFOQueue.sol#L42) 
```solidity
41:  function values(FIFOQueue storage arr) internal view returns (uint32[] memory _values) {
``` 



[File:FIFOQueue.sol#L55](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FIFOQueue.sol#L55) 
```solidity
54:  function push(FIFOQueue storage arr, uint32 value) internal {
``` 



[File:FIFOQueue.sol#L55](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FIFOQueue.sol#L55) 
```solidity
54:  function push(FIFOQueue storage arr, uint32 value) internal {
``` 



[File:FIFOQueue.sol#L61](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FIFOQueue.sol#L61) 
```solidity
60:  function shift(FIFOQueue storage arr) internal {
``` 



[File:FIFOQueue.sol#L70](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FIFOQueue.sol#L70) 
```solidity
69:  function shiftN(FIFOQueue storage arr, uint128 n) internal {
``` 



[File:FIFOQueue.sol#L70](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FIFOQueue.sol#L70) 
```solidity
69:  function shiftN(FIFOQueue storage arr, uint128 n) internal {
``` 



[File:WildcatMarketController.sol#L87](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L87) 
```solidity
86:  modifier onlyControlledMarket(address market) {
``` 



[File:WildcatMarketController.sol#L126](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L126) 
```solidity
125:    uint256 start,
``` 



[File:WildcatMarketController.sol#L127](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L127) 
```solidity
126:    uint256 end
127:  ) external view returns (address[] memory arr) {
``` 



[File:WildcatMarketController.sol#L128](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L128) 
```solidity
127:  ) external view returns (address[] memory arr) {
``` 



[File:WildcatMarketController.sol#L142](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L142) 
```solidity
141:  function isAuthorizedLender(address lender) external view virtual returns (bool) {
``` 



[File:WildcatMarketController.sol#L153](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L153) 
```solidity
152:  function authorizeLenders(address[] memory lenders) external onlyBorrower {
``` 



[File:WildcatMarketController.sol#L169](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L169) 
```solidity
168:  function deauthorizeLenders(address[] memory lenders) external onlyBorrower {
``` 



[File:WildcatMarketController.sol#L182](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L182) 
```solidity
181:  function updateLenderAuthorization(address lender, address[] memory markets) external {
``` 



[File:WildcatMarketController.sol#L182](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L182) 
```solidity
181:  function updateLenderAuthorization(address lender, address[] memory markets) external {
``` 



[File:WildcatMarketController.sol#L196](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L196) 
```solidity
195:  function isControlledMarket(address market) external view returns (bool) {
``` 



[File:WildcatMarketController.sol#L205](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L205) 
```solidity
204:    uint256 start,
``` 



[File:WildcatMarketController.sol#L206](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L206) 
```solidity
205:    uint256 end
206:  ) external view returns (address[] memory arr) {
``` 



[File:WildcatMarketController.sol#L207](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L207) 
```solidity
206:  ) external view returns (address[] memory arr) {
``` 



[File:WildcatMarketController.sol#L222](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L222) 
```solidity
221:    address asset,
``` 



[File:WildcatMarketController.sol#L238](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L238) 
```solidity
237:  function getMarketParameters() external view returns (MarketParameters memory parameters) {
``` 



[File:WildcatMarketController.sol#L292](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L292) 
```solidity
291:    address asset,
``` 



[File:WildcatMarketController.sol#L301](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L301) 
```solidity
300:  ) external returns (address market) {
``` 



[File:WildcatMarketController.sol#L371](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L371) 
```solidity
370:    address asset,
``` 



[File:WildcatMarketController.sol#L374](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L374) 
```solidity
373:  ) internal pure returns (bytes32 salt) {
``` 



[File:WildcatMarketController.sol#L449](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L449) 
```solidity
448:    returns (MarketParameterConstraints memory constraints)
``` 



[File:WildcatMarketController.sol#L469](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L469) 
```solidity
468:    address market,
``` 



[File:WildcatMarketController.sol#L490](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L490) 
```solidity
489:  function resetReserveRatio(address market) external virtual {
``` 



[File:WildcatMarketController.sol#L504](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L504) 
```solidity
503:    uint256 value,
``` 



[File:WildcatMarketController.sol#L505](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L505) 
```solidity
504:    uint256 min,
``` 



[File:WildcatMarketController.sol#L506](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L506) 
```solidity
505:    uint256 max,
``` 



[File:Withdrawal.sol#L38](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/Withdrawal.sol#L38) 
```solidity
37:  function scaledOwedAmount(WithdrawalBatch memory batch) internal pure returns (uint104) {
``` 



[File:Withdrawal.sol#L48](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/Withdrawal.sol#L48) 
```solidity
47:    WithdrawalBatch memory batch,
``` 



[File:Withdrawal.sol#L49](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/Withdrawal.sol#L49) 
```solidity
48:    MarketState memory state,
``` 



[File:BoolUtils.sol#L5](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/BoolUtils.sol#L5) 
```solidity
4:  function and(bool a, bool b) internal pure returns (bool c) {
``` 



[File:BoolUtils.sol#L5](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/BoolUtils.sol#L5) 
```solidity
4:  function and(bool a, bool b) internal pure returns (bool c) {
``` 



[File:BoolUtils.sol#L5](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/BoolUtils.sol#L5) 
```solidity
4:  function and(bool a, bool b) internal pure returns (bool c) {
``` 



[File:BoolUtils.sol#L11](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/BoolUtils.sol#L11) 
```solidity
10:  function or(bool a, bool b) internal pure returns (bool c) {
``` 



[File:BoolUtils.sol#L11](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/BoolUtils.sol#L11) 
```solidity
10:  function or(bool a, bool b) internal pure returns (bool c) {
``` 



[File:BoolUtils.sol#L11](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/BoolUtils.sol#L11) 
```solidity
10:  function or(bool a, bool b) internal pure returns (bool c) {
``` 



[File:BoolUtils.sol#L17](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/BoolUtils.sol#L17) 
```solidity
16:  function xor(bool a, bool b) internal pure returns (bool c) {
``` 



[File:BoolUtils.sol#L17](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/BoolUtils.sol#L17) 
```solidity
16:  function xor(bool a, bool b) internal pure returns (bool c) {
``` 



[File:BoolUtils.sol#L17](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/BoolUtils.sol#L17) 
```solidity
16:  function xor(bool a, bool b) internal pure returns (bool c) {
``` 



[File:WildcatMarketToken.sol#L16](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketToken.sol#L16) 
```solidity
15:  function balanceOf(address account) public view virtual nonReentrantView returns (uint256) {
``` 



[File:WildcatMarketToken.sol#L17](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketToken.sol#L17) 
```solidity
16:    (MarketState memory state, , ) = _calculateCurrentState();
``` 



[File:WildcatMarketToken.sol#L23](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketToken.sol#L23) 
```solidity
22:    (MarketState memory state, , ) = _calculateCurrentState();
``` 



[File:WildcatMarketToken.sol#L31](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketToken.sol#L31) 
```solidity
30:  function approve(address spender, uint256 amount) external virtual nonReentrant returns (bool) {
``` 



[File:WildcatMarketToken.sol#L31](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketToken.sol#L31) 
```solidity
30:  function approve(address spender, uint256 amount) external virtual nonReentrant returns (bool) {
``` 



[File:WildcatMarketToken.sol#L36](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketToken.sol#L36) 
```solidity
35:  function transfer(address to, uint256 amount) external virtual nonReentrant returns (bool) {
``` 



[File:WildcatMarketToken.sol#L36](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketToken.sol#L36) 
```solidity
35:  function transfer(address to, uint256 amount) external virtual nonReentrant returns (bool) {
``` 



[File:WildcatMarketToken.sol#L42](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketToken.sol#L42) 
```solidity
41:    address from,
``` 



[File:WildcatMarketToken.sol#L43](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketToken.sol#L43) 
```solidity
42:    address to,
``` 



[File:WildcatMarketToken.sol#L44](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketToken.sol#L44) 
```solidity
43:    uint256 amount
44:  ) external virtual nonReentrant returns (bool) {
``` 



[File:WildcatMarketToken.sol#L59](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketToken.sol#L59) 
```solidity
58:  function _approve(address approver, address spender, uint256 amount) internal virtual {
``` 



[File:WildcatMarketToken.sol#L59](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketToken.sol#L59) 
```solidity
58:  function _approve(address approver, address spender, uint256 amount) internal virtual {
``` 



[File:WildcatMarketToken.sol#L59](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketToken.sol#L59) 
```solidity
58:  function _approve(address approver, address spender, uint256 amount) internal virtual {
``` 



[File:WildcatMarketToken.sol#L64](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketToken.sol#L64) 
```solidity
63:  function _transfer(address from, address to, uint256 amount) internal virtual {
``` 



[File:WildcatMarketToken.sol#L64](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketToken.sol#L64) 
```solidity
63:  function _transfer(address from, address to, uint256 amount) internal virtual {
``` 



[File:WildcatMarketToken.sol#L64](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketToken.sol#L64) 
```solidity
63:  function _transfer(address from, address to, uint256 amount) internal virtual {
``` 



[File:IChainalysisSanctionsList.sol#L5](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IChainalysisSanctionsList.sol#L5) 
```solidity
4:  function isSanctioned(address addr) external view returns (bool);
``` 



[File:FeeMath.sol#L22](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FeeMath.sol#L22) 
```solidity
21:  ) internal pure returns (uint256 result) {
``` 



[File:FeeMath.sol#L31](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FeeMath.sol#L31) 
```solidity
30:    MarketState memory state,
``` 



[File:FeeMath.sol#L32](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FeeMath.sol#L32) 
```solidity
31:    uint256 timestamp
32:  ) internal pure returns (uint256 baseInterestRay) {
``` 



[File:FeeMath.sol#L41](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FeeMath.sol#L41) 
```solidity
40:    MarketState memory state,
``` 



[File:FeeMath.sol#L54](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FeeMath.sol#L54) 
```solidity
53:    MarketState memory state,
``` 



[File:FeeMath.sol#L55](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FeeMath.sol#L55) 
```solidity
54:    uint256 timestamp,
``` 



[File:FeeMath.sol#L90](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FeeMath.sol#L90) 
```solidity
89:    MarketState memory state,
``` 



[File:FeeMath.sol#L143](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FeeMath.sol#L143) 
```solidity
142:    MarketState memory state,
``` 



[File:FeeMath.sol#L147](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/FeeMath.sol#L147) 
```solidity
146:    uint256 timestamp
147:  )
``` 



[File:IWildcatMarketController.sol#L48](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketController.sol#L48) 
```solidity
47:    returns (MarketParameterConstraints memory constraints);
``` 



[File:IWildcatMarketController.sol#L57](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketController.sol#L57) 
```solidity
56:    uint256 start,
``` 



[File:IWildcatMarketController.sol#L58](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketController.sol#L58) 
```solidity
57:    uint256 end
58:  ) external view returns (address[] memory);
``` 



[File:IWildcatMarketController.sol#L63](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketController.sol#L63) 
```solidity
62:  function isAuthorizedLender(address lender) external view returns (bool);
``` 



[File:IWildcatMarketController.sol#L72](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketController.sol#L72) 
```solidity
71:  function authorizeLenders(address[] memory lenders) external;
``` 



[File:IWildcatMarketController.sol#L81](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketController.sol#L81) 
```solidity
80:  function deauthorizeLenders(address[] memory lenders) external;
``` 



[File:IWildcatMarketController.sol#L87](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketController.sol#L87) 
```solidity
86:  function updateLenderAuthorization(address lender, address[] memory markets) external;
``` 



[File:IWildcatMarketController.sol#L87](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketController.sol#L87) 
```solidity
86:  function updateLenderAuthorization(address lender, address[] memory markets) external;
``` 



[File:IWildcatMarketController.sol#L98](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketController.sol#L98) 
```solidity
97:  function setAnnualInterestBips(address market, uint16 annualInterestBips) external;
``` 



[File:IWildcatMarketController.sol#L104](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketController.sol#L104) 
```solidity
103:  function resetReserveRatio(address market) external;
``` 



[File:IWildcatMarketController.sol#L108](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketController.sol#L108) 
```solidity
107:  ) external view returns (uint128 reserveRatioBips, uint128 expiry);
``` 



[File:IWildcatMarketController.sol#L127](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/interfaces/IWildcatMarketController.sol#L127) 
```solidity
126:    address asset,
``` 



[File:WildcatMarketWithdrawals.sol#L29](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketWithdrawals.sol#L29) 
```solidity
28:    uint32 expiry
29:  ) external view nonReentrantView returns (WithdrawalBatch memory) {
``` 



[File:WildcatMarketWithdrawals.sol#L40](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketWithdrawals.sol#L40) 
```solidity
39:    uint32 expiry
40:  ) external view nonReentrantView returns (AccountWithdrawalStatus memory) {
``` 



[File:WildcatMarketWithdrawals.sol#L47](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketWithdrawals.sol#L47) 
```solidity
46:    uint32 expiry
47:  ) external view nonReentrantView returns (uint256) {
``` 



[File:WildcatMarketWithdrawals.sol#L77](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketWithdrawals.sol#L77) 
```solidity
76:  function queueWithdrawal(uint256 amount) external nonReentrant {
``` 



[File:WildcatMarketWithdrawals.sol#L139](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketWithdrawals.sol#L139) 
```solidity
138:    uint32 expiry
139:  ) external nonReentrant returns (uint256) {
``` 



[File:MarketState.sol#L51](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MarketState.sol#L51) 
```solidity
50:  function totalSupply(MarketState memory state) internal pure returns (uint256) {
``` 



[File:MarketState.sol#L59](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MarketState.sol#L59) 
```solidity
58:  function maximumDeposit(MarketState memory state) internal pure returns (uint256) {
``` 



[File:MarketState.sol#L67](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MarketState.sol#L67) 
```solidity
66:    MarketState memory state,
``` 



[File:MarketState.sol#L68](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MarketState.sol#L68) 
```solidity
67:    uint256 amount
68:  ) internal pure returns (uint256) {
``` 



[File:MarketState.sol#L76](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MarketState.sol#L76) 
```solidity
75:  function scaleAmount(MarketState memory state, uint256 amount) internal pure returns (uint256) {
``` 



[File:MarketState.sol#L76](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MarketState.sol#L76) 
```solidity
75:  function scaleAmount(MarketState memory state, uint256 amount) internal pure returns (uint256) {
``` 



[File:MarketState.sol#L88](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MarketState.sol#L88) 
```solidity
87:    MarketState memory state
88:  ) internal pure returns (uint256 _liquidityRequired) {
``` 



[File:MarketState.sol#L106](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MarketState.sol#L106) 
```solidity
105:    MarketState memory state,
``` 



[File:MarketState.sol#L124](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MarketState.sol#L124) 
```solidity
123:    MarketState memory state,
``` 



[File:MarketState.sol#L130](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MarketState.sol#L130) 
```solidity
129:  function hasPendingExpiredBatch(MarketState memory state) internal view returns (bool result) {
``` 



[File:MarketState.sol#L130](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MarketState.sol#L130) 
```solidity
129:  function hasPendingExpiredBatch(MarketState memory state) internal view returns (bool result) {
``` 



[File:MarketState.sol#L138](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/libraries/MarketState.sol#L138) 
```solidity
137:  function totalDebts(MarketState memory state) internal pure returns (uint256) {
``` 



[File:WildcatMarket.sol#L43](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarket.sol#L43) 
```solidity
42:    uint256 amount
43:  ) public virtual nonReentrant returns (uint256 /* actualAmount */) {
``` 



[File:WildcatMarket.sol#L86](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarket.sol#L86) 
```solidity
85:  function deposit(uint256 amount) external virtual {
``` 



[File:WildcatMarket.sol#L119](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarket.sol#L119) 
```solidity
118:  function borrow(uint256 amount) external onlyBorrower nonReentrant {
``` 



[File:WildcatMarketConfig.sol#L113](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketConfig.sol#L113) 
```solidity
112:    address _account,
``` 



 --- 

<a name=[NonCritical-12]></a>
### [NonCritical-12] Consider explicitly naming mapping parameters - Instances: 4 

 > Consider explicitly naming mapping parameters for readability 

 --- 

[File:WildcatMarketToken.sol#L13](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketToken.sol#L13) 
```solidity
12:  mapping(address => mapping(address => uint256)) public allowance;
``` 



[File:WildcatMarketBase.sol#L68](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/market/WildcatMarketBase.sol#L68) 
```solidity
67:  mapping(address => Account) internal _accounts;
``` 



[File:WildcatSanctionsSentinel.sol#L20](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatSanctionsSentinel.sol#L20) 
```solidity
19:  mapping(address borrower => mapping(address account => bool sanctionOverride))
20:    public
21:    override sanctionOverrides;
``` 



[File:WildcatMarketController.sol#L76](https://github.com/code-423n4/2023-10-wildcat/blob/main/src/WildcatMarketController.sol#L76) 
```solidity
75:  mapping(address => TemporaryReserveRatio) public temporaryExcessReserveRatio;
``` 



 --- 


