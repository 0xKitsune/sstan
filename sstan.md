# Sstan - v0.1.0 

 --- 
 ## Authors: 0x00face, 0xOsiris 
 --- 
 TODO: add description

# Summary




## Vulnerabilities 

 | Classification | Title | Instances | 
 |:-------:|:---------|:-------:| 
 | [[High-0]](#[High-0]) | Uninitialized storage variables | 1 |
 | [[Medium-1]](#[Medium-1]) | Division before multiplication | 9 |
 | [[Low-2]](#[Low-2]) | Use a locked pragma version instead of a floating pragma version | 2 |
 | [[Low-3]](#[Low-3]) | Unsafe ERC20 Operation | 12 |
## Optimizations 

 | Classification | Title | Instances | 
 |:-------:|:---------|:-------:| 
 | [[Gas-0]](#[Gas-0]) | Tightly pack storage variables | 1 |
 | [[Gas-1]](#[Gas-1]) | Mark storage variables as `constant` if they never change. | 3 |
 | [[Gas-2]](#[Gas-2]) | Mark storage variables as `immutable` if they never change after contract initialization. | 3 |
 | [[Gas-3]](#[Gas-3]) | `unchecked{++i}` instead of `i++` (or use assembly when applicable) | 5 |
 | [[Gas-4]](#[Gas-4]) | Cache Storage Variables in Memory | 6 |
 | [[Gas-5]](#[Gas-5]) | Use `calldata` instead of `memory` for function arguments that do not get mutated. | 4 |
 | [[Gas-6]](#[Gas-6]) | Use assembly to hash instead of Solidity | 2 |
 | [[Gas-7]](#[Gas-7]) | Use custom errors instead of string error messages | 7 |
 | [[Gas-8]](#[Gas-8]) | Use assembly for math (add, sub, mul, div) | 6 |
 | [[Gas-9]](#[Gas-9]) | Use assembly to write storage values | 6 |
 | [[Gas-10]](#[Gas-10]) | Event is not properly indexed. | 7 |
 | [[Gas-11]](#[Gas-11]) | Right shift or Left shift instead of dividing or multiplying by powers of two | 5 |
 | [[Gas-12]](#[Gas-12]) | Use multiple require() statments insted of require(expression && expression && ...) | 7 |
 | [[Gas-13]](#[Gas-13]) | Optimal Comparison | 5 |
 | [[Gas-14]](#[Gas-14]) | Mark functions as payable (with discretion) | 6 |
 | [[Gas-15]](#[Gas-15]) | Consider marking constants as private | 5 |
 | [[Gas-16]](#[Gas-16]) | Use assembly to check for address(0) | 6 |
 | [[Gas-17]](#[Gas-17]) | Cache array length during for loop definition. | 4 |
## Quality Assurance 

 | Classification | Title | Instances | 
 |:-------:|:---------|:-------:| 
 | [[NonCritical-0]](#[NonCritical-0]) | Private variables should contain a leading underscore | 11 |
 | [[NonCritical-1]](#[NonCritical-1]) | Constructor should check that all parameters are not 0 | 6 |
 | [[NonCritical-2]](#[NonCritical-2]) | Consider importing specific identifiers instead of the whole file | 30 |
 | [[NonCritical-3]](#[NonCritical-3]) | Storage variables should be named with camel case | 1 |
 | [[NonCritical-4]](#[NonCritical-4]) | Function names should be in camelCase | 59 |
 | [[NonCritical-5]](#[NonCritical-5]) | Constant and immutable variable names should be in SCREAMING_SNAKE_CASE | 36 |
 | [[NonCritical-6]](#[NonCritical-6]) | Remove any unused returns | 11 |
 | [[NonCritical-7]](#[NonCritical-7]) | Consider marking public function External | 3 |
 | [[NonCritical-8]](#[NonCritical-8]) | Consider adding a message with require and revert statements | 53 |
 | [[NonCritical-9]](#[NonCritical-9]) | Storage variables should not have implicit visibility | 1 |
 | [[NonCritical-10]](#[NonCritical-10]) | This variables default value is the same as the value it is initialized with | 3 |
 | [[NonCritical-11]](#[NonCritical-11]) | Large contracts with many external functions should inherit an interface | 2 |
 | [[NonCritical-12]](#[NonCritical-12]) | Function parameters should be in camelCase | 264 |
 | [[NonCritical-13]](#[NonCritical-13]) | Require/Revert statements should be consistent across the codebase | 36 |
 | [[NonCritical-14]](#[NonCritical-14]) | Consider explicitly naming mapping parameters | 53 |

## Vulnerabilities - Total: 24 

<a name=[High-0]></a>
### [High-0] Uninitialized storage variables - Instances: 1 

 > 
A storage variable that is declared but not initialized will have a default value of zero (or the equivalent, such as an empty array for array types or zero-address for address types). Failing to initialize a storage variable can pose risks if the contract logic assumes that the variable has been explicitly set to a particular value. 

 --- 

File:Voter.sol#L19
```solidity
18:    address public convenience;
``` 



 --- 

<a name=[Medium-1]></a>
### [Medium-1] Division before multiplication - Instances: 9 

 > 
Consider ordering multiplication before division to avoid loss of precision because integer division might truncate. Loss of precision in Solidity can lead to vulnerabilities because it can result in unexpected behavior in smart contracts. This can be particularly problematic in financial applications, where even small errors in calculations can have significant consequences. For example, if a contract uses integer division to calculate a result and the division operation truncates the fractional part of the result, it could lead to incorrect pricing or loss of funds due to miscalculated balances. 

<details>
<summary>Expand Example</summary>

#### Unsafe Division

```js
    n = 5 / 2 * 4; // n = 8 because 5 / 2 == 2 since division truncates.
```

#### Safe Division
```js
    n = 5 * 4 / 2; // n = 10
```

</details>
         

 --- 

File:Voter.sol#L423
```solidity
422:        activePeriod = block.timestamp / DURATION * DURATION;
``` 



File:Voter.sol#L429
```solidity
428:        nextPeriod = (block.timestamp + DURATION) / DURATION * DURATION;
``` 



File:Minter.sol#L59
```solidity
58:        active_period = (block.timestamp + (2*week)) / week * week;
``` 



File:Minter.sol#L78
```solidity
77:        active_period = block.timestamp / week * week; // 
``` 



File:Minter.sol#L125
```solidity
124:            _period = block.timestamp / week * week;
``` 



File:SwapPair.sol#L392
```solidity
391:        return x0*(y*y/1e18*y/1e18)/1e18+(x0*x0/1e18*x0/1e18)*y/1e18;
``` 



File:SwapPair.sol#L392
```solidity
391:        return x0*(y*y/1e18*y/1e18)/1e18+(x0*x0/1e18*x0/1e18)*y/1e18;
``` 



File:SwapPair.sol#L392
```solidity
391:        return x0*(y*y/1e18*y/1e18)/1e18+(x0*x0/1e18*x0/1e18)*y/1e18;
``` 



File:SwapPair.sol#L396
```solidity
395:        return 3*x0*(y*y/1e18)/1e18+(x0*x0/1e18*x0/1e18);
``` 



 --- 

<a name=[Low-2]></a>
### [Low-2] Use a locked pragma version instead of a floating pragma version - Instances: 2 

 > 
Floating pragma is a vulnerability in smart contract code that can cause unexpected behavior by allowing the compiler to use a specified range of versions. 
 This can lead to issues such as using an older compiler version with known vulnerabilities, using a newer compiler version with undiscovered vulnerabilities, inconsistency across files using different versions, or unpredictable behavior because the compiler can use any version within the specified range. It is recommended to use a locked pragma version in order to avoid these potential vulnerabilities. In some cases it may be acceptable to use a floating pragma, such as when a contract is intended for consumption by other developers and needs to be compatible with a range of compiler versions.
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
 

 --- 

File:SwapPair.sol#L2
```solidity
1:pragma solidity ^0.8.11;
``` 



File:SwapFactory.sol#L2
```solidity
1:pragma solidity ^0.8.11;
``` 



 --- 

<a name=[Low-3]></a>
### [Low-3] Unsafe ERC20 Operation - Instances: 12 

 > 
ERC20 operations can be unsafe due to different implementations and vulnerabilities in the standard. To account for this, either use OpenZeppelin's SafeERC20 library or wrap each operation in a require statement. 

> Additionally, ERC20's approve functions have a known race-condition vulnerability. To account for this, use OpenZeppelin's SafeERC20 library's `safeIncrease` or `safeDecrease` Allowance functions.
<details>
<summary>Expand Example</summary>

#### Unsafe Transfer

```js
IERC20(token).transfer(msg.sender, amount);
```

#### OpenZeppelin SafeTransfer

```js
import {SafeERC20} from "openzeppelin/token/utils/SafeERC20.sol";
//--snip--

IERC20(token).safeTransfer(msg.sender, address(this), amount);
```
        
#### Safe Transfer with require statement.

```js
bool success = IERC20(token).transfer(msg.sender, amount);
require(success, "ERC20 transfer failed");
```
        
#### Unsafe TransferFrom

```js
IERC20(token).transferFrom(msg.sender, address(this), amount);
```

#### OpenZeppelin SafeTransferFrom

```js
import {SafeERC20} from "openzeppelin/token/utils/SafeERC20.sol";
//--snip--

IERC20(token).safeTransferFrom(msg.sender, address(this), amount);
```
        
#### Safe TransferFrom with require statement.

```js
bool success = IERC20(token).transferFrom(msg.sender, address(this), amount);
require(success, "ERC20 transfer failed");
```

</details>
        
         

 --- 

File:Bribe.sol#L456
```solidity
455:        token.call(abi.encodeWithSelector(IERC20.transfer.selector, to, value));
``` 



File:Bribe.sol#L463
```solidity
462:        token.call(abi.encodeWithSelector(IERC20.transferFrom.selector, from, to, value));
``` 



File:SwapPair.sol#L540
```solidity
539:        token.call(abi.encodeWithSelector(IERC20.transfer.selector, to, value));
``` 



File:Gauge.sol#L569
```solidity
568:        token.call(abi.encodeWithSelector(IERC20.transfer.selector, to, value));
``` 



File:Gauge.sol#L576
```solidity
575:        token.call(abi.encodeWithSelector(IERC20.transferFrom.selector, from, to, value));
``` 



File:Gauge.sol#L583
```solidity
582:        token.call(abi.encodeWithSelector(IERC20.approve.selector, spender, value));
``` 



File:Minter.sol#L136
```solidity
135:            require(_token.transfer(address(_ve_dist), _growth), "growth transfer failed");
``` 



File:Minter.sol#L141
```solidity
140:            _token.approve(address(_voter), weekly);
``` 



File:Voter.sol#L254
```solidity
253:        IERC20(base).approve(_gauge, type(uint).max);
``` 



File:Voter.sol#L436
```solidity
435:        token.call(abi.encodeWithSelector(IERC20.transferFrom.selector, from, to, value));
``` 



File:Multiswap.sol#L55
```solidity
54:            (bool transferFromSuccess) = IERC20(_token).transferFrom(msg.sender, address(this), _amount);
``` 



File:Multiswap.sol#L57
```solidity
56:            IERC20(_token).approve(router, _amount);
``` 



 --- 



## Optimizations - Total: 88 

<a name=[Gas-0]></a>
### [Gas-0] Tightly pack storage variables - Instances: 1 

 > 
 When defining storage variables, make sure to declare them in ascending order, according to size. When multiple variables are able to fit into one 256 bit slot, this will save storage size and gas during runtime. For example, if you have a `bool`, `uint256` and a `bool`, instead of defining the variables in the previously mentioned order, defining the two boolean variables first will pack them both into one storage slot since they only take up one byte of storage. 
 
#### Gas Report - Savings: ~0 
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
        bool bool0 = true;
        bool bool1 = false;
        uint256 num0 = 200;
        uint256 num1 = 100;
        c0.accessNonTightlyPacked(bool0, bool1, num0, num1);
        c1.accessTightlyPacked(bool0, bool1, num0, num1);
    }
}

contract Contract0 {
    uint256 num0 = 100;
    bool bool0 = false;
    uint256 num1 = 200;
    bool bool1 = true;

    function accessNonTightlyPacked(
        bool _bool0,
        bool _bool1,
        uint256 _num0,
        uint256 _num1
    ) public {
        bool0 = _bool0;
        bool1 = _bool1;
        num0 = _num0;
        num1 = _num1;
    }
}

contract Contract1 {
    bool bool0 = false;
    bool bool1 = true;
    uint256 num0 = 100;
    uint256 num1 = 200;

    function accessTightlyPacked(
        bool _bool0,
        bool _bool1,
        uint256 _num0,
        uint256 _num1
    ) public {
        bool0 = _bool0;
        bool1 = _bool1;
        num0 = _num0;
        num1 = _num1;
    }
}

```

```solidity
╭───────────────────────────────────────────┬─────────────────┬───────┬────────┬───────┬─────────╮
│ src/test/GasTest.t.sol:Contract0 contract ┆                 ┆       ┆        ┆       ┆         │
╞═══════════════════════════════════════════╪═════════════════╪═══════╪════════╪═══════╪═════════╡
│ Deployment Cost                           ┆ Deployment Size ┆       ┆        ┆       ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 122268                                    ┆ 334             ┆       ┆        ┆       ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name                             ┆ min             ┆ avg   ┆ median ┆ max   ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ accessNonTightlyPacked                    ┆ 32774           ┆ 32774 ┆ 32774  ┆ 32774 ┆ 1       │
╰───────────────────────────────────────────┴─────────────────┴───────┴────────┴───────┴─────────╯
╭───────────────────────────────────────────┬─────────────────┬───────┬────────┬───────┬─────────╮
│ src/test/GasTest.t.sol:Contract1 contract ┆                 ┆       ┆        ┆       ┆         │
╞═══════════════════════════════════════════╪═════════════════╪═══════╪════════╪═══════╪═════════╡
│ Deployment Cost                           ┆ Deployment Size ┆       ┆        ┆       ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 126247                                    ┆ 356             ┆       ┆        ┆       ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name                             ┆ min             ┆ avg   ┆ median ┆ max   ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ accessTightlyPacked                       ┆ 15476           ┆ 15476 ┆ 15476  ┆ 15476 ┆ 1       │
╰───────────────────────────────────────────┴─────────────────┴───────┴────────┴───────┴─────────╯

```

 
 </details> 
 

 --- 

File:SwapPair.sol#L15
```solidity
14:    string public name;
``` 



 --- 

<a name=[Gas-1]></a>
### [Gas-1] Mark storage variables as `constant` if they never change. - Instances: 3 

 > 
 State variables can be declared as constant or immutable. In both cases, the variables cannot be modified after the contract has been constructed. For constant variables, the value has to be fixed at compile-time, while for immutable, it can still be assigned at construction time. 
 The compiler does not reserve a storage slot for these variables, and every occurrence is inlined by the respective value. 
 Compared to regular state variables, the gas costs of constant and immutable variables are much lower. For a constant variable, the expression assigned to it is copied to all the places where it is accessed and also re-evaluated each time. This allows for local optimizations. Immutable variables are evaluated once at construction time and their value is copied to all the places in the code where they are accessed. For these values, 32 bytes are reserved, even if they would fit in fewer bytes. Due to this, constant values can sometimes be cheaper than immutable values. 
 
#### Gas Report - Savings: ~2103 
 <details>  
 <summary>  
  </summary> 
 

```solidity

contract GasTest is DSTest {
    Contract0 c0;
    Contract1 c1;
    Contract2  c2;

    function setUp() public {
        c0 = new Contract0();
        c1 = new Contract1();
        c2 = new Contract2();
        
    }

    function testGas() public view {
        c0.addValue();
        c1.addImmutableValue();
        c2.addConstantValue();
    }
}

contract Contract0 {
    uint256 val;

    constructor() {
        val = 10000;
    }

    function addValue() public view {
        uint256 newVal = val + 1000;
    }
}

contract Contract1 {
    uint256 immutable val;

    constructor() {
        val = 10000;
    }

    function addImmutableValue() public view {
        uint256 newVal = val + 1000;
    }
    }

    contract Contract2 {
    uint256 constant val = 10;

    function addConstantValue() public view {
        uint256 newVal = val + 1000;
    }
}

```

```solidity
╭────────────────────┬─────────────────┬──────┬────────┬──────┬─────────╮
│ Contract0 contract ┆                 ┆      ┆        ┆      ┆         │
╞════════════════════╪═════════════════╪══════╪════════╪══════╪═════════╡
│ Deployment Cost    ┆ Deployment Size ┆      ┆        ┆      ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 54593              ┆ 198             ┆      ┆        ┆      ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name      ┆ min             ┆ avg  ┆ median ┆ max  ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ addValue           ┆ 2302            ┆ 2302 ┆ 2302   ┆ 2302 ┆ 1       │
╰────────────────────┴─────────────────┴──────┴────────┴──────┴─────────╯
╭────────────────────┬─────────────────┬─────┬────────┬─────┬─────────╮
│ Contract1 contract ┆                 ┆     ┆        ┆     ┆         │
╞════════════════════╪═════════════════╪═════╪════════╪═════╪═════════╡
│ Deployment Cost    ┆ Deployment Size ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 38514              ┆ 239             ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name      ┆ min             ┆ avg ┆ median ┆ max ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ addImmutableValue  ┆ 199             ┆ 199 ┆ 199    ┆ 199 ┆ 1       │
╰────────────────────┴─────────────────┴─────┴────────┴─────┴─────────╯
╭────────────────────┬─────────────────┬─────┬────────┬─────┬─────────╮
│ Contract2 contract ┆                 ┆     ┆        ┆     ┆         │
╞════════════════════╪═════════════════╪═════╪════════╪═════╪═════════╡
│ Deployment Cost    ┆ Deployment Size ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 32287              ┆ 191             ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name      ┆ min             ┆ avg ┆ median ┆ max ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ addConstantValue   ┆ 199             ┆ 199 ┆ 199    ┆ 199 ┆ 1       │
╰────────────────────┴─────────────────┴─────┴────────┴─────┴─────────╯
```

 
 </details> 
 

 --- 

File:Voter.sol#L19
```solidity
18:    address public convenience;
``` 



File:SwapPair.sol#L20
```solidity
19:    bool public immutable stable;
``` 



File:SwapPair.sol#L38
```solidity
37:    uint public immutable fee;
``` 



File:SwapPair.sol#L35
```solidity
34:    address public immutable token1;
``` 



File:SwapPair.sol#L34
```solidity
33:    address public immutable token0;
``` 



File:SwapFactory.sol#L22
```solidity
21:    uint internal _temp3;
``` 



File:SwapFactory.sol#L20
```solidity
19:    address internal _temp1;
``` 



File:SwapFactory.sol#L19
```solidity
18:    address internal _temp0;
``` 



File:SwapFactory.sol#L21
```solidity
20:    bool internal _temp2;
``` 



 --- 

<a name=[Gas-2]></a>
### [Gas-2] Mark storage variables as `immutable` if they never change after contract initialization. - Instances: 3 

 > 
 State variables can be declared as constant or immutable. In both cases, the variables cannot be modified after the contract has been constructed. For constant variables, the value has to be fixed at compile-time, while for immutable, it can still be assigned at construction time. 
 The compiler does not reserve a storage slot for these variables, and every occurrence is inlined by the respective value. 
 Compared to regular state variables, the gas costs of constant and immutable variables are much lower. For a constant variable, the expression assigned to it is copied to all the places where it is accessed and also re-evaluated each time. This allows for local optimizations. Immutable variables are evaluated once at construction time and their value is copied to all the places in the code where they are accessed. For these values, 32 bytes are reserved, even if they would fit in fewer bytes. Due to this, constant values can sometimes be cheaper than immutable values. 
 
 
#### Gas Report - Savings: ~2103 
 <details>  
 <summary>  
  </summary> 
 

```solidity

contract GasTest is DSTest {
    Contract0 c0;
    Contract1 c1;
    Contract2  c2;
    
    function setUp() public {
        c0 = new Contract0();
        c1 = new Contract1();
        c2 = new Contract2();
        
    }

    function testGas() public view {
        c0.addValue();
        c1.addImmutableValue();
        c2.addConstantValue();
    }
}

contract Contract0 {
    uint256 val;

    constructor() {
        val = 10000;
    }

    function addValue() public view {
        uint256 newVal = val + 1000;
    }
}

contract Contract1 {
    uint256 immutable val;

    constructor() {
        val = 10000;
    }

    function addImmutableValue() public view {
        uint256 newVal = val + 1000;
    }
}

contract Contract2 {
    uint256 constant val = 10;

    function addConstantValue() public view {
        uint256 newVal = val + 1000;
    }
}

```

```solidity
╭────────────────────┬─────────────────┬──────┬────────┬──────┬─────────╮
│ Contract0 contract ┆                 ┆      ┆        ┆      ┆         │
╞════════════════════╪═════════════════╪══════╪════════╪══════╪═════════╡
│ Deployment Cost    ┆ Deployment Size ┆      ┆        ┆      ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 54593              ┆ 198             ┆      ┆        ┆      ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name      ┆ min             ┆ avg  ┆ median ┆ max  ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ addValue           ┆ 2302            ┆ 2302 ┆ 2302   ┆ 2302 ┆ 1       │
╰────────────────────┴─────────────────┴──────┴────────┴──────┴─────────╯
╭────────────────────┬─────────────────┬─────┬────────┬─────┬─────────╮
│ Contract1 contract ┆                 ┆     ┆        ┆     ┆         │
╞════════════════════╪═════════════════╪═════╪════════╪═════╪═════════╡
│ Deployment Cost    ┆ Deployment Size ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 38514              ┆ 239             ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name      ┆ min             ┆ avg ┆ median ┆ max ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ addImmutableValue  ┆ 199             ┆ 199 ┆ 199    ┆ 199 ┆ 1       │
╰────────────────────┴─────────────────┴─────┴────────┴─────┴─────────╯
╭────────────────────┬─────────────────┬─────┬────────┬─────┬─────────╮
│ Contract2 contract ┆                 ┆     ┆        ┆     ┆         │
╞════════════════════╪═════════════════╪═════╪════════╪═════╪═════════╡
│ Deployment Cost    ┆ Deployment Size ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 32287              ┆ 191             ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name      ┆ min             ┆ avg ┆ median ┆ max ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ addConstantValue   ┆ 199             ┆ 199 ┆ 199    ┆ 199 ┆ 1       │
╰────────────────────┴─────────────────┴─────┴────────┴─────┴─────────╯
```

         
 </details> 
 

 --- 

File:Minter.sol#L30
```solidity
29:    address internal admin;
``` 



File:SwapPair.sol#L16
```solidity
15:    string public symbol;
``` 



File:SwapPair.sol#L15
```solidity
14:    string public name;
``` 



File:Voter.sol#L21
```solidity
20:    address public gaugeFactory;
``` 



 --- 

<a name=[Gas-3]></a>
### [Gas-3] `unchecked{++i}` instead of `i++` (or use assembly when applicable) - Instances: 5 

 > 
 Use `++i` instead of `i++`. This is especially useful in for loops but this optimization can be used anywhere in your code. You can also use `unchecked{++i;}` for even more gas savings but this will not check to see if `i` overflows. For extra safety if you are worried about this, you can add a require statement after the loop checking if `i` is equal to the final incremented value. For best gas savings, use inline assembly, however this limits the functionality you can achieve. For example you cant use Solidity syntax to internally call your own contract within an assembly block and external calls must be done with the `call()` or `delegatecall()` instruction. However when applicable, inline assembly will save much more gas. 
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

File:Multiswap.sol#L58
```solidity
57:            for (uint i = 0; i < length; ++i) {
``` 



File:Multiswap.sol#L45
```solidity
44:            for (uint i = 0; i < length; ++i) { 
``` 



File:Multiswap.sol#L75
```solidity
74:        for (uint i = 0; i < length; ++i) {
``` 



File:Bribe.sol#L235
```solidity
234:        for (uint i = 0; i < tokens.length; i++) {
``` 



File:Bribe.sol#L373
```solidity
372:            for (uint i = _startIndex; i <= _endIndex - 1; i++) {
``` 



File:Bribe.sol#L289
```solidity
288:        for (uint i = _startIndex; i < _endIndex; i++) {
``` 



File:Bribe.sol#L314
```solidity
313:      for (uint i; i < length; i++) {
``` 



File:Bribe.sol#L336
```solidity
335:            for (uint i = _startIndex; i <= _endIndex - 1; i++) {
``` 



File:Bribe.sol#L251
```solidity
250:        for (uint i = 0; i < tokens.length; i++) {
``` 



File:Voter.sol#L409
```solidity
408:        for (uint x = start; x < finish; x++) {
``` 



File:Voter.sol#L415
```solidity
414:        for (uint x = 0; x < _gauges.length; x++) {
``` 



File:Voter.sol#L309
```solidity
308:        for (uint i = 0; i < _gauges.length; i++) {
``` 



File:Voter.sol#L363
```solidity
362:        for (uint i = 0; i < _bribes.length; i++) {
``` 



File:Voter.sol#L382
```solidity
381:        for (uint i = 0; i < _gauges.length; i++) {
``` 



File:Voter.sol#L150
```solidity
149:        for (uint i = 0; i < _gaugeVoteCnt; i++) {
``` 



File:Voter.sol#L374
```solidity
373:        for (uint i = 0; i < _fees.length; i++) {
``` 



File:Voter.sol#L352
```solidity
351:        for (uint i = 0; i < _gauges.length; i++) {
``` 



File:Voter.sol#L194
```solidity
193:        for (uint i = 0; i < _gaugeCnt; i++) {
``` 



File:Voter.sol#L179
```solidity
178:        for (uint i = 0; i < _gaugeCnt; i++) {
``` 



File:Voter.sol#L198
```solidity
197:        for (uint i = 0; i < _gaugeCnt; i++) {
``` 



File:Voter.sol#L315
```solidity
314:        for (uint i = start; i < end; i++) {
``` 



File:Gauge.sol#L400
```solidity
399:            for (uint i = _startIndex; i <= _endIndex - 1; i++) {
``` 



File:Gauge.sol#L437
```solidity
436:            for (uint i = _startIndex; i <= _endIndex-1; i++) {
``` 



File:Gauge.sol#L299
```solidity
298:        for (uint i = 0; i < tokens.length; i++) {
``` 



File:Gauge.sol#L378
```solidity
377:        for (uint i; i < length; i++) {
``` 



File:Gauge.sol#L351
```solidity
350:        for (uint i = _startIndex; i < _endIndex; i++) {
``` 



File:SwapPair.sol#L268
```solidity
267:        for (uint i = 0; i < _prices.length; i++) {
``` 



File:SwapPair.sol#L400
```solidity
399:        for (uint i = 0; i < 255; i++) {
``` 



File:SwapPair.sol#L497
```solidity
496:                keccak256(abi.encode(PERMIT_TYPEHASH, owner, spender, value, nonces[owner]++, deadline))
``` 



 --- 

<a name=[Gas-4]></a>
### [Gas-4] Cache Storage Variables in Memory - Instances: 6 

 > 
  
 Cache Array Length - Gas Report - Savings: ~0 
 <details>  
 <summary>  
  </summary> 
  
 </details> 
 

 --- 

File:SwapPair.sol#L125
```solidity
124:        return observations[observations.length-1];
``` 



File:SwapPair.sol#L144
```solidity
143:            claimable0[msg.sender] = 0;
``` 



File:SwapPair.sol#L145
```solidity
144:            claimable1[msg.sender] = 0;
``` 



File:SwapPair.sol#L190
```solidity
189:            supplyIndex0[recipient] = _index0; // update user current position to global position
``` 



File:SwapPair.sol#L191
```solidity
190:            supplyIndex1[recipient] = _index1;
``` 



File:SwapPair.sol#L203
```solidity
202:            supplyIndex0[recipient] = index0; // new users are set to the default global state
``` 



File:SwapPair.sol#L203
```solidity
202:            supplyIndex0[recipient] = index0; // new users are set to the default global state
``` 



File:SwapPair.sol#L204
```solidity
203:            supplyIndex1[recipient] = index1;
``` 



File:SwapPair.sol#L204
```solidity
203:            supplyIndex1[recipient] = index1;
``` 



File:SwapPair.sol#L226
```solidity
225:            observations.push(Observation(blockTimestamp, reserve0CumulativeLast, reserve1CumulativeLast));
``` 



File:SwapPair.sol#L226
```solidity
225:            observations.push(Observation(blockTimestamp, reserve0CumulativeLast, reserve1CumulativeLast));
``` 



File:SwapPair.sol#L230
```solidity
229:        blockTimestampLast = blockTimestamp;
``` 



File:SwapPair.sol#L231
```solidity
230:        emit Sync(reserve0, reserve1);
``` 



File:SwapPair.sol#L231
```solidity
230:        emit Sync(reserve0, reserve1);
``` 



File:SwapPair.sol#L255
```solidity
254:            _observation = observations[observations.length-2];
``` 



File:SwapPair.sol#L289
```solidity
288:            uint timeElapsed = observations[nextIndex].timestamp - observations[i].timestamp;
``` 



File:SwapPair.sol#L289
```solidity
288:            uint timeElapsed = observations[nextIndex].timestamp - observations[i].timestamp;
``` 



File:SwapPair.sol#L290
```solidity
289:            uint _reserve0 = (observations[nextIndex].reserve0Cumulative - observations[i].reserve0Cumulative) / timeElapsed;
``` 



File:SwapPair.sol#L290
```solidity
289:            uint _reserve0 = (observations[nextIndex].reserve0Cumulative - observations[i].reserve0Cumulative) / timeElapsed;
``` 



File:SwapPair.sol#L291
```solidity
290:            uint _reserve1 = (observations[nextIndex].reserve1Cumulative - observations[i].reserve1Cumulative) / timeElapsed;
``` 



File:SwapPair.sol#L291
```solidity
290:            uint _reserve1 = (observations[nextIndex].reserve1Cumulative - observations[i].reserve1Cumulative) / timeElapsed;
``` 



File:SwapPair.sol#L496
```solidity
495:                DOMAIN_SEPARATOR,
``` 



File:SwapPair.sol#L518
```solidity
517:            allowance[src][spender] = newAllowance;
``` 



File:SwapPair.sol#L532
```solidity
531:        balanceOf[dst] += amount;
``` 



File:SwapFactory.sol#L54
```solidity
53:        pauser = pendingPauser;
``` 



File:SwapFactory.sol#L69
```solidity
68:        admin = _admin;
``` 



File:SwapFactory.sol#L88
```solidity
87:        getPair[token0][token1][stable] = pair;
``` 



File:SwapFactory.sol#L89
```solidity
88:        getPair[token1][token0][stable] = pair; // populate mapping in the reverse direction
``` 



File:SwapFactory.sol#L92
```solidity
91:        emit PairCreated(token0, token1, stable, pair, allPairs.length);
``` 



File:Gauge.sol#L121
```solidity
120:                fees0 = 0;
``` 



File:Gauge.sol#L125
```solidity
124:                fees0 = _fees0;
``` 



File:Gauge.sol#L128
```solidity
127:                fees1 = 0;
``` 



File:Gauge.sol#L132
```solidity
131:                fees1 = _fees1;
``` 



File:Gauge.sol#L158
```solidity
157:        if (checkpoints[account][0].timestamp > timestamp) {
``` 



File:Gauge.sol#L166
```solidity
165:            Checkpoint memory cp = checkpoints[account][center];
``` 



File:Gauge.sol#L190
```solidity
189:        if (supplyCheckpoints[0].timestamp > timestamp) {
``` 



File:Gauge.sol#L198
```solidity
197:            SupplyCheckpoint memory cp = supplyCheckpoints[center];
``` 



File:Gauge.sol#L218
```solidity
217:            return (rewardPerTokenCheckpoints[token][nCheckpoints - 1].rewardPerToken, rewardPerTokenCheckpoints[token][nCheckpoints - 1].timestamp);
``` 



File:Gauge.sol#L218
```solidity
217:            return (rewardPerTokenCheckpoints[token][nCheckpoints - 1].rewardPerToken, rewardPerTokenCheckpoints[token][nCheckpoints - 1].timestamp);
``` 



File:Gauge.sol#L222
```solidity
221:        if (rewardPerTokenCheckpoints[token][0].timestamp > timestamp) {
``` 



File:Gauge.sol#L230
```solidity
229:            RewardPerTokenCheckpoint memory cp = rewardPerTokenCheckpoints[token][center];
``` 



File:Gauge.sol#L239
```solidity
238:        return (rewardPerTokenCheckpoints[token][lower].rewardPerToken, rewardPerTokenCheckpoints[token][lower].timestamp);
``` 



File:Gauge.sol#L239
```solidity
238:        return (rewardPerTokenCheckpoints[token][lower].rewardPerToken, rewardPerTokenCheckpoints[token][lower].timestamp);
``` 



File:Gauge.sol#L247
```solidity
246:            checkpoints[account][_nCheckPoints - 1].balanceOf = balance;
``` 



File:Gauge.sol#L249
```solidity
248:            checkpoints[account][_nCheckPoints] = Checkpoint(_timestamp, balance);
``` 



File:Gauge.sol#L250
```solidity
249:            numCheckpoints[account] = _nCheckPoints + 1;
``` 



File:Gauge.sol#L258
```solidity
257:            rewardPerTokenCheckpoints[token][_nCheckPoints - 1].rewardPerToken = reward;
``` 



File:Gauge.sol#L260
```solidity
259:            rewardPerTokenCheckpoints[token][_nCheckPoints] = RewardPerTokenCheckpoint(timestamp, reward);
``` 



File:Gauge.sol#L261
```solidity
260:            rewardPerTokenNumCheckpoints[token] = _nCheckPoints + 1;
``` 



File:Gauge.sol#L270
```solidity
269:            supplyCheckpoints[_nCheckPoints - 1].supply = derivedSupply;
``` 



File:Gauge.sol#L272
```solidity
271:            supplyCheckpoints[_nCheckPoints] = SupplyCheckpoint(_timestamp, derivedSupply);
``` 



File:Gauge.sol#L272
```solidity
271:            supplyCheckpoints[_nCheckPoints] = SupplyCheckpoint(_timestamp, derivedSupply);
``` 



File:Gauge.sol#L273
```solidity
272:            supplyNumCheckpoints = _nCheckPoints + 1;
``` 



File:Gauge.sol#L297
```solidity
296:        _unlocked = 2;
``` 



File:Gauge.sol#L304
```solidity
303:            userRewardPerTokenStored[tokens[i]][account] = rewardPerTokenStored[tokens[i]];
``` 



File:Gauge.sol#L313
```solidity
312:        derivedBalances[account] = _derivedBalance;
``` 



File:Gauge.sol#L314
```solidity
313:        derivedSupply += _derivedBalance;
``` 



File:Gauge.sol#L316
```solidity
315:        _writeCheckpoint(account, derivedBalances[account]);
``` 



File:Gauge.sol#L297
```solidity
296:        _unlocked = 2;
``` 



File:Gauge.sol#L304
```solidity
303:            userRewardPerTokenStored[tokens[i]][account] = rewardPerTokenStored[tokens[i]];
``` 



File:Gauge.sol#L313
```solidity
312:        derivedBalances[account] = _derivedBalance;
``` 



File:Gauge.sol#L314
```solidity
313:        derivedSupply += _derivedBalance;
``` 



File:Gauge.sol#L316
```solidity
315:        _writeCheckpoint(account, derivedBalances[account]);
``` 



File:Gauge.sol#L325
```solidity
324:        return rewardPerTokenStored[token] + ((lastTimeRewardApplicable(token) - Math.min(lastUpdateTime[token], periodFinish[token])) * rewardRate[token] * PRECISION / derivedSupply);
``` 



File:Gauge.sol#L325
```solidity
324:        return rewardPerTokenStored[token] + ((lastTimeRewardApplicable(token) - Math.min(lastUpdateTime[token], periodFinish[token])) * rewardRate[token] * PRECISION / derivedSupply);
``` 



File:Gauge.sol#L349
```solidity
348:        uint _endIndex = Math.min(supplyNumCheckpoints-1, maxRuns);
``` 



File:Gauge.sol#L354
```solidity
353:                SupplyCheckpoint memory sp1 = supplyCheckpoints[i+1];
``` 



File:Gauge.sol#L367
```solidity
366:        return (((Math.min(endTime, periodFinish[token]) - Math.min(Math.max(timestamp0, startTimestamp), periodFinish[token])) * rewardRate[token] * PRECISION / supply), endTime);
``` 



File:Gauge.sol#L379
```solidity
378:            address token = rewards[i];
``` 



File:Gauge.sol#L397
```solidity
396:        uint _endIndex = Math.min(supplyNumCheckpoints - 1, maxRuns);
``` 



File:Gauge.sol#L403
```solidity
402:                    SupplyCheckpoint memory sp1 = supplyCheckpoints[i+1];
``` 



File:Gauge.sol#L412
```solidity
411:        SupplyCheckpoint memory sp = supplyCheckpoints[_endIndex];
``` 



File:Gauge.sol#L432
```solidity
431:        uint _endIndex = numCheckpoints[account]-1;
``` 



File:Gauge.sol#L439
```solidity
438:                Checkpoint memory cp1 = checkpoints[account][i+1];
``` 



File:Gauge.sol#L446
```solidity
445:        Checkpoint memory cp = checkpoints[account][_endIndex];
``` 



File:Gauge.sol#L468
```solidity
467:                tokenIds[msg.sender] = tokenId;
``` 



File:Gauge.sol#L471
```solidity
470:            require(tokenIds[msg.sender] == tokenId);
``` 



File:Gauge.sol#L473
```solidity
472:            tokenId = tokenIds[msg.sender];
``` 



File:Gauge.sol#L479
```solidity
478:        derivedBalances[msg.sender] = _derivedBalance;
``` 



File:Gauge.sol#L480
```solidity
479:        derivedSupply += _derivedBalance;
``` 



File:Gauge.sol#L468
```solidity
467:                tokenIds[msg.sender] = tokenId;
``` 



File:Gauge.sol#L471
```solidity
470:            require(tokenIds[msg.sender] == tokenId);
``` 



File:Gauge.sol#L473
```solidity
472:            tokenId = tokenIds[msg.sender];
``` 



File:Gauge.sol#L479
```solidity
478:        derivedBalances[msg.sender] = _derivedBalance;
``` 



File:Gauge.sol#L480
```solidity
479:        derivedSupply += _derivedBalance;
``` 



File:Gauge.sol#L510
```solidity
509:            tokenIds[msg.sender] = 0;
``` 



File:Gauge.sol#L513
```solidity
512:            tokenId = tokenIds[msg.sender];
``` 



File:Gauge.sol#L519
```solidity
518:        derivedBalances[msg.sender] = _derivedBalance;
``` 



File:Gauge.sol#L520
```solidity
519:        derivedSupply += _derivedBalance;
``` 



File:Gauge.sol#L522
```solidity
521:        _writeCheckpoint(msg.sender, derivedBalances[msg.sender]);
``` 



File:Gauge.sol#L510
```solidity
509:            tokenIds[msg.sender] = 0;
``` 



File:Gauge.sol#L513
```solidity
512:            tokenId = tokenIds[msg.sender];
``` 



File:Gauge.sol#L519
```solidity
518:        derivedBalances[msg.sender] = _derivedBalance;
``` 



File:Gauge.sol#L520
```solidity
519:        derivedSupply += _derivedBalance;
``` 



File:Gauge.sol#L522
```solidity
521:        _writeCheckpoint(msg.sender, derivedBalances[msg.sender]);
``` 



File:Gauge.sol#L531
```solidity
530:        uint _remaining = periodFinish[token] - block.timestamp;
``` 



File:Gauge.sol#L541
```solidity
540:            isReward[token] = true; 
``` 



File:Gauge.sol#L542
```solidity
541:            rewards.push(token);
``` 



File:Gauge.sol#L550
```solidity
549:            rewardRate[token] = amount / DURATION;
``` 



File:Gauge.sol#L552
```solidity
551:            uint _remaining = periodFinish[token] - block.timestamp;
``` 



File:Gauge.sol#L553
```solidity
552:            uint _left = _remaining * rewardRate[token];
``` 



File:Gauge.sol#L556
```solidity
555:            rewardRate[token] = (amount + _left) / DURATION;
``` 



File:Gauge.sol#L558
```solidity
557:        require(rewardRate[token] > 0);
``` 



File:Gauge.sol#L560
```solidity
559:        require(rewardRate[token] <= balance / DURATION, "Provided reward too high");
``` 



File:Gauge.sol#L561
```solidity
560:        periodFinish[token] = block.timestamp + DURATION;
``` 



File:Gauge.sol#L541
```solidity
540:            isReward[token] = true; 
``` 



File:Gauge.sol#L542
```solidity
541:            rewards.push(token);
``` 



File:Gauge.sol#L550
```solidity
549:            rewardRate[token] = amount / DURATION;
``` 



File:Gauge.sol#L552
```solidity
551:            uint _remaining = periodFinish[token] - block.timestamp;
``` 



File:Gauge.sol#L553
```solidity
552:            uint _left = _remaining * rewardRate[token];
``` 



File:Gauge.sol#L556
```solidity
555:            rewardRate[token] = (amount + _left) / DURATION;
``` 



File:Gauge.sol#L558
```solidity
557:        require(rewardRate[token] > 0);
``` 



File:Gauge.sol#L560
```solidity
559:        require(rewardRate[token] <= balance / DURATION, "Provided reward too high");
``` 



File:Gauge.sol#L561
```solidity
560:        periodFinish[token] = block.timestamp + DURATION;
``` 



File:Minter.sol#L77
```solidity
76:        initializer = address(0);
``` 



File:Minter.sol#L87
```solidity
86:        last_epoch = block.timestamp + 26 weeks;
``` 



File:Minter.sol#L87
```solidity
86:        last_epoch = block.timestamp + 26 weeks;
``` 



File:Minter.sol#L126
```solidity
125:            active_period = _period;
``` 



File:Minter.sol#L129
```solidity
128:            uint _growth = calculate_growth(weekly);
``` 



File:Minter.sol#L130
```solidity
129:            uint _required = _growth + weekly;
``` 



File:Minter.sol#L141
```solidity
140:            _token.approve(address(_voter), weekly);
``` 



File:Minter.sol#L142
```solidity
141:            _voter.notifyRewardAmount(weekly);
``` 



File:Minter.sol#L144
```solidity
143:            emit Mint(msg.sender, weekly, circulating_supply(), circulating_emission());
``` 



File:Bribe.sol#L104
```solidity
103:        if (checkpoints[tokenId][0].timestamp > timestamp) {
``` 



File:Bribe.sol#L112
```solidity
111:            Checkpoint memory cp = checkpoints[tokenId][center];
``` 



File:Bribe.sol#L136
```solidity
135:        if (supplyCheckpoints[0].timestamp > timestamp) {
``` 



File:Bribe.sol#L144
```solidity
143:            SupplyCheckpoint memory cp = supplyCheckpoints[center];
``` 



File:Bribe.sol#L164
```solidity
163:            return (rewardPerTokenCheckpoints[token][nCheckpoints - 1].rewardPerToken, rewardPerTokenCheckpoints[token][nCheckpoints - 1].timestamp);
``` 



File:Bribe.sol#L164
```solidity
163:            return (rewardPerTokenCheckpoints[token][nCheckpoints - 1].rewardPerToken, rewardPerTokenCheckpoints[token][nCheckpoints - 1].timestamp);
``` 



File:Bribe.sol#L168
```solidity
167:        if (rewardPerTokenCheckpoints[token][0].timestamp > timestamp) {
``` 



File:Bribe.sol#L176
```solidity
175:            RewardPerTokenCheckpoint memory cp = rewardPerTokenCheckpoints[token][center];
``` 



File:Bribe.sol#L185
```solidity
184:        return (rewardPerTokenCheckpoints[token][lower].rewardPerToken, rewardPerTokenCheckpoints[token][lower].timestamp);
``` 



File:Bribe.sol#L185
```solidity
184:        return (rewardPerTokenCheckpoints[token][lower].rewardPerToken, rewardPerTokenCheckpoints[token][lower].timestamp);
``` 



File:Bribe.sol#L193
```solidity
192:            checkpoints[tokenId][_nCheckPoints - 1].balanceOf = balance;
``` 



File:Bribe.sol#L195
```solidity
194:            checkpoints[tokenId][_nCheckPoints] = Checkpoint(_timestamp, balance);
``` 



File:Bribe.sol#L196
```solidity
195:            numCheckpoints[tokenId] = _nCheckPoints + 1;
``` 



File:Bribe.sol#L204
```solidity
203:            rewardPerTokenCheckpoints[token][_nCheckPoints - 1].rewardPerToken = reward;
``` 



File:Bribe.sol#L206
```solidity
205:            rewardPerTokenCheckpoints[token][_nCheckPoints] = RewardPerTokenCheckpoint(timestamp, reward);
``` 



File:Bribe.sol#L207
```solidity
206:            rewardPerTokenNumCheckpoints[token] = _nCheckPoints + 1;
``` 



File:Bribe.sol#L216
```solidity
215:            supplyCheckpoints[_nCheckPoints - 1].supply = totalSupply;
``` 



File:Bribe.sol#L218
```solidity
217:            supplyCheckpoints[_nCheckPoints] = SupplyCheckpoint(_timestamp, totalSupply);
``` 



File:Bribe.sol#L218
```solidity
217:            supplyCheckpoints[_nCheckPoints] = SupplyCheckpoint(_timestamp, totalSupply);
``` 



File:Bribe.sol#L219
```solidity
218:            supplyNumCheckpoints = _nCheckPoints + 1;
``` 



File:Bribe.sol#L240
```solidity
239:            userRewardPerTokenStored[tokens[i]][tokenId] = rewardPerTokenStored[tokens[i]];
``` 



File:Bribe.sol#L240
```solidity
239:            userRewardPerTokenStored[tokens[i]][tokenId] = rewardPerTokenStored[tokens[i]];
``` 



File:Bribe.sol#L256
```solidity
255:            userRewardPerTokenStored[tokens[i]][tokenId] = rewardPerTokenStored[tokens[i]];
``` 



File:Bribe.sol#L256
```solidity
255:            userRewardPerTokenStored[tokens[i]][tokenId] = rewardPerTokenStored[tokens[i]];
``` 



File:Bribe.sol#L267
```solidity
266:        return rewardPerTokenStored[token] + ((lastTimeRewardApplicable(token) - Math.min(lastUpdateTime[token], periodFinish[token])) * rewardRate[token] * PRECISION / totalSupply);
``` 



File:Bribe.sol#L267
```solidity
266:        return rewardPerTokenStored[token] + ((lastTimeRewardApplicable(token) - Math.min(lastUpdateTime[token], periodFinish[token])) * rewardRate[token] * PRECISION / totalSupply);
``` 



File:Bribe.sol#L287
```solidity
286:        uint _endIndex = Math.min(supplyNumCheckpoints-1, maxRuns);
``` 



File:Bribe.sol#L292
```solidity
291:                SupplyCheckpoint memory sp1 = supplyCheckpoints[i+1];
``` 



File:Bribe.sol#L305
```solidity
304:        return (((Math.min(endTime, periodFinish[token]) - Math.min(Math.max(timestamp0, startTimestamp), periodFinish[token])) * rewardRate[token] * PRECISION / supply), endTime);
``` 



File:Bribe.sol#L315
```solidity
314:        address token = rewards[i];
``` 



File:Bribe.sol#L333
```solidity
332:        uint _endIndex = Math.min(supplyNumCheckpoints - 1, maxRuns);
``` 



File:Bribe.sol#L339
```solidity
338:                    SupplyCheckpoint memory sp1 = supplyCheckpoints[i+1];
``` 



File:Bribe.sol#L349
```solidity
348:            SupplyCheckpoint memory sp = supplyCheckpoints[_endIndex];
``` 



File:Bribe.sol#L368
```solidity
367:        uint _endIndex = numCheckpoints[tokenId]-1;
``` 



File:Bribe.sol#L375
```solidity
374:                Checkpoint memory cp1 = checkpoints[tokenId][i+1];
``` 



File:Bribe.sol#L382
```solidity
381:        Checkpoint memory cp = checkpoints[tokenId][_endIndex];
``` 



File:Bribe.sol#L397
```solidity
396:        _writeCheckpoint(tokenId, balanceOf[tokenId]);
``` 



File:Bribe.sol#L410
```solidity
409:        _writeCheckpoint(tokenId, balanceOf[tokenId]);
``` 



File:Bribe.sol#L418
```solidity
417:        uint _remaining = periodFinish[token] - block.timestamp;
``` 



File:Bribe.sol#L428
```solidity
427:            isReward[token] = true; 
``` 



File:Bribe.sol#L429
```solidity
428:            rewards.push(token);
``` 



File:Bribe.sol#L437
```solidity
436:            rewardRate[token] = amount / DURATION;
``` 



File:Bribe.sol#L439
```solidity
438:            uint _remaining = periodFinish[token] - block.timestamp;
``` 



File:Bribe.sol#L440
```solidity
439:            uint _left = _remaining * rewardRate[token];
``` 



File:Bribe.sol#L443
```solidity
442:            rewardRate[token] = (amount + _left) / DURATION;
``` 



File:Bribe.sol#L445
```solidity
444:        require(rewardRate[token] > 0);
``` 



File:Bribe.sol#L447
```solidity
446:        require(rewardRate[token] <= balance / DURATION, "Provided reward too high");
``` 



File:Bribe.sol#L448
```solidity
447:        periodFinish[token] = block.timestamp + DURATION;
``` 



File:Bribe.sol#L428
```solidity
427:            isReward[token] = true; 
``` 



File:Bribe.sol#L429
```solidity
428:            rewards.push(token);
``` 



File:Bribe.sol#L437
```solidity
436:            rewardRate[token] = amount / DURATION;
``` 



File:Bribe.sol#L439
```solidity
438:            uint _remaining = periodFinish[token] - block.timestamp;
``` 



File:Bribe.sol#L440
```solidity
439:            uint _left = _remaining * rewardRate[token];
``` 



File:Bribe.sol#L443
```solidity
442:            rewardRate[token] = (amount + _left) / DURATION;
``` 



File:Bribe.sol#L445
```solidity
444:        require(rewardRate[token] > 0);
``` 



File:Bribe.sol#L447
```solidity
446:        require(rewardRate[token] <= balance / DURATION, "Provided reward too high");
``` 



File:Bribe.sol#L448
```solidity
447:        periodFinish[token] = block.timestamp + DURATION;
``` 



File:Voter.sol#L102
```solidity
101:        minter = _minter;
``` 



File:Voter.sol#L126
```solidity
125:        isLive[_gauge] = false;
``` 



File:Voter.sol#L126
```solidity
125:        isLive[_gauge] = false;
``` 



File:Voter.sol#L133
```solidity
132:        isLive[_gauge] = true;
``` 



File:Voter.sol#L133
```solidity
132:        isLive[_gauge] = true;
``` 



File:Voter.sol#L141
```solidity
140:        lastVote[_tokenId] = block.timestamp;
``` 



File:Voter.sol#L156
```solidity
155:                        votes[_tokenId][_gauge] -= _votes;
``` 



File:Voter.sol#L209
```solidity
208:                votes[_tokenId][_gauge] += _gaugeWeight;
``` 



File:Voter.sol#L231
```solidity
230:        lastVote[tokenId] = block.timestamp;
``` 



File:Voter.sol#L241
```solidity
240:        isWhitelisted[_token] = true;
``` 



File:Voter.sol#L251
```solidity
250:        require(isWhitelisted[_tokenA] && isWhitelisted[_tokenB], "!whitelisted");
``` 



File:Voter.sol#L256
```solidity
255:        gauges[_pair] = _gauge;
``` 



File:Voter.sol#L261
```solidity
260:        isReward[_gauge][_tokenB] = true;
``` 



File:Voter.sol#L262
```solidity
261:        isReward[_gauge][base] = true;
``` 



File:Voter.sol#L264
```solidity
263:        isBribe[_bribe][_tokenB] = true;
``` 



File:Voter.sol#L335
```solidity
334:            supplyIndex[_gauge] = _index; // update _gauge current position to global position
``` 



File:Voter.sol#L344
```solidity
343:            supplyIndex[_gauge] = index; // new users are set to the default global state
``` 



File:Voter.sol#L344
```solidity
343:            supplyIndex[_gauge] = index; // new users are set to the default global state
``` 



File:Voter.sol#L394
```solidity
393:            claimable[_gauge] = 0;
``` 



File:Voter.sol#L394
```solidity
393:            claimable[_gauge] = 0;
``` 



 --- 

<a name=[Gas-5]></a>
### [Gas-5] Use `calldata` instead of `memory` for function arguments that do not get mutated. - Instances: 4 

 > 
 Mark data types as `calldata` instead of `memory` where possible. This makes it so that the data is not automatically loaded into memory. If the data passed into the function does not need to be changed (like updating values in an array), it can be passed in as `calldata`. The one exception to this is if the argument must later be passed into another function that takes an argument that specifies `memory` storage. 
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

File:Voter.sol#L186
```solidity
185:    function _vote(uint _tokenId, address[] memory _gaugeVote, uint256[] memory _weights) internal {
``` 



File:Voter.sol#L186
```solidity
185:    function _vote(uint _tokenId, address[] memory _gaugeVote, uint256[] memory _weights) internal {
``` 



File:Voter.sol#L308
```solidity
307:    function updateFor(address[] memory _gauges) external {
``` 



File:Voter.sol#L351
```solidity
350:    function claimRewards(address[] memory _gauges, address[][] memory _tokens) external {
``` 



File:Voter.sol#L351
```solidity
350:    function claimRewards(address[] memory _gauges, address[][] memory _tokens) external {
``` 



File:Voter.sol#L361
```solidity
360:    function claimBribes(address[] memory _bribes, address[][] memory _tokens, uint _tokenId) external {
``` 



File:Voter.sol#L361
```solidity
360:    function claimBribes(address[] memory _bribes, address[][] memory _tokens, uint _tokenId) external {
``` 



File:Voter.sol#L372
```solidity
371:    function claimFees(address[] memory _fees, address[][] memory _tokens, uint _tokenId) external {
``` 



File:Voter.sol#L372
```solidity
371:    function claimFees(address[] memory _fees, address[][] memory _tokens, uint _tokenId) external {
``` 



File:Voter.sol#L381
```solidity
380:    function distributeFees(address[] memory _gauges) external {
``` 



File:Voter.sol#L414
```solidity
413:    function distribute(address[] memory _gauges) external {
``` 



File:Bribe.sol#L233
```solidity
232:    function getReward(uint tokenId, address[] memory tokens) external lock  {
``` 



File:Bribe.sol#L248
```solidity
247:    function getRewardForOwner(uint tokenId, address[] memory tokens) external lock  {
``` 



File:Multiswap.sol#L32
```solidity
31:        bytes[] memory _swapData,
``` 



File:Gauge.sol#L293
```solidity
292:    function getReward(address account, address[] memory tokens) external lock {
``` 



 --- 

<a name=[Gas-6]></a>
### [Gas-6] Use assembly to hash instead of Solidity - Instances: 2 

 > 
 Hashing is a safe operation to perform in assembly, and it is cheaper than Solidity's `keccak256` function. 
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

File:SwapPair.sol#L484
```solidity
483:        DOMAIN_SEPARATOR = keccak256(
484:            abi.encode(
485:                keccak256('EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)'),
486:                keccak256(bytes(name)),
487:                keccak256('1'),
488:                block.chainid,
489:                address(this)
490:            )
491:        );
``` 



File:SwapPair.sol#L486
```solidity
485:                keccak256('EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)'),
``` 



File:SwapPair.sol#L487
```solidity
486:                keccak256(bytes(name)),
``` 



File:SwapPair.sol#L488
```solidity
487:                keccak256('1'),
``` 



File:SwapPair.sol#L493
```solidity
492:        bytes32 digest = keccak256(
493:            abi.encodePacked(
494:                '\x19\x01',
495:                DOMAIN_SEPARATOR,
496:                keccak256(abi.encode(PERMIT_TYPEHASH, owner, spender, value, nonces[owner]++, deadline))
497:            )
498:        );
``` 



File:SwapPair.sol#L497
```solidity
496:                keccak256(abi.encode(PERMIT_TYPEHASH, owner, spender, value, nonces[owner]++, deadline))
497:            )
``` 



File:SwapFactory.sol#L73
```solidity
72:        return keccak256(type(SwapPair).creationCode);
``` 



File:SwapFactory.sol#L85
```solidity
84:        bytes32 salt = keccak256(abi.encodePacked(token0, token1, stable)); // notice salt includes stable as well, 3 parameters
``` 



 --- 

<a name=[Gas-7]></a>
### [Gas-7] Use custom errors instead of string error messages - Instances: 7 

 > 
 Using custom errors will save you gas, and can be used to provide more information about the error. 
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

File:Multiswap.sol#L12
```solidity
11:        require(
12:            _router != address(0),
13:            "Multiswap: zero address provided in constructor"
14:        );
``` 



File:Multiswap.sol#L37
```solidity
36:        require(length > 1 && length <= 5 && _weights.length == length, "length");
``` 



File:Multiswap.sol#L38
```solidity
37:        require(_assertWeights(_weights), "wrong weights");
``` 



File:Multiswap.sol#L44
```solidity
43:            require(msg.value > 0, "no ETH sent");
``` 



File:Multiswap.sol#L54
```solidity
53:            require(_amount > 0, "no tokens sent");
``` 



File:Multiswap.sol#L56
```solidity
55:            require(transferFromSuccess, "transferFrom failed");
``` 



File:SwapPair.sol#L314
```solidity
313:        require(liquidity > 0, 'ILM'); // SwapPair: INSUFFICIENT_LIQUIDITY_MINTED
``` 



File:SwapPair.sol#L333
```solidity
332:        require(amount0 > 0 && amount1 > 0, 'ILB'); // SwapPair: INSUFFICIENT_LIQUIDITY_BURNED
``` 



File:SwapPair.sol#L347
```solidity
346:        require(amount0Out > 0 || amount1Out > 0, 'IOA'); // SwapPair: INSUFFICIENT_OUTPUT_AMOUNT
``` 



File:SwapPair.sol#L349
```solidity
348:        require(amount0Out < _reserve0 && amount1Out < _reserve1, 'IL'); // SwapPair: INSUFFICIENT_LIQUIDITY
``` 



File:SwapPair.sol#L355
```solidity
354:        require(to != _token0 && to != _token1, 'IT'); // SwapPair: INVALID_TO
``` 



File:SwapPair.sol#L364
```solidity
363:        require(amount0In > 0 || amount1In > 0, 'IIA'); // SwapPair: INSUFFICIENT_INPUT_AMOUNT
``` 



File:SwapPair.sol#L372
```solidity
371:        require(_k(_balance0, _balance1) >= _k(_reserve0, _reserve1), 'K'); // SwapPair: K
``` 



File:SwapPair.sol#L478
```solidity
477:        require(deadline >= block.timestamp, 'SwapPair: EXPIRED');
``` 



File:SwapPair.sol#L479
```solidity
478:        require(v == 27 || v == 28, 'SwapPair: INVALID_SIGNATURE');
``` 



File:SwapPair.sol#L480
```solidity
479:        require(
480:            s < 0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF5D576E7357A4501DDFE92F46681B20A1,
481:            'SwapPair: INVALID_SIGNATURE'
482:        );
``` 



File:SwapPair.sol#L501
```solidity
500:        require(recoveredAddress != address(0) && recoveredAddress == owner, 'SwapPair: INVALID_SIGNATURE');
``` 



File:Minter.sol#L47
```solidity
46:        require(
47:            __voter != address(0) &&
48:            __ve != address(0) &&
49:            __ve_dist != address(0) &&
50:            _admin != address(0),
51:            "Minter: zero address provided in constructor"
52:        );
``` 



File:Minter.sol#L84
```solidity
83:        require(block.timestamp >= last_epoch + 26 weeks, "must wait next period");
``` 



File:Minter.sol#L136
```solidity
135:            require(_token.transfer(address(_ve_dist), _growth), "growth transfer failed");
``` 



File:Gauge.sol#L85
```solidity
84:        require(
85:            _stake != address(0) &&
86:            _bribe != address(0) &&
87:            __ve != address(0) &&
88:            _voter != address(0),
89:            "Gauge: zero address provided in constructor"
90:        );
``` 



File:Gauge.sol#L539
```solidity
538:            require(IVoter(voter).isReward(address(this), token), "rewards tokens must be whitelisted");
``` 



File:Gauge.sol#L540
```solidity
539:            require(rewards.length < MAX_REWARD_TOKENS, "too many rewards tokens");
``` 



File:Gauge.sol#L560
```solidity
559:        require(rewardRate[token] <= balance / DURATION, "Provided reward too high");
``` 



File:Bribe.sol#L426
```solidity
425:            require(IVoter(voter).isBribe(address(this), token), "rewards tokens must be whitelisted");
``` 



File:Bribe.sol#L427
```solidity
426:            require(rewards.length < MAX_REWARD_TOKENS, "too many rewards tokens");
``` 



File:Bribe.sol#L447
```solidity
446:        require(rewardRate[token] <= balance / DURATION, "Provided reward too high");
``` 



File:SwapFactory.sol#L27
```solidity
26:        require(msg.sender == admin, "Voter: only admin");
``` 



File:SwapFactory.sol#L32
```solidity
31:        require(
32:            _feeCollector != address(0),
33:            "SwapFactory: zero address provided in constructor"
34:        );
``` 



File:SwapFactory.sol#L63
```solidity
62:        require(msg.sender == admin, "SwapFactory: only admin");
``` 



File:SwapFactory.sol#L68
```solidity
67:        require(msg.sender == admin && _admin != address(0), "SwapFactory; wrong input parameters");
``` 



File:SwapFactory.sol#L81
```solidity
80:        require(tokenA != tokenB, 'IA'); // BaseV1: IDENTICAL_ADDRESSES
``` 



File:SwapFactory.sol#L83
```solidity
82:        require(token0 != address(0), 'ZA'); // BaseV1: ZERO_ADDRESS
``` 



File:SwapFactory.sol#L84
```solidity
83:        require(getPair[token0][token1][stable] == address(0), 'PE'); // BaseV1: PAIR_EXISTS - single check is sufficient
``` 



File:Voter.sol#L64
```solidity
63:        require(msg.sender == admin, "Voter: only admin");
``` 



File:Voter.sol#L69
```solidity
68:        require(
69:            __ve != address(0) &&
70:            _factory != address(0) &&
71:            _gauges != address(0) &&
72:            _bribes != address(0),
73:            "Voter: zero address provided in constructor"
74:        );
``` 



File:Voter.sol#L112
```solidity
111:        require(_admin != address(0), "zero address");
``` 



File:Voter.sol#L125
```solidity
124:        require(isLive[_gauge], "gauge is not live");
``` 



File:Voter.sol#L132
```solidity
131:        require(!isLive[_gauge], "gauge is live");
``` 



File:Voter.sol#L139
```solidity
138:        require(_activePeriod() > lastVote[_tokenId], "voted recently");
``` 



File:Voter.sol#L227
```solidity
226:        require(_activePeriod() > lastVote[tokenId], "already voted");
``` 



File:Voter.sol#L228
```solidity
227:        require(_nextPeriod() < _lockEnd, "lock expires soon");
``` 



File:Voter.sol#L248
```solidity
247:        require(ISwapFactory(factory).isPair(_pair), "!pair");
``` 



File:Voter.sol#L249
```solidity
248:        require(gauges[_pair] == address(0x0), "exists");
``` 



File:Voter.sol#L251
```solidity
250:        require(isWhitelisted[_tokenA] && isWhitelisted[_tokenB], "!whitelisted");
``` 



 --- 

<a name=[Gas-8]></a>
### [Gas-8] Use assembly for math (add, sub, mul, div) - Instances: 6 

 > 
 Use assembly for math instead of Solidity. You can check for overflow/underflow in assembly to ensure safety. If using Solidity versions < 0.8.0 and you are using Safemath, you can gain significant gas savings by using assembly to calculate values and checking for overflow/underflow. 
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

File:Bribe.sol#L99
```solidity
98:        if (checkpoints[tokenId][nCheckpoints - 1].timestamp <= timestamp) {
``` 



File:Bribe.sol#L100
```solidity
99:            return (nCheckpoints - 1);
``` 



File:Bribe.sol#L109
```solidity
108:        uint upper = nCheckpoints - 1;
``` 



File:Bribe.sol#L111
```solidity
110:            uint center = upper - (upper - lower) / 2; // ceil, avoiding overflow
``` 



File:Bribe.sol#L111
```solidity
110:            uint center = upper - (upper - lower) / 2; // ceil, avoiding overflow
``` 



File:Bribe.sol#L111
```solidity
110:            uint center = upper - (upper - lower) / 2; // ceil, avoiding overflow
``` 



File:Bribe.sol#L118
```solidity
117:                upper = center - 1;
``` 



File:Bribe.sol#L131
```solidity
130:        if (supplyCheckpoints[nCheckpoints - 1].timestamp <= timestamp) {
``` 



File:Bribe.sol#L132
```solidity
131:            return (nCheckpoints - 1);
``` 



File:Bribe.sol#L141
```solidity
140:        uint upper = nCheckpoints - 1;
``` 



File:Bribe.sol#L143
```solidity
142:            uint center = upper - (upper - lower) / 2; // ceil, avoiding overflow
``` 



File:Bribe.sol#L143
```solidity
142:            uint center = upper - (upper - lower) / 2; // ceil, avoiding overflow
``` 



File:Bribe.sol#L143
```solidity
142:            uint center = upper - (upper - lower) / 2; // ceil, avoiding overflow
``` 



File:Bribe.sol#L150
```solidity
149:                upper = center - 1;
``` 



File:Bribe.sol#L163
```solidity
162:        if (rewardPerTokenCheckpoints[token][nCheckpoints - 1].timestamp <= timestamp) {
``` 



File:Bribe.sol#L164
```solidity
163:            return (rewardPerTokenCheckpoints[token][nCheckpoints - 1].rewardPerToken, rewardPerTokenCheckpoints[token][nCheckpoints - 1].timestamp);
``` 



File:Bribe.sol#L164
```solidity
163:            return (rewardPerTokenCheckpoints[token][nCheckpoints - 1].rewardPerToken, rewardPerTokenCheckpoints[token][nCheckpoints - 1].timestamp);
``` 



File:Bribe.sol#L173
```solidity
172:        uint upper = nCheckpoints - 1;
``` 



File:Bribe.sol#L175
```solidity
174:            uint center = upper - (upper - lower) / 2; // ceil, avoiding overflow
``` 



File:Bribe.sol#L175
```solidity
174:            uint center = upper - (upper - lower) / 2; // ceil, avoiding overflow
``` 



File:Bribe.sol#L175
```solidity
174:            uint center = upper - (upper - lower) / 2; // ceil, avoiding overflow
``` 



File:Bribe.sol#L182
```solidity
181:                upper = center - 1;
``` 



File:Bribe.sol#L192
```solidity
191:        if (_nCheckPoints > 0 && checkpoints[tokenId][_nCheckPoints - 1].timestamp == _timestamp) {
``` 



File:Bribe.sol#L193
```solidity
192:            checkpoints[tokenId][_nCheckPoints - 1].balanceOf = balance;
``` 



File:Bribe.sol#L196
```solidity
195:            numCheckpoints[tokenId] = _nCheckPoints + 1;
``` 



File:Bribe.sol#L203
```solidity
202:        if (_nCheckPoints > 0 && rewardPerTokenCheckpoints[token][_nCheckPoints - 1].timestamp == timestamp) {
``` 



File:Bribe.sol#L204
```solidity
203:            rewardPerTokenCheckpoints[token][_nCheckPoints - 1].rewardPerToken = reward;
``` 



File:Bribe.sol#L207
```solidity
206:            rewardPerTokenNumCheckpoints[token] = _nCheckPoints + 1;
``` 



File:Bribe.sol#L215
```solidity
214:        if (_nCheckPoints > 0 && supplyCheckpoints[_nCheckPoints - 1].timestamp == _timestamp) {
``` 



File:Bribe.sol#L216
```solidity
215:            supplyCheckpoints[_nCheckPoints - 1].supply = totalSupply;
``` 



File:Bribe.sol#L219
```solidity
218:            supplyNumCheckpoints = _nCheckPoints + 1;
``` 



File:Bribe.sol#L267
```solidity
266:        return rewardPerTokenStored[token] + ((lastTimeRewardApplicable(token) - Math.min(lastUpdateTime[token], periodFinish[token])) * rewardRate[token] * PRECISION / totalSupply);
``` 



File:Bribe.sol#L267
```solidity
266:        return rewardPerTokenStored[token] + ((lastTimeRewardApplicable(token) - Math.min(lastUpdateTime[token], periodFinish[token])) * rewardRate[token] * PRECISION / totalSupply);
``` 



File:Bribe.sol#L267
```solidity
266:        return rewardPerTokenStored[token] + ((lastTimeRewardApplicable(token) - Math.min(lastUpdateTime[token], periodFinish[token])) * rewardRate[token] * PRECISION / totalSupply);
``` 



File:Bribe.sol#L267
```solidity
266:        return rewardPerTokenStored[token] + ((lastTimeRewardApplicable(token) - Math.min(lastUpdateTime[token], periodFinish[token])) * rewardRate[token] * PRECISION / totalSupply);
``` 



File:Bribe.sol#L267
```solidity
266:        return rewardPerTokenStored[token] + ((lastTimeRewardApplicable(token) - Math.min(lastUpdateTime[token], periodFinish[token])) * rewardRate[token] * PRECISION / totalSupply);
``` 



File:Bribe.sol#L287
```solidity
286:        uint _endIndex = Math.min(supplyNumCheckpoints-1, maxRuns);
``` 



File:Bribe.sol#L292
```solidity
291:                SupplyCheckpoint memory sp1 = supplyCheckpoints[i+1];
``` 



File:Bribe.sol#L305
```solidity
304:        return (((Math.min(endTime, periodFinish[token]) - Math.min(Math.max(timestamp0, startTimestamp), periodFinish[token])) * rewardRate[token] * PRECISION / supply), endTime);
``` 



File:Bribe.sol#L305
```solidity
304:        return (((Math.min(endTime, periodFinish[token]) - Math.min(Math.max(timestamp0, startTimestamp), periodFinish[token])) * rewardRate[token] * PRECISION / supply), endTime);
``` 



File:Bribe.sol#L305
```solidity
304:        return (((Math.min(endTime, periodFinish[token]) - Math.min(Math.max(timestamp0, startTimestamp), periodFinish[token])) * rewardRate[token] * PRECISION / supply), endTime);
``` 



File:Bribe.sol#L305
```solidity
304:        return (((Math.min(endTime, periodFinish[token]) - Math.min(Math.max(timestamp0, startTimestamp), periodFinish[token])) * rewardRate[token] * PRECISION / supply), endTime);
``` 



File:Bribe.sol#L333
```solidity
332:        uint _endIndex = Math.min(supplyNumCheckpoints - 1, maxRuns);
``` 



File:Bribe.sol#L336
```solidity
335:            for (uint i = _startIndex; i <= _endIndex - 1; i++) {
``` 



File:Bribe.sol#L339
```solidity
338:                    SupplyCheckpoint memory sp1 = supplyCheckpoints[i+1];
``` 



File:Bribe.sol#L368
```solidity
367:        uint _endIndex = numCheckpoints[tokenId]-1;
``` 



File:Bribe.sol#L373
```solidity
372:            for (uint i = _startIndex; i <= _endIndex - 1; i++) {
``` 



File:Bribe.sol#L375
```solidity
374:                Checkpoint memory cp1 = checkpoints[tokenId][i+1];
``` 



File:Bribe.sol#L378
```solidity
377:                reward += cp0.balanceOf * (_rewardPerTokenStored1 - _rewardPerTokenStored0) / PRECISION;
``` 



File:Bribe.sol#L378
```solidity
377:                reward += cp0.balanceOf * (_rewardPerTokenStored1 - _rewardPerTokenStored0) / PRECISION;
``` 



File:Bribe.sol#L378
```solidity
377:                reward += cp0.balanceOf * (_rewardPerTokenStored1 - _rewardPerTokenStored0) / PRECISION;
``` 



File:Bribe.sol#L384
```solidity
383:        reward += cp.balanceOf * (rewardPerToken(token) - Math.max(_rewardPerTokenStored, userRewardPerTokenStored[token][tokenId])) / PRECISION;
``` 



File:Bribe.sol#L384
```solidity
383:        reward += cp.balanceOf * (rewardPerToken(token) - Math.max(_rewardPerTokenStored, userRewardPerTokenStored[token][tokenId])) / PRECISION;
``` 



File:Bribe.sol#L384
```solidity
383:        reward += cp.balanceOf * (rewardPerToken(token) - Math.max(_rewardPerTokenStored, userRewardPerTokenStored[token][tokenId])) / PRECISION;
``` 



File:Bribe.sol#L418
```solidity
417:        uint _remaining = periodFinish[token] - block.timestamp;
``` 



File:Bribe.sol#L419
```solidity
418:        return _remaining * rewardRate[token];
``` 



File:Bribe.sol#L437
```solidity
436:            rewardRate[token] = amount / DURATION;
``` 



File:Bribe.sol#L439
```solidity
438:            uint _remaining = periodFinish[token] - block.timestamp;
``` 



File:Bribe.sol#L440
```solidity
439:            uint _left = _remaining * rewardRate[token];
``` 



File:Bribe.sol#L443
```solidity
442:            rewardRate[token] = (amount + _left) / DURATION;
``` 



File:Bribe.sol#L443
```solidity
442:            rewardRate[token] = (amount + _left) / DURATION;
``` 



File:Bribe.sol#L447
```solidity
446:        require(rewardRate[token] <= balance / DURATION, "Provided reward too high");
``` 



File:Bribe.sol#L448
```solidity
447:        periodFinish[token] = block.timestamp + DURATION;
``` 



File:Multiswap.sol#L46
```solidity
45:                uint amount_ = msg.value * _weights[i] / 10000;
``` 



File:Multiswap.sol#L46
```solidity
45:                uint amount_ = msg.value * _weights[i] / 10000;
``` 



File:Multiswap.sol#L50
```solidity
49:                amountsOut[i] = out[out.length - 1];
``` 



File:Multiswap.sol#L62
```solidity
61:                amountsOut[i] = out[out.length - 1];
``` 



File:Voter.sol#L108
```solidity
107:        return (IERC20(base).totalSupply() - IERC20(_ve).totalSupply()) / 10000;
``` 



File:Voter.sol#L108
```solidity
107:        return (IERC20(base).totalSupply() - IERC20(_ve).totalSupply()) / 10000;
``` 



File:Voter.sol#L201
```solidity
200:                uint256 _gaugeWeight = _weights[i] * _weight / _totalVoteWeight;
``` 



File:Voter.sol#L201
```solidity
200:                uint256 _gaugeWeight = _weights[i] * _weight / _totalVoteWeight;
``` 



File:Voter.sol#L301
```solidity
300:        uint256 _ratio = _amount * 1e18 / totalWeight; // 1e18 adjustment is removed during claim
``` 



File:Voter.sol#L301
```solidity
300:        uint256 _ratio = _amount * 1e18 / totalWeight; // 1e18 adjustment is removed during claim
``` 



File:Voter.sol#L336
```solidity
335:            uint _delta = _index - _supplyIndex; // see if there is any difference that need to be accrued
``` 



File:Voter.sol#L338
```solidity
337:                uint _share = uint(_supplied) * _delta / 1e18; // add accrued difference for each supplied token
``` 



File:Voter.sol#L338
```solidity
337:                uint _share = uint(_supplied) * _delta / 1e18; // add accrued difference for each supplied token
``` 



File:Voter.sol#L393
```solidity
392:        if (_claimable > IGauge(_gauge).left(base) && _claimable / DURATION > 0) {
``` 



File:Voter.sol#L423
```solidity
422:        activePeriod = block.timestamp / DURATION * DURATION;
``` 



File:Voter.sol#L423
```solidity
422:        activePeriod = block.timestamp / DURATION * DURATION;
``` 



File:Voter.sol#L429
```solidity
428:        nextPeriod = (block.timestamp + DURATION) / DURATION * DURATION;
``` 



File:Voter.sol#L429
```solidity
428:        nextPeriod = (block.timestamp + DURATION) / DURATION * DURATION;
``` 



File:Voter.sol#L429
```solidity
428:        nextPeriod = (block.timestamp + DURATION) / DURATION * DURATION;
``` 



File:Gauge.sol#L117
```solidity
116:            uint _fees0 = fees0 + claimed0;
``` 



File:Gauge.sol#L118
```solidity
117:            uint _fees1 = fees1 + claimed1;
``` 



File:Gauge.sol#L120
```solidity
119:            if (_fees0 > IBribe(bribe).left(_token0) && _fees0 / DURATION > 0) {
``` 



File:Gauge.sol#L127
```solidity
126:            if (_fees1 > IBribe(bribe).left(_token1) && _fees1 / DURATION > 0) {
``` 



File:Gauge.sol#L153
```solidity
152:        if (checkpoints[account][nCheckpoints - 1].timestamp <= timestamp) {
``` 



File:Gauge.sol#L154
```solidity
153:            return (nCheckpoints - 1);
``` 



File:Gauge.sol#L163
```solidity
162:        uint upper = nCheckpoints - 1;
``` 



File:Gauge.sol#L165
```solidity
164:            uint center = upper - (upper - lower) / 2; // ceil, avoiding overflow
``` 



File:Gauge.sol#L165
```solidity
164:            uint center = upper - (upper - lower) / 2; // ceil, avoiding overflow
``` 



File:Gauge.sol#L165
```solidity
164:            uint center = upper - (upper - lower) / 2; // ceil, avoiding overflow
``` 



File:Gauge.sol#L172
```solidity
171:                upper = center - 1;
``` 



File:Gauge.sol#L185
```solidity
184:        if (supplyCheckpoints[nCheckpoints - 1].timestamp <= timestamp) {
``` 



File:Gauge.sol#L186
```solidity
185:            return (nCheckpoints - 1);
``` 



File:Gauge.sol#L195
```solidity
194:        uint upper = nCheckpoints - 1;
``` 



File:Gauge.sol#L197
```solidity
196:            uint center = upper - (upper - lower) / 2; // ceil, avoiding overflow
``` 



File:Gauge.sol#L197
```solidity
196:            uint center = upper - (upper - lower) / 2; // ceil, avoiding overflow
``` 



File:Gauge.sol#L197
```solidity
196:            uint center = upper - (upper - lower) / 2; // ceil, avoiding overflow
``` 



File:Gauge.sol#L204
```solidity
203:                upper = center - 1;
``` 



File:Gauge.sol#L217
```solidity
216:        if (rewardPerTokenCheckpoints[token][nCheckpoints - 1].timestamp <= timestamp) {
``` 



File:Gauge.sol#L218
```solidity
217:            return (rewardPerTokenCheckpoints[token][nCheckpoints - 1].rewardPerToken, rewardPerTokenCheckpoints[token][nCheckpoints - 1].timestamp);
``` 



File:Gauge.sol#L218
```solidity
217:            return (rewardPerTokenCheckpoints[token][nCheckpoints - 1].rewardPerToken, rewardPerTokenCheckpoints[token][nCheckpoints - 1].timestamp);
``` 



File:Gauge.sol#L227
```solidity
226:        uint upper = nCheckpoints - 1;
``` 



File:Gauge.sol#L229
```solidity
228:            uint center = upper - (upper - lower) / 2; // ceil, avoiding overflow
``` 



File:Gauge.sol#L229
```solidity
228:            uint center = upper - (upper - lower) / 2; // ceil, avoiding overflow
``` 



File:Gauge.sol#L229
```solidity
228:            uint center = upper - (upper - lower) / 2; // ceil, avoiding overflow
``` 



File:Gauge.sol#L236
```solidity
235:                upper = center - 1;
``` 



File:Gauge.sol#L246
```solidity
245:        if (_nCheckPoints > 0 && checkpoints[account][_nCheckPoints - 1].timestamp == _timestamp) {
``` 



File:Gauge.sol#L247
```solidity
246:            checkpoints[account][_nCheckPoints - 1].balanceOf = balance;
``` 



File:Gauge.sol#L250
```solidity
249:            numCheckpoints[account] = _nCheckPoints + 1;
``` 



File:Gauge.sol#L257
```solidity
256:        if (_nCheckPoints > 0 && rewardPerTokenCheckpoints[token][_nCheckPoints - 1].timestamp == timestamp) {
``` 



File:Gauge.sol#L258
```solidity
257:            rewardPerTokenCheckpoints[token][_nCheckPoints - 1].rewardPerToken = reward;
``` 



File:Gauge.sol#L261
```solidity
260:            rewardPerTokenNumCheckpoints[token] = _nCheckPoints + 1;
``` 



File:Gauge.sol#L269
```solidity
268:        if (_nCheckPoints > 0 && supplyCheckpoints[_nCheckPoints - 1].timestamp == _timestamp) {
``` 



File:Gauge.sol#L270
```solidity
269:            supplyCheckpoints[_nCheckPoints - 1].supply = derivedSupply;
``` 



File:Gauge.sol#L273
```solidity
272:            supplyNumCheckpoints = _nCheckPoints + 1;
``` 



File:Gauge.sol#L325
```solidity
324:        return rewardPerTokenStored[token] + ((lastTimeRewardApplicable(token) - Math.min(lastUpdateTime[token], periodFinish[token])) * rewardRate[token] * PRECISION / derivedSupply);
``` 



File:Gauge.sol#L325
```solidity
324:        return rewardPerTokenStored[token] + ((lastTimeRewardApplicable(token) - Math.min(lastUpdateTime[token], periodFinish[token])) * rewardRate[token] * PRECISION / derivedSupply);
``` 



File:Gauge.sol#L325
```solidity
324:        return rewardPerTokenStored[token] + ((lastTimeRewardApplicable(token) - Math.min(lastUpdateTime[token], periodFinish[token])) * rewardRate[token] * PRECISION / derivedSupply);
``` 



File:Gauge.sol#L325
```solidity
324:        return rewardPerTokenStored[token] + ((lastTimeRewardApplicable(token) - Math.min(lastUpdateTime[token], periodFinish[token])) * rewardRate[token] * PRECISION / derivedSupply);
``` 



File:Gauge.sol#L325
```solidity
324:        return rewardPerTokenStored[token] + ((lastTimeRewardApplicable(token) - Math.min(lastUpdateTime[token], periodFinish[token])) * rewardRate[token] * PRECISION / derivedSupply);
``` 



File:Gauge.sol#L349
```solidity
348:        uint _endIndex = Math.min(supplyNumCheckpoints-1, maxRuns);
``` 



File:Gauge.sol#L354
```solidity
353:                SupplyCheckpoint memory sp1 = supplyCheckpoints[i+1];
``` 



File:Gauge.sol#L367
```solidity
366:        return (((Math.min(endTime, periodFinish[token]) - Math.min(Math.max(timestamp0, startTimestamp), periodFinish[token])) * rewardRate[token] * PRECISION / supply), endTime);
``` 



File:Gauge.sol#L367
```solidity
366:        return (((Math.min(endTime, periodFinish[token]) - Math.min(Math.max(timestamp0, startTimestamp), periodFinish[token])) * rewardRate[token] * PRECISION / supply), endTime);
``` 



File:Gauge.sol#L367
```solidity
366:        return (((Math.min(endTime, periodFinish[token]) - Math.min(Math.max(timestamp0, startTimestamp), periodFinish[token])) * rewardRate[token] * PRECISION / supply), endTime);
``` 



File:Gauge.sol#L367
```solidity
366:        return (((Math.min(endTime, periodFinish[token]) - Math.min(Math.max(timestamp0, startTimestamp), periodFinish[token])) * rewardRate[token] * PRECISION / supply), endTime);
``` 



File:Gauge.sol#L397
```solidity
396:        uint _endIndex = Math.min(supplyNumCheckpoints - 1, maxRuns);
``` 



File:Gauge.sol#L400
```solidity
399:            for (uint i = _startIndex; i <= _endIndex - 1; i++) {
``` 



File:Gauge.sol#L403
```solidity
402:                    SupplyCheckpoint memory sp1 = supplyCheckpoints[i+1];
``` 



File:Gauge.sol#L432
```solidity
431:        uint _endIndex = numCheckpoints[account]-1;
``` 



File:Gauge.sol#L437
```solidity
436:            for (uint i = _startIndex; i <= _endIndex-1; i++) {
``` 



File:Gauge.sol#L439
```solidity
438:                Checkpoint memory cp1 = checkpoints[account][i+1];
``` 



File:Gauge.sol#L442
```solidity
441:                reward += cp0.balanceOf * (_rewardPerTokenStored1 - _rewardPerTokenStored0) / PRECISION;
``` 



File:Gauge.sol#L442
```solidity
441:                reward += cp0.balanceOf * (_rewardPerTokenStored1 - _rewardPerTokenStored0) / PRECISION;
``` 



File:Gauge.sol#L442
```solidity
441:                reward += cp0.balanceOf * (_rewardPerTokenStored1 - _rewardPerTokenStored0) / PRECISION;
``` 



File:Gauge.sol#L448
```solidity
447:        reward += cp.balanceOf * (rewardPerToken(token) - Math.max(_rewardPerTokenStored, userRewardPerTokenStored[token][account])) / PRECISION;
``` 



File:Gauge.sol#L448
```solidity
447:        reward += cp.balanceOf * (rewardPerToken(token) - Math.max(_rewardPerTokenStored, userRewardPerTokenStored[token][account])) / PRECISION;
``` 



File:Gauge.sol#L448
```solidity
447:        reward += cp.balanceOf * (rewardPerToken(token) - Math.max(_rewardPerTokenStored, userRewardPerTokenStored[token][account])) / PRECISION;
``` 



File:Gauge.sol#L531
```solidity
530:        uint _remaining = periodFinish[token] - block.timestamp;
``` 



File:Gauge.sol#L532
```solidity
531:        return _remaining * rewardRate[token];
``` 



File:Gauge.sol#L550
```solidity
549:            rewardRate[token] = amount / DURATION;
``` 



File:Gauge.sol#L552
```solidity
551:            uint _remaining = periodFinish[token] - block.timestamp;
``` 



File:Gauge.sol#L553
```solidity
552:            uint _left = _remaining * rewardRate[token];
``` 



File:Gauge.sol#L556
```solidity
555:            rewardRate[token] = (amount + _left) / DURATION;
``` 



File:Gauge.sol#L556
```solidity
555:            rewardRate[token] = (amount + _left) / DURATION;
``` 



File:Gauge.sol#L560
```solidity
559:        require(rewardRate[token] <= balance / DURATION, "Provided reward too high");
``` 



File:Gauge.sol#L561
```solidity
560:        periodFinish[token] = block.timestamp + DURATION;
``` 



File:SwapPair.sol#L125
```solidity
124:        return observations[observations.length-1];
``` 



File:SwapPair.sol#L156
```solidity
155:        uint256 _protocolFee = amount / 10; // 10% of the amount
``` 



File:SwapPair.sol#L157
```solidity
156:        uint256 _feeIncrease = amount - _protocolFee; // Might leave tokens in this contract due to rounding but ok, reserves updated after this function
``` 



File:SwapPair.sol#L160
```solidity
159:        uint256 _ratio = _feeIncrease * 1e18 / totalSupply; // 1e18 adjustment is removed during claim
``` 



File:SwapPair.sol#L160
```solidity
159:        uint256 _ratio = _feeIncrease * 1e18 / totalSupply; // 1e18 adjustment is removed during claim
``` 



File:SwapPair.sol#L170
```solidity
169:        uint256 _protocolFee = amount / 10; // 10% of the amount
``` 



File:SwapPair.sol#L171
```solidity
170:        uint256 _feeIncrease = amount - _protocolFee; // Might leave tokens in this contract due to rounding but ok, reserves updated after this function
``` 



File:SwapPair.sol#L174
```solidity
173:        uint256 _ratio = _feeIncrease * 1e18 / totalSupply;
``` 



File:SwapPair.sol#L174
```solidity
173:        uint256 _ratio = _feeIncrease * 1e18 / totalSupply;
``` 



File:SwapPair.sol#L192
```solidity
191:            uint _delta0 = _index0 - _supplyIndex0; // see if there is any difference that need to be accrued
``` 



File:SwapPair.sol#L193
```solidity
192:            uint _delta1 = _index1 - _supplyIndex1;
``` 



File:SwapPair.sol#L195
```solidity
194:                uint _share = _supplied * _delta0 / 1e18; // add accrued difference for each supplied token
``` 



File:SwapPair.sol#L195
```solidity
194:                uint _share = _supplied * _delta0 / 1e18; // add accrued difference for each supplied token
``` 



File:SwapPair.sol#L199
```solidity
198:                uint _share = _supplied * _delta1 / 1e18;
``` 



File:SwapPair.sol#L199
```solidity
198:                uint _share = _supplied * _delta1 / 1e18;
``` 



File:SwapPair.sol#L217
```solidity
216:        uint timeElapsed = blockTimestamp - blockTimestampLast; // overflow is desired
``` 



File:SwapPair.sol#L219
```solidity
218:            reserve0CumulativeLast += _reserve0 * timeElapsed;
``` 



File:SwapPair.sol#L220
```solidity
219:            reserve1CumulativeLast += _reserve1 * timeElapsed;
``` 



File:SwapPair.sol#L224
```solidity
223:        timeElapsed = blockTimestamp - _point.timestamp; // compare the last observation with current timestamp, if greater than 30 minutes, record a new event
``` 



File:SwapPair.sol#L244
```solidity
243:            uint timeElapsed = blockTimestamp - _blockTimestampLast;
``` 



File:SwapPair.sol#L245
```solidity
244:            reserve0Cumulative += _reserve0 * timeElapsed;
``` 



File:SwapPair.sol#L246
```solidity
245:            reserve1Cumulative += _reserve1 * timeElapsed;
``` 



File:SwapPair.sol#L255
```solidity
254:            _observation = observations[observations.length-2];
``` 



File:SwapPair.sol#L258
```solidity
257:        uint timeElapsed = block.timestamp - _observation.timestamp;
``` 



File:SwapPair.sol#L259
```solidity
258:        uint _reserve0 = (reserve0Cumulative - _observation.reserve0Cumulative) / timeElapsed;
``` 



File:SwapPair.sol#L259
```solidity
258:        uint _reserve0 = (reserve0Cumulative - _observation.reserve0Cumulative) / timeElapsed;
``` 



File:SwapPair.sol#L260
```solidity
259:        uint _reserve1 = (reserve1Cumulative - _observation.reserve1Cumulative) / timeElapsed;
``` 



File:SwapPair.sol#L260
```solidity
259:        uint _reserve1 = (reserve1Cumulative - _observation.reserve1Cumulative) / timeElapsed;
``` 



File:SwapPair.sol#L271
```solidity
270:        return priceAverageCumulative / granularity;
``` 



File:SwapPair.sol#L282
```solidity
281:        uint length = observations.length-1;
``` 



File:SwapPair.sol#L283
```solidity
282:        uint i = length - (points * window);
``` 



File:SwapPair.sol#L283
```solidity
282:        uint i = length - (points * window);
``` 



File:SwapPair.sol#L288
```solidity
287:            nextIndex = i + window;
``` 



File:SwapPair.sol#L289
```solidity
288:            uint timeElapsed = observations[nextIndex].timestamp - observations[i].timestamp;
``` 



File:SwapPair.sol#L290
```solidity
289:            uint _reserve0 = (observations[nextIndex].reserve0Cumulative - observations[i].reserve0Cumulative) / timeElapsed;
``` 



File:SwapPair.sol#L290
```solidity
289:            uint _reserve0 = (observations[nextIndex].reserve0Cumulative - observations[i].reserve0Cumulative) / timeElapsed;
``` 



File:SwapPair.sol#L291
```solidity
290:            uint _reserve1 = (observations[nextIndex].reserve1Cumulative - observations[i].reserve1Cumulative) / timeElapsed;
``` 



File:SwapPair.sol#L291
```solidity
290:            uint _reserve1 = (observations[nextIndex].reserve1Cumulative - observations[i].reserve1Cumulative) / timeElapsed;
``` 



File:SwapPair.sol#L293
```solidity
292:            index = index + 1;
``` 



File:SwapPair.sol#L304
```solidity
303:        uint _amount0 = _balance0 - _reserve0;
``` 



File:SwapPair.sol#L305
```solidity
304:        uint _amount1 = _balance1 - _reserve1;
``` 



File:SwapPair.sol#L309
```solidity
308:            liquidity = Math.sqrt(_amount0 * _amount1) - MINIMUM_LIQUIDITY;
``` 



File:SwapPair.sol#L309
```solidity
308:            liquidity = Math.sqrt(_amount0 * _amount1) - MINIMUM_LIQUIDITY;
``` 



File:SwapPair.sol#L312
```solidity
311:            liquidity = Math.min(_amount0 * _totalSupply / _reserve0, _amount1 * _totalSupply / _reserve1);
``` 



File:SwapPair.sol#L312
```solidity
311:            liquidity = Math.min(_amount0 * _totalSupply / _reserve0, _amount1 * _totalSupply / _reserve1);
``` 



File:SwapPair.sol#L312
```solidity
311:            liquidity = Math.min(_amount0 * _totalSupply / _reserve0, _amount1 * _totalSupply / _reserve1);
``` 



File:SwapPair.sol#L312
```solidity
311:            liquidity = Math.min(_amount0 * _totalSupply / _reserve0, _amount1 * _totalSupply / _reserve1);
``` 



File:SwapPair.sol#L331
```solidity
330:        amount0 = _liquidity * _balance0 / _totalSupply; // using balances ensures pro-rata distribution
``` 



File:SwapPair.sol#L331
```solidity
330:        amount0 = _liquidity * _balance0 / _totalSupply; // using balances ensures pro-rata distribution
``` 



File:SwapPair.sol#L332
```solidity
331:        amount1 = _liquidity * _balance1 / _totalSupply; // using balances ensures pro-rata distribution
``` 



File:SwapPair.sol#L332
```solidity
331:        amount1 = _liquidity * _balance1 / _totalSupply; // using balances ensures pro-rata distribution
``` 



File:SwapPair.sol#L362
```solidity
361:        uint amount0In = _balance0 > _reserve0 - amount0Out ? _balance0 - (_reserve0 - amount0Out) : 0;
``` 



File:SwapPair.sol#L362
```solidity
361:        uint amount0In = _balance0 > _reserve0 - amount0Out ? _balance0 - (_reserve0 - amount0Out) : 0;
``` 



File:SwapPair.sol#L362
```solidity
361:        uint amount0In = _balance0 > _reserve0 - amount0Out ? _balance0 - (_reserve0 - amount0Out) : 0;
``` 



File:SwapPair.sol#L363
```solidity
362:        uint amount1In = _balance1 > _reserve1 - amount1Out ? _balance1 - (_reserve1 - amount1Out) : 0;
``` 



File:SwapPair.sol#L363
```solidity
362:        uint amount1In = _balance1 > _reserve1 - amount1Out ? _balance1 - (_reserve1 - amount1Out) : 0;
``` 



File:SwapPair.sol#L363
```solidity
362:        uint amount1In = _balance1 > _reserve1 - amount1Out ? _balance1 - (_reserve1 - amount1Out) : 0;
``` 



File:SwapPair.sol#L367
```solidity
366:        if (amount0In > 0) _update0(amount0In * fee / 1e6); // accrue fees for token0 and move them out of pool
``` 



File:SwapPair.sol#L367
```solidity
366:        if (amount0In > 0) _update0(amount0In * fee / 1e6); // accrue fees for token0 and move them out of pool
``` 



File:SwapPair.sol#L368
```solidity
367:        if (amount1In > 0) _update1(amount1In * fee / 1e6); // accrue fees for token1 and move them out of pool
``` 



File:SwapPair.sol#L368
```solidity
367:        if (amount1In > 0) _update1(amount1In * fee / 1e6); // accrue fees for token1 and move them out of pool
``` 



File:SwapPair.sol#L382
```solidity
381:        _safeTransfer(_token0, to, IERC20(_token0).balanceOf(address(this)) - (reserve0));
``` 



File:SwapPair.sol#L383
```solidity
382:        _safeTransfer(_token1, to, IERC20(_token1).balanceOf(address(this)) - (reserve1));
``` 



File:SwapPair.sol#L392
```solidity
391:        return x0*(y*y/1e18*y/1e18)/1e18+(x0*x0/1e18*x0/1e18)*y/1e18;
``` 



File:SwapPair.sol#L392
```solidity
391:        return x0*(y*y/1e18*y/1e18)/1e18+(x0*x0/1e18*x0/1e18)*y/1e18;
``` 



File:SwapPair.sol#L392
```solidity
391:        return x0*(y*y/1e18*y/1e18)/1e18+(x0*x0/1e18*x0/1e18)*y/1e18;
``` 



File:SwapPair.sol#L392
```solidity
391:        return x0*(y*y/1e18*y/1e18)/1e18+(x0*x0/1e18*x0/1e18)*y/1e18;
``` 



File:SwapPair.sol#L392
```solidity
391:        return x0*(y*y/1e18*y/1e18)/1e18+(x0*x0/1e18*x0/1e18)*y/1e18;
``` 



File:SwapPair.sol#L392
```solidity
391:        return x0*(y*y/1e18*y/1e18)/1e18+(x0*x0/1e18*x0/1e18)*y/1e18;
``` 



File:SwapPair.sol#L392
```solidity
391:        return x0*(y*y/1e18*y/1e18)/1e18+(x0*x0/1e18*x0/1e18)*y/1e18;
``` 



File:SwapPair.sol#L392
```solidity
391:        return x0*(y*y/1e18*y/1e18)/1e18+(x0*x0/1e18*x0/1e18)*y/1e18;
``` 



File:SwapPair.sol#L392
```solidity
391:        return x0*(y*y/1e18*y/1e18)/1e18+(x0*x0/1e18*x0/1e18)*y/1e18;
``` 



File:SwapPair.sol#L392
```solidity
391:        return x0*(y*y/1e18*y/1e18)/1e18+(x0*x0/1e18*x0/1e18)*y/1e18;
``` 



File:SwapPair.sol#L392
```solidity
391:        return x0*(y*y/1e18*y/1e18)/1e18+(x0*x0/1e18*x0/1e18)*y/1e18;
``` 



File:SwapPair.sol#L392
```solidity
391:        return x0*(y*y/1e18*y/1e18)/1e18+(x0*x0/1e18*x0/1e18)*y/1e18;
``` 



File:SwapPair.sol#L392
```solidity
391:        return x0*(y*y/1e18*y/1e18)/1e18+(x0*x0/1e18*x0/1e18)*y/1e18;
``` 



File:SwapPair.sol#L396
```solidity
395:        return 3*x0*(y*y/1e18)/1e18+(x0*x0/1e18*x0/1e18);
``` 



File:SwapPair.sol#L396
```solidity
395:        return 3*x0*(y*y/1e18)/1e18+(x0*x0/1e18*x0/1e18);
``` 



File:SwapPair.sol#L396
```solidity
395:        return 3*x0*(y*y/1e18)/1e18+(x0*x0/1e18*x0/1e18);
``` 



File:SwapPair.sol#L396
```solidity
395:        return 3*x0*(y*y/1e18)/1e18+(x0*x0/1e18*x0/1e18);
``` 



File:SwapPair.sol#L396
```solidity
395:        return 3*x0*(y*y/1e18)/1e18+(x0*x0/1e18*x0/1e18);
``` 



File:SwapPair.sol#L396
```solidity
395:        return 3*x0*(y*y/1e18)/1e18+(x0*x0/1e18*x0/1e18);
``` 



File:SwapPair.sol#L396
```solidity
395:        return 3*x0*(y*y/1e18)/1e18+(x0*x0/1e18*x0/1e18);
``` 



File:SwapPair.sol#L396
```solidity
395:        return 3*x0*(y*y/1e18)/1e18+(x0*x0/1e18*x0/1e18);
``` 



File:SwapPair.sol#L396
```solidity
395:        return 3*x0*(y*y/1e18)/1e18+(x0*x0/1e18*x0/1e18);
``` 



File:SwapPair.sol#L396
```solidity
395:        return 3*x0*(y*y/1e18)/1e18+(x0*x0/1e18*x0/1e18);
``` 



File:SwapPair.sol#L404
```solidity
403:                uint dy = (xy - k)*1e18/_d(x0, y);
``` 



File:SwapPair.sol#L404
```solidity
403:                uint dy = (xy - k)*1e18/_d(x0, y);
``` 



File:SwapPair.sol#L404
```solidity
403:                uint dy = (xy - k)*1e18/_d(x0, y);
``` 



File:SwapPair.sol#L405
```solidity
404:                y = y + dy;
``` 



File:SwapPair.sol#L407
```solidity
406:                uint dy = (k - xy)*1e18/_d(x0, y);
``` 



File:SwapPair.sol#L407
```solidity
406:                uint dy = (k - xy)*1e18/_d(x0, y);
``` 



File:SwapPair.sol#L407
```solidity
406:                uint dy = (k - xy)*1e18/_d(x0, y);
``` 



File:SwapPair.sol#L408
```solidity
407:                y = y - dy;
``` 



File:SwapPair.sol#L411
```solidity
410:                if (y - y_prev <= 1) {
``` 



File:SwapPair.sol#L415
```solidity
414:                if (y_prev - y <= 1) {
``` 



File:SwapPair.sol#L425
```solidity
424:        amountIn -= amountIn * fee / 1e6; // remove fee from amount received
``` 



File:SwapPair.sol#L425
```solidity
424:        amountIn -= amountIn * fee / 1e6; // remove fee from amount received
``` 



File:SwapPair.sol#L432
```solidity
431:            _reserve0 = _reserve0 * 1e18 / decimals0;
``` 



File:SwapPair.sol#L432
```solidity
431:            _reserve0 = _reserve0 * 1e18 / decimals0;
``` 



File:SwapPair.sol#L433
```solidity
432:            _reserve1 = _reserve1 * 1e18 / decimals1;
``` 



File:SwapPair.sol#L433
```solidity
432:            _reserve1 = _reserve1 * 1e18 / decimals1;
``` 



File:SwapPair.sol#L435
```solidity
434:            amountIn = tokenIn == token0 ? amountIn * 1e18 / decimals0 : amountIn * 1e18 / decimals1;
``` 



File:SwapPair.sol#L435
```solidity
434:            amountIn = tokenIn == token0 ? amountIn * 1e18 / decimals0 : amountIn * 1e18 / decimals1;
``` 



File:SwapPair.sol#L435
```solidity
434:            amountIn = tokenIn == token0 ? amountIn * 1e18 / decimals0 : amountIn * 1e18 / decimals1;
``` 



File:SwapPair.sol#L435
```solidity
434:            amountIn = tokenIn == token0 ? amountIn * 1e18 / decimals0 : amountIn * 1e18 / decimals1;
``` 



File:SwapPair.sol#L436
```solidity
435:            uint y = reserveB - _get_y(amountIn+reserveA, xy, reserveB);
``` 



File:SwapPair.sol#L436
```solidity
435:            uint y = reserveB - _get_y(amountIn+reserveA, xy, reserveB);
``` 



File:SwapPair.sol#L437
```solidity
436:            return y * (tokenIn == token0 ? decimals1 : decimals0) / 1e18;
``` 



File:SwapPair.sol#L437
```solidity
436:            return y * (tokenIn == token0 ? decimals1 : decimals0) / 1e18;
``` 



File:SwapPair.sol#L440
```solidity
439:            return amountIn * reserveB / (reserveA + amountIn);
``` 



File:SwapPair.sol#L440
```solidity
439:            return amountIn * reserveB / (reserveA + amountIn);
``` 



File:SwapPair.sol#L440
```solidity
439:            return amountIn * reserveB / (reserveA + amountIn);
``` 



File:SwapPair.sol#L446
```solidity
445:            uint _x = x * 1e18 / decimals0;
``` 



File:SwapPair.sol#L446
```solidity
445:            uint _x = x * 1e18 / decimals0;
``` 



File:SwapPair.sol#L447
```solidity
446:            uint _y = y * 1e18 / decimals1;
``` 



File:SwapPair.sol#L447
```solidity
446:            uint _y = y * 1e18 / decimals1;
``` 



File:SwapPair.sol#L448
```solidity
447:            uint _a = (_x * _y) / 1e18;
``` 



File:SwapPair.sol#L448
```solidity
447:            uint _a = (_x * _y) / 1e18;
``` 



File:SwapPair.sol#L449
```solidity
448:            uint _b = ((_x * _x) / 1e18 + (_y * _y) / 1e18);
``` 



File:SwapPair.sol#L449
```solidity
448:            uint _b = ((_x * _x) / 1e18 + (_y * _y) / 1e18);
``` 



File:SwapPair.sol#L449
```solidity
448:            uint _b = ((_x * _x) / 1e18 + (_y * _y) / 1e18);
``` 



File:SwapPair.sol#L449
```solidity
448:            uint _b = ((_x * _x) / 1e18 + (_y * _y) / 1e18);
``` 



File:SwapPair.sol#L449
```solidity
448:            uint _b = ((_x * _x) / 1e18 + (_y * _y) / 1e18);
``` 



File:SwapPair.sol#L450
```solidity
449:            return _a * _b / 1e18;  // x3y+y3x >= k
``` 



File:SwapPair.sol#L450
```solidity
449:            return _a * _b / 1e18;  // x3y+y3x >= k
``` 



File:SwapPair.sol#L452
```solidity
451:            return x * y; // xy >= k
``` 



File:SwapPair.sol#L517
```solidity
516:            uint newAllowance = spenderAllowance - amount;
``` 



File:Minter.sol#L14
```solidity
13:    uint internal constant week = 86400 * 7; // allows minting once per week (reset every Thursday 00:00 UTC)
``` 



File:Minter.sol#L27
```solidity
26:    uint internal constant lock = 86400 * 7 * 52 * 4;
``` 



File:Minter.sol#L27
```solidity
26:    uint internal constant lock = 86400 * 7 * 52 * 4;
``` 



File:Minter.sol#L27
```solidity
26:    uint internal constant lock = 86400 * 7 * 52 * 4;
``` 



File:Minter.sol#L59
```solidity
58:        active_period = (block.timestamp + (2*week)) / week * week;
``` 



File:Minter.sol#L59
```solidity
58:        active_period = (block.timestamp + (2*week)) / week * week;
``` 



File:Minter.sol#L59
```solidity
58:        active_period = (block.timestamp + (2*week)) / week * week;
``` 



File:Minter.sol#L59
```solidity
58:        active_period = (block.timestamp + (2*week)) / week * week;
``` 



File:Minter.sol#L78
```solidity
77:        active_period = block.timestamp / week * week; // 
``` 



File:Minter.sol#L78
```solidity
77:        active_period = block.timestamp / week * week; // 
``` 



File:Minter.sol#L84
```solidity
83:        require(block.timestamp >= last_epoch + 26 weeks, "must wait next period");
``` 



File:Minter.sol#L87
```solidity
86:        last_epoch = block.timestamp + 26 weeks;
``` 



File:Minter.sol#L92
```solidity
91:        return _token.totalSupply() - _ve.totalSupply();
``` 



File:Minter.sol#L97
```solidity
96:        return weekly * decay * circulating_supply() / target_base / _token.totalSupply();
``` 



File:Minter.sol#L97
```solidity
96:        return weekly * decay * circulating_supply() / target_base / _token.totalSupply();
``` 



File:Minter.sol#L97
```solidity
96:        return weekly * decay * circulating_supply() / target_base / _token.totalSupply();
``` 



File:Minter.sol#L97
```solidity
96:        return weekly * decay * circulating_supply() / target_base / _token.totalSupply();
``` 



File:Minter.sol#L103
```solidity
102:        return _emission + boost;
``` 



File:Minter.sol#L113
```solidity
112:        return circulating_supply() * tail_emission / tail_base;
``` 



File:Minter.sol#L113
```solidity
112:        return circulating_supply() * tail_emission / tail_base;
``` 



File:Minter.sol#L118
```solidity
117:        return _ve.totalSupply() * _minted / _token.totalSupply();
``` 



File:Minter.sol#L118
```solidity
117:        return _ve.totalSupply() * _minted / _token.totalSupply();
``` 



File:Minter.sol#L124
```solidity
123:        if (block.timestamp >= _period + week && initializer == address(0)) { // only trigger if new week
``` 



File:Minter.sol#L125
```solidity
124:            _period = block.timestamp / week * week;
``` 



File:Minter.sol#L125
```solidity
124:            _period = block.timestamp / week * week;
``` 



File:Minter.sol#L130
```solidity
129:            uint _required = _growth + weekly;
``` 



File:Minter.sol#L133
```solidity
132:                _token.mint(address(this), _required-_balanceOf);
``` 



 --- 

<a name=[Gas-9]></a>
### [Gas-9] Use assembly to write storage values - Instances: 6 

 > 
 You can save a fair amount of gas by using assembly to write storage values. 
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

File:SwapPair.sol#L98
```solidity
97:            name = string(abi.encodePacked("Stable Pair - ", cIERC20(_token0).symbol(), "/", cIERC20(_token1).symbol()));
``` 



File:SwapPair.sol#L99
```solidity
98:            symbol = string(abi.encodePacked("sAMM-", cIERC20(_token0).symbol(), "/", cIERC20(_token1).symbol()));
``` 



File:SwapPair.sol#L101
```solidity
100:            name = string(abi.encodePacked("Variable Pair - ", cIERC20(_token0).symbol(), "/", cIERC20(_token1).symbol()));
``` 



File:SwapPair.sol#L102
```solidity
101:            symbol = string(abi.encodePacked("vAMM-", cIERC20(_token0).symbol(), "/", cIERC20(_token1).symbol()));
``` 



File:SwapPair.sol#L115
```solidity
114:        _unlocked = 2;
``` 



File:SwapPair.sol#L117
```solidity
116:        _unlocked = 1;
``` 



File:SwapPair.sol#L228
```solidity
227:        reserve0 = balance0;
``` 



File:SwapPair.sol#L229
```solidity
228:        reserve1 = balance1;
``` 



File:SwapPair.sol#L230
```solidity
229:        blockTimestampLast = blockTimestamp;
``` 



File:SwapPair.sol#L484
```solidity
483:        DOMAIN_SEPARATOR = keccak256(
484:            abi.encode(
485:                keccak256('EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)'),
486:                keccak256(bytes(name)),
487:                keccak256('1'),
488:                block.chainid,
489:                address(this)
490:            )
491:        );
``` 



File:SwapFactory.sol#L36
```solidity
35:        pauser = msg.sender;
``` 



File:SwapFactory.sol#L40
```solidity
39:        admin = msg.sender;
``` 



File:SwapFactory.sol#L49
```solidity
48:        pendingPauser = _pauser;
``` 



File:SwapFactory.sol#L54
```solidity
53:        pauser = pendingPauser;
``` 



File:SwapFactory.sol#L59
```solidity
58:        isPaused = _state;
``` 



File:SwapFactory.sol#L69
```solidity
68:        admin = _admin;
``` 



File:Voter.sol#L79
```solidity
78:        gaugeFactory = _gauges;
``` 



File:Voter.sol#L81
```solidity
80:        minter = msg.sender;
``` 



File:Voter.sol#L82
```solidity
81:        admin = msg.sender;
``` 



File:Voter.sol#L89
```solidity
88:        _unlocked = 2;
``` 



File:Voter.sol#L91
```solidity
90:        _unlocked = 1;
``` 



File:Voter.sol#L102
```solidity
101:        minter = _minter;
``` 



File:Voter.sol#L113
```solidity
112:        admin = _admin;
``` 



File:Gauge.sol#L102
```solidity
101:        _unlocked = 2;
``` 



File:Gauge.sol#L104
```solidity
103:        _unlocked = 1;
``` 



File:Gauge.sol#L121
```solidity
120:                fees0 = 0;
``` 



File:Gauge.sol#L125
```solidity
124:                fees0 = _fees0;
``` 



File:Gauge.sol#L128
```solidity
127:                fees1 = 0;
``` 



File:Gauge.sol#L132
```solidity
131:                fees1 = _fees1;
``` 



File:Gauge.sol#L273
```solidity
272:            supplyNumCheckpoints = _nCheckPoints + 1;
``` 



File:Gauge.sol#L295
```solidity
294:        _unlocked = 1;
``` 



File:Gauge.sol#L297
```solidity
296:        _unlocked = 2;
``` 



File:Minter.sol#L46
```solidity
45:        initializer = msg.sender;
``` 



File:Minter.sol#L54
```solidity
53:        admin = _admin;
``` 



File:Minter.sol#L59
```solidity
58:        active_period = (block.timestamp + (2*week)) / week * week;
``` 



File:Minter.sol#L60
```solidity
59:        last_epoch = block.timestamp;
``` 



File:Minter.sol#L61
```solidity
60:        boost = 0;
``` 



File:Minter.sol#L63
```solidity
62:        weekly = _weekly;
``` 



File:Minter.sol#L77
```solidity
76:        initializer = address(0);
``` 



File:Minter.sol#L78
```solidity
77:        active_period = block.timestamp / week * week; // 
``` 



File:Minter.sol#L85
```solidity
84:        decay = _decay;
``` 



File:Minter.sol#L86
```solidity
85:        boost = _boost;
``` 



File:Minter.sol#L87
```solidity
86:        last_epoch = block.timestamp + 26 weeks;
``` 



File:Minter.sol#L126
```solidity
125:            active_period = _period;
``` 



File:Minter.sol#L127
```solidity
126:            weekly = weekly_emission();
``` 



File:Bribe.sol#L80
```solidity
79:        _unlocked = 2;
``` 



File:Bribe.sol#L82
```solidity
81:        _unlocked = 1;
``` 



File:Bribe.sol#L219
```solidity
218:            supplyNumCheckpoints = _nCheckPoints + 1;
``` 



 --- 

<a name=[Gas-10]></a>
### [Gas-10] Event is not properly indexed. - Instances: 7 

 > 
 When possible, always include a minimum of 3 indexed event topics to save gas 
 #### Gas Report - Savings: ~0 
 <details>  
 <summary>  
  </summary> 
  
 </details> 
 

 --- 

File:SwapFactory.sol#L24
```solidity
23:    event PairCreated(address indexed token0, address indexed token1, bool stable, address pair, uint);
``` 



File:Minter.sol#L32
```solidity
31:    event Mint(address indexed sender, uint weekly, uint circulating_supply, uint circulating_emission);
``` 



File:Multiswap.sol#L21
```solidity
20:    event Multiswapped(address indexed _token, uint _amountIn, uint[] amountsOut);
``` 



File:Gauge.sol#L78
```solidity
77:    event Deposit(address indexed from, uint tokenId, uint amount);
``` 



File:Gauge.sol#L79
```solidity
78:    event Withdraw(address indexed from, uint tokenId, uint amount);
``` 



File:Gauge.sol#L80
```solidity
79:    event NotifyReward(address indexed from, address indexed reward, uint amount);
``` 



File:Gauge.sol#L81
```solidity
80:    event ClaimFees(address indexed from, uint claimed0, uint claimed1);
``` 



File:Gauge.sol#L82
```solidity
81:    event ClaimRewards(address indexed from, address indexed reward, uint amount);
``` 



File:Voter.sol#L51
```solidity
50:    event Voted(address indexed voter, uint tokenId, uint256 weight);
``` 



File:Voter.sol#L52
```solidity
51:    event Abstained(uint tokenId, uint256 weight);
``` 



File:Voter.sol#L53
```solidity
52:    event Deposit(address indexed lp, address indexed gauge, uint tokenId, uint amount);
``` 



File:Voter.sol#L54
```solidity
53:    event Withdraw(address indexed lp, address indexed gauge, uint tokenId, uint amount);
``` 



File:Voter.sol#L55
```solidity
54:    event NotifyReward(address indexed sender, address indexed reward, uint amount);
``` 



File:Voter.sol#L56
```solidity
55:    event DistributeReward(address indexed sender, address indexed gauge, uint amount);
``` 



File:Voter.sol#L57
```solidity
56:    event Attach(address indexed owner, address indexed gauge, uint tokenId);
``` 



File:Voter.sol#L58
```solidity
57:    event Detach(address indexed owner, address indexed gauge, uint tokenId);
``` 



File:Bribe.sol#L65
```solidity
64:    event Deposit(address indexed from, uint tokenId, uint amount);
``` 



File:Bribe.sol#L66
```solidity
65:    event Withdraw(address indexed from, uint tokenId, uint amount);
``` 



File:Bribe.sol#L67
```solidity
66:    event NotifyReward(address indexed from, address indexed reward, uint amount);
``` 



File:Bribe.sol#L68
```solidity
67:    event ClaimRewards(address indexed from, address indexed reward, uint amount);
``` 



File:SwapPair.sol#L75
```solidity
74:    event Fees(address indexed sender, uint amount0, uint amount1);
``` 



File:SwapPair.sol#L76
```solidity
75:    event Mint(address indexed sender, uint amount0, uint amount1);
``` 



File:SwapPair.sol#L77
```solidity
76:    event Burn(address indexed sender, uint amount0, uint amount1, address indexed to);
``` 



File:SwapPair.sol#L78
```solidity
77:    event Swap(
78:        address indexed sender,
79:        uint amount0In,
80:        uint amount1In,
81:        uint amount0Out,
82:        uint amount1Out,
83:        address indexed to
84:    );
``` 



File:SwapPair.sol#L86
```solidity
85:    event Sync(uint reserve0, uint reserve1);
``` 



File:SwapPair.sol#L87
```solidity
86:    event Claim(address indexed sender, address indexed recipient, uint amount0, uint amount1);
``` 



File:SwapPair.sol#L89
```solidity
88:    event Transfer(address indexed from, address indexed to, uint amount);
``` 



File:SwapPair.sol#L90
```solidity
89:    event Approval(address indexed owner, address indexed spender, uint amount);
``` 



 --- 

<a name=[Gas-11]></a>
### [Gas-11] Right shift or Left shift instead of dividing or multiplying by powers of two - Instances: 5 

 > 
 Right shift or left shift when possible to save gas. 
 
#### Gas Report - Savings: ~65 
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

    function testGas() public view {
        c0.mul2();
        c1.shl2();
        c2.div2();
        c3.shr2();
    }
}

contract Contract0 {
    function mul2() public pure {
        uint256 val = 10;
        uint256 valMulTwo = val * 2;
        valMulTwo++;
    }
}

contract Contract1 {
    function shl2() public pure {
        uint256 val = 10;
        uint256 valMulTwo = val << 1;
        valMulTwo++;
    }
}

contract Contract2 {
    function div2() public pure {
        uint256 val = 10;
        uint256 valDivTwo = val / 2;
        valDivTwo++;
    }
}

contract Contract3 {
    function shr2() public pure {
        uint256 val = 10;
        uint256 valDivTwo = val >> 1;
        valDivTwo++;
    }
}


```


```solidity

╭───────────────────────────────────────────┬─────────────────┬─────┬────────┬─────┬─────────╮
│ src/test/GasTest.t.sol:Contract0 contract ┆                 ┆     ┆        ┆     ┆         │
╞═══════════════════════════════════════════╪═════════════════╪═════╪════════╪═════╪═════════╡
│ Deployment Cost                           ┆ Deployment Size ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 58911                                     ┆ 326             ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name                             ┆ min             ┆ avg ┆ median ┆ max ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ mul2                                      ┆ 297             ┆ 297 ┆ 297    ┆ 297 ┆ 1       │
╰───────────────────────────────────────────┴─────────────────┴─────┴────────┴─────┴─────────╯
╭───────────────────────────────────────────┬─────────────────┬─────┬────────┬─────┬─────────╮
│ src/test/GasTest.t.sol:Contract1 contract ┆                 ┆     ┆        ┆     ┆         │
╞═══════════════════════════════════════════╪═════════════════╪═════╪════════╪═════╪═════════╡
│ Deployment Cost                           ┆ Deployment Size ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 43893                                     ┆ 250             ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name                             ┆ min             ┆ avg ┆ median ┆ max ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ shl2                                      ┆ 203             ┆ 203 ┆ 203    ┆ 203 ┆ 1       │
╰───────────────────────────────────────────┴─────────────────┴─────┴────────┴─────┴─────────╯
╭───────────────────────────────────────────┬─────────────────┬─────┬────────┬─────┬─────────╮
│ src/test/GasTest.t.sol:Contract2 contract ┆                 ┆     ┆        ┆     ┆         │
╞═══════════════════════════════════════════╪═════════════════╪═════╪════════╪═════╪═════════╡
│ Deployment Cost                           ┆ Deployment Size ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 57705                                     ┆ 320             ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name                             ┆ min             ┆ avg ┆ median ┆ max ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ div2                                      ┆ 268             ┆ 268 ┆ 268    ┆ 268 ┆ 1       │
╰───────────────────────────────────────────┴─────────────────┴─────┴────────┴─────┴─────────╯
╭───────────────────────────────────────────┬─────────────────┬─────┬────────┬─────┬─────────╮
│ src/test/GasTest.t.sol:Contract3 contract ┆                 ┆     ┆        ┆     ┆         │
╞═══════════════════════════════════════════╪═════════════════╪═════╪════════╪═════╪═════════╡
│ Deployment Cost                           ┆ Deployment Size ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 43893                                     ┆ 250             ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name                             ┆ min             ┆ avg ┆ median ┆ max ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ shr2                                      ┆ 203             ┆ 203 ┆ 203    ┆ 203 ┆ 1       │
╰───────────────────────────────────────────┴─────────────────┴─────┴────────┴─────┴─────────╯

```

         
 </details> 
 

 --- 

File:Gauge.sol#L165
```solidity
164:            uint center = upper - (upper - lower) / 2; // ceil, avoiding overflow
``` 



File:Gauge.sol#L197
```solidity
196:            uint center = upper - (upper - lower) / 2; // ceil, avoiding overflow
``` 



File:Gauge.sol#L229
```solidity
228:            uint center = upper - (upper - lower) / 2; // ceil, avoiding overflow
``` 



File:SwapPair.sol#L160
```solidity
159:        uint256 _ratio = _feeIncrease * 1e18 / totalSupply; // 1e18 adjustment is removed during claim
``` 



File:SwapPair.sol#L174
```solidity
173:        uint256 _ratio = _feeIncrease * 1e18 / totalSupply;
``` 



File:SwapPair.sol#L195
```solidity
194:                uint _share = _supplied * _delta0 / 1e18; // add accrued difference for each supplied token
``` 



File:SwapPair.sol#L199
```solidity
198:                uint _share = _supplied * _delta1 / 1e18;
``` 



File:SwapPair.sol#L367
```solidity
366:        if (amount0In > 0) _update0(amount0In * fee / 1e6); // accrue fees for token0 and move them out of pool
``` 



File:SwapPair.sol#L368
```solidity
367:        if (amount1In > 0) _update1(amount1In * fee / 1e6); // accrue fees for token1 and move them out of pool
``` 



File:SwapPair.sol#L392
```solidity
391:        return x0*(y*y/1e18*y/1e18)/1e18+(x0*x0/1e18*x0/1e18)*y/1e18;
``` 



File:SwapPair.sol#L392
```solidity
391:        return x0*(y*y/1e18*y/1e18)/1e18+(x0*x0/1e18*x0/1e18)*y/1e18;
``` 



File:SwapPair.sol#L392
```solidity
391:        return x0*(y*y/1e18*y/1e18)/1e18+(x0*x0/1e18*x0/1e18)*y/1e18;
``` 



File:SwapPair.sol#L392
```solidity
391:        return x0*(y*y/1e18*y/1e18)/1e18+(x0*x0/1e18*x0/1e18)*y/1e18;
``` 



File:SwapPair.sol#L392
```solidity
391:        return x0*(y*y/1e18*y/1e18)/1e18+(x0*x0/1e18*x0/1e18)*y/1e18;
``` 



File:SwapPair.sol#L392
```solidity
391:        return x0*(y*y/1e18*y/1e18)/1e18+(x0*x0/1e18*x0/1e18)*y/1e18;
``` 



File:SwapPair.sol#L396
```solidity
395:        return 3*x0*(y*y/1e18)/1e18+(x0*x0/1e18*x0/1e18);
``` 



File:SwapPair.sol#L396
```solidity
395:        return 3*x0*(y*y/1e18)/1e18+(x0*x0/1e18*x0/1e18);
``` 



File:SwapPair.sol#L396
```solidity
395:        return 3*x0*(y*y/1e18)/1e18+(x0*x0/1e18*x0/1e18);
``` 



File:SwapPair.sol#L396
```solidity
395:        return 3*x0*(y*y/1e18)/1e18+(x0*x0/1e18*x0/1e18);
``` 



File:SwapPair.sol#L404
```solidity
403:                uint dy = (xy - k)*1e18/_d(x0, y);
``` 



File:SwapPair.sol#L407
```solidity
406:                uint dy = (k - xy)*1e18/_d(x0, y);
``` 



File:SwapPair.sol#L425
```solidity
424:        amountIn -= amountIn * fee / 1e6; // remove fee from amount received
``` 



File:SwapPair.sol#L432
```solidity
431:            _reserve0 = _reserve0 * 1e18 / decimals0;
``` 



File:SwapPair.sol#L433
```solidity
432:            _reserve1 = _reserve1 * 1e18 / decimals1;
``` 



File:SwapPair.sol#L435
```solidity
434:            amountIn = tokenIn == token0 ? amountIn * 1e18 / decimals0 : amountIn * 1e18 / decimals1;
``` 



File:SwapPair.sol#L435
```solidity
434:            amountIn = tokenIn == token0 ? amountIn * 1e18 / decimals0 : amountIn * 1e18 / decimals1;
``` 



File:SwapPair.sol#L437
```solidity
436:            return y * (tokenIn == token0 ? decimals1 : decimals0) / 1e18;
``` 



File:SwapPair.sol#L446
```solidity
445:            uint _x = x * 1e18 / decimals0;
``` 



File:SwapPair.sol#L447
```solidity
446:            uint _y = y * 1e18 / decimals1;
``` 



File:SwapPair.sol#L448
```solidity
447:            uint _a = (_x * _y) / 1e18;
``` 



File:SwapPair.sol#L449
```solidity
448:            uint _b = ((_x * _x) / 1e18 + (_y * _y) / 1e18);
``` 



File:SwapPair.sol#L449
```solidity
448:            uint _b = ((_x * _x) / 1e18 + (_y * _y) / 1e18);
``` 



File:SwapPair.sol#L450
```solidity
449:            return _a * _b / 1e18;  // x3y+y3x >= k
``` 



File:Minter.sol#L27
```solidity
26:    uint internal constant lock = 86400 * 7 * 52 * 4;
``` 



File:Minter.sol#L59
```solidity
58:        active_period = (block.timestamp + (2*week)) / week * week;
``` 



File:Voter.sol#L301
```solidity
300:        uint256 _ratio = _amount * 1e18 / totalWeight; // 1e18 adjustment is removed during claim
``` 



File:Voter.sol#L338
```solidity
337:                uint _share = uint(_supplied) * _delta / 1e18; // add accrued difference for each supplied token
``` 



File:Bribe.sol#L111
```solidity
110:            uint center = upper - (upper - lower) / 2; // ceil, avoiding overflow
``` 



File:Bribe.sol#L143
```solidity
142:            uint center = upper - (upper - lower) / 2; // ceil, avoiding overflow
``` 



File:Bribe.sol#L175
```solidity
174:            uint center = upper - (upper - lower) / 2; // ceil, avoiding overflow
``` 



 --- 

<a name=[Gas-12]></a>
### [Gas-12] Use multiple require() statments insted of require(expression && expression && ...) - Instances: 7 

 > 
 You can safe gas by breaking up a require statement with multiple conditions, into multiple require statements with a single condition. 
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

File:SwapPair.sol#L333
```solidity
332:        require(amount0 > 0 && amount1 > 0, 'ILB'); // SwapPair: INSUFFICIENT_LIQUIDITY_BURNED
``` 



File:SwapPair.sol#L349
```solidity
348:        require(amount0Out < _reserve0 && amount1Out < _reserve1, 'IL'); // SwapPair: INSUFFICIENT_LIQUIDITY
``` 



File:SwapPair.sol#L355
```solidity
354:        require(to != _token0 && to != _token1, 'IT'); // SwapPair: INVALID_TO
``` 



File:SwapPair.sol#L501
```solidity
500:        require(recoveredAddress != address(0) && recoveredAddress == owner, 'SwapPair: INVALID_SIGNATURE');
``` 



File:SwapPair.sol#L541
```solidity
540:        require(success && (data.length == 0 || abi.decode(data, (bool))));
``` 



File:SwapFactory.sol#L68
```solidity
67:        require(msg.sender == admin && _admin != address(0), "SwapFactory; wrong input parameters");
``` 



File:Minter.sol#L47
```solidity
46:        require(
47:            __voter != address(0) &&
48:            __ve != address(0) &&
49:            __ve_dist != address(0) &&
50:            _admin != address(0),
51:            "Minter: zero address provided in constructor"
52:        );
``` 



File:Multiswap.sol#L37
```solidity
36:        require(length > 1 && length <= 5 && _weights.length == length, "length");
``` 



File:Bribe.sol#L457
```solidity
456:        require(success && (data.length == 0 || abi.decode(data, (bool))));
``` 



File:Bribe.sol#L464
```solidity
463:        require(success && (data.length == 0 || abi.decode(data, (bool))));
``` 



File:Voter.sol#L69
```solidity
68:        require(
69:            __ve != address(0) &&
70:            _factory != address(0) &&
71:            _gauges != address(0) &&
72:            _bribes != address(0),
73:            "Voter: zero address provided in constructor"
74:        );
``` 



File:Voter.sol#L251
```solidity
250:        require(isWhitelisted[_tokenA] && isWhitelisted[_tokenB], "!whitelisted");
``` 



File:Voter.sol#L437
```solidity
436:        require(success && (data.length == 0 || abi.decode(data, (bool))));
``` 



File:Gauge.sol#L85
```solidity
84:        require(
85:            _stake != address(0) &&
86:            _bribe != address(0) &&
87:            __ve != address(0) &&
88:            _voter != address(0),
89:            "Gauge: zero address provided in constructor"
90:        );
``` 



File:Gauge.sol#L570
```solidity
569:        require(success && (data.length == 0 || abi.decode(data, (bool))));
``` 



File:Gauge.sol#L577
```solidity
576:        require(success && (data.length == 0 || abi.decode(data, (bool))));
``` 



File:Gauge.sol#L584
```solidity
583:        require(success && (data.length == 0 || abi.decode(data, (bool))));
``` 



 --- 

<a name=[Gas-13]></a>
### [Gas-13] Optimal Comparison - Instances: 5 

 > 
 When comparing integers, it is cheaper to use strict `>` & `<` operators over `>=` & `<=` operators, even if you must increment or decrement one of the operands. 
 Note: before using this technique, it's important to consider whether incrementing/decrementing one of the operators could result in an over/underflow. This optimization is applicable when the optimizer is turned off. 
 #### Gas Report - Savings: ~3 
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

    function testGas() public view {
        c0.gte();
        c1.gtPlusMinusOne();
        c2.lte();
        c3.ltPlusOne();
    }
}

contract Contract0 {
    function gte() external pure returns (bool) {
        return 2 >= 2;
    }
}

contract Contract1 {
    function gtPlusMinusOne() external pure returns (bool) {
        return 2 > 2 - 1;
    }
}

contract Contract2 {
    function lte() external pure returns (bool) {
        return 2 <= 2;
    }
}

contract Contract3 {
    function ltPlusOne() external pure returns (bool) {
        return 2 < 2 + 1;
    }
}

```


```solidity
╭───────────────────────────────────────────┬─────────────────┬─────┬────────┬─────┬─────────╮
│ src/test/GasTest.t.sol:Contract0 contract ┆                 ┆     ┆        ┆     ┆         │
╞═══════════════════════════════════════════╪═════════════════╪═════╪════════╪═════╪═════════╡
│ Deployment Cost                           ┆ Deployment Size ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 37487                                     ┆ 218             ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name                             ┆ min             ┆ avg ┆ median ┆ max ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ gte                                       ┆ 330             ┆ 330 ┆ 330    ┆ 330 ┆ 1       │
╰───────────────────────────────────────────┴─────────────────┴─────┴────────┴─────┴─────────╯
╭───────────────────────────────────────────┬─────────────────┬─────┬────────┬─────┬─────────╮
│ src/test/GasTest.t.sol:Contract1 contract ┆                 ┆     ┆        ┆     ┆         │
╞═══════════════════════════════════════════╪═════════════════╪═════╪════════╪═════╪═════════╡
│ Deployment Cost                           ┆ Deployment Size ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 37487                                     ┆ 218             ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name                             ┆ min             ┆ avg ┆ median ┆ max ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ gtPlusMinusOne                            ┆ 327             ┆ 327 ┆ 327    ┆ 327 ┆ 1       │
╰───────────────────────────────────────────┴─────────────────┴─────┴────────┴─────┴─────────╯
╭───────────────────────────────────────────┬─────────────────┬─────┬────────┬─────┬─────────╮
│ src/test/GasTest.t.sol:Contract2 contract ┆                 ┆     ┆        ┆     ┆         │
╞═══════════════════════════════════════════╪═════════════════╪═════╪════════╪═════╪═════════╡
│ Deployment Cost                           ┆ Deployment Size ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 37487                                     ┆ 218             ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name                             ┆ min             ┆ avg ┆ median ┆ max ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ lte                                       ┆ 330             ┆ 330 ┆ 330    ┆ 330 ┆ 1       │
╰───────────────────────────────────────────┴─────────────────┴─────┴────────┴─────┴─────────╯
╭───────────────────────────────────────────┬─────────────────┬─────┬────────┬─────┬─────────╮
│ src/test/GasTest.t.sol:Contract3 contract ┆                 ┆     ┆        ┆     ┆         │
╞═══════════════════════════════════════════╪═════════════════╪═════╪════════╪═════╪═════════╡
│ Deployment Cost                           ┆ Deployment Size ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 37487                                     ┆ 218             ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name                             ┆ min             ┆ avg ┆ median ┆ max ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ ltPlusOne                                 ┆ 327             ┆ 327 ┆ 327    ┆ 327 ┆ 1       │
╰───────────────────────────────────────────┴─────────────────┴─────┴────────┴─────┴─────────╯

```

 
 </details> 
 

 --- 

File:Minter.sol#L84
```solidity
83:        require(block.timestamp >= last_epoch + 26 weeks, "must wait next period");
``` 



File:Minter.sol#L124
```solidity
123:        if (block.timestamp >= _period + week && initializer == address(0)) { // only trigger if new week
``` 



File:Bribe.sol#L99
```solidity
98:        if (checkpoints[tokenId][nCheckpoints - 1].timestamp <= timestamp) {
``` 



File:Bribe.sol#L131
```solidity
130:        if (supplyCheckpoints[nCheckpoints - 1].timestamp <= timestamp) {
``` 



File:Bribe.sol#L163
```solidity
162:        if (rewardPerTokenCheckpoints[token][nCheckpoints - 1].timestamp <= timestamp) {
``` 



File:Bribe.sol#L336
```solidity
335:            for (uint i = _startIndex; i <= _endIndex - 1; i++) {
``` 



File:Bribe.sol#L373
```solidity
372:            for (uint i = _startIndex; i <= _endIndex - 1; i++) {
``` 



File:Bribe.sol#L417
```solidity
416:        if (block.timestamp >= periodFinish[token]) return 0;
``` 



File:Bribe.sol#L435
```solidity
434:        if (block.timestamp >= periodFinish[token]) {
``` 



File:Bribe.sol#L447
```solidity
446:        require(rewardRate[token] <= balance / DURATION, "Provided reward too high");
``` 



File:Multiswap.sol#L37
```solidity
36:        require(length > 1 && length <= 5 && _weights.length == length, "length");
``` 



File:SwapPair.sol#L372
```solidity
371:        require(_k(_balance0, _balance1) >= _k(_reserve0, _reserve1), 'K'); // SwapPair: K
``` 



File:SwapPair.sol#L411
```solidity
410:                if (y - y_prev <= 1) {
``` 



File:SwapPair.sol#L415
```solidity
414:                if (y_prev - y <= 1) {
``` 



File:SwapPair.sol#L478
```solidity
477:        require(deadline >= block.timestamp, 'SwapPair: EXPIRED');
``` 



File:Gauge.sol#L153
```solidity
152:        if (checkpoints[account][nCheckpoints - 1].timestamp <= timestamp) {
``` 



File:Gauge.sol#L185
```solidity
184:        if (supplyCheckpoints[nCheckpoints - 1].timestamp <= timestamp) {
``` 



File:Gauge.sol#L217
```solidity
216:        if (rewardPerTokenCheckpoints[token][nCheckpoints - 1].timestamp <= timestamp) {
``` 



File:Gauge.sol#L400
```solidity
399:            for (uint i = _startIndex; i <= _endIndex - 1; i++) {
``` 



File:Gauge.sol#L437
```solidity
436:            for (uint i = _startIndex; i <= _endIndex-1; i++) {
``` 



File:Gauge.sol#L530
```solidity
529:        if (block.timestamp >= periodFinish[token]) return 0;
``` 



File:Gauge.sol#L548
```solidity
547:        if (block.timestamp >= periodFinish[token]) {
``` 



File:Gauge.sol#L560
```solidity
559:        require(rewardRate[token] <= balance / DURATION, "Provided reward too high");
``` 



 --- 

<a name=[Gas-14]></a>
### [Gas-14] Mark functions as payable (with discretion) - Instances: 6 

 > 
 You can mark public or external functions as payable to save gas. Functions that are not payable have additional logic to check if there was a value sent with a call, however, making a function payable eliminates this check. This optimization should be carefully considered due to potentially unwanted behavior when a function does not need to accept ether. 
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

File:Gauge.sol#L110
```solidity
109:    function claimFees() external lock returns (uint claimed0, uint claimed1) {
``` 



File:Gauge.sol#L146
```solidity
145:    function getPriorBalanceIndex(address account, uint timestamp) public view returns (uint) {
``` 



File:Gauge.sol#L178
```solidity
177:    function getPriorSupplyIndex(uint timestamp) public view returns (uint) {
``` 



File:Gauge.sol#L210
```solidity
209:    function getPriorRewardPerToken(address token, uint timestamp) public view returns (uint, uint) {
``` 



File:Gauge.sol#L277
```solidity
276:    function rewardsListLength() external view returns (uint) {
``` 



File:Gauge.sol#L284
```solidity
283:    function lastTimeRewardApplicable(address token) public view returns (uint) {
``` 



File:Gauge.sol#L293
```solidity
292:    function getReward(address account, address[] memory tokens) external lock {
``` 



File:Gauge.sol#L321
```solidity
320:    function rewardPerToken(address token) public view returns (uint) {
``` 



File:Gauge.sol#L328
```solidity
327:    function derivedBalance(address account) public view returns (uint) {
``` 



File:Gauge.sol#L332
```solidity
331:    function batchRewardPerToken(address token, uint maxRuns) external {
``` 



File:Gauge.sol#L372
```solidity
371:    function batchUpdateRewardPerToken(address token, uint maxRuns) external {
``` 



File:Gauge.sol#L425
```solidity
424:    function earned(address token, address account) public view returns (uint) {
``` 



File:Gauge.sol#L453
```solidity
452:    function depositAll(uint tokenId) external {
``` 



File:Gauge.sol#L457
```solidity
456:    function deposit(uint amount, uint tokenId) public lock {
``` 



File:Gauge.sol#L489
```solidity
488:    function withdrawAll() external {
``` 



File:Gauge.sol#L493
```solidity
492:    function withdraw(uint amount) public {
``` 



File:Gauge.sol#L501
```solidity
500:    function withdrawToken(uint amount, uint tokenId) public lock {
``` 



File:Gauge.sol#L529
```solidity
528:    function left(address token) external view returns (uint) {
``` 



File:Gauge.sol#L535
```solidity
534:    function notifyRewardAmount(address token, uint amount) external lock {
``` 



File:Voter.sol#L97
```solidity
96:    function initialize(/*address[] memory _tokens*/address _minter) external {
``` 



File:Voter.sol#L107
```solidity
106:    function listing_fee() public view returns (uint) {
``` 



File:Voter.sol#L111
```solidity
110:    function setAdmin(address _admin) external onlyAdmin {
``` 



File:Voter.sol#L116
```solidity
115:    function setReward(address _gauge, address _token, bool _status) external onlyAdmin {
``` 



File:Voter.sol#L120
```solidity
119:    function setBribe(address _bribe, address _token, bool _status) external onlyAdmin {
``` 



File:Voter.sol#L124
```solidity
123:    function killGauge(address _gauge) external onlyAdmin {
``` 



File:Voter.sol#L131
```solidity
130:    function reviveGauge(address _gauge) external onlyAdmin {
``` 



File:Voter.sol#L138
```solidity
137:    function reset(uint _tokenId) external { // OR msg.sender == votingescrow when withdrawing
``` 



File:Voter.sol#L174
```solidity
173:    function poke(uint _tokenId) external {
``` 



File:Voter.sol#L225
```solidity
224:    function vote(uint tokenId, address[] calldata _gaugeVote, uint256[] calldata _weights) external {
``` 



File:Voter.sol#L235
```solidity
234:    function whitelist(address _token) public onlyAdmin {
``` 



File:Voter.sol#L247
```solidity
246:    function createSwapGauge(address _pair) external returns (address gauge) {
``` 



File:Voter.sol#L271
```solidity
270:    function attachTokenToGauge(uint tokenId, address account) external {
``` 



File:Voter.sol#L277
```solidity
276:    function emitDeposit(uint tokenId, address account, uint amount) external {
``` 



File:Voter.sol#L282
```solidity
281:    function detachTokenFromGauge(uint tokenId, address account) external {
``` 



File:Voter.sol#L288
```solidity
287:    function emitWithdraw(uint tokenId, address account, uint amount) external {
``` 



File:Voter.sol#L293
```solidity
292:    function length() external view returns (uint) {
``` 



File:Voter.sol#L299
```solidity
298:    function notifyRewardAmount(uint _amount) external {
``` 



File:Voter.sol#L308
```solidity
307:    function updateFor(address[] memory _gauges) external {
``` 



File:Voter.sol#L314
```solidity
313:    function updateForRange(uint start, uint end) public {
``` 



File:Voter.sol#L320
```solidity
319:    function updateAll() external {
``` 



File:Voter.sol#L326
```solidity
325:    function updateGauge(address _gauge) external {
``` 



File:Voter.sol#L351
```solidity
350:    function claimRewards(address[] memory _gauges, address[][] memory _tokens) external {
``` 



File:Voter.sol#L361
```solidity
360:    function claimBribes(address[] memory _bribes, address[][] memory _tokens, uint _tokenId) external {
``` 



File:Voter.sol#L372
```solidity
371:    function claimFees(address[] memory _fees, address[][] memory _tokens, uint _tokenId) external {
``` 



File:Voter.sol#L381
```solidity
380:    function distributeFees(address[] memory _gauges) external {
``` 



File:Voter.sol#L389
```solidity
388:    function distribute(address _gauge) public lock {
``` 



File:Voter.sol#L400
```solidity
399:    function distro() external {
``` 



File:Voter.sol#L404
```solidity
403:    function distribute() external {
``` 



File:Voter.sol#L408
```solidity
407:    function distribute(uint start, uint finish) public {
``` 



File:Voter.sol#L414
```solidity
413:    function distribute(address[] memory _gauges) external {
``` 



File:SwapPair.sol#L120
```solidity
119:    function observationLength() external view returns (uint) {
``` 



File:SwapPair.sol#L124
```solidity
123:    function lastObservation() public view returns (Observation memory) {
``` 



File:SwapPair.sol#L128
```solidity
127:    function metadata() external view returns (uint dec0, uint dec1, uint r0, uint r1, bool st, address t0, address t1) {
``` 



File:SwapPair.sol#L132
```solidity
131:    function tokens() external view returns (address, address) {
``` 



File:SwapPair.sol#L137
```solidity
136:    function claimFees() external returns (uint claimed0, uint claimed1) {
``` 



File:SwapPair.sol#L208
```solidity
207:    function getReserves() public view returns (uint _reserve0, uint _reserve1, uint _blockTimestampLast) {
``` 



File:SwapPair.sol#L235
```solidity
234:    function currentCumulativePrices() public view returns (uint reserve0Cumulative, uint reserve1Cumulative, uint blockTimestamp) {
``` 



File:SwapPair.sol#L251
```solidity
250:    function current(address tokenIn, uint amountIn) external view returns (uint amountOut) {
``` 



File:SwapPair.sol#L265
```solidity
264:    function quote(address tokenIn, uint amountIn, uint granularity) external view returns (uint amountOut) {
``` 



File:SwapPair.sol#L275
```solidity
274:    function prices(address tokenIn, uint amountIn, uint points) external view returns (uint[] memory) {
``` 



File:SwapPair.sol#L279
```solidity
278:    function sample(address tokenIn, uint amountIn, uint points, uint window) public view returns (uint[] memory) {
``` 



File:SwapPair.sol#L300
```solidity
299:    function mint(address to) external lock returns (uint liquidity) {
``` 



File:SwapPair.sol#L323
```solidity
322:    function burn(address to) external lock returns (uint amount0, uint amount1) {
``` 



File:SwapPair.sol#L345
```solidity
344:    function swap(uint amount0Out, uint amount1Out, address to, bytes calldata data) external lock {
``` 



File:SwapPair.sol#L380
```solidity
379:    function skim(address to) external lock {
``` 



File:SwapPair.sol#L387
```solidity
386:    function sync() external lock {
``` 



File:SwapPair.sol#L423
```solidity
422:    function getAmountOut(uint amountIn, address tokenIn) external view returns (uint) {
``` 



File:SwapPair.sol#L470
```solidity
469:    function approve(address spender, uint amount) external returns (bool) {
``` 



File:SwapPair.sol#L477
```solidity
476:    function permit(address owner, address spender, uint value, uint deadline, uint8 v, bytes32 r, bytes32 s) external {
``` 



File:SwapPair.sol#L507
```solidity
506:    function transfer(address dst, uint amount) external returns (bool) {
``` 



File:SwapPair.sol#L512
```solidity
511:    function transferFrom(address src, address dst, uint amount) external returns (bool) {
``` 



File:Minter.sol#L66
```solidity
65:    function initialize(
66:        // address[] memory claimants,
67:        // uint[] memory amounts,
68:        // uint max // sum amounts / max = % ownership of top protocols, so if initial 20m is distributed, and target is 25% protocol ownership, then max - 4 x 20m = 80m
69:    ) external {
``` 



File:Minter.sol#L83
```solidity
82:    function setEmissions(uint _decay, uint _boost) public onlyAdmin {
``` 



File:Minter.sol#L91
```solidity
90:    function circulating_supply() public view returns (uint) {
``` 



File:Minter.sol#L101
```solidity
100:    function calculate_emission() public view returns (uint) {
``` 



File:Minter.sol#L107
```solidity
106:    function weekly_emission() public view returns (uint) {
``` 



File:Minter.sol#L112
```solidity
111:    function circulating_emission() public view returns (uint) {
``` 



File:Minter.sol#L117
```solidity
116:    function calculate_growth(uint _minted) public view returns (uint) {
``` 



File:Minter.sol#L122
```solidity
121:    function update_period() external returns (uint) {
``` 



File:Bribe.sol#L92
```solidity
91:    function getPriorBalanceIndex(uint tokenId, uint timestamp) public view returns (uint) {
``` 



File:Bribe.sol#L124
```solidity
123:    function getPriorSupplyIndex(uint timestamp) public view returns (uint) {
``` 



File:Bribe.sol#L156
```solidity
155:    function getPriorRewardPerToken(address token, uint timestamp) public view returns (uint, uint) {
``` 



File:Bribe.sol#L223
```solidity
222:    function rewardsListLength() external view returns (uint) {
``` 



File:Bribe.sol#L228
```solidity
227:    function lastTimeRewardApplicable(address token) public view returns (uint) {
``` 



File:Bribe.sol#L233
```solidity
232:    function getReward(uint tokenId, address[] memory tokens) external lock  {
``` 



File:Bribe.sol#L248
```solidity
247:    function getRewardForOwner(uint tokenId, address[] memory tokens) external lock  {
``` 



File:Bribe.sol#L263
```solidity
262:    function rewardPerToken(address token) public view returns (uint) {
``` 



File:Bribe.sol#L270
```solidity
269:    function batchRewardPerToken(address token, uint maxRuns) external {
``` 



File:Bribe.sol#L308
```solidity
307:    function batchUpdateRewardPerToken(address token, uint maxRuns) external {
``` 



File:Bribe.sol#L361
```solidity
360:    function earned(address token, uint tokenId) public view returns (uint) {
``` 



File:Bribe.sol#L390
```solidity
389:    function _deposit(uint amount, uint tokenId) external {
``` 



File:Bribe.sol#L403
```solidity
402:    function _withdraw(uint amount, uint tokenId) external {
``` 



File:Bribe.sol#L416
```solidity
415:    function left(address token) external view returns (uint) {
``` 



File:Bribe.sol#L423
```solidity
422:    function notifyRewardAmount(address token, uint amount) external lock {
``` 



File:SwapFactory.sol#L43
```solidity
42:    function allPairsLength() external view returns (uint) {
``` 



File:SwapFactory.sol#L47
```solidity
46:    function setPauser(address _pauser) external {
``` 



File:SwapFactory.sol#L52
```solidity
51:    function acceptPauser() external {
``` 



File:SwapFactory.sol#L57
```solidity
56:    function setPause(bool _state) external {
``` 



File:SwapFactory.sol#L62
```solidity
61:    function setFeeTier(bool _stable, uint _fee) external {
``` 



File:SwapFactory.sol#L67
```solidity
66:    function setAdmin(address _admin) external {
``` 



File:SwapFactory.sol#L72
```solidity
71:    function pairCodeHash() external pure returns (bytes32) {
``` 



File:SwapFactory.sol#L76
```solidity
75:    function getInitializable() external view returns (address, address, bool, uint) {
``` 



File:SwapFactory.sol#L80
```solidity
79:    function createPair(address tokenA, address tokenB, bool stable) external returns (address pair) {
``` 



 --- 

<a name=[Gas-15]></a>
### [Gas-15] Consider marking constants as private - Instances: 5 

 > 
 Consider marking constant variables in storage as private to save gas (unless a constant variable should be easily accessible by another protocol or offchain logic). 
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

File:SwapPair.sol#L17
```solidity
16:    uint8 public constant decimals = 18;
``` 



File:SwapPair.sol#L29
```solidity
28:    bytes32 internal constant PERMIT_TYPEHASH = 0x6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c9;
``` 



File:SwapPair.sol#L32
```solidity
31:    uint internal constant MINIMUM_LIQUIDITY = 10**3;
``` 



File:SwapPair.sol#L48
```solidity
47:    uint constant periodSize = 1800;
``` 



File:Minter.sol#L14
```solidity
13:    uint internal constant week = 86400 * 7; // allows minting once per week (reset every Thursday 00:00 UTC)
``` 



File:Minter.sol#L16
```solidity
15:    uint internal constant target_base = 10000; // 2% per week target emission
``` 



File:Minter.sol#L17
```solidity
16:    uint internal constant tail_emission = 3; // 0.03% per week tail emission
``` 



File:Minter.sol#L18
```solidity
17:    uint internal constant tail_base = 1000; // 0.2% per week target emission
``` 



File:Minter.sol#L27
```solidity
26:    uint internal constant lock = 86400 * 7 * 52 * 4;
``` 



File:Gauge.sol#L23
```solidity
22:    uint internal constant DURATION = 7 days; // rewards are released over 7 days
``` 



File:Gauge.sol#L24
```solidity
23:    uint internal constant PRECISION = 10 ** 18;
``` 



File:Gauge.sol#L25
```solidity
24:    uint internal constant MAX_REWARD_TOKENS = 16;
``` 



File:Voter.sol#L23
```solidity
22:    uint internal constant DURATION = 7 days; // rewards are released over 7 days
``` 



File:Bribe.sol#L15
```solidity
14:    uint public constant DURATION = 7 days; // rewards are released over 7 days
``` 



File:Bribe.sol#L16
```solidity
15:    uint public constant PRECISION = 10 ** 18;
``` 



File:Bribe.sol#L17
```solidity
16:    uint public constant MAX_REWARD_TOKENS = 16; // max number of reward tokens that can be added
``` 



 --- 

<a name=[Gas-16]></a>
### [Gas-16] Use assembly to check for address(0) - Instances: 6 

 > 
  
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

File:Gauge.sol#L86
```solidity
85:            _stake != address(0) &&
``` 



File:Gauge.sol#L87
```solidity
86:            _bribe != address(0) &&
``` 



File:Gauge.sol#L88
```solidity
87:            __ve != address(0) &&
``` 



File:Gauge.sol#L89
```solidity
88:            _voter != address(0),
``` 



File:SwapFactory.sol#L33
```solidity
32:            _feeCollector != address(0),
``` 



File:SwapFactory.sol#L68
```solidity
67:        require(msg.sender == admin && _admin != address(0), "SwapFactory; wrong input parameters");
``` 



File:SwapFactory.sol#L83
```solidity
82:        require(token0 != address(0), 'ZA'); // BaseV1: ZERO_ADDRESS
``` 



File:SwapFactory.sol#L84
```solidity
83:        require(getPair[token0][token1][stable] == address(0), 'PE'); // BaseV1: PAIR_EXISTS - single check is sufficient
``` 



File:Voter.sol#L70
```solidity
69:            __ve != address(0) &&
``` 



File:Voter.sol#L71
```solidity
70:            _factory != address(0) &&
``` 



File:Voter.sol#L72
```solidity
71:            _gauges != address(0) &&
``` 



File:Voter.sol#L73
```solidity
72:            _bribes != address(0),
``` 



File:Voter.sol#L112
```solidity
111:        require(_admin != address(0), "zero address");
``` 



File:Multiswap.sol#L13
```solidity
12:            _router != address(0),
``` 



File:Multiswap.sol#L42
```solidity
41:        if (eth = (_token == address(0))) {
``` 



File:Minter.sol#L48
```solidity
47:            __voter != address(0) &&
``` 



File:Minter.sol#L49
```solidity
48:            __ve != address(0) &&
``` 



File:Minter.sol#L50
```solidity
49:            __ve_dist != address(0) &&
``` 



File:Minter.sol#L51
```solidity
50:            _admin != address(0),
``` 



File:Minter.sol#L124
```solidity
123:        if (block.timestamp >= _period + week && initializer == address(0)) { // only trigger if new week
``` 



File:SwapPair.sol#L501
```solidity
500:        require(recoveredAddress != address(0) && recoveredAddress == owner, 'SwapPair: INVALID_SIGNATURE');
``` 



 --- 

<a name=[Gas-17]></a>
### [Gas-17] Cache array length during for loop definition. - Instances: 4 

 > 
 A typical for loop definition may look like: `for (uint256 i; i < arr.length; i++){}`. Instead of using `array.length`, cache the array length before the loop, and use the cached value to safe gas. This will avoid an `MLOAD` every loop for arrays stored in memory and an `SLOAD` for arrays stored in storage. This can have significant gas savings for arrays with a large length, especially if the array is stored in storage. 
 #### Gas Report - Savings: ~22 
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

    function testGas() public view {
        uint256[] memory arr = new uint256[](10);
        c0.nonCachedMemoryListLength(arr);
        c1.cachedMemoryListLength(arr);
        c2.nonCachedStorageListLength();
        c3.cachedStorageListLength();
    }
}

contract Contract0 {
    function nonCachedMemoryListLength(uint256[] memory arr) public pure {
        uint256 j;
        for (uint256 i; i < arr.length; i++) {
            j = arr[i] + 10;
        }
    }
}

contract Contract1 {
    function cachedMemoryListLength(uint256[] memory arr) public pure {
        uint256 j;

        uint256 length = arr.length;
        for (uint256 i; i < length; i++) {
            j = arr[i] + 10;
        }
    }
}

contract Contract2 {
    uint256[] arr = new uint256[](10);

    function nonCachedStorageListLength() public view {
        uint256 j;
        for (uint256 i; i < arr.length; i++) {
            j = arr[i] + 10;
        }
    }
}

contract Contract3 {
    uint256[] arr = new uint256[](10);

    function cachedStorageListLength() public view {
        uint256 j;
        uint256 length = arr.length;

        for (uint256 i; i < length; i++) {
            j = arr[i] + 10;
        }
    }
}


```

```solidity
╭───────────────────────────────────────────┬─────────────────┬──────┬────────┬──────┬─────────╮
│ src/test/GasTest.t.sol:Contract0 contract ┆                 ┆      ┆        ┆      ┆         │
╞═══════════════════════════════════════════╪═════════════════╪══════╪════════╪══════╪═════════╡
│ Deployment Cost                           ┆ Deployment Size ┆      ┆        ┆      ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 128171                                    ┆ 672             ┆      ┆        ┆      ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name                             ┆ min             ┆ avg  ┆ median ┆ max  ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ nonCachedMemoryListLength                 ┆ 3755            ┆ 3755 ┆ 3755   ┆ 3755 ┆ 1       │
╰───────────────────────────────────────────┴─────────────────┴──────┴────────┴──────┴─────────╯
╭───────────────────────────────────────────┬─────────────────┬──────┬────────┬──────┬─────────╮
│ src/test/GasTest.t.sol:Contract1 contract ┆                 ┆      ┆        ┆      ┆         │
╞═══════════════════════════════════════════╪═════════════════╪══════╪════════╪══════╪═════════╡
│ Deployment Cost                           ┆ Deployment Size ┆      ┆        ┆      ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 128777                                    ┆ 675             ┆      ┆        ┆      ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name                             ┆ min             ┆ avg  ┆ median ┆ max  ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ cachedMemoryListLength                    ┆ 3733            ┆ 3733 ┆ 3733   ┆ 3733 ┆ 1       │
╰───────────────────────────────────────────┴─────────────────┴──────┴────────┴──────┴─────────╯
╭───────────────────────────────────────────┬─────────────────┬───────┬────────┬───────┬─────────╮
│ src/test/GasTest.t.sol:Contract2 contract ┆                 ┆       ┆        ┆       ┆         │
╞═══════════════════════════════════════════╪═════════════════╪═══════╪════════╪═══════╪═════════╡
│ Deployment Cost                           ┆ Deployment Size ┆       ┆        ┆       ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 118474                                    ┆ 539             ┆       ┆        ┆       ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name                             ┆ min             ┆ avg   ┆ median ┆ max   ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ nonCachedStorageListLength                ┆ 27979           ┆ 27979 ┆ 27979  ┆ 27979 ┆ 1       │
╰───────────────────────────────────────────┴─────────────────┴───────┴────────┴───────┴─────────╯
╭───────────────────────────────────────────┬─────────────────┬───────┬────────┬───────┬─────────╮
│ src/test/GasTest.t.sol:Contract3 contract ┆                 ┆       ┆        ┆       ┆         │
╞═══════════════════════════════════════════╪═════════════════╪═══════╪════════╪═══════╪═════════╡
│ Deployment Cost                           ┆ Deployment Size ┆       ┆        ┆       ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 118674                                    ┆ 540             ┆       ┆        ┆       ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name                             ┆ min             ┆ avg   ┆ median ┆ max   ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ cachedStorageListLength                   ┆ 26984           ┆ 26984 ┆ 26984  ┆ 26984 ┆ 1       │
╰───────────────────────────────────────────┴─────────────────┴───────┴────────┴───────┴─────────╯

```
    
 
 </details> 
 

 --- 

File:SwapPair.sol#L268
```solidity
267:        for (uint i = 0; i < _prices.length; i++) {
``` 



File:Voter.sol#L309
```solidity
308:        for (uint i = 0; i < _gauges.length; i++) {
``` 



File:Voter.sol#L352
```solidity
351:        for (uint i = 0; i < _gauges.length; i++) {
``` 



File:Voter.sol#L363
```solidity
362:        for (uint i = 0; i < _bribes.length; i++) {
``` 



File:Voter.sol#L374
```solidity
373:        for (uint i = 0; i < _fees.length; i++) {
``` 



File:Voter.sol#L382
```solidity
381:        for (uint i = 0; i < _gauges.length; i++) {
``` 



File:Voter.sol#L415
```solidity
414:        for (uint x = 0; x < _gauges.length; x++) {
``` 



File:Gauge.sol#L299
```solidity
298:        for (uint i = 0; i < tokens.length; i++) {
``` 



File:Bribe.sol#L235
```solidity
234:        for (uint i = 0; i < tokens.length; i++) {
``` 



File:Bribe.sol#L251
```solidity
250:        for (uint i = 0; i < tokens.length; i++) {
``` 



 --- 



## Quality Assurance - Total: 569 

<a name=[NonCritical-0]></a>
### [NonCritical-0] Private variables should contain a leading underscore - Instances: 11 

 > Consider adding an underscore to the beginning of the variable name 

 --- 

File:Bribe.sol#L13
```solidity
12:    address public immutable _ve;
``` 



File:SwapPair.sol#L27
```solidity
26:    bytes32 internal DOMAIN_SEPARATOR;
``` 



File:SwapPair.sol#L53
```solidity
52:    uint internal immutable decimals1;
``` 



File:SwapPair.sol#L52
```solidity
51:    uint internal immutable decimals0;
``` 



File:Gauge.sol#L16
```solidity
15:    address public immutable _ve; // the ve token used for gauges
``` 



File:Minter.sol#L29
```solidity
28:    address internal initializer;
``` 



File:Minter.sol#L15
```solidity
14:    uint internal decay = 9900; // weekly decay of 1%
``` 



File:Minter.sol#L30
```solidity
29:    address internal admin;
``` 



File:Voter.sol#L17
```solidity
16:    address public immutable _ve;
``` 



File:Voter.sol#L46
```solidity
45:    uint internal index;
``` 



File:Voter.sol#L20
```solidity
19:    address internal immutable base;
``` 



 --- 

<a name=[NonCritical-1]></a>
### [NonCritical-1] Constructor should check that all parameters are not 0 - Instances: 6 

 > Consider adding a require statement to check that all parameters are not 0 in the constructor 

 --- 

File:SwapPair.sol#L92
```solidity
91:    constructor() {
``` 



File:SwapPair.sol#L92
```solidity
91:    constructor() {
``` 



File:SwapPair.sol#L92
```solidity
91:    constructor() {
``` 



File:SwapPair.sol#L92
```solidity
91:    constructor() {
``` 



File:Minter.sol#L39
```solidity
38:    constructor(
39:        address __voter, // the voting & distribution system
40:        address  __ve, // the ve(3,3) system that will be locked into
41:        address __ve_dist, // the distribution system that ensures users aren't diluted
42:        address _admin,
43:        uint _weekly
44:    ) {
``` 



File:Bribe.sol#L70
```solidity
69:    constructor(address _voter) {
``` 



 --- 

<a name=[NonCritical-2]></a>
### [NonCritical-2] Consider importing specific identifiers instead of the whole file - Instances: 30 

 > This will minimize compiled code size and help with readability 

 --- 

File:SwapPair.sol#L4
```solidity
3:import "./SwapFactory.sol";
``` 



File:SwapPair.sol#L5
```solidity
4:import "./SwapFees.sol";
``` 



File:SwapPair.sol#L6
```solidity
5:import "./libraries/Math.sol";
``` 



File:SwapPair.sol#L7
```solidity
6:import '@openzeppelin/contracts/token/ERC20/IERC20.sol';
``` 



File:SwapPair.sol#L9
```solidity
8:import "./interfaces/callback/ISwapCallee.sol";
``` 



File:SwapPair.sol#L10
```solidity
9:import "./interfaces/ISwapFactory.sol";
``` 



File:SwapFactory.sol#L4
```solidity
3:import "./SwapPair.sol";
``` 



File:Gauge.sol#L4
```solidity
3:import "./libraries/Math.sol";
``` 



File:Gauge.sol#L5
```solidity
4:import '@openzeppelin/contracts/token/ERC20/IERC20.sol';
``` 



File:Gauge.sol#L6
```solidity
5:import "./interfaces/IVotingEscrow.sol";
``` 



File:Gauge.sol#L8
```solidity
7:import "../core/interfaces/ISwapFactory.sol";
``` 



File:Gauge.sol#L9
```solidity
8:import "../core/interfaces/ISwapPair.sol";
``` 



File:Gauge.sol#L10
```solidity
9:import "./interfaces/IBribe.sol";
``` 



File:Voter.sol#L4
```solidity
3:import '@openzeppelin/contracts/token/ERC20/IERC20.sol';
``` 



File:Voter.sol#L5
```solidity
4:import "./interfaces/IVotingEscrow.sol";
``` 



File:Voter.sol#L6
```solidity
5:import "../core/interfaces/ISwapPair.sol";
``` 



File:Voter.sol#L7
```solidity
6:import "./interfaces/IGaugeFactory.sol";
``` 



File:Voter.sol#L8
```solidity
7:import "./interfaces/IBribeFactory.sol";
``` 



File:Voter.sol#L9
```solidity
8:import "./interfaces/IGauge.sol";
``` 



File:Voter.sol#L10
```solidity
9:import "./interfaces/IBribe.sol";
``` 



File:Voter.sol#L11
```solidity
10:import "../core/interfaces/ISwapFactory.sol";
``` 



File:Voter.sol#L12
```solidity
11:import "./interfaces/IMinter.sol";
``` 



File:Minter.sol#L4
```solidity
3:import "./libraries/Math.sol";
``` 



File:Minter.sol#L5
```solidity
4:import "./interfaces/IUnderlying.sol";
``` 



File:Minter.sol#L6
```solidity
5:import "./interfaces/IVotingEscrow.sol";
``` 



File:Minter.sol#L8
```solidity
7:import "./interfaces/IVotingDist.sol";
``` 



File:Bribe.sol#L4
```solidity
3:import "./libraries/Math.sol";
``` 



File:Bribe.sol#L5
```solidity
4:import '@openzeppelin/contracts/token/ERC20/IERC20.sol';
``` 



File:Bribe.sol#L6
```solidity
5:import "./interfaces/IVotingEscrow.sol";
``` 



File:Multiswap.sol#L5
```solidity
4:import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
``` 



 --- 

<a name=[NonCritical-3]></a>
### [NonCritical-3] Storage variables should be named with camel case - Instances: 1 

 > Consider renaming to follow convention 

 --- 

File:SwapPair.sol#L27
```solidity
26:    bytes32 internal DOMAIN_SEPARATOR;
``` 



 --- 

<a name=[NonCritical-4]></a>
### [NonCritical-4] Function names should be in camelCase - Instances: 59 

 > Ensure that function definitions are declared using camelCase 

 --- 

File:Voter.sol#L87
```solidity
86:    modifier lock() {
``` 



File:Voter.sol#L97
```solidity
96:    function initialize(/*address[] memory _tokens*/address _minter) external {
``` 



File:Voter.sol#L107
```solidity
106:    function listing_fee() public view returns (uint) {
``` 



File:Voter.sol#L138
```solidity
137:    function reset(uint _tokenId) external { // OR msg.sender == votingescrow when withdrawing
``` 



File:Voter.sol#L146
```solidity
145:    function _reset(uint _tokenId) internal {
``` 



File:Voter.sol#L174
```solidity
173:    function poke(uint _tokenId) external {
``` 



File:Voter.sol#L186
```solidity
185:    function _vote(uint _tokenId, address[] memory _gaugeVote, uint256[] memory _weights) internal {
``` 



File:Voter.sol#L225
```solidity
224:    function vote(uint tokenId, address[] calldata _gaugeVote, uint256[] calldata _weights) external {
``` 



File:Voter.sol#L235
```solidity
234:    function whitelist(address _token) public onlyAdmin {
``` 



File:Voter.sol#L239
```solidity
238:    function _whitelist(address _token) internal {
``` 



File:Voter.sol#L293
```solidity
292:    function length() external view returns (uint) {
``` 



File:Voter.sol#L389
```solidity
388:    function distribute(address _gauge) public lock {
``` 



File:Voter.sol#L400
```solidity
399:    function distro() external {
``` 



File:Voter.sol#L404
```solidity
403:    function distribute() external {
``` 



File:Voter.sol#L408
```solidity
407:    function distribute(uint start, uint finish) public {
``` 



File:Voter.sol#L414
```solidity
413:    function distribute(address[] memory _gauges) external {
``` 



File:Minter.sol#L66
```solidity
65:    function initialize(
66:        // address[] memory claimants,
67:        // uint[] memory amounts,
68:        // uint max // sum amounts / max = % ownership of top protocols, so if initial 20m is distributed, and target is 25% protocol ownership, then max - 4 x 20m = 80m
69:    ) external {
``` 



File:Minter.sol#L91
```solidity
90:    function circulating_supply() public view returns (uint) {
``` 



File:Minter.sol#L96
```solidity
95:    function _calculate_emission() private view returns (uint) {
``` 



File:Minter.sol#L101
```solidity
100:    function calculate_emission() public view returns (uint) {
``` 



File:Minter.sol#L107
```solidity
106:    function weekly_emission() public view returns (uint) {
``` 



File:Minter.sol#L112
```solidity
111:    function circulating_emission() public view returns (uint) {
``` 



File:Minter.sol#L117
```solidity
116:    function calculate_growth(uint _minted) public view returns (uint) {
``` 



File:Minter.sol#L122
```solidity
121:    function update_period() external returns (uint) {
``` 



File:Multiswap.sol#L29
```solidity
28:    function multiswap(
29:        address _token,
30:        uint _amount,
31:        bytes[] memory _swapData,
32:        uint[] calldata _weights
33:    ) external payable returns (uint[] memory) {
``` 



File:Bribe.sol#L78
```solidity
77:    modifier lock() {
``` 



File:Bribe.sol#L361
```solidity
360:    function earned(address token, uint tokenId) public view returns (uint) {
``` 



File:Bribe.sol#L390
```solidity
389:    function _deposit(uint amount, uint tokenId) external {
``` 



File:Bribe.sol#L403
```solidity
402:    function _withdraw(uint amount, uint tokenId) external {
``` 



File:Bribe.sol#L416
```solidity
415:    function left(address token) external view returns (uint) {
``` 



File:SwapPair.sol#L113
```solidity
112:    modifier lock() {
``` 



File:SwapPair.sol#L128
```solidity
127:    function metadata() external view returns (uint dec0, uint dec1, uint r0, uint r1, bool st, address t0, address t1) {
``` 



File:SwapPair.sol#L132
```solidity
131:    function tokens() external view returns (address, address) {
``` 



File:SwapPair.sol#L154
```solidity
153:    function _update0(uint amount) internal {
``` 



File:SwapPair.sol#L168
```solidity
167:    function _update1(uint amount) internal {
``` 



File:SwapPair.sol#L215
```solidity
214:    function _update(uint balance0, uint balance1, uint _reserve0, uint _reserve1) internal {
``` 



File:SwapPair.sol#L251
```solidity
250:    function current(address tokenIn, uint amountIn) external view returns (uint amountOut) {
``` 



File:SwapPair.sol#L265
```solidity
264:    function quote(address tokenIn, uint amountIn, uint granularity) external view returns (uint amountOut) {
``` 



File:SwapPair.sol#L275
```solidity
274:    function prices(address tokenIn, uint amountIn, uint points) external view returns (uint[] memory) {
``` 



File:SwapPair.sol#L279
```solidity
278:    function sample(address tokenIn, uint amountIn, uint points, uint window) public view returns (uint[] memory) {
``` 



File:SwapPair.sol#L300
```solidity
299:    function mint(address to) external lock returns (uint liquidity) {
``` 



File:SwapPair.sol#L323
```solidity
322:    function burn(address to) external lock returns (uint amount0, uint amount1) {
``` 



File:SwapPair.sol#L345
```solidity
344:    function swap(uint amount0Out, uint amount1Out, address to, bytes calldata data) external lock {
``` 



File:SwapPair.sol#L380
```solidity
379:    function skim(address to) external lock {
``` 



File:SwapPair.sol#L387
```solidity
386:    function sync() external lock {
``` 



File:SwapPair.sol#L391
```solidity
390:    function _f(uint x0, uint y) internal pure returns (uint) {
``` 



File:SwapPair.sol#L395
```solidity
394:    function _d(uint x0, uint y) internal pure returns (uint) {
``` 



File:SwapPair.sol#L399
```solidity
398:    function _get_y(uint x0, uint xy, uint y) internal pure returns (uint) {
``` 



File:SwapPair.sol#L444
```solidity
443:    function _k(uint x, uint y) internal view returns (uint) {
``` 



File:SwapPair.sol#L456
```solidity
455:    function _mint(address dst, uint amount) internal {
``` 



File:SwapPair.sol#L463
```solidity
462:    function _burn(address dst, uint amount) internal {
``` 



File:SwapPair.sol#L470
```solidity
469:    function approve(address spender, uint amount) external returns (bool) {
``` 



File:SwapPair.sol#L477
```solidity
476:    function permit(address owner, address spender, uint value, uint deadline, uint8 v, bytes32 r, bytes32 s) external {
``` 



File:SwapPair.sol#L507
```solidity
506:    function transfer(address dst, uint amount) external returns (bool) {
``` 



File:Gauge.sol#L100
```solidity
99:    modifier lock() {
``` 



File:Gauge.sol#L425
```solidity
424:    function earned(address token, address account) public view returns (uint) {
``` 



File:Gauge.sol#L457
```solidity
456:    function deposit(uint amount, uint tokenId) public lock {
``` 



File:Gauge.sol#L493
```solidity
492:    function withdraw(uint amount) public {
``` 



File:Gauge.sol#L529
```solidity
528:    function left(address token) external view returns (uint) {
``` 



 --- 

<a name=[NonCritical-5]></a>
### [NonCritical-5] Constant and immutable variable names should be in SCREAMING_SNAKE_CASE - Instances: 36 

 > Ensure that Constant and immutable variable names are declared using SCREAMING_SNAKE_CASE 

 --- 

File:SwapFactory.sol#L17
```solidity
16:    address public immutable feeCollector;
``` 



File:Minter.sol#L14
```solidity
13:    uint internal constant week = 86400 * 7; // allows minting once per week (reset every Thursday 00:00 UTC)
``` 



File:Minter.sol#L16
```solidity
15:    uint internal constant target_base = 10000; // 2% per week target emission
``` 



File:Minter.sol#L17
```solidity
16:    uint internal constant tail_emission = 3; // 0.03% per week tail emission
``` 



File:Minter.sol#L18
```solidity
17:    uint internal constant tail_base = 1000; // 0.2% per week target emission
``` 



File:Minter.sol#L27
```solidity
26:    uint internal constant lock = 86400 * 7 * 52 * 4;
``` 



File:Minter.sol#L19
```solidity
18:    underlying public immutable _token;
``` 



File:Minter.sol#L20
```solidity
19:    IVoter public immutable _voter;
``` 



File:Minter.sol#L21
```solidity
20:    IVotingEscrow public immutable _ve;
``` 



File:Minter.sol#L22
```solidity
21:    IVotingDist public immutable _ve_dist;
``` 



File:Voter.sol#L23
```solidity
22:    uint internal constant DURATION = 7 days; // rewards are released over 7 days
``` 



File:Voter.sol#L17
```solidity
16:    address public immutable _ve;
``` 



File:Voter.sol#L18
```solidity
17:    address public immutable factory;
``` 



File:Voter.sol#L20
```solidity
19:    address internal immutable base;
``` 



File:Voter.sol#L22
```solidity
21:    address public immutable bribeFactory;
``` 



File:Gauge.sol#L23
```solidity
22:    uint internal constant DURATION = 7 days; // rewards are released over 7 days
``` 



File:Gauge.sol#L24
```solidity
23:    uint internal constant PRECISION = 10 ** 18;
``` 



File:Gauge.sol#L15
```solidity
14:    address public immutable stake; // the LP token that needs to be staked for rewards
``` 



File:Gauge.sol#L16
```solidity
15:    address public immutable _ve; // the ve token used for gauges
``` 



File:Gauge.sol#L17
```solidity
16:    address public immutable bribe;
``` 



File:Gauge.sol#L18
```solidity
17:    address public immutable voter;
``` 



File:Multiswap.sol#L9
```solidity
8:    address public immutable router;
``` 



File:Bribe.sol#L15
```solidity
14:    uint public constant DURATION = 7 days; // rewards are released over 7 days
``` 



File:Bribe.sol#L16
```solidity
15:    uint public constant PRECISION = 10 ** 18;
``` 



File:Bribe.sol#L12
```solidity
11:    address public immutable voter; // only voter can modify balances (since it only happens on vote())
``` 



File:Bribe.sol#L13
```solidity
12:    address public immutable _ve;
``` 



File:SwapPair.sol#L17
```solidity
16:    uint8 public constant decimals = 18;
``` 



File:SwapPair.sol#L48
```solidity
47:    uint constant periodSize = 1800;
``` 



File:SwapPair.sol#L20
```solidity
19:    bool public immutable stable;
``` 



File:SwapPair.sol#L34
```solidity
33:    address public immutable token0;
``` 



File:SwapPair.sol#L35
```solidity
34:    address public immutable token1;
``` 



File:SwapPair.sol#L36
```solidity
35:    address public immutable fees;
``` 



File:SwapPair.sol#L37
```solidity
36:    address public immutable factory;
``` 



File:SwapPair.sol#L38
```solidity
37:    uint public immutable fee;
``` 



File:SwapPair.sol#L52
```solidity
51:    uint internal immutable decimals0;
``` 



File:SwapPair.sol#L53
```solidity
52:    uint internal immutable decimals1;
``` 



 --- 

<a name=[NonCritical-6]></a>
### [NonCritical-6] Remove any unused returns - Instances: 11 

 > Either remove the return parameter names, or use them as the returns of the function. 

 --- 

File:Gauge.sol#L110
```solidity
109:    function claimFees() external lock returns (uint claimed0, uint claimed1) {
``` 



File:Gauge.sol#L110
```solidity
109:    function claimFees() external lock returns (uint claimed0, uint claimed1) {
``` 



File:SwapPair.sol#L128
```solidity
127:    function metadata() external view returns (uint dec0, uint dec1, uint r0, uint r1, bool st, address t0, address t1) {
``` 



File:SwapPair.sol#L128
```solidity
127:    function metadata() external view returns (uint dec0, uint dec1, uint r0, uint r1, bool st, address t0, address t1) {
``` 



File:SwapPair.sol#L128
```solidity
127:    function metadata() external view returns (uint dec0, uint dec1, uint r0, uint r1, bool st, address t0, address t1) {
``` 



File:SwapPair.sol#L128
```solidity
127:    function metadata() external view returns (uint dec0, uint dec1, uint r0, uint r1, bool st, address t0, address t1) {
``` 



File:SwapPair.sol#L128
```solidity
127:    function metadata() external view returns (uint dec0, uint dec1, uint r0, uint r1, bool st, address t0, address t1) {
``` 



File:SwapPair.sol#L128
```solidity
127:    function metadata() external view returns (uint dec0, uint dec1, uint r0, uint r1, bool st, address t0, address t1) {
``` 



File:SwapPair.sol#L128
```solidity
127:    function metadata() external view returns (uint dec0, uint dec1, uint r0, uint r1, bool st, address t0, address t1) {
``` 



File:SwapPair.sol#L265
```solidity
264:    function quote(address tokenIn, uint amountIn, uint granularity) external view returns (uint amountOut) {
``` 



File:Voter.sol#L247
```solidity
246:    function createSwapGauge(address _pair) external returns (address gauge) {
``` 



 --- 

<a name=[NonCritical-7]></a>
### [NonCritical-7] Consider marking public function External - Instances: 3 

 > If a public function is never called internally. It is best practice to mark it as external. 

 --- 

File:Voter.sol#L235
```solidity
234:    function whitelist(address _token) public onlyAdmin {
``` 



File:Voter.sol#L107
```solidity
106:    function listing_fee() public view returns (uint) {
``` 



File:Minter.sol#L83
```solidity
82:    function setEmissions(uint _decay, uint _boost) public onlyAdmin {
``` 



 --- 

<a name=[NonCritical-8]></a>
### [NonCritical-8] Consider adding a message with require and revert statements - Instances: 53 

 > Adding a message to accompany require statements will provide more context when a transaction fails. 

 --- 

File:Voter.sol#L88
```solidity
87:        require(_unlocked == 1);
``` 



File:Voter.sol#L98
```solidity
97:        require(msg.sender == minter);
``` 



File:Voter.sol#L140
```solidity
139:        require(IVotingEscrow(_ve).isApprovedOrOwner(msg.sender, _tokenId));
``` 



File:Voter.sol#L202
```solidity
201:                require(votes[_tokenId][_gauge] == 0);
``` 



File:Voter.sol#L203
```solidity
202:                require(_gaugeWeight != 0);
``` 



File:Voter.sol#L229
```solidity
228:        require(IVotingEscrow(_ve).isApprovedOrOwner(msg.sender, tokenId));
``` 



File:Voter.sol#L230
```solidity
229:        require(_gaugeVote.length == _weights.length);
``` 



File:Voter.sol#L240
```solidity
239:        require(!isWhitelisted[_token]);
``` 



File:Voter.sol#L272
```solidity
271:        require(isGauge[msg.sender]);
``` 



File:Voter.sol#L278
```solidity
277:        require(isGauge[msg.sender]);
``` 



File:Voter.sol#L283
```solidity
282:        require(isGauge[msg.sender]);
``` 



File:Voter.sol#L289
```solidity
288:        require(isGauge[msg.sender]);
``` 



File:Voter.sol#L362
```solidity
361:        require(IVotingEscrow(_ve).isApprovedOrOwner(msg.sender, _tokenId));
``` 



File:Voter.sol#L373
```solidity
372:        require(IVotingEscrow(_ve).isApprovedOrOwner(msg.sender, _tokenId));
``` 



File:Voter.sol#L434
```solidity
433:        require(token.code.length > 0);
``` 



File:Voter.sol#L437
```solidity
436:        require(success && (data.length == 0 || abi.decode(data, (bool))));
``` 



File:SwapFactory.sol#L48
```solidity
47:        require(msg.sender == pauser);
``` 



File:SwapFactory.sol#L53
```solidity
52:        require(msg.sender == pendingPauser);
``` 



File:SwapFactory.sol#L58
```solidity
57:        require(msg.sender == pauser);
``` 



File:Gauge.sol#L101
```solidity
100:        require(_unlocked == 1);
``` 



File:Gauge.sol#L294
```solidity
293:        require(msg.sender == account || msg.sender == voter);
``` 



File:Gauge.sol#L458
```solidity
457:        require(amount > 0);
``` 



File:Gauge.sol#L466
```solidity
465:            require(IVotingEscrow(_ve).ownerOf(tokenId) == msg.sender);
``` 



File:Gauge.sol#L471
```solidity
470:            require(tokenIds[msg.sender] == tokenId);
``` 



File:Gauge.sol#L509
```solidity
508:            require(tokenId == tokenIds[msg.sender]);
``` 



File:Gauge.sol#L536
```solidity
535:        require(token != stake);
``` 



File:Gauge.sol#L537
```solidity
536:        require(amount > 0);
``` 



File:Gauge.sol#L554
```solidity
553:            require(amount > _left);
``` 



File:Gauge.sol#L558
```solidity
557:        require(rewardRate[token] > 0);
``` 



File:Gauge.sol#L567
```solidity
566:        require(token.code.length > 0);
``` 



File:Gauge.sol#L570
```solidity
569:        require(success && (data.length == 0 || abi.decode(data, (bool))));
``` 



File:Gauge.sol#L574
```solidity
573:        require(token.code.length > 0);
``` 



File:Gauge.sol#L577
```solidity
576:        require(success && (data.length == 0 || abi.decode(data, (bool))));
``` 



File:Gauge.sol#L581
```solidity
580:        require(token.code.length > 0);
``` 



File:Gauge.sol#L584
```solidity
583:        require(success && (data.length == 0 || abi.decode(data, (bool))));
``` 



File:Bribe.sol#L79
```solidity
78:        require(_unlocked == 1);
``` 



File:Bribe.sol#L234
```solidity
233:        require(IVotingEscrow(_ve).isApprovedOrOwner(msg.sender, tokenId));
``` 



File:Bribe.sol#L249
```solidity
248:        require(msg.sender == voter);
``` 



File:Bribe.sol#L391
```solidity
390:        require(msg.sender == voter);
``` 



File:Bribe.sol#L404
```solidity
403:        require(msg.sender == voter);
``` 



File:Bribe.sol#L424
```solidity
423:        require(amount > 0);
``` 



File:Bribe.sol#L441
```solidity
440:            require(amount > _left);
``` 



File:Bribe.sol#L445
```solidity
444:        require(rewardRate[token] > 0);
``` 



File:Bribe.sol#L454
```solidity
453:        require(token.code.length > 0);
``` 



File:Bribe.sol#L457
```solidity
456:        require(success && (data.length == 0 || abi.decode(data, (bool))));
``` 



File:Bribe.sol#L461
```solidity
460:        require(token.code.length > 0);
``` 



File:Bribe.sol#L464
```solidity
463:        require(success && (data.length == 0 || abi.decode(data, (bool))));
``` 



File:SwapPair.sol#L114
```solidity
113:        require(_unlocked == 1);
``` 



File:SwapPair.sol#L346
```solidity
345:        require(!SwapFactory(factory).isPaused());
``` 



File:SwapPair.sol#L538
```solidity
537:        require(token.code.length > 0);
``` 



File:SwapPair.sol#L541
```solidity
540:        require(success && (data.length == 0 || abi.decode(data, (bool))));
``` 



File:Minter.sol#L35
```solidity
34:        require(msg.sender == admin);
``` 



File:Minter.sol#L71
```solidity
70:        require(initializer == msg.sender);
``` 



 --- 

<a name=[NonCritical-9]></a>
### [NonCritical-9] Storage variables should not have implicit visibility - Instances: 1 

 > Consider explicitly specifying the visibility of storage variables for readability 

 --- 

File:SwapPair.sol#L48
```solidity
47:    uint constant periodSize = 1800;
``` 



 --- 

<a name=[NonCritical-10]></a>
### [NonCritical-10] This variables default value is the same as the value it is initialized with - Instances: 3 

 > This is unnecessary and will have some overhead on Gas 

 --- 

File:SwapPair.sol#L22
```solidity
21:    uint public totalSupply = 0;
``` 



File:SwapPair.sol#L64
```solidity
63:    uint public index0 = 0;
``` 



File:SwapPair.sol#L65
```solidity
64:    uint public index1 = 0;
``` 



 --- 

<a name=[NonCritical-11]></a>
### [NonCritical-11] Large contracts with many external functions should inherit an interface - Instances: 2 

 > Consider inheriting the interface to ensure the interface matches the contract spec 

 --- 

File:SwapPair.sol#L13
```solidity
12:contract SwapPair {
``` 



File:Voter.sol#L15
```solidity
14:contract Voter {
``` 



 --- 

<a name=[NonCritical-12]></a>
### [NonCritical-12] Function parameters should be in camelCase - Instances: 264 

 > Ensure that function parameters are declared using camelCase 

 --- 

File:SwapFactory.sol#L47
```solidity
46:    function setPauser(address _pauser) external {
``` 



File:SwapFactory.sol#L57
```solidity
56:    function setPause(bool _state) external {
``` 



File:SwapFactory.sol#L62
```solidity
61:    function setFeeTier(bool _stable, uint _fee) external {
``` 



File:SwapFactory.sol#L62
```solidity
61:    function setFeeTier(bool _stable, uint _fee) external {
``` 



File:SwapFactory.sol#L67
```solidity
66:    function setAdmin(address _admin) external {
``` 



File:SwapFactory.sol#L80
```solidity
79:    function createPair(address tokenA, address tokenB, bool stable) external returns (address pair) {
``` 



File:SwapFactory.sol#L82
```solidity
81:        (address token0, address token1) = tokenA < tokenB ? (tokenA, tokenB) : (tokenB, tokenA);
``` 



File:SwapFactory.sol#L82
```solidity
81:        (address token0, address token1) = tokenA < tokenB ? (tokenA, tokenB) : (tokenB, tokenA);
``` 



File:SwapFactory.sol#L80
```solidity
79:    function createPair(address tokenA, address tokenB, bool stable) external returns (address pair) {
``` 



File:Voter.sol#L68
```solidity
67:    constructor(address __ve, address _factory, address  _gauges, address _bribes) {
``` 



File:Voter.sol#L68
```solidity
67:    constructor(address __ve, address _factory, address  _gauges, address _bribes) {
``` 



File:Voter.sol#L68
```solidity
67:    constructor(address __ve, address _factory, address  _gauges, address _bribes) {
``` 



File:Voter.sol#L68
```solidity
67:    constructor(address __ve, address _factory, address  _gauges, address _bribes) {
``` 



File:Voter.sol#L97
```solidity
96:    function initialize(/*address[] memory _tokens*/address _minter) external {
``` 



File:Voter.sol#L111
```solidity
110:    function setAdmin(address _admin) external onlyAdmin {
``` 



File:Voter.sol#L116
```solidity
115:    function setReward(address _gauge, address _token, bool _status) external onlyAdmin {
``` 



File:Voter.sol#L116
```solidity
115:    function setReward(address _gauge, address _token, bool _status) external onlyAdmin {
``` 



File:Voter.sol#L116
```solidity
115:    function setReward(address _gauge, address _token, bool _status) external onlyAdmin {
``` 



File:Voter.sol#L120
```solidity
119:    function setBribe(address _bribe, address _token, bool _status) external onlyAdmin {
``` 



File:Voter.sol#L120
```solidity
119:    function setBribe(address _bribe, address _token, bool _status) external onlyAdmin {
``` 



File:Voter.sol#L120
```solidity
119:    function setBribe(address _bribe, address _token, bool _status) external onlyAdmin {
``` 



File:Voter.sol#L124
```solidity
123:    function killGauge(address _gauge) external onlyAdmin {
``` 



File:Voter.sol#L131
```solidity
130:    function reviveGauge(address _gauge) external onlyAdmin {
``` 



File:Voter.sol#L186
```solidity
185:    function _vote(uint _tokenId, address[] memory _gaugeVote, uint256[] memory _weights) internal {
``` 



File:Voter.sol#L225
```solidity
224:    function vote(uint tokenId, address[] calldata _gaugeVote, uint256[] calldata _weights) external {
``` 



File:Voter.sol#L235
```solidity
234:    function whitelist(address _token) public onlyAdmin {
``` 



File:Voter.sol#L239
```solidity
238:    function _whitelist(address _token) internal {
``` 



File:Voter.sol#L247
```solidity
246:    function createSwapGauge(address _pair) external returns (address gauge) {
``` 



File:Voter.sol#L247
```solidity
246:    function createSwapGauge(address _pair) external returns (address gauge) {
``` 



File:Voter.sol#L271
```solidity
270:    function attachTokenToGauge(uint tokenId, address account) external {
``` 



File:Voter.sol#L277
```solidity
276:    function emitDeposit(uint tokenId, address account, uint amount) external {
``` 



File:Voter.sol#L277
```solidity
276:    function emitDeposit(uint tokenId, address account, uint amount) external {
``` 



File:Voter.sol#L282
```solidity
281:    function detachTokenFromGauge(uint tokenId, address account) external {
``` 



File:Voter.sol#L288
```solidity
287:    function emitWithdraw(uint tokenId, address account, uint amount) external {
``` 



File:Voter.sol#L288
```solidity
287:    function emitWithdraw(uint tokenId, address account, uint amount) external {
``` 



File:Voter.sol#L299
```solidity
298:    function notifyRewardAmount(uint _amount) external {
``` 



File:Voter.sol#L308
```solidity
307:    function updateFor(address[] memory _gauges) external {
``` 



File:Voter.sol#L314
```solidity
313:    function updateForRange(uint start, uint end) public {
``` 



File:Voter.sol#L314
```solidity
313:    function updateForRange(uint start, uint end) public {
``` 



File:Voter.sol#L326
```solidity
325:    function updateGauge(address _gauge) external {
``` 



File:Voter.sol#L330
```solidity
329:    function _updateFor(address _gauge) internal {
``` 



File:Voter.sol#L351
```solidity
350:    function claimRewards(address[] memory _gauges, address[][] memory _tokens) external {
``` 



File:Voter.sol#L351
```solidity
350:    function claimRewards(address[] memory _gauges, address[][] memory _tokens) external {
``` 



File:Voter.sol#L361
```solidity
360:    function claimBribes(address[] memory _bribes, address[][] memory _tokens, uint _tokenId) external {
``` 



File:Voter.sol#L361
```solidity
360:    function claimBribes(address[] memory _bribes, address[][] memory _tokens, uint _tokenId) external {
``` 



File:Voter.sol#L372
```solidity
371:    function claimFees(address[] memory _fees, address[][] memory _tokens, uint _tokenId) external {
``` 



File:Voter.sol#L372
```solidity
371:    function claimFees(address[] memory _fees, address[][] memory _tokens, uint _tokenId) external {
``` 



File:Voter.sol#L381
```solidity
380:    function distributeFees(address[] memory _gauges) external {
``` 



File:Voter.sol#L389
```solidity
388:    function distribute(address _gauge) public lock {
``` 



File:Voter.sol#L408
```solidity
407:    function distribute(uint start, uint finish) public {
``` 



File:Voter.sol#L408
```solidity
407:    function distribute(uint start, uint finish) public {
``` 



File:Voter.sol#L414
```solidity
413:    function distribute(address[] memory _gauges) external {
``` 



File:Voter.sol#L433
```solidity
432:    function _safeTransferFrom(address token, address from, address to, uint256 value) internal {
``` 



File:Voter.sol#L433
```solidity
432:    function _safeTransferFrom(address token, address from, address to, uint256 value) internal {
``` 



File:Voter.sol#L433
```solidity
432:    function _safeTransferFrom(address token, address from, address to, uint256 value) internal {
``` 



File:Voter.sol#L433
```solidity
432:    function _safeTransferFrom(address token, address from, address to, uint256 value) internal {
``` 



File:Voter.sol#L435
```solidity
434:        (bool success, bytes memory data) =
``` 



File:Voter.sol#L435
```solidity
434:        (bool success, bytes memory data) =
``` 



File:Minter.sol#L40
```solidity
39:        address __voter, // the voting & distribution system
``` 



File:Minter.sol#L41
```solidity
40:        address  __ve, // the ve(3,3) system that will be locked into
``` 



File:Minter.sol#L42
```solidity
41:        address __ve_dist, // the distribution system that ensures users aren't diluted
``` 



File:Minter.sol#L43
```solidity
42:        address _admin,
``` 



File:Minter.sol#L44
```solidity
43:        uint _weekly
44:    ) {
``` 



File:Minter.sol#L83
```solidity
82:    function setEmissions(uint _decay, uint _boost) public onlyAdmin {
``` 



File:Minter.sol#L83
```solidity
82:    function setEmissions(uint _decay, uint _boost) public onlyAdmin {
``` 



File:Minter.sol#L117
```solidity
116:    function calculate_growth(uint _minted) public view returns (uint) {
``` 



File:Gauge.sol#L84
```solidity
83:    constructor(address _stake, address _bribe, address  __ve, address _voter) {
``` 



File:Gauge.sol#L84
```solidity
83:    constructor(address _stake, address _bribe, address  __ve, address _voter) {
``` 



File:Gauge.sol#L84
```solidity
83:    constructor(address _stake, address _bribe, address  __ve, address _voter) {
``` 



File:Gauge.sol#L84
```solidity
83:    constructor(address _stake, address _bribe, address  __ve, address _voter) {
``` 



File:Gauge.sol#L110
```solidity
109:    function claimFees() external lock returns (uint claimed0, uint claimed1) {
``` 



File:Gauge.sol#L110
```solidity
109:    function claimFees() external lock returns (uint claimed0, uint claimed1) {
``` 



File:Gauge.sol#L119
```solidity
118:            (address _token0, address _token1) = ISwapPair(stake).tokens();
``` 



File:Gauge.sol#L119
```solidity
118:            (address _token0, address _token1) = ISwapPair(stake).tokens();
``` 



File:Gauge.sol#L114
```solidity
113:    function _claimFees() internal returns (uint claimed0, uint claimed1) {
``` 



File:Gauge.sol#L114
```solidity
113:    function _claimFees() internal returns (uint claimed0, uint claimed1) {
``` 



File:Gauge.sol#L146
```solidity
145:    function getPriorBalanceIndex(address account, uint timestamp) public view returns (uint) {
``` 



File:Gauge.sol#L146
```solidity
145:    function getPriorBalanceIndex(address account, uint timestamp) public view returns (uint) {
``` 



File:Gauge.sol#L178
```solidity
177:    function getPriorSupplyIndex(uint timestamp) public view returns (uint) {
``` 



File:Gauge.sol#L210
```solidity
209:    function getPriorRewardPerToken(address token, uint timestamp) public view returns (uint, uint) {
``` 



File:Gauge.sol#L210
```solidity
209:    function getPriorRewardPerToken(address token, uint timestamp) public view returns (uint, uint) {
``` 



File:Gauge.sol#L242
```solidity
241:    function _writeCheckpoint(address account, uint balance) internal {
``` 



File:Gauge.sol#L242
```solidity
241:    function _writeCheckpoint(address account, uint balance) internal {
``` 



File:Gauge.sol#L254
```solidity
253:    function _writeRewardPerTokenCheckpoint(address token, uint reward, uint timestamp) internal {
``` 



File:Gauge.sol#L254
```solidity
253:    function _writeRewardPerTokenCheckpoint(address token, uint reward, uint timestamp) internal {
``` 



File:Gauge.sol#L254
```solidity
253:    function _writeRewardPerTokenCheckpoint(address token, uint reward, uint timestamp) internal {
``` 



File:Gauge.sol#L284
```solidity
283:    function lastTimeRewardApplicable(address token) public view returns (uint) {
``` 



File:Gauge.sol#L293
```solidity
292:    function getReward(address account, address[] memory tokens) external lock {
``` 



File:Gauge.sol#L293
```solidity
292:    function getReward(address account, address[] memory tokens) external lock {
``` 



File:Gauge.sol#L321
```solidity
320:    function rewardPerToken(address token) public view returns (uint) {
``` 



File:Gauge.sol#L328
```solidity
327:    function derivedBalance(address account) public view returns (uint) {
``` 



File:Gauge.sol#L332
```solidity
331:    function batchRewardPerToken(address token, uint maxRuns) external {
``` 



File:Gauge.sol#L336
```solidity
335:    function _batchRewardPerToken(address token, uint maxRuns) internal returns (uint, uint) {
``` 



File:Gauge.sol#L355
```solidity
354:                (uint _reward, uint _endTime) = _calcRewardPerToken(token, sp1.timestamp, sp0.timestamp, sp0.supply, _startTimestamp);
``` 



File:Gauge.sol#L365
```solidity
364:    function _calcRewardPerToken(address token, uint timestamp1, uint timestamp0, uint supply, uint startTimestamp) internal view returns (uint, uint) {
``` 



File:Gauge.sol#L365
```solidity
364:    function _calcRewardPerToken(address token, uint timestamp1, uint timestamp0, uint supply, uint startTimestamp) internal view returns (uint, uint) {
``` 



File:Gauge.sol#L365
```solidity
364:    function _calcRewardPerToken(address token, uint timestamp1, uint timestamp0, uint supply, uint startTimestamp) internal view returns (uint, uint) {
``` 



File:Gauge.sol#L365
```solidity
364:    function _calcRewardPerToken(address token, uint timestamp1, uint timestamp0, uint supply, uint startTimestamp) internal view returns (uint, uint) {
``` 



File:Gauge.sol#L372
```solidity
371:    function batchUpdateRewardPerToken(address token, uint maxRuns) external {
``` 



File:Gauge.sol#L384
```solidity
383:    function _updateRewardPerToken(address token, uint maxRuns, bool actualLast) internal returns (uint, uint) {
``` 



File:Gauge.sol#L404
```solidity
403:                    (uint _reward, uint _endTime) = _calcRewardPerToken(token, sp1.timestamp, sp0.timestamp, sp0.supply, _startTimestamp);
``` 



File:Gauge.sol#L414
```solidity
413:                (uint _reward,) = _calcRewardPerToken(token, lastTimeRewardApplicable(token), Math.max(sp.timestamp, _startTimestamp), sp.supply, _startTimestamp);
``` 



File:Gauge.sol#L425
```solidity
424:    function earned(address token, address account) public view returns (uint) {
``` 



File:Gauge.sol#L425
```solidity
424:    function earned(address token, address account) public view returns (uint) {
``` 



File:Gauge.sol#L457
```solidity
456:    function deposit(uint amount, uint tokenId) public lock {
``` 



File:Gauge.sol#L493
```solidity
492:    function withdraw(uint amount) public {
``` 



File:Gauge.sol#L501
```solidity
500:    function withdrawToken(uint amount, uint tokenId) public lock {
``` 



File:Gauge.sol#L529
```solidity
528:    function left(address token) external view returns (uint) {
``` 



File:Gauge.sol#L535
```solidity
534:    function notifyRewardAmount(address token, uint amount) external lock {
``` 



File:Gauge.sol#L535
```solidity
534:    function notifyRewardAmount(address token, uint amount) external lock {
``` 



File:Gauge.sol#L566
```solidity
565:    function _safeTransfer(address token, address to, uint256 value) internal {
``` 



File:Gauge.sol#L566
```solidity
565:    function _safeTransfer(address token, address to, uint256 value) internal {
``` 



File:Gauge.sol#L566
```solidity
565:    function _safeTransfer(address token, address to, uint256 value) internal {
``` 



File:Gauge.sol#L568
```solidity
567:        (bool success, bytes memory data) =
``` 



File:Gauge.sol#L568
```solidity
567:        (bool success, bytes memory data) =
``` 



File:Gauge.sol#L573
```solidity
572:    function _safeTransferFrom(address token, address from, address to, uint256 value) internal {
``` 



File:Gauge.sol#L573
```solidity
572:    function _safeTransferFrom(address token, address from, address to, uint256 value) internal {
``` 



File:Gauge.sol#L573
```solidity
572:    function _safeTransferFrom(address token, address from, address to, uint256 value) internal {
``` 



File:Gauge.sol#L573
```solidity
572:    function _safeTransferFrom(address token, address from, address to, uint256 value) internal {
``` 



File:Gauge.sol#L575
```solidity
574:        (bool success, bytes memory data) =
``` 



File:Gauge.sol#L575
```solidity
574:        (bool success, bytes memory data) =
``` 



File:Gauge.sol#L580
```solidity
579:    function _safeApprove(address token, address spender, uint256 value) internal {
``` 



File:Gauge.sol#L580
```solidity
579:    function _safeApprove(address token, address spender, uint256 value) internal {
``` 



File:Gauge.sol#L580
```solidity
579:    function _safeApprove(address token, address spender, uint256 value) internal {
``` 



File:Gauge.sol#L582
```solidity
581:        (bool success, bytes memory data) =
``` 



File:Gauge.sol#L582
```solidity
581:        (bool success, bytes memory data) =
``` 



File:Bribe.sol#L70
```solidity
69:    constructor(address _voter) {
``` 



File:Bribe.sol#L92
```solidity
91:    function getPriorBalanceIndex(uint tokenId, uint timestamp) public view returns (uint) {
``` 



File:Bribe.sol#L124
```solidity
123:    function getPriorSupplyIndex(uint timestamp) public view returns (uint) {
``` 



File:Bribe.sol#L156
```solidity
155:    function getPriorRewardPerToken(address token, uint timestamp) public view returns (uint, uint) {
``` 



File:Bribe.sol#L156
```solidity
155:    function getPriorRewardPerToken(address token, uint timestamp) public view returns (uint, uint) {
``` 



File:Bribe.sol#L188
```solidity
187:    function _writeCheckpoint(uint tokenId, uint balance) internal {
``` 



File:Bribe.sol#L200
```solidity
199:    function _writeRewardPerTokenCheckpoint(address token, uint reward, uint timestamp) internal {
``` 



File:Bribe.sol#L200
```solidity
199:    function _writeRewardPerTokenCheckpoint(address token, uint reward, uint timestamp) internal {
``` 



File:Bribe.sol#L200
```solidity
199:    function _writeRewardPerTokenCheckpoint(address token, uint reward, uint timestamp) internal {
``` 



File:Bribe.sol#L228
```solidity
227:    function lastTimeRewardApplicable(address token) public view returns (uint) {
``` 



File:Bribe.sol#L233
```solidity
232:    function getReward(uint tokenId, address[] memory tokens) external lock  {
``` 



File:Bribe.sol#L248
```solidity
247:    function getRewardForOwner(uint tokenId, address[] memory tokens) external lock  {
``` 



File:Bribe.sol#L263
```solidity
262:    function rewardPerToken(address token) public view returns (uint) {
``` 



File:Bribe.sol#L270
```solidity
269:    function batchRewardPerToken(address token, uint maxRuns) external {
``` 



File:Bribe.sol#L274
```solidity
273:    function _batchRewardPerToken(address token, uint maxRuns) internal returns (uint, uint) {
``` 



File:Bribe.sol#L293
```solidity
292:                (uint _reward, uint endTime) = _calcRewardPerToken(token, sp1.timestamp, sp0.timestamp, sp0.supply, _startTimestamp);
``` 



File:Bribe.sol#L303
```solidity
302:    function _calcRewardPerToken(address token, uint timestamp1, uint timestamp0, uint supply, uint startTimestamp) internal view returns (uint, uint) {
``` 



File:Bribe.sol#L303
```solidity
302:    function _calcRewardPerToken(address token, uint timestamp1, uint timestamp0, uint supply, uint startTimestamp) internal view returns (uint, uint) {
``` 



File:Bribe.sol#L303
```solidity
302:    function _calcRewardPerToken(address token, uint timestamp1, uint timestamp0, uint supply, uint startTimestamp) internal view returns (uint, uint) {
``` 



File:Bribe.sol#L303
```solidity
302:    function _calcRewardPerToken(address token, uint timestamp1, uint timestamp0, uint supply, uint startTimestamp) internal view returns (uint, uint) {
``` 



File:Bribe.sol#L308
```solidity
307:    function batchUpdateRewardPerToken(address token, uint maxRuns) external {
``` 



File:Bribe.sol#L320
```solidity
319:    function _updateRewardPerToken(address token, uint maxRuns, bool actualLast) internal returns (uint, uint) {
``` 



File:Bribe.sol#L340
```solidity
339:                    (uint _reward, uint _endTime) = _calcRewardPerToken(token, sp1.timestamp, sp0.timestamp, sp0.supply, _startTimestamp);
``` 



File:Bribe.sol#L351
```solidity
350:                (uint _reward,) = _calcRewardPerToken(token, lastTimeRewardApplicable(token), Math.max(sp.timestamp, _startTimestamp), sp.supply, _startTimestamp);
``` 



File:Bribe.sol#L361
```solidity
360:    function earned(address token, uint tokenId) public view returns (uint) {
``` 



File:Bribe.sol#L390
```solidity
389:    function _deposit(uint amount, uint tokenId) external {
``` 



File:Bribe.sol#L403
```solidity
402:    function _withdraw(uint amount, uint tokenId) external {
``` 



File:Bribe.sol#L416
```solidity
415:    function left(address token) external view returns (uint) {
``` 



File:Bribe.sol#L423
```solidity
422:    function notifyRewardAmount(address token, uint amount) external lock {
``` 



File:Bribe.sol#L423
```solidity
422:    function notifyRewardAmount(address token, uint amount) external lock {
``` 



File:Bribe.sol#L453
```solidity
452:    function _safeTransfer(address token, address to, uint256 value) internal {
``` 



File:Bribe.sol#L453
```solidity
452:    function _safeTransfer(address token, address to, uint256 value) internal {
``` 



File:Bribe.sol#L453
```solidity
452:    function _safeTransfer(address token, address to, uint256 value) internal {
``` 



File:Bribe.sol#L455
```solidity
454:        (bool success, bytes memory data) =
``` 



File:Bribe.sol#L455
```solidity
454:        (bool success, bytes memory data) =
``` 



File:Bribe.sol#L460
```solidity
459:    function _safeTransferFrom(address token, address from, address to, uint256 value) internal {
``` 



File:Bribe.sol#L460
```solidity
459:    function _safeTransferFrom(address token, address from, address to, uint256 value) internal {
``` 



File:Bribe.sol#L460
```solidity
459:    function _safeTransferFrom(address token, address from, address to, uint256 value) internal {
``` 



File:Bribe.sol#L460
```solidity
459:    function _safeTransferFrom(address token, address from, address to, uint256 value) internal {
``` 



File:Bribe.sol#L462
```solidity
461:        (bool success, bytes memory data) =
``` 



File:Bribe.sol#L462
```solidity
461:        (bool success, bytes memory data) =
``` 



File:Multiswap.sol#L11
```solidity
10:    constructor(address _router) {
``` 



File:Multiswap.sol#L30
```solidity
29:        address _token,
``` 



File:Multiswap.sol#L31
```solidity
30:        uint _amount,
``` 



File:Multiswap.sol#L33
```solidity
32:        uint[] calldata _weights
33:    ) external payable returns (uint[] memory) {
``` 



File:Multiswap.sol#L47
```solidity
46:                (bool success, bytes memory data) = router.call{value: amount_}(_swapData[i]);
``` 



File:Multiswap.sol#L47
```solidity
46:                (bool success, bytes memory data) = router.call{value: amount_}(_swapData[i]);
``` 



File:Multiswap.sol#L59
```solidity
58:                (bool callSuccess, bytes memory data) = router.call(_swapData[i]);
``` 



File:Multiswap.sol#L71
```solidity
70:    function _assertWeights(uint[] calldata _weights) internal pure returns (bool) {
``` 



File:SwapPair.sol#L94
```solidity
93:        (address _token0, address _token1, bool _stable, uint _fee) = SwapFactory(msg.sender).getInitializable();
``` 



File:SwapPair.sol#L94
```solidity
93:        (address _token0, address _token1, bool _stable, uint _fee) = SwapFactory(msg.sender).getInitializable();
``` 



File:SwapPair.sol#L94
```solidity
93:        (address _token0, address _token1, bool _stable, uint _fee) = SwapFactory(msg.sender).getInitializable();
``` 



File:SwapPair.sol#L94
```solidity
93:        (address _token0, address _token1, bool _stable, uint _fee) = SwapFactory(msg.sender).getInitializable();
``` 



File:SwapPair.sol#L128
```solidity
127:    function metadata() external view returns (uint dec0, uint dec1, uint r0, uint r1, bool st, address t0, address t1) {
``` 



File:SwapPair.sol#L128
```solidity
127:    function metadata() external view returns (uint dec0, uint dec1, uint r0, uint r1, bool st, address t0, address t1) {
``` 



File:SwapPair.sol#L128
```solidity
127:    function metadata() external view returns (uint dec0, uint dec1, uint r0, uint r1, bool st, address t0, address t1) {
``` 



File:SwapPair.sol#L128
```solidity
127:    function metadata() external view returns (uint dec0, uint dec1, uint r0, uint r1, bool st, address t0, address t1) {
``` 



File:SwapPair.sol#L128
```solidity
127:    function metadata() external view returns (uint dec0, uint dec1, uint r0, uint r1, bool st, address t0, address t1) {
``` 



File:SwapPair.sol#L128
```solidity
127:    function metadata() external view returns (uint dec0, uint dec1, uint r0, uint r1, bool st, address t0, address t1) {
``` 



File:SwapPair.sol#L128
```solidity
127:    function metadata() external view returns (uint dec0, uint dec1, uint r0, uint r1, bool st, address t0, address t1) {
``` 



File:SwapPair.sol#L137
```solidity
136:    function claimFees() external returns (uint claimed0, uint claimed1) {
``` 



File:SwapPair.sol#L137
```solidity
136:    function claimFees() external returns (uint claimed0, uint claimed1) {
``` 



File:SwapPair.sol#L154
```solidity
153:    function _update0(uint amount) internal {
``` 



File:SwapPair.sol#L168
```solidity
167:    function _update1(uint amount) internal {
``` 



File:SwapPair.sol#L183
```solidity
182:    function _updateFor(address recipient) internal {
``` 



File:SwapPair.sol#L208
```solidity
207:    function getReserves() public view returns (uint _reserve0, uint _reserve1, uint _blockTimestampLast) {
``` 



File:SwapPair.sol#L208
```solidity
207:    function getReserves() public view returns (uint _reserve0, uint _reserve1, uint _blockTimestampLast) {
``` 



File:SwapPair.sol#L215
```solidity
214:    function _update(uint balance0, uint balance1, uint _reserve0, uint _reserve1) internal {
``` 



File:SwapPair.sol#L215
```solidity
214:    function _update(uint balance0, uint balance1, uint _reserve0, uint _reserve1) internal {
``` 



File:SwapPair.sol#L215
```solidity
214:    function _update(uint balance0, uint balance1, uint _reserve0, uint _reserve1) internal {
``` 



File:SwapPair.sol#L215
```solidity
214:    function _update(uint balance0, uint balance1, uint _reserve0, uint _reserve1) internal {
``` 



File:SwapPair.sol#L241
```solidity
240:        (uint _reserve0, uint _reserve1, uint _blockTimestampLast) = getReserves();
``` 



File:SwapPair.sol#L241
```solidity
240:        (uint _reserve0, uint _reserve1, uint _blockTimestampLast) = getReserves();
``` 



File:SwapPair.sol#L265
```solidity
264:    function quote(address tokenIn, uint amountIn, uint granularity) external view returns (uint amountOut) {
``` 



File:SwapPair.sol#L275
```solidity
274:    function prices(address tokenIn, uint amountIn, uint points) external view returns (uint[] memory) {
``` 



File:SwapPair.sol#L279
```solidity
278:    function sample(address tokenIn, uint amountIn, uint points, uint window) public view returns (uint[] memory) {
``` 



File:SwapPair.sol#L279
```solidity
278:    function sample(address tokenIn, uint amountIn, uint points, uint window) public view returns (uint[] memory) {
``` 



File:SwapPair.sol#L300
```solidity
299:    function mint(address to) external lock returns (uint liquidity) {
``` 



File:SwapPair.sol#L301
```solidity
300:        (uint _reserve0, uint _reserve1) = (reserve0, reserve1);
``` 



File:SwapPair.sol#L301
```solidity
300:        (uint _reserve0, uint _reserve1) = (reserve0, reserve1);
``` 



File:SwapPair.sol#L300
```solidity
299:    function mint(address to) external lock returns (uint liquidity) {
``` 



File:SwapPair.sol#L323
```solidity
322:    function burn(address to) external lock returns (uint amount0, uint amount1) {
``` 



File:SwapPair.sol#L324
```solidity
323:        (uint _reserve0, uint _reserve1) = (reserve0, reserve1);
``` 



File:SwapPair.sol#L324
```solidity
323:        (uint _reserve0, uint _reserve1) = (reserve0, reserve1);
``` 



File:SwapPair.sol#L325
```solidity
324:        (address _token0, address _token1) = (token0, token1);
``` 



File:SwapPair.sol#L325
```solidity
324:        (address _token0, address _token1) = (token0, token1);
``` 



File:SwapPair.sol#L323
```solidity
322:    function burn(address to) external lock returns (uint amount0, uint amount1) {
``` 



File:SwapPair.sol#L323
```solidity
322:    function burn(address to) external lock returns (uint amount0, uint amount1) {
``` 



File:SwapPair.sol#L345
```solidity
344:    function swap(uint amount0Out, uint amount1Out, address to, bytes calldata data) external lock {
``` 



File:SwapPair.sol#L345
```solidity
344:    function swap(uint amount0Out, uint amount1Out, address to, bytes calldata data) external lock {
``` 



File:SwapPair.sol#L348
```solidity
347:        (uint _reserve0, uint _reserve1) =  (reserve0, reserve1);
``` 



File:SwapPair.sol#L348
```solidity
347:        (uint _reserve0, uint _reserve1) =  (reserve0, reserve1);
``` 



File:SwapPair.sol#L354
```solidity
353:        (address _token0, address _token1) = (token0, token1);
``` 



File:SwapPair.sol#L354
```solidity
353:        (address _token0, address _token1) = (token0, token1);
``` 



File:SwapPair.sol#L366
```solidity
365:        (address _token0, address _token1) = (token0, token1);
``` 



File:SwapPair.sol#L366
```solidity
365:        (address _token0, address _token1) = (token0, token1);
``` 



File:SwapPair.sol#L380
```solidity
379:    function skim(address to) external lock {
``` 



File:SwapPair.sol#L381
```solidity
380:        (address _token0, address _token1) = (token0, token1);
``` 



File:SwapPair.sol#L381
```solidity
380:        (address _token0, address _token1) = (token0, token1);
``` 



File:SwapPair.sol#L391
```solidity
390:    function _f(uint x0, uint y) internal pure returns (uint) {
``` 



File:SwapPair.sol#L391
```solidity
390:    function _f(uint x0, uint y) internal pure returns (uint) {
``` 



File:SwapPair.sol#L395
```solidity
394:    function _d(uint x0, uint y) internal pure returns (uint) {
``` 



File:SwapPair.sol#L395
```solidity
394:    function _d(uint x0, uint y) internal pure returns (uint) {
``` 



File:SwapPair.sol#L399
```solidity
398:    function _get_y(uint x0, uint xy, uint y) internal pure returns (uint) {
``` 



File:SwapPair.sol#L399
```solidity
398:    function _get_y(uint x0, uint xy, uint y) internal pure returns (uint) {
``` 



File:SwapPair.sol#L399
```solidity
398:    function _get_y(uint x0, uint xy, uint y) internal pure returns (uint) {
``` 



File:SwapPair.sol#L424
```solidity
423:        (uint _reserve0, uint _reserve1) = (reserve0, reserve1);
``` 



File:SwapPair.sol#L424
```solidity
423:        (uint _reserve0, uint _reserve1) = (reserve0, reserve1);
``` 



File:SwapPair.sol#L429
```solidity
428:    function _getAmountOut(uint amountIn, address tokenIn, uint _reserve0, uint _reserve1) internal view returns (uint) {
``` 



File:SwapPair.sol#L429
```solidity
428:    function _getAmountOut(uint amountIn, address tokenIn, uint _reserve0, uint _reserve1) internal view returns (uint) {
``` 



File:SwapPair.sol#L444
```solidity
443:    function _k(uint x, uint y) internal view returns (uint) {
``` 



File:SwapPair.sol#L444
```solidity
443:    function _k(uint x, uint y) internal view returns (uint) {
``` 



File:SwapPair.sol#L456
```solidity
455:    function _mint(address dst, uint amount) internal {
``` 



File:SwapPair.sol#L456
```solidity
455:    function _mint(address dst, uint amount) internal {
``` 



File:SwapPair.sol#L463
```solidity
462:    function _burn(address dst, uint amount) internal {
``` 



File:SwapPair.sol#L463
```solidity
462:    function _burn(address dst, uint amount) internal {
``` 



File:SwapPair.sol#L470
```solidity
469:    function approve(address spender, uint amount) external returns (bool) {
``` 



File:SwapPair.sol#L470
```solidity
469:    function approve(address spender, uint amount) external returns (bool) {
``` 



File:SwapPair.sol#L477
```solidity
476:    function permit(address owner, address spender, uint value, uint deadline, uint8 v, bytes32 r, bytes32 s) external {
``` 



File:SwapPair.sol#L477
```solidity
476:    function permit(address owner, address spender, uint value, uint deadline, uint8 v, bytes32 r, bytes32 s) external {
``` 



File:SwapPair.sol#L477
```solidity
476:    function permit(address owner, address spender, uint value, uint deadline, uint8 v, bytes32 r, bytes32 s) external {
``` 



File:SwapPair.sol#L477
```solidity
476:    function permit(address owner, address spender, uint value, uint deadline, uint8 v, bytes32 r, bytes32 s) external {
``` 



File:SwapPair.sol#L477
```solidity
476:    function permit(address owner, address spender, uint value, uint deadline, uint8 v, bytes32 r, bytes32 s) external {
``` 



File:SwapPair.sol#L477
```solidity
476:    function permit(address owner, address spender, uint value, uint deadline, uint8 v, bytes32 r, bytes32 s) external {
``` 



File:SwapPair.sol#L477
```solidity
476:    function permit(address owner, address spender, uint value, uint deadline, uint8 v, bytes32 r, bytes32 s) external {
``` 



File:SwapPair.sol#L507
```solidity
506:    function transfer(address dst, uint amount) external returns (bool) {
``` 



File:SwapPair.sol#L507
```solidity
506:    function transfer(address dst, uint amount) external returns (bool) {
``` 



File:SwapPair.sol#L512
```solidity
511:    function transferFrom(address src, address dst, uint amount) external returns (bool) {
``` 



File:SwapPair.sol#L512
```solidity
511:    function transferFrom(address src, address dst, uint amount) external returns (bool) {
``` 



File:SwapPair.sol#L512
```solidity
511:    function transferFrom(address src, address dst, uint amount) external returns (bool) {
``` 



File:SwapPair.sol#L527
```solidity
526:    function _transferTokens(address src, address dst, uint amount) internal {
``` 



File:SwapPair.sol#L527
```solidity
526:    function _transferTokens(address src, address dst, uint amount) internal {
``` 



File:SwapPair.sol#L527
```solidity
526:    function _transferTokens(address src, address dst, uint amount) internal {
``` 



File:SwapPair.sol#L537
```solidity
536:    function _safeTransfer(address token,address to,uint256 value) internal {
``` 



File:SwapPair.sol#L537
```solidity
536:    function _safeTransfer(address token,address to,uint256 value) internal {
``` 



File:SwapPair.sol#L537
```solidity
536:    function _safeTransfer(address token,address to,uint256 value) internal {
``` 



File:SwapPair.sol#L539
```solidity
538:        (bool success, bytes memory data) =
``` 



File:SwapPair.sol#L539
```solidity
538:        (bool success, bytes memory data) =
``` 



 --- 

<a name=[NonCritical-13]></a>
### [NonCritical-13] Require/Revert statements should be consistent across the codebase - Instances: 36 

 > Consider using require/revert statements consistently across the codebase 

 --- 

File:Gauge.sol#L23
```solidity
22:    uint internal constant DURATION = 7 days; // rewards are released over 7 days
``` 



File:Gauge.sol#L24
```solidity
23:    uint internal constant PRECISION = 10 ** 18;
``` 



File:Gauge.sol#L15
```solidity
14:    address public immutable stake; // the LP token that needs to be staked for rewards
``` 



File:Gauge.sol#L16
```solidity
15:    address public immutable _ve; // the ve token used for gauges
``` 



File:Gauge.sol#L17
```solidity
16:    address public immutable bribe;
``` 



File:Gauge.sol#L18
```solidity
17:    address public immutable voter;
``` 



File:Multiswap.sol#L9
```solidity
8:    address public immutable router;
``` 



File:Minter.sol#L14
```solidity
13:    uint internal constant week = 86400 * 7; // allows minting once per week (reset every Thursday 00:00 UTC)
``` 



File:Minter.sol#L16
```solidity
15:    uint internal constant target_base = 10000; // 2% per week target emission
``` 



File:Minter.sol#L17
```solidity
16:    uint internal constant tail_emission = 3; // 0.03% per week tail emission
``` 



File:Minter.sol#L18
```solidity
17:    uint internal constant tail_base = 1000; // 0.2% per week target emission
``` 



File:Minter.sol#L27
```solidity
26:    uint internal constant lock = 86400 * 7 * 52 * 4;
``` 



File:Minter.sol#L19
```solidity
18:    underlying public immutable _token;
``` 



File:Minter.sol#L20
```solidity
19:    IVoter public immutable _voter;
``` 



File:Minter.sol#L21
```solidity
20:    IVotingEscrow public immutable _ve;
``` 



File:Minter.sol#L22
```solidity
21:    IVotingDist public immutable _ve_dist;
``` 



File:SwapPair.sol#L17
```solidity
16:    uint8 public constant decimals = 18;
``` 



File:SwapPair.sol#L48
```solidity
47:    uint constant periodSize = 1800;
``` 



File:SwapPair.sol#L20
```solidity
19:    bool public immutable stable;
``` 



File:SwapPair.sol#L34
```solidity
33:    address public immutable token0;
``` 



File:SwapPair.sol#L35
```solidity
34:    address public immutable token1;
``` 



File:SwapPair.sol#L36
```solidity
35:    address public immutable fees;
``` 



File:SwapPair.sol#L37
```solidity
36:    address public immutable factory;
``` 



File:SwapPair.sol#L38
```solidity
37:    uint public immutable fee;
``` 



File:SwapPair.sol#L52
```solidity
51:    uint internal immutable decimals0;
``` 



File:SwapPair.sol#L53
```solidity
52:    uint internal immutable decimals1;
``` 



File:SwapFactory.sol#L17
```solidity
16:    address public immutable feeCollector;
``` 



File:Voter.sol#L23
```solidity
22:    uint internal constant DURATION = 7 days; // rewards are released over 7 days
``` 



File:Voter.sol#L17
```solidity
16:    address public immutable _ve;
``` 



File:Voter.sol#L18
```solidity
17:    address public immutable factory;
``` 



File:Voter.sol#L20
```solidity
19:    address internal immutable base;
``` 



File:Voter.sol#L22
```solidity
21:    address public immutable bribeFactory;
``` 



File:Bribe.sol#L15
```solidity
14:    uint public constant DURATION = 7 days; // rewards are released over 7 days
``` 



File:Bribe.sol#L16
```solidity
15:    uint public constant PRECISION = 10 ** 18;
``` 



File:Bribe.sol#L12
```solidity
11:    address public immutable voter; // only voter can modify balances (since it only happens on vote())
``` 



File:Bribe.sol#L13
```solidity
12:    address public immutable _ve;
``` 



 --- 

<a name=[NonCritical-14]></a>
### [NonCritical-14] Consider explicitly naming mapping parameters - Instances: 53 

 > Consider explicitly naming mapping parameters for readability 

 --- 

File:SwapPair.sol#L24
```solidity
23:    mapping(address => mapping (address => uint)) public allowance;
``` 



File:SwapPair.sol#L25
```solidity
24:    mapping(address => uint) public balanceOf;
``` 



File:SwapPair.sol#L30
```solidity
29:    mapping(address => uint) public nonces;
``` 



File:SwapPair.sol#L68
```solidity
67:    mapping(address => uint) public supplyIndex0;
``` 



File:SwapPair.sol#L69
```solidity
68:    mapping(address => uint) public supplyIndex1;
``` 



File:SwapPair.sol#L72
```solidity
71:    mapping(address => uint) public claimable0;
``` 



File:SwapPair.sol#L73
```solidity
72:    mapping(address => uint) public claimable1;
``` 



File:SwapFactory.sol#L13
```solidity
12:    mapping(address => mapping(address => mapping(bool => address))) public getPair;
``` 



File:SwapFactory.sol#L15
```solidity
14:    mapping(address => bool) public isPair; // simplified check if its a pair, given that `stable` flag might not be available in peripherals
``` 



File:SwapFactory.sol#L16
```solidity
15:    mapping(bool => uint) public fee;
``` 



File:Bribe.sol#L20
```solidity
19:    mapping(address => uint) public rewardRate;
``` 



File:Bribe.sol#L21
```solidity
20:    mapping(address => uint) public periodFinish;
``` 



File:Bribe.sol#L22
```solidity
21:    mapping(address => uint) public lastUpdateTime;
``` 



File:Bribe.sol#L23
```solidity
22:    mapping(address => uint) public rewardPerTokenStored;
``` 



File:Bribe.sol#L25
```solidity
24:    mapping(address => mapping(uint => uint)) public lastEarn;
``` 



File:Bribe.sol#L26
```solidity
25:    mapping(address => mapping(uint => uint)) public userRewardPerTokenStored;
``` 



File:Bribe.sol#L29
```solidity
28:    mapping(address => bool) public isReward;
``` 



File:Bribe.sol#L32
```solidity
31:    mapping(uint => uint) public balanceOf;
``` 



File:Bribe.sol#L53
```solidity
52:    mapping (uint => mapping (uint => Checkpoint)) public checkpoints;
``` 



File:Bribe.sol#L55
```solidity
54:    mapping (uint => uint) public numCheckpoints;
``` 



File:Bribe.sol#L57
```solidity
56:    mapping (uint => SupplyCheckpoint) public supplyCheckpoints;
``` 



File:Bribe.sol#L61
```solidity
60:    mapping (address => mapping (uint => RewardPerTokenCheckpoint)) public rewardPerTokenCheckpoints;
``` 



File:Bribe.sol#L63
```solidity
62:    mapping (address => uint) public rewardPerTokenNumCheckpoints;
``` 



File:Voter.sol#L29
```solidity
28:    mapping(address => address) public gauges; // pair => maturity => gauge
``` 



File:Voter.sol#L30
```solidity
29:    mapping(address => address) public poolForGauge; // gauge => pool
``` 



File:Voter.sol#L31
```solidity
30:    mapping(address => address) public bribes; // gauge => bribe
``` 



File:Voter.sol#L32
```solidity
31:    mapping(address => uint256) public weights; // gauge => weight
``` 



File:Voter.sol#L33
```solidity
32:    mapping(uint => mapping(address => uint256)) public votes; // nft => gauge => votes
``` 



File:Voter.sol#L34
```solidity
33:    mapping(uint => address[]) public gaugeVote; // nft => gauge
``` 



File:Voter.sol#L35
```solidity
34:    mapping(uint => uint) public usedWeights;  // nft => total voting weight of user
``` 



File:Voter.sol#L36
```solidity
35:    mapping(uint => uint) public lastVote; // nft id => timestamp of last vote 
``` 



File:Voter.sol#L37
```solidity
36:    mapping(address => bool) public isGauge;
``` 



File:Voter.sol#L38
```solidity
37:    mapping(address => bool) public isLive; // gauge => status (live or not)
``` 



File:Voter.sol#L40
```solidity
39:    mapping(address => bool) public isWhitelisted;
``` 



File:Voter.sol#L41
```solidity
40:    mapping(address => mapping(address => bool)) public isReward;
``` 



File:Voter.sol#L42
```solidity
41:    mapping(address => mapping(address => bool)) public isBribe;
``` 



File:Voter.sol#L45
```solidity
44:    mapping(address => uint) public claimable;
``` 



File:Voter.sol#L47
```solidity
46:    mapping(address => uint) internal supplyIndex;
``` 



File:Gauge.sol#L21
```solidity
20:    mapping(address => uint) public derivedBalances;
``` 



File:Gauge.sol#L28
```solidity
27:    mapping(address => uint) public rewardRate;
``` 



File:Gauge.sol#L29
```solidity
28:    mapping(address => uint) public periodFinish;
``` 



File:Gauge.sol#L30
```solidity
29:    mapping(address => uint) public lastUpdateTime;
``` 



File:Gauge.sol#L31
```solidity
30:    mapping(address => uint) public rewardPerTokenStored;
``` 



File:Gauge.sol#L33
```solidity
32:    mapping(address => mapping(address => uint)) public lastEarn;
``` 



File:Gauge.sol#L34
```solidity
33:    mapping(address => mapping(address => uint)) public userRewardPerTokenStored;
``` 



File:Gauge.sol#L36
```solidity
35:    mapping(address => uint) public tokenIds;
``` 



File:Gauge.sol#L39
```solidity
38:    mapping(address => uint) public balanceOf;
``` 



File:Gauge.sol#L42
```solidity
41:    mapping(address => bool) public isReward;
``` 



File:Gauge.sol#L63
```solidity
62:    mapping (address => mapping (uint => Checkpoint)) public checkpoints;
``` 



File:Gauge.sol#L65
```solidity
64:    mapping (address => uint) public numCheckpoints;
``` 



File:Gauge.sol#L67
```solidity
66:    mapping (uint => SupplyCheckpoint) public supplyCheckpoints;
``` 



File:Gauge.sol#L71
```solidity
70:    mapping (address => mapping (uint => RewardPerTokenCheckpoint)) public rewardPerTokenCheckpoints;
``` 



File:Gauge.sol#L73
```solidity
72:    mapping (address => uint) public rewardPerTokenNumCheckpoints;
``` 



 --- 


