# Sstan Report 

 ()

TODO: add description

TODO: add date

0.1.0

0x00face, 0xOsiris



# Summary




# <h3>Vulnerabilities</h3> 

 | Classification | Title | 
 |-------|---------| 
 | [[L-0]](#[L-0]) | <Strong>Use a locked pragma version instead of a floating pragma version</Strong> - Instances: 56 |
 | [[H-1]](#[H-1]) | <Strong>Uninitialized storage variables</Strong> - Instances: 2 |
 | [[L-2]](#[L-2]) | <Strong>Unsafe ERC20 Operation</Strong> - Instances: 31 |
# <h3>Optimizations</h3> 

 | Classification | Title | 
 |-------|---------| 
 | [[G-0]](#[G-0]) | <Strong>Use assembly when getting a contract's balance of ETH</Strong> - Instances: 1 |
 | [[G-1]](#[G-1]) | <Strong>Use assembly to check for address(0)</Strong> - Instances: 7 |
 | [[G-2]](#[G-2]) | <Strong>Cache array length during for loop definition.</Strong> - Instances: 8 |
 | [[G-3]](#[G-3]) | <Strong>Cache Storage Variables in Memory</Strong> - Instances: 6 |
 | [[G-4]](#[G-4]) | <Strong>Event is not properly indexed.</Strong> - Instances: 4 |
 | [[G-5]](#[G-5]) | <Strong>Mark storage variables as `immutable` if they never change after contract initialization.</Strong> - Instances: 2 |
 | [[G-6]](#[G-6]) | <Strong> `unchecked{++i}` instead of `i++` (or use assembly when applicable)</Strong> - Instances: 5 |
 | [[G-7]](#[G-7]) | <Strong>Use `calldata` instead of `memory` for function arguments that do not get mutated.</Strong> - Instances: 6 |
 | [[G-8]](#[G-8]) | <Strong>Use multiple require() statments insted of require(expression && expression && ...)</Strong> - Instances: 2 |
 | [[G-9]](#[G-9]) | <Strong>Optimal Comparison</Strong> - Instances: 6 |
 | [[G-10]](#[G-10]) | <Strong>Tightly pack storage variables</Strong> - Instances: 4 |
 | [[G-11]](#[G-11]) | <Strong>Mark functions as payable (with discretion)</Strong> - Instances: 44 |
 | [[G-12]](#[G-12]) | <Strong>Consider marking constants as private</Strong> - Instances: 13 |
 | [[G-13]](#[G-13]) | <Strong>Avoid Reading From Storage in a for loop</Strong> - Instances: 3 |
 | [[G-14]](#[G-14]) | <Strong>Use assembly to hash instead of Solidity</Strong> - Instances: 2 |
 | [[G-15]](#[G-15]) | <Strong>Use assembly for math (add, sub, mul, div)</Strong> - Instances: 13 |
 | [[G-16]](#[G-16]) | <Strong>Use assembly to write storage values</Strong> - Instances: 6 |
 | [[G-17]](#[G-17]) | <Strong>Use custom errors instead of string error messages</Strong> - Instances: 10 |
# <h3>Quality Assurance</h3> 

 | Classification | Title | 
 |-------|---------| 
 | [[NC-0]](#[NC-0]) | <Strong>Constructor should be listed before any other function</Strong> - Instances: 1 |
 | [[NC-1]](#[NC-1]) | <Strong>Private variables should contain a leading underscore</Strong> - Instances: 1 |
 | [[NC-2]](#[NC-2]) | <Strong>Constructor should initialize all variables</Strong> - Instances: 13 |
 | [[NC-3]](#[NC-3]) | <Strong>Consider importing specific identifiers instead of the whole file</Strong> - Instances: 157 |
 | [[NC-4]](#[NC-4]) | <Strong>Constants & Immutables should be named with screaming snake case</Strong> - Instances: 6 |
 | [[NC-5]](#[NC-5]) | <Strong>Consider using scientific notation for large multiples of 10</Strong> - Instances: 17 |
 | [[NC-6]](#[NC-6]) | <Strong>Remove any unused functions</Strong> - Instances: 28 |
 | [[NC-7]](#[NC-7]) | <Strong>Storage variables should be named with camel case</Strong> - Instances: 1 |
 | [[NC-8]](#[NC-8]) | <Strong>Remove any unused returns</Strong> - Instances: 11 |

 <details open> 
 <summary> 
 <h3>Vulnerabilities - Instances: 3 </h3> 
 </summary> 
  


 <details open> 
 <summary> 
 <a name=[L-0]></a> [L-0] 
 <h3> Use a locked pragma version instead of a floating pragma version - Instances: 56 </h3> 
 </summary>
 
> Floating pragma is a vulnerability in smart contract code that can cause unexpected behavior by allowing the compiler to use a specified range of versions. This can lead to issues such as using an older compiler version with known vulnerabilities, using a newer compiler version with undiscovered vulnerabilities, inconsistency across files using different versions, or unpredictable behavior because the compiler can use any version within the specified range. It is recommended to use a locked pragma version in order to avoid these potential vulnerabilities.

> In some cases it may be acceptable to use a floating pragma, such as when a contract is intended for consumption by other developers and needs to be compatible with a range of compiler versions.

#### Bad

```js
    pragma solidity ^0.8.0;
```

#### Good

```js
    pragma solidity 0.8.15;
```

 

 <span style="color: green;">File: </span> UniswapV3Callback.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> OracleLibraryV2.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> BabySwapCallback.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> DeployAvalancheAggregator.s.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> ConveyorFeeMath.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> ConveyorTickMath.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> DeployTest.s.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> ConveyorErrors.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> BiswapCallback.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> DystopiaCallback.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> LinkSwapCallback.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> ISandboxLimitOrderRouter.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> CafeSwapCallback.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> ConveyorSwapCallbacks.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> DeployMainnetAggregator.s.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> AlgebraCallback.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> UniswapV2Callback.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderRouter.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> IConveyorExecutor.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> DeployFantomAggregator.s.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> VerseCallback.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> MdexSwapCallback.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> ConvergenceXCallback.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> DeployArbitrumAggregator.s.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> ILimitOrderQuoter.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> ISandboxLimitOrderBook.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> KyberSwapV3Callback.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> TraderJoeCallback.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> ILimitOrderRouter.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> DeployPolygonAggregator.s.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> UniFiCallback.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> NomiswapCallback.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> DXSwapCallback.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> DeployBSCAggregator.s.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> DefiSwapCallback.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> PancakeV2Callback.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> ILimitOrderSwapRouter.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> JetSwapCallback.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> ApeSwapCallback.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> SakeSwapCallback.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> MeerkatCallback.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> ElkSwapCallback.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> PancakeV3Callback.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> IConveyorRouterV1.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> BabyDogeCallback.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> WaultSwapCallback.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> ILimitOrderBook.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ```

 <span style="color: green;">File: </span> DeployOptimismAggregator.s.sol 2-2 
 ```solidity 
 pragma solidity ^0.8.19; 
 ``` 
 </details>

 <details open> 
 <summary> 
 <a name=[H-1]></a> [H-1] 
 <h3> Uninitialized storage variables - Instances: 2 </h3> 
 </summary>
 
> A storage variable that is declared but not initialized will have a default value of zero (or the equivalent, such as an empty array for array types or zero-address for address types). Failing to initialize a storage variable can pose risks if the contract logic assumes that the variable has been explicitly set to a particular value. 

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 92-92 
 ```solidity 
 mapping(address => uint256) dexToIndex; 
 ```

 <span style="color: green;">File: </span> ConveyorTickMath.sol 25-25 
 ```solidity 
 mapping(int24 => Tick.Info) public ticks; 
 ``` 
 </details>

 <details open> 
 <summary> 
 <a name=[L-2]></a> [L-2] 
 <h3> Unsafe ERC20 Operation - Instances: 31 </h3> 
 </summary>
 
> ERC20 operations can be unsafe due to different implementations and vulnerabilities in the standard. To account for this, either use OpenZeppelin's SafeERC20 library or wrap each operation in a require statement.
> Additionally, ERC20's approve functions have a known race-condition vulnerability. To account for this, use OpenZeppelin's SafeERC20 library's `safeIncrease` or `safeDecrease` Allowance functions.
        
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
        
         

 <span style="color: green;">File: </span> CafeSwapCallback.sol 23-23 
 ```solidity 
 IERC20(_tokenIn).transfer 
 ```

 <span style="color: green;">File: </span> NomiswapCallback.sol 23-23 
 ```solidity 
 IERC20(_tokenIn).transfer 
 ```

 <span style="color: green;">File: </span> UniswapV2Callback.sol 23-23 
 ```solidity 
 IERC20(_tokenIn).transfer 
 ```

 <span style="color: green;">File: </span> BabyDogeCallback.sol 23-23 
 ```solidity 
 IERC20(_tokenIn).transfer 
 ```

 <span style="color: green;">File: </span> LinkSwapCallback.sol 23-23 
 ```solidity 
 IERC20(_tokenIn).transfer 
 ```

 <span style="color: green;">File: </span> DefiSwapCallback.sol 23-23 
 ```solidity 
 IERC20(_tokenIn).transfer 
 ```

 <span style="color: green;">File: </span> PancakeV2Callback.sol 23-23 
 ```solidity 
 IERC20(_tokenIn).transfer 
 ```

 <span style="color: green;">File: </span> TraderJoeCallback.sol 23-23 
 ```solidity 
 IERC20(_tokenIn).transfer 
 ```

 <span style="color: green;">File: </span> MeerkatCallback.sol 23-23 
 ```solidity 
 IERC20(_tokenIn).transfer 
 ```

 <span style="color: green;">File: </span> AlgebraCallback.sol 20-20 
 ```solidity 
 IERC20(_tokenIn).transferFrom 
 ```

 <span style="color: green;">File: </span> AlgebraCallback.sol 22-22 
 ```solidity 
 IERC20(_tokenIn).transfer 
 ```

 <span style="color: green;">File: </span> WaultSwapCallback.sol 23-23 
 ```solidity 
 IERC20(_tokenIn).transfer 
 ```

 <span style="color: green;">File: </span> ConvergenceXCallback.sol 23-23 
 ```solidity 
 IERC20(_tokenIn).transfer 
 ```

 <span style="color: green;">File: </span> MdexSwapCallback.sol 23-23 
 ```solidity 
 IERC20(_tokenIn).transfer 
 ```

 <span style="color: green;">File: </span> SakeSwapCallback.sol 23-23 
 ```solidity 
 IERC20(_tokenIn).transfer 
 ```

 <span style="color: green;">File: </span> DystopiaCallback.sol 23-23 
 ```solidity 
 IERC20(_tokenIn).transfer 
 ```

 <span style="color: green;">File: </span> KyberSwapV3Callback.sol 20-20 
 ```solidity 
 IERC20(_tokenIn).transferFrom 
 ```

 <span style="color: green;">File: </span> KyberSwapV3Callback.sol 22-22 
 ```solidity 
 IERC20(_tokenIn).transfer 
 ```

 <span style="color: green;">File: </span> VerseCallback.sol 23-23 
 ```solidity 
 IERC20(_tokenIn).transfer 
 ```

 <span style="color: green;">File: </span> ElkSwapCallback.sol 23-23 
 ```solidity 
 IERC20(_tokenIn).transfer 
 ```

 <span style="color: green;">File: </span> BiswapCallback.sol 23-23 
 ```solidity 
 IERC20(_tokenIn).transfer 
 ```

 <span style="color: green;">File: </span> DXSwapCallback.sol 23-23 
 ```solidity 
 IERC20(_tokenIn).transfer 
 ```

 <span style="color: green;">File: </span> ApeSwapCallback.sol 23-23 
 ```solidity 
 IERC20(_tokenIn).transfer 
 ```

 <span style="color: green;">File: </span> BabySwapCallback.sol 23-23 
 ```solidity 
 IERC20(_tokenIn).transfer 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 121-121 
 ```solidity 
 IERC20(swapData.tokenIn).transferFrom 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 172-172 
 ```solidity 
 IERC20(WETH).transfer 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 217-217 
 ```solidity 
 IERC20(swapData.tokenIn).transferFrom 
 ```

 <span style="color: green;">File: </span> JetSwapCallback.sol 23-23 
 ```solidity 
 IERC20(_tokenIn).transfer 
 ```

 <span style="color: green;">File: </span> PancakeV3Callback.sol 20-20 
 ```solidity 
 IERC20(_tokenIn).transferFrom 
 ```

 <span style="color: green;">File: </span> PancakeV3Callback.sol 22-22 
 ```solidity 
 IERC20(_tokenIn).transfer 
 ```

 <span style="color: green;">File: </span> UniFiCallback.sol 23-23 
 ```solidity 
 IERC20(_tokenIn).transfer 
 ``` 
 </details> 
 </details>

 <details open> 
 <summary> 
 <h3>Optimizations - Instances: 18 </h3> 
 </summary> 
  


 <details open> 
 <summary> 
 <a name=[G-0]></a> [G-0] 
 <h3> Use assembly when getting a contract's balance of ETH - Instances: 1 </h3> 
 </summary>
 
 &nbsp; 
    You can use `selfbalance()` instead of `address(this).balance` when getting your contract's balance of ETH to save gas. Additionally, you can use `balance(address)` instead of `address.balance()` when getting an external contract's balance of ETH.
     
 &nbsp; <details> &nbsp; 
 <summary> 
 &nbsp; &nbsp; Address Balance Optimization - Gas Report Savings: ~15 
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
        c0.addressInternalBalance();
        c1.assemblyInternalBalance();
        c2.addressExternalBalance(address(this));
        c3.assemblyExternalBalance(address(this));
    }
}

contract Contract0 {
    function addressInternalBalance() public returns (uint256) {
        return address(this).balance;
    }
}

contract Contract1 {
    function assemblyInternalBalance() public returns (uint256) {
        assembly {
            let c := selfbalance()
            mstore(0x00, c)
            return(0x00, 0x20)
        }
    }
}

contract Contract2 {
    function addressExternalBalance(address addr) public {
        uint256 bal = address(addr).balance;
        bal++;
    }
}

contract Contract3 {
    function assemblyExternalBalance(address addr) public {
        uint256 bal;
        assembly {
            bal := balance(addr)
        }
        bal++;
    }
}
```


```solidity
╭────────────────────────┬─────────────────┬─────┬────────┬─────┬─────────╮
│ Contract0 contract     ┆                 ┆     ┆        ┆     ┆         │
╞════════════════════════╪═════════════════╪═════╪════════╪═════╪═════════╡
│ Deployment Cost        ┆ Deployment Size ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 23675                  ┆ 147             ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name          ┆ min             ┆ avg ┆ median ┆ max ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ addressInternalBalance ┆ 148             ┆ 148 ┆ 148    ┆ 148 ┆ 1       │
╰────────────────────────┴─────────────────┴─────┴────────┴─────┴─────────╯
╭─────────────────────────┬─────────────────┬─────┬────────┬─────┬─────────╮
│ Contract1 contract      ┆                 ┆     ┆        ┆     ┆         │
╞═════════════════════════╪═════════════════╪═════╪════════╪═════╪═════════╡
│ Deployment Cost         ┆ Deployment Size ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 27081                   ┆ 165             ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name           ┆ min             ┆ avg ┆ median ┆ max ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ assemblyInternalBalance ┆ 133             ┆ 133 ┆ 133    ┆ 133 ┆ 1       │
╰─────────────────────────┴─────────────────┴─────┴────────┴─────┴─────────╯
╭────────────────────────┬─────────────────┬─────┬────────┬─────┬─────────╮
│ Contract2 contract     ┆                 ┆     ┆        ┆     ┆         │
╞════════════════════════╪═════════════════╪═════╪════════╪═════╪═════════╡
│ Deployment Cost        ┆ Deployment Size ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 61511                  ┆ 339             ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name          ┆ min             ┆ avg ┆ median ┆ max ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ addressExternalBalance ┆ 417             ┆ 417 ┆ 417    ┆ 417 ┆ 1       │
╰────────────────────────┴─────────────────┴─────┴────────┴─────┴─────────╯
╭─────────────────────────┬─────────────────┬─────┬────────┬─────┬─────────╮
│ Contract3 contract      ┆                 ┆     ┆        ┆     ┆         │
╞═════════════════════════╪═════════════════╪═════╪════════╪═════╪═════════╡
│ Deployment Cost         ┆ Deployment Size ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ 57105                   ┆ 317             ┆     ┆        ┆     ┆         │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ Function Name           ┆ min             ┆ avg ┆ median ┆ max ┆ # calls │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
│ assemblyExternalBalance ┆ 411             ┆ 411 ┆ 411    ┆ 411 ┆ 1       │
╰─────────────────────────┴─────────────────┴─────┴────────┴─────┴─────────╯

```
 
 </details> 
 

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 364-364 
 ```solidity 
 address(this).balance 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 365-365 
 ```solidity 
 address(this).balance 
 ``` 
 </details>

 <details open> 
 <summary> 
 <a name=[G-1]></a> [G-1] 
 <h3> Use assembly to check for address(0) - Instances: 7 </h3> 
 </summary>
 
 &nbsp;  
 &nbsp; <details> &nbsp; 
 <summary> 
 &nbsp; &nbsp; Address Zero Optimization - Gas Report Savings: ~6 
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

### Gas Report

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
 

 <span style="color: green;">File: </span> ConveyorExecutor.sol 119-119 
 ```solidity 
 _weth != address(0) 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 120-120 
 ```solidity 
 _usdc != address(0) 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 121-121 
 ```solidity 
 _limitOrderQuoterAddress != address(0) 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 530-530 
 ```solidity 
 newOwner == address(0) 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 65-65 
 ```solidity 
 _weth != address(0) 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 139-139 
 ```solidity 
 affiliate == address(0) 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 147-147 
 ```solidity 
 referrer == address(0) 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 192-192 
 ```solidity 
 affiliate == address(0) 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 200-200 
 ```solidity 
 referrer == address(0) 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 215-215 
 ```solidity 
 swapAggregatorMulticall.tokenInDestination != address(0) 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 245-245 
 ```solidity 
 affiliate == address(0) 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 253-253 
 ```solidity 
 referrer == address(0) 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 381-381 
 ```solidity 
 newOwner == address(0) 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 94-94 
 ```solidity 
 _limitOrderExecutor != address(0) 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 1268-1268 
 ```solidity 
 newOwner == address(0) 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 132-132 
 ```solidity 
 _dexFactories[i] != address(0) 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 433-433 
 ```solidity 
 address(0) == pairAddress 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 500-500 
 ```solidity 
 pool == address(0) 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 541-541 
 ```solidity 
 token0 == address(0) 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 17-17 
 ```solidity 
 _weth != address(0) 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 43-43 
 ```solidity 
 _limitOrderExecutor != address(0) 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 70-70 
 ```solidity 
 _limitOrderExecutor != address(0) 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 360-360 
 ```solidity 
 newOwner == address(0) 
 ``` 
 </details>

 <details open> 
 <summary> 
 <a name=[G-2]></a> [G-2] 
 <h3> Cache array length during for loop definition. - Instances: 8 </h3> 
 </summary>
 
 &nbsp; A typical for loop definition may look like: `for (uint256 i; i < arr.length; i++){}`. Instead of using `array.length`, cache the array length before the loop, and use the cached value to safe gas. This will avoid an `MLOAD` every loop for arrays stored in memory and an `SLOAD` for arrays stored in storage. This can have significant gas savings for arrays with a large length, especially if the array is stored in storage. 
 &nbsp; <details> &nbsp; 
 <summary> 
 &nbsp; &nbsp; Cache Array Length - Gas Report Savings: ~22 
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

### Gas Report
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
 

 <span style="color: green;">File: </span> LimitOrderBook.sol 281-281 
 ```solidity 
 orderGroup.length 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 485-485 
 ```solidity 
 orderIds.length 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 59-59 
 ```solidity 
 executionPrices.length 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 75-75 
 ```solidity 
 executionPrices.length 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 103-103 
 ```solidity 
 executionPrices.length 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 117-117 
 ```solidity 
 executionPrices.length 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 148-148 
 ```solidity 
 spotReserveAToWeth.length 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 187-187 
 ```solidity 
 spotReserveWethToB.length 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 207-207 
 ```solidity 
 spotReserveAToWeth.length 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 209-209 
 ```solidity 
 spotReserveWethToB.length 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 317-317 
 ```solidity 
 orderGroup.length 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 331-331 
 ```solidity 
 spRes.length 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 513-513 
 ```solidity 
 orderIds.length 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 634-634 
 ```solidity 
 orderIds.length 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 750-750 
 ```solidity 
 orderIdBundles.length 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 753-753 
 ```solidity 
 orderIdBundle.length 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 815-815 
 ```solidity 
 orderIdBundles.length 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderRouter.sol 77-77 
 ```solidity 
 sandboxMulticall.calls.length 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 92-92 
 ```solidity 
 orderIds.length 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 211-211 
 ```solidity 
 orders.length 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 284-284 
 ```solidity 
 orderIds.length 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 324-324 
 ```solidity 
 orderIds.length 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 339-339 
 ```solidity 
 orders.length 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 128-128 
 ```solidity 
 _dexFactories.length 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 589-589 
 ```solidity 
 dexes.length 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 183-183 
 ```solidity 
 orders.length 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 309-309 
 ```solidity 
 orders.length 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 432-432 
 ```solidity 
 orders.length 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 453-453 
 ```solidity 
 orders.length 
 ```

 <span style="color: green;">File: </span> UniswapInterfaceMulticall.sol 30-30 
 ```solidity 
 calls.length 
 ``` 
 </details>

 <details open> 
 <summary> 
 <a name=[G-3]></a> [G-3] 
 <h3> Cache Storage Variables in Memory - Instances: 6 </h3> 
 </summary>
 
 &nbsp;  
 &nbsp; <details> &nbsp; 
 <summary> 
 &nbsp; &nbsp; Cache Array Length - Gas Report Savings: ~0 
  </summary> 
  
 </details> 
 

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 66-66 
 ```solidity 
 reentrancyStatus 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 68-68 
 ```solidity 
 reentrancyStatus 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 253-253 
 ```solidity 
 minExecutionCredit 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 257-257 
 ```solidity 
 orderIdToSandboxLimitOrder 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 276-276 
 ```solidity 
 orderIdToSandboxLimitOrder 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 278-278 
 ```solidity 
 orderIdToSandboxLimitOrder 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 393-393 
 ```solidity 
 orderNonce 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 472-472 
 ```solidity 
 orderIdToSandboxLimitOrder 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 473-473 
 ```solidity 
 orderIdToSandboxLimitOrder 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 501-501 
 ```solidity 
 orderIdToSandboxLimitOrder 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 502-502 
 ```solidity 
 orderIdToSandboxLimitOrder 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 537-537 
 ```solidity 
 orderIdToSandboxLimitOrder 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 549-549 
 ```solidity 
 addressToOrderIds 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 608-608 
 ```solidity 
 orderIdToSandboxLimitOrder 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 685-685 
 ```solidity 
 orderIdToSandboxLimitOrder 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 1055-1055 
 ```solidity 
 orderIdToSandboxLimitOrder 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 1058-1058 
 ```solidity 
 orderIdToSandboxLimitOrder 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 1060-1060 
 ```solidity 
 orderIdToSandboxLimitOrder 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 1069-1069 
 ```solidity 
 orderIdToSandboxLimitOrder 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 1076-1076 
 ```solidity 
 orderIdToSandboxLimitOrder 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 1096-1096 
 ```solidity 
 orderIdToSandboxLimitOrder 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 1118-1118 
 ```solidity 
 orderIdToSandboxLimitOrder 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 1128-1128 
 ```solidity 
 addressToOrderIds 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 1253-1253 
 ```solidity 
 minExecutionCredit 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 1263-1263 
 ```solidity 
 tempOwner 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 69-69 
 ```solidity 
 reentrancyStatus 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 71-71 
 ```solidity 
 reentrancyStatus 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 511-511 
 ```solidity 
 conveyorBalance 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 513-513 
 ```solidity 
 conveyorBalance 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 524-524 
 ```solidity 
 tempOwner 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 32-32 
 ```solidity 
 reentrancyStatus 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 34-34 
 ```solidity 
 reentrancyStatus 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 187-187 
 ```solidity 
 minExecutionCredit 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 190-190 
 ```solidity 
 orderIdToLimitOrder 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 215-215 
 ```solidity 
 orderIdToLimitOrder 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 217-217 
 ```solidity 
 orderIdToLimitOrder 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 319-319 
 ```solidity 
 orderNonce 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 409-409 
 ```solidity 
 orderIdToLimitOrder 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 410-410 
 ```solidity 
 orderIdToLimitOrder 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 438-438 
 ```solidity 
 orderIdToLimitOrder 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 439-439 
 ```solidity 
 orderIdToLimitOrder 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 462-462 
 ```solidity 
 orderIdToLimitOrder 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 474-474 
 ```solidity 
 addressToOrderIds 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 500-500 
 ```solidity 
 orderIdToLimitOrder 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 521-521 
 ```solidity 
 orderIdToLimitOrder 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 531-531 
 ```solidity 
 addressToOrderIds 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 355-355 
 ```solidity 
 tempOwner 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 355-355 
 ```solidity 
 uniV3AmountOut 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 383-383 
 ```solidity 
 uniV3AmountOut 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 387-387 
 ```solidity 
 uniV3AmountOut 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 388-388 
 ```solidity 
 uniV3AmountOut 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 586-586 
 ```solidity 
 dexes 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 589-589 
 ```solidity 
 dexes 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 590-590 
 ```solidity 
 dexes 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 594-594 
 ```solidity 
 dexes 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 606-606 
 ```solidity 
 dexes 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 624-624 
 ```solidity 
 dexes 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 625-625 
 ```solidity 
 dexes 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 375-375 
 ```solidity 
 tempOwner 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 409-409 
 ```solidity 
 affiliateNonce 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 420-420 
 ```solidity 
 referrerIndex 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 425-425 
 ```solidity 
 referrerNonce 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 446-446 
 ```solidity 
 locked 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 448-448 
 ```solidity 
 locked 
 ``` 
 </details>

 <details open> 
 <summary> 
 <a name=[G-4]></a> [G-4] 
 <h3> Event is not properly indexed. - Instances: 4 </h3> 
 </summary>
 
 &nbsp; When possible, always include a minimum of 3 indexed event topics to save gas 
 &nbsp; <details> &nbsp; 
 <summary> 
 &nbsp; &nbsp; Event Indexing - Gas Report Savings: ~0 
  </summary> 
  
 </details> 
 

 <span style="color: green;">File: </span> ConveyorExecutor.sol 104-104 
 ```solidity 
 event ExecutorCheckIn(address executor, uint256 timestamp); 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 58-58 
 ```solidity 
 event OrderPlaced(bytes32[] orderIds); 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 64-64 
 ```solidity 
 event OrderCanceled(bytes32[] orderIds); 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 70-70 
 ```solidity 
 event OrderUpdated(bytes32[] orderIds); 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 75-75 
 ```solidity 
 event OrderExecutionCreditUpdated(bytes32 orderId, uint256 newExecutionCredit); 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 81-81 
 ```solidity 
 event OrderFilled(bytes32[] orderIds); 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 91-91 
 ```solidity 
 event MinExecutionCreditUpdated(uint256 newMinExecutionCredit, uint256 oldMinExecutionCredit); 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 112-112 
 ```solidity 
 event OrderPlaced(bytes32[] orderIds); 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 118-118 
 ```solidity 
 event OrderCanceled(bytes32[] orderIds); 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 124-124 
 ```solidity 
 event OrderUpdated(bytes32[] orderIds); 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 130-130 
 ```solidity 
 event OrderFilled(bytes32[] orderIds); 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 152-152 
 ```solidity 
 event OrderExecutionCreditUpdated(bytes32 orderId, uint256 newExecutionCredit); 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 157-157 
 ```solidity 
 event MinExecutionCreditUpdated(uint256 newMinExecutionCredit, uint256 oldMinExecutionCredit); 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 35-35 
 ```solidity 
 event Withdraw(address indexed receiver, uint256 amount); 
 ``` 
 </details>

 <details open> 
 <summary> 
 <a name=[G-5]></a> [G-5] 
 <h3> Mark storage variables as `immutable` if they never change after contract initialization. - Instances: 2 </h3> 
 </summary>
 
 &nbsp; State variables can be declared as constant or immutable. In both cases, the variables cannot be modified after the contract has been constructed. For constant variables, the value has to be fixed at compile-time, while for immutable, it can still be assigned at construction time. 
 The compiler does not reserve a storage slot for these variables, and every occurrence is inlined by the respective value. 
 Compared to regular state variables, the gas costs of constant and immutable variables are much lower. For a constant variable, the expression assigned to it is copied to all the places where it is accessed and also re-evaluated each time. This allows for local optimizations. Immutable variables are evaluated once at construction time and their value is copied to all the places in the code where they are accessed. For these values, 32 bytes are reserved, even if they would fit in fewer bytes. Due to this, constant values can sometimes be cheaper than immutable values. 
 
 &nbsp; <details> &nbsp; 
 <summary> 
 &nbsp; &nbsp; Immutable Variable - Gas Report Savings: ~2103 
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

### Gas Report
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
 

 <span style="color: green;">File: </span> LimitOrderBook.sol 22-22 
 ```solidity 
 uint256 minExecutionCredit; 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 22-22 
 ```solidity 
 address public CONVEYOR_MULTICALL; 
 ``` 
 </details>

 <details open> 
 <summary> 
 <a name=[G-6]></a> [G-6] 
 <h3>  `unchecked{++i}` instead of `i++` (or use assembly when applicable) - Instances: 5 </h3> 
 </summary>
 
 &nbsp; Use `++i` instead of `i++`. This is especially useful in for loops but this optimization can be used anywhere in your code. You can also use `unchecked{++i;}` for even more gas savings but this will not check to see if `i` overflows. For extra safety if you are worried about this, you can add a require statement after the loop checking if `i` is equal to the final incremented value. For best gas savings, use inline assembly, however this limits the functionality you can achieve. For example you cant use Solidity syntax to internally call your own contract within an assembly block and external calls must be done with the `call()` or `delegatecall()` instruction. However when applicable, inline assembly will save much more gas. 
 &nbsp; <details> &nbsp; 
 <summary> 
 &nbsp; &nbsp; Increment Decrement - Gas Report Savings: ~342 
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

### Gas Report

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
 

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 128-128 
 ```solidity 
 ++i 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 525-525 
 ```solidity 
 --totalOrdersPerAddress[order.owner] 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 599-599 
 ```solidity 
 ++orderIdIndex 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 503-503 
 ```solidity 
 --totalOrdersPerAddress[order.owner] 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 339-339 
 ```solidity 
 ++totalOrdersPerAddress[msg.sender] 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 468-468 
 ```solidity 
 --totalOrdersPerAddress[msg.sender] 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 993-993 
 ```solidity 
 ++offset 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 416-416 
 ```solidity 
 ++totalOrdersPerAddress[msg.sender] 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 1099-1099 
 ```solidity 
 --totalOrdersPerAddress[order.owner] 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 1235-1235 
 ```solidity 
 ++orderIdIndex 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 832-832 
 ```solidity 
 ++orderIdIndex 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 543-543 
 ```solidity 
 --totalOrdersPerAddress[msg.sender] 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 1122-1122 
 ```solidity 
 --totalOrdersPerAddress[order.owner] 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 423-423 
 ```solidity 
 tempReferrerNonce++ 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 407-407 
 ```solidity 
 tempAffiliateNonce++ 
 ```

 <span style="color: green;">File: </span> UniswapInterfaceMulticall.sol 30-30 
 ```solidity 
 i++ 
 ``` 
 </details>

 <details open> 
 <summary> 
 <a name=[G-7]></a> [G-7] 
 <h3> Use `calldata` instead of `memory` for function arguments that do not get mutated. - Instances: 6 </h3> 
 </summary>
 
 &nbsp; Mark data types as `calldata` instead of `memory` where possible. This makes it so that the data is not automatically loaded into memory. If the data passed into the function does not need to be changed (like updating values in an array), it can be passed in as `calldata`. The one exception to this is if the argument must later be passed into another function that takes an argument that specifies `memory` storage. 
 &nbsp; <details> &nbsp; 
 <summary> 
 &nbsp; &nbsp; Memory to Calldata - Gas Report Savings: ~1716 
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
 

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 295-296 
 ```solidity 
 LimitOrderSwapRouter.TokenToTokenExecutionPrice memory executionPrice 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 326-327 
 ```solidity 
 LimitOrderSwapRouter.TokenToTokenExecutionPrice memory executionPrice 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 343-344 
 ```solidity 
 LimitOrderSwapRouter.TokenToTokenExecutionPrice memory executionPrice 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 371-372 
 ```solidity 
 LimitOrderSwapRouter.TokenToTokenExecutionPrice memory executionPrice 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 467-468 
 ```solidity 
 LimitOrderSwapRouter.TokenToWethExecutionPrice memory executionPrice 
 ```

 <span style="color: green;">File: </span> UniswapInterfaceMulticall.sol 27-27 
 ```solidity 
 Call[] memory calls 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 389-389 
 ```solidity 
 bytes memory bytecode 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 111-111 
 ```solidity 
 LimitOrder memory order 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 178-178 
 ```solidity 
 LimitOrder memory order 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 209-209 
 ```solidity 
 LimitOrder[] memory orders 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 588-588 
 ```solidity 
 SandboxLimitOrder memory order 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 655-655 
 ```solidity 
 SandboxLimitOrder memory order 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 809-810 
 ```solidity 
 PreSandboxExecutionState memory preSandboxExecutionState 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 847-847 
 ```solidity 
 SandboxLimitOrder memory currentOrder 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 904-905 
 ```solidity 
 PreSandboxExecutionState memory preSandboxExecutionState 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 903-903 
 ```solidity 
 uint128[] memory fillAmounts 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 1004-1004 
 ```solidity 
 SandboxLimitOrder memory prevOrder 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 1006-1006 
 ```solidity 
 uint128[] memory fillAmounts 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 220-221 
 ```solidity 
 LimitOrderSwapRouter.TokenToWethExecutionPrice memory executionPrice 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 344-345 
 ```solidity 
 TokenToTokenExecutionPrice memory executionPrice 
 ``` 
 </details>

 <details open> 
 <summary> 
 <a name=[G-8]></a> [G-8] 
 <h3> Use multiple require() statments insted of require(expression && expression && ...) - Instances: 2 </h3> 
 </summary>
 
 &nbsp; You can safe gas by breaking up a require statement with multiple conditions, into multiple require statements with a single condition. 
 &nbsp; <details> &nbsp; 
 <summary> 
 &nbsp; &nbsp; Multiple Require - Gas Report Savings: ~16 
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

### Gas Report

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
 

 <span style="color: green;">File: </span> OracleLibraryV2.sol 11-11 
 ```solidity 
 require(reserveIn > 0 && reserveOut > 0, "UniswapV2Library: INSUFFICIENT_LIQUIDITY") 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 54-54 
 ```solidity 
 require(answer >= 0x0 && answer <= MAX_64x64) 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 87-87 
 ```solidity 
 require(result >= MIN_64x64 && result <= type(int128).max) 
 ``` 
 </details>

 <details open> 
 <summary> 
 <a name=[G-9]></a> [G-9] 
 <h3> Optimal Comparison - Instances: 6 </h3> 
 </summary>
 
 &nbsp; When comparing integers, it is cheaper to use strict `>` & `<` operators over `>=` & `<=` operators, even if you must increment or decrement one of the operands. 
 Note: before using this technique, it's important to consider whether incrementing/decrementing one of the operators could result in an over/underflow.
        
 This optimization is applicable when the optimizer is turned off. 
 &nbsp; <details> &nbsp; 
 <summary> 
 &nbsp; &nbsp; Optimal Comparison - Gas Report Savings: ~3 
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

### Gas Report

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
 

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 187-187 
 ```solidity 
 amountInUSDCDollarValue >= 1000000 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 197-197 
 ```solidity 
 exponent >= 0x400000000000000000 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 474-475 
 ```solidity 
 token0Decimals <= 18 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 477-478 
 ```solidity 
 token1Decimals <= 18 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 276-277 
 ```solidity 
 tokenInDecimals <= 18 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 480-481 
 ```solidity 
 tokenInDecimals <= 18 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 349-350 
 ```solidity 
 tokenInDecimals <= 18 
 ```

 <span style="color: green;">File: </span> ConveyorFeeMath.sol 31-31 
 ```solidity 
 percentFee <= ZERO_POINT_ZERO_ZERO_FIVE 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 23-23 
 ```solidity 
 x <= MAX_UINT64 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 42-42 
 ```solidity 
 x <= 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 54-54 
 ```solidity 
 answer >= 0x0 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 54-54 
 ```solidity 
 answer <= MAX_64x64 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 75-75 
 ```solidity 
 answer <= MAX_64x64 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 87-87 
 ```solidity 
 result >= MIN_64x64 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 87-87 
 ```solidity 
 result <= type(int128).max 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 119-119 
 ```solidity 
 answer <= MAX_64x64 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 150-150 
 ```solidity 
 hi <= 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 153-153 
 ```solidity 
 hi <= MAX_128x128 - lo 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 189-189 
 ```solidity 
 answer <= 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 208-208 
 ```solidity 
 hi <= MAX_128x128 - lo 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 221-221 
 ```solidity 
 answer <= uint128(MAX_64x64) 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 236-236 
 ```solidity 
 x <= 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 241-241 
 ```solidity 
 xc >= 0x100000000 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 245-245 
 ```solidity 
 xc >= 0x10000 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 249-249 
 ```solidity 
 xc >= 0x100 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 253-253 
 ```solidity 
 xc >= 0x10 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 257-257 
 ```solidity 
 xc >= 0x4 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 261-261 
 ```solidity 
 xc >= 0x2 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 264-264 
 ```solidity 
 answer <= 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 283-283 
 ```solidity 
 answer <= 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 498-498 
 ```solidity 
 answer <= uint256(MAX_64x64) 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 525-525 
 ```solidity 
 xx >= 0x100000000000000000000000000000000 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 529-529 
 ```solidity 
 xx >= 0x10000000000000000 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 533-533 
 ```solidity 
 xx >= 0x100000000 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 537-537 
 ```solidity 
 xx >= 0x10000 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 541-541 
 ```solidity 
 xx >= 0x100 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 545-545 
 ```solidity 
 xx >= 0x10 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 549-549 
 ```solidity 
 xx >= 0x8 
 ```

 <span style="color: green;">File: </span> ConveyorTickMath.sol 91-91 
 ```solidity 
 priceX128 <= type(uint256).max 
 ``` 
 </details>

 <details open> 
 <summary> 
 <a name=[G-10]></a> [G-10] 
 <h3> Tightly pack storage variables - Instances: 4 </h3> 
 </summary>
 
 &nbsp; When defining storage variables, make sure to declare them in ascending order, according to size. When multiple variables are able to fit into one 256 bit slot, this will save storage size and gas during runtime. For example, if you have a `bool`, `uint256` and a `bool`, instead of defining the variables in the previously mentioned order, defining the two boolean variables first will pack them both into one storage slot since they only take up one byte of storage. 
 &nbsp; <details> &nbsp; 
 <summary> 
 &nbsp; &nbsp; Pack Storage Variables - Gas Report Savings: ~0 
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

### Gas Report
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
 

 <span style="color: green;">File: </span> ConveyorMath.sol 8-8 
 ```solidity 
 uint128 private constant MAX_64x64 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF; 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 14-14 
 ```solidity 
 address immutable LIMIT_ORDER_EXECUTOR; 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 84-84 
 ```solidity 
 uint256 uniV3AmountOut; 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 21-21 
 ```solidity 
 address immutable WETH; 
 ``` 
 </details>

 <details open> 
 <summary> 
 <a name=[G-11]></a> [G-11] 
 <h3> Mark functions as payable (with discretion) - Instances: 44 </h3> 
 </summary>
 
 &nbsp; You can mark public or external functions as payable to save gas. Functions that are not payable have additional logic to check if there was a value sent with a call, however, making a function payable eliminates this check. This optimization should be carefully considered due to potentially unwanted behavior when a function does not need to accept ether. 
 &nbsp; <details> &nbsp; 
 <summary> 
 &nbsp; &nbsp; Payable Functions - Gas Report Savings: ~24 
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

### Gas Report
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
 

 <span style="color: green;">File: </span> NomiswapCallback.sol 13-13 
 ```solidity 
 function nomiswapCall(address, uint256 amount0, uint256 amount1, bytes calldata data) external {(bool _zeroForOne, address _tokenIn, uint24 _swapFee) = abi.decode(data, (bool, address, uint24)) uint256 amountOut = _zeroForOne ? amount1 : amount0; (uint112 reserve0, uint112 reserve1) = IUniswapV2Pair(msg.sender).getReserves() uint256 amountIn = OracleLibraryV2.getAmountIn(amountOut, _zeroForOne ? reserve0 : reserve1, _zeroForOne ? reserve1 : reserve0, _swapFee); IERC20(_tokenIn).transfer(msg.sender, amountIn)} 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 363-363 
 ```solidity 
 function withdraw() external onlyOwner {_safeTransferETH(msg.sender, address(this).balance) emit Withdraw(msg.sender, address(this).balance);} 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 369-369 
 ```solidity 
 function confirmTransferOwnership() external {if (msg.sender != tempOwner) {revert UnauthorizedCaller();} tempOwner = address(0) owner = msg.sender} 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 380-380 
 ```solidity 
 function transferOwnership(address newOwner) external onlyOwner {if (newOwner == address(0)) {revert InvalidAddress();} tempOwner = newOwner} 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 402-402 
 ```solidity 
 function initializeAffiliate(address affiliateAddress) external onlyOwner {uint16 tempAffiliateNonce = affiliateNonce; affiliates[tempAffiliateNonce] = affiliateAddress affiliateIndex[affiliateAddress] = tempAffiliateNonce unchecked {tempAffiliateNonce++ require(tempAffiliateNonce < type(uint16).max >> 0x1, "Affiliate nonce overflow") affiliateNonce = tempAffiliateNonce}} 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 453-453 
 ```solidity 
 function executeMulticall(ConveyorRouterV1.Call[] calldata calls) external lock {assembly ("memory-safe") {let freeMemoryPointer := 0x40 let size := mul(0x20, calls.length) calldatacopy(freeMemoryPointer, calls.offset, size) size := add(freeMemoryPointer, size) for {} 1 {} {let o := add(calls.offset, mload(freeMemoryPointer)) let len := calldataload(add(o, 0x40)) let memPtr := mload(add(sub(size, freeMemoryPointer), 0x20)) calldatacopy(memPtr, add(o, 0x60), len) if iszero(call(gas(), calldataload(o), 0, memPtr, len, 0, 0)) {returndatacopy(0, 0, returndatasize()) revert(0, returndatasize())} freeMemoryPointer := add(freeMemoryPointer, 0x20) if iszero(lt(freeMemoryPointer, size)) {break}}}} 
 ```

 <span style="color: green;">File: </span> BabyDogeCallback.sol 13-13 
 ```solidity 
 function BabyDogeCall(address, uint256 amount0, uint256 amount1, bytes calldata data) external {(bool _zeroForOne, address _tokenIn, uint24 _swapFee) = abi.decode(data, (bool, address, uint24)) uint256 amountOut = _zeroForOne ? amount1 : amount0; (uint112 reserve0, uint112 reserve1) = IUniswapV2Pair(msg.sender).getReserves() uint256 amountIn = OracleLibraryV2.getAmountIn(amountOut, _zeroForOne ? reserve0 : reserve1, _zeroForOne ? reserve1 : reserve0, _swapFee); IERC20(_tokenIn).transfer(msg.sender, amountIn)} 
 ```

 <span style="color: green;">File: </span> PancakeV2Callback.sol 13-13 
 ```solidity 
 function pancakeCall(address, uint256 amount0, uint256 amount1, bytes calldata data) external {(bool _zeroForOne, address _tokenIn, uint24 _swapFee) = abi.decode(data, (bool, address, uint24)) uint256 amountOut = _zeroForOne ? amount1 : amount0; (uint112 reserve0, uint112 reserve1) = IUniswapV2Pair(msg.sender).getReserves() uint256 amountIn = OracleLibraryV2.getAmountIn(amountOut, _zeroForOne ? reserve0 : reserve1, _zeroForOne ? reserve1 : reserve0, _swapFee); IERC20(_tokenIn).transfer(msg.sender, amountIn)} 
 ```

 <span style="color: green;">File: </span> BiswapCallback.sol 13-13 
 ```solidity 
 function BiswapCall(address, uint256 amount0, uint256 amount1, bytes calldata data) external {(bool _zeroForOne, address _tokenIn, uint24 _swapFee) = abi.decode(data, (bool, address, uint24)) uint256 amountOut = _zeroForOne ? amount1 : amount0; (uint112 reserve0, uint112 reserve1) = IUniswapV2Pair(msg.sender).getReserves() uint256 amountIn = OracleLibraryV2.getAmountIn(amountOut, _zeroForOne ? reserve0 : reserve1, _zeroForOne ? reserve1 : reserve0, _swapFee); IERC20(_tokenIn).transfer(msg.sender, amountIn)} 
 ```

 <span style="color: green;">File: </span> JetSwapCallback.sol 13-13 
 ```solidity 
 function jetswapCall(address, uint256 amount0, uint256 amount1, bytes calldata data) external {(bool _zeroForOne, address _tokenIn, uint24 _swapFee) = abi.decode(data, (bool, address, uint24)) uint256 amountOut = _zeroForOne ? amount1 : amount0; (uint112 reserve0, uint112 reserve1) = IUniswapV2Pair(msg.sender).getReserves() uint256 amountIn = OracleLibraryV2.getAmountIn(amountOut, _zeroForOne ? reserve0 : reserve1, _zeroForOne ? reserve1 : reserve0, _swapFee); IERC20(_tokenIn).transfer(msg.sender, amountIn)} 
 ```

 <span style="color: green;">File: </span> TraderJoeCallback.sol 13-13 
 ```solidity 
 function joeCall(address, uint256 amount0, uint256 amount1, bytes calldata data) external {(bool _zeroForOne, address _tokenIn, uint24 _swapFee) = abi.decode(data, (bool, address, uint24)) uint256 amountOut = _zeroForOne ? amount1 : amount0; (uint112 reserve0, uint112 reserve1) = IUniswapV2Pair(msg.sender).getReserves() uint256 amountIn = OracleLibraryV2.getAmountIn(amountOut, _zeroForOne ? reserve0 : reserve1, _zeroForOne ? reserve1 : reserve0, _swapFee); IERC20(_tokenIn).transfer(msg.sender, amountIn)} 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 50-53 
 ```solidity 
 function findBestTokenToWethExecutionPrice(LimitOrderSwapRouter.TokenToWethExecutionPrice[] calldata executionPrices, bool buyOrder) external pure returns (uint256 bestPriceIndex) {if (buyOrder) {uint256 bestPrice = MAX_UINT256; for (uint256 i = 0; i < executionPrices.length;) {uint256 executionPrice = executionPrices[i].price; if (executionPrice < bestPrice && executionPrice != 0) {bestPrice = executionPrice bestPriceIndex = i} unchecked {++i}}} else {uint256 bestPrice = ZERO; for (uint256 i = 0; i < executionPrices.length;) {uint256 executionPrice = executionPrices[i].price; if (executionPrice > bestPrice) {bestPrice = executionPrice bestPriceIndex = i} unchecked {++i}}}} 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 95-98 
 ```solidity 
 function findBestTokenToTokenExecutionPrice(LimitOrderSwapRouter.TokenToTokenExecutionPrice[] calldata executionPrices, bool buyOrder) external pure returns (uint256 bestPriceIndex) {if (buyOrder) {uint256 bestPrice = MAX_UINT256; for (uint256 i = 0; i < executionPrices.length;) {uint256 executionPrice = executionPrices[i].price; if (executionPrice < bestPrice && executionPrice != 0) {bestPrice = executionPrice bestPriceIndex = i} unchecked {++i}}} else {uint256 bestPrice = ZERO; for (uint256 i = 0; i < executionPrices.length;) {uint256 executionPrice = executionPrices[i].price; if (executionPrice > bestPrice) {bestPrice = executionPrice bestPriceIndex = i} unchecked {++i}}}} 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 135-138 
 ```solidity 
 function initializeTokenToWethExecutionPrices(LimitOrderSwapRouter.SpotReserve[] calldata spotReserveAToWeth, address[] calldata lpAddressesAToWeth) external pure returns (LimitOrderSwapRouter.TokenToWethExecutionPrice[] memory) {LimitOrderSwapRouter.TokenToWethExecutionPrice[] memory executionPrices = new LimitOrderSwapRouter.TokenToWethExecutionPrice[](spotReserveAToWeth.length); {for (uint256 i = 0; i < spotReserveAToWeth.length;) {executionPrices[i] = LimitOrderSwapRouter.TokenToWethExecutionPrice(spotReserveAToWeth[i].res0, spotReserveAToWeth[i].res1, spotReserveAToWeth[i].spotPrice, lpAddressesAToWeth[i]) unchecked {++i}}} return (executionPrices);} 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 171-177 
 ```solidity 
 function initializeTokenToTokenExecutionPrices(address tokenIn, LimitOrderSwapRouter.SpotReserve[] calldata spotReserveAToWeth, address[] calldata lpAddressesAToWeth, LimitOrderSwapRouter.SpotReserve[] calldata spotReserveWethToB, address[] calldata lpAddressesWethToB) external view returns (LimitOrderSwapRouter.TokenToTokenExecutionPrice[] memory) {LimitOrderSwapRouter.TokenToTokenExecutionPrice[] memory executionPrices = new LimitOrderSwapRouter.TokenToTokenExecutionPrice[](spotReserveAToWeth.length * spotReserveWethToB.length); if (tokenIn == WETH) {for (uint256 i = 0; i < spotReserveWethToB.length;) {executionPrices[i] = LimitOrderSwapRouter.TokenToTokenExecutionPrice(0, 0, spotReserveWethToB[i].res0, spotReserveWethToB[i].res1, spotReserveWethToB[i].spotPrice, address(0), lpAddressesWethToB[i]) unchecked {++i}}} else {uint256 index = 0; for (uint256 i = 0; i < spotReserveAToWeth.length;) {for (uint256 j = 0; j < spotReserveWethToB.length;) {uint256 spotPriceFinal = uint256(_calculateTokenToWethToTokenSpotPrice(spotReserveAToWeth[i].spotPrice, spotReserveWethToB[j].spotPrice)) << 64; executionPrices[index] = LimitOrderSwapRouter.TokenToTokenExecutionPrice(spotReserveAToWeth[i].res0, spotReserveAToWeth[i].res1, spotReserveWethToB[j].res1, spotReserveWethToB[j].res0, spotPriceFinal, lpAddressesAToWeth[i], lpAddressesWethToB[j]) unchecked {++index} unchecked {++j}} unchecked {++i}}} return (executionPrices);} 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 249-252 
 ```solidity 
 function simulateTokenToTokenPriceChange(uint128 alphaX, LimitOrderSwapRouter.TokenToTokenExecutionPrice memory executionPrice) external returns (LimitOrderSwapRouter.TokenToTokenExecutionPrice memory) {if (executionPrice.aToWethReserve0 != 0 && executionPrice.aToWethReserve1 != 0) {address pool = executionPrice.lpAddressAToWeth; address token0; address token1; bool _isUniV2 = _lpIsNotUniV3(pool); {if (_isUniV2) {token0 = IUniswapV2Pair(pool).token0() token1 = IUniswapV2Pair(pool).token1()} else {token0 = IUniswapV3Pool(pool).token0() token1 = IUniswapV3Pool(pool).token1()}} uint8 tokenInDecimals = token1 == WETH ? IERC20(token0).decimals() : IERC20(token1).decimals(); uint128 amountIn = tokenInDecimals <= 18 ? uint128(alphaX * 10 ** (18 - tokenInDecimals)) : uint128(alphaX / (10 ** (tokenInDecimals - 18))); executionPrice = _simulateTokenToTokenPriceChange(amountIn, executionPrice)} else {executionPrice = _simulateWethToTokenPriceChange(alphaX, executionPrice)} return executionPrice;} 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 465-468 
 ```solidity 
 function simulateTokenToWethPriceChange(uint128 alphaX, LimitOrderSwapRouter.TokenToWethExecutionPrice memory executionPrice) external returns (LimitOrderSwapRouter.TokenToWethExecutionPrice memory) {address pool = executionPrice.lpAddressAToWeth; address token0 = IUniswapV2Pair(pool).token0(); address token1 = IUniswapV2Pair(pool).token1(); uint8 tokenInDecimals = token1 == WETH ? IERC20(token0).decimals() : IERC20(token1).decimals(); uint128 amountIn = tokenInDecimals <= 18 ? uint128(alphaX * 10 ** (18 - tokenInDecimals)) : uint128(alphaX / (10 ** (tokenInDecimals - 18))); (executionPrice.price, executionPrice.aToWethReserve0, executionPrice.aToWethReserve1) = _simulateAToBPriceChange(amountIn, executionPrice.aToWethReserve0, executionPrice.aToWethReserve1, pool, true) return executionPrice;} 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 549-555 
 ```solidity 
 function calculateAmountOutMinAToWeth(address lpAddressAToWeth, uint256 amountInOrder, uint16 taxIn, uint24 feeIn, address tokenIn) external returns (uint256 amountOutMinAToWeth) {if (!_lpIsNotUniV3(lpAddressAToWeth)) {uint256 amountInBuffer = (amountInOrder * taxIn) / 10 ** 5; uint256 amountIn = amountInOrder - amountInBuffer; address token0 = IUniswapV3Pool(lpAddressAToWeth).token0(); uint128 liquidity = IUniswapV3Pool(lpAddressAToWeth).liquidity(); int24 tickSpacing = IUniswapV3Pool(lpAddressAToWeth).tickSpacing(); (amountOutMinAToWeth) = ConveyorTickMath.simulateAmountOutOnSqrtPriceX96(token0, tokenIn, lpAddressAToWeth, amountIn, tickSpacing, liquidity, feeIn)} else {(uint112 reserve0, uint112 reserve1) = IUniswapV2Pair(lpAddressAToWeth).getReserves() if (WETH == IUniswapV2Pair(lpAddressAToWeth).token0()) {uint256 amountInBuffer = (amountInOrder * taxIn) / 10 ** 5; uint256 amountIn = amountInOrder - amountInBuffer; amountOutMinAToWeth = getAmountOut(amountIn, uint256(reserve1), uint256(reserve0))} else {uint256 amountInBuffer = (amountInOrder * taxIn) / 10 ** 5; uint256 amountIn = amountInOrder - amountInBuffer; amountOutMinAToWeth = getAmountOut(amountIn, uint256(reserve0), uint256(reserve1))}}} 
 ```

 <span style="color: green;">File: </span> UniFiCallback.sol 13-13 
 ```solidity 
 function unifiCall(address, uint256 amount0, uint256 amount1, bytes calldata data) external {(bool _zeroForOne, address _tokenIn, uint24 _swapFee) = abi.decode(data, (bool, address, uint24)) uint256 amountOut = _zeroForOne ? amount1 : amount0; (uint112 reserve0, uint112 reserve1) = IUniswapV2Pair(msg.sender).getReserves() uint256 amountIn = OracleLibraryV2.getAmountIn(amountOut, _zeroForOne ? reserve0 : reserve1, _zeroForOne ? reserve1 : reserve0, _swapFee); IERC20(_tokenIn).transfer(msg.sender, amountIn)} 
 ```

 <span style="color: green;">File: </span> DeployMainnetAggregator.s.sol 12-12 
 ```solidity 
 function run() public returns (address conveyorRouterV1) {bytes32 salt = bytes32("0x8fbb158"); bytes memory creationCode = abi.encodePacked(type(ConveyorRouterV1).creationCode, abi.encode(WETH)); vm.startBroadcast() conveyorRouterV1 = ICREATE3Factory(0x93FEC2C00BfE902F733B57c5a6CeeD7CD1384AE1).deploy(salt, creationCode) vm.stopBroadcast()} 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderRouter.sol 61-61 
 ```solidity 
 function executeSandboxMulticall(SandboxMulticall calldata sandboxMultiCall) external {uint256 lastCheckInTime = IConveyorExecutor(LIMIT_ORDER_EXECUTOR).lastCheckIn(msg.sender); if (block.timestamp - lastCheckInTime > CHECK_IN_INTERVAL) {revert ExecutorNotCheckedIn();} ISandboxLimitOrderBook(SANDBOX_LIMIT_ORDER_BOOK).executeOrdersViaSandboxMulticall(sandboxMultiCall)} 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderRouter.sol 75-75 
 ```solidity 
 function sandboxRouterCallback(SandboxMulticall calldata sandboxMulticall) external onlyLimitOrderExecutor {for (uint256 i = 0; i < sandboxMulticall.calls.length;) {Call memory sandBoxCall = sandboxMulticall.calls[i]; (bool success) = sandBoxCall.target.call(sandBoxCall.callData) if (!success) {revert SandboxCallFailed(i);} unchecked {++i}}} 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderRouter.sol 96-96 
 ```solidity 
 function uniswapV3SwapCallback(int256 amount0Delta, int256 amount1Delta, bytes calldata data) external {(bool _zeroForOne, address tokenIn, address _sender) = abi.decode(data, (bool, address, address)) uint256 amountIn = _zeroForOne ? uint256(amount0Delta) : uint256(amount1Delta); if (!(_sender == address(this))) {IERC20(tokenIn).safeTransferFrom(_sender, msg.sender, amountIn)} else {IERC20(tokenIn).safeTransfer(msg.sender, amountIn)}} 
 ```

 <span style="color: green;">File: </span> VerseCallback.sol 13-13 
 ```solidity 
 function swapsCall(address, uint256 amount0, uint256 amount1, bytes calldata data) external {(bool _zeroForOne, address _tokenIn, uint24 _swapFee) = abi.decode(data, (bool, address, uint24)) uint256 amountOut = _zeroForOne ? amount1 : amount0; (uint112 reserve0, uint112 reserve1) = IUniswapV2Pair(msg.sender).getReserves() uint256 amountIn = OracleLibraryV2.getAmountIn(amountOut, _zeroForOne ? reserve0 : reserve1, _zeroForOne ? reserve1 : reserve0, _swapFee); IERC20(_tokenIn).transfer(msg.sender, amountIn)} 
 ```

 <span style="color: green;">File: </span> ApeSwapCallback.sol 13-13 
 ```solidity 
 function apeCall(address, uint256 amount0, uint256 amount1, bytes calldata data) external {(bool _zeroForOne, address _tokenIn, uint24 _swapFee) = abi.decode(data, (bool, address, uint24)) uint256 amountOut = _zeroForOne ? amount1 : amount0; (uint112 reserve0, uint112 reserve1) = IUniswapV2Pair(msg.sender).getReserves() uint256 amountIn = OracleLibraryV2.getAmountIn(amountOut, _zeroForOne ? reserve0 : reserve1, _zeroForOne ? reserve1 : reserve0, _swapFee); IERC20(_tokenIn).transfer(msg.sender, amountIn)} 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 154-154 
 ```solidity 
 function checkIn() external {lastCheckIn[msg.sender] = block.timestamp emit ExecutorCheckIn(msg.sender, block.timestamp);} 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 162-166 
 ```solidity 
 function executeTokenToWethOrders(LimitOrderBook.LimitOrder[] calldata orders) external onlyLimitOrderRouter returns (uint256, uint256) {(SpotReserve[] memory spotReserveAToWeth, address[] memory lpAddressesAToWeth) = getAllPrices(orders[0].tokenIn, WETH, orders[0].feeIn) TokenToWethExecutionPrice[] memory executionPrices = ILimitOrderQuoter(LIMIT_ORDER_QUOTER).initializeTokenToWethExecutionPrices(spotReserveAToWeth, lpAddressesAToWeth); uint256 totalBeaconReward = 0; uint256 totalConveyorReward = 0; for (uint256 i = 0; i < orders.length;) {uint256 bestPriceIndex = ILimitOrderQuoter(LIMIT_ORDER_QUOTER).findBestTokenToWethExecutionPrice(executionPrices, orders[i].buy); {(uint256 conveyorReward, uint256 beaconReward) = _executeTokenToWethOrder(orders[i], executionPrices[bestPriceIndex]) totalBeaconReward += beaconReward totalConveyorReward += conveyorReward} executionPrices[bestPriceIndex] = ILimitOrderQuoter(LIMIT_ORDER_QUOTER).simulateTokenToWethPriceChange(uint128(orders[i].quantity), executionPrices[bestPriceIndex]) unchecked {++i}} _transferBeaconReward(totalBeaconReward, tx.origin, WETH) conveyorBalance += totalConveyorReward return (totalBeaconReward, totalConveyorReward);} 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 277-281 
 ```solidity 
 function executeTokenToTokenOrders(LimitOrderBook.LimitOrder[] calldata orders) external onlyLimitOrderRouter returns (uint256, uint256) {TokenToTokenExecutionPrice[] memory executionPrices; address tokenIn = orders[0].tokenIn; uint24 feeIn = orders[0].feeIn; uint24 feeOut = orders[0].feeOut; {(SpotReserve[] memory spotReserveAToWeth, address[] memory lpAddressesAToWeth) = getAllPrices(tokenIn, WETH, feeIn) (SpotReserve[] memory spotReserveWethToB, address[] memory lpAddressWethToB) = getAllPrices(WETH, orders[0].tokenOut, feeOut) executionPrices = ILimitOrderQuoter(LIMIT_ORDER_QUOTER).initializeTokenToTokenExecutionPrices(tokenIn, spotReserveAToWeth, lpAddressesAToWeth, spotReserveWethToB, lpAddressWethToB)} uint256 totalBeaconReward = 0; uint256 totalConveyorReward = 0; for (uint256 i = 0; i < orders.length;) {uint256 bestPriceIndex = ILimitOrderQuoter(LIMIT_ORDER_QUOTER).findBestTokenToTokenExecutionPrice(executionPrices, orders[i].buy); {(uint256 conveyorReward, uint256 beaconReward) = _executeTokenToTokenOrder(orders[i], executionPrices[bestPriceIndex]) totalBeaconReward += beaconReward totalConveyorReward += conveyorReward} executionPrices[bestPriceIndex] = ILimitOrderQuoter(LIMIT_ORDER_QUOTER).simulateTokenToTokenPriceChange(uint128(orders[i].quantity), executionPrices[bestPriceIndex]) unchecked {++i}} _transferBeaconReward(totalBeaconReward, tx.origin, WETH) conveyorBalance += totalConveyorReward return (totalBeaconReward, totalConveyorReward);} 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 419-422 
 ```solidity 
 function executeSandboxLimitOrders(SandboxLimitOrderBook.SandboxLimitOrder[] calldata orders, SandboxLimitOrderRouter.SandboxMulticall calldata sandboxMulticall) external onlySandboxLimitOrderBook nonReentrant {uint256 expectedAccumulatedFees = 0; if (sandboxMulticall.transferAddresses.length > 0) {if (sandboxMulticall.transferAddresses.length != orders.length) {revert InvalidTransferAddressArray();} for (uint256 i = 0; i < orders.length;) {uint128 fillAmount = sandboxMulticall.fillAmounts[i]; IERC20(orders[i].tokenIn).safeTransferFrom(orders[i].owner, sandboxMulticall.transferAddresses[i], fillAmount) uint256 feeRequired = ConveyorMath.mul64U(ConveyorMath.divUU(fillAmount, orders[i].amountInRemaining), orders[i].feeRemaining); if (feeRequired == 0) {revert InsufficientFillAmountSpecified(fillAmount, orders[i].amountInRemaining);} expectedAccumulatedFees += feeRequired unchecked {++i}}} else {for (uint256 i = 0; i < orders.length;) {uint128 fillAmount = sandboxMulticall.fillAmounts[i]; IERC20(orders[i].tokenIn).safeTransferFrom(orders[i].owner, SANDBOX_LIMIT_ORDER_ROUTER, fillAmount) uint256 feeRequired = ConveyorMath.mul64U(ConveyorMath.divUU(fillAmount, orders[i].amountInRemaining), orders[i].feeRemaining); if (feeRequired == 0) {revert InsufficientFillAmountSpecified(fillAmount, orders[i].amountInRemaining);} expectedAccumulatedFees += feeRequired unchecked {++i}}} uint256 contractBalancePreExecution = IERC20(WETH).balanceOf(address(this)); ISandboxLimitOrderRouter(SANDBOX_LIMIT_ORDER_ROUTER).sandboxRouterCallback(sandboxMulticall) _requireConveyorFeeIsPaid(contractBalancePreExecution, expectedAccumulatedFees)} 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 507-507 
 ```solidity 
 function withdrawConveyorFees() external nonReentrant onlyOwner {IWETH(WETH).withdraw(conveyorBalance) uint256 withdrawAmount = conveyorBalance; conveyorBalance = 0 _safeTransferETH(owner, withdrawAmount)} 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 518-518 
 ```solidity 
 function confirmTransferOwnership() external {if (msg.sender != tempOwner) {revert UnauthorizedCaller();} tempOwner = address(0) owner = msg.sender} 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 529-529 
 ```solidity 
 function transferOwnership(address newOwner) external onlyOwner {if (newOwner == address(0)) {revert InvalidAddress();} tempOwner = newOwner} 
 ```

 <span style="color: green;">File: </span> DystopiaCallback.sol 13-13 
 ```solidity 
 function hook(address, uint256 amount0, uint256 amount1, bytes calldata data) external {(bool _zeroForOne, address _tokenIn, uint24 _swapFee) = abi.decode(data, (bool, address, uint24)) uint256 amountOut = _zeroForOne ? amount1 : amount0; (uint112 reserve0, uint112 reserve1) = IUniswapV2Pair(msg.sender).getReserves() uint256 amountIn = OracleLibraryV2.getAmountIn(amountOut, _zeroForOne ? reserve0 : reserve1, _zeroForOne ? reserve1 : reserve0, _swapFee); IERC20(_tokenIn).transfer(msg.sender, amountIn)} 
 ```

 <span style="color: green;">File: </span> ConveyorFeeMath.sol 19-23 
 ```solidity 
 function calculateReward(uint128 percentFee, uint128 wethValue) public pure returns (uint128 conveyorReward, uint128 beaconReward) {uint256 totalWethReward = ConveyorMath.mul64U(percentFee, uint256(wethValue)); uint128 conveyorPercent; if (percentFee <= ZERO_POINT_ZERO_ZERO_FIVE) {int256 innerPartial = int256(uint256(ZERO_POINT_ZERO_ZERO_FIVE)) - int128(percentFee); conveyorPercent = (percentFee + ConveyorMath.div64x64(uint128(uint256(innerPartial)), uint128(2) << 64) + uint128(ZERO_POINT_ZERO_ZERO_ONE)) * 10 ** 2} else {conveyorPercent = MAX_CONVEYOR_PERCENT} if (conveyorPercent < MIN_CONVEYOR_PERCENT) {conveyorPercent = MIN_CONVEYOR_PERCENT} conveyorReward = uint128(ConveyorMath.mul64U(conveyorPercent, totalWethReward)) beaconReward = uint128(totalWethReward) - conveyorReward return (conveyorReward, beaconReward);} 
 ```

 <span style="color: green;">File: </span> DeployBSCAggregator.s.sol 14-14 
 ```solidity 
 function run() public returns (address conveyorRouterV1) {uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY"); bytes32 salt = bytes32("0x8fbb158"); bytes memory creationCode = abi.encodePacked(type(ConveyorRouterV1).creationCode, abi.encode(WBNB)); vm.startBroadcast(deployerPrivateKey) conveyorRouterV1 = ICREATE3Factory(0x93FEC2C00BfE902F733B57c5a6CeeD7CD1384AE1).deploy(salt, creationCode) vm.stopBroadcast()} 
 ```

 <span style="color: green;">File: </span> UniswapV2Callback.sol 13-13 
 ```solidity 
 function uniswapV2Call(address, uint256 amount0, uint256 amount1, bytes calldata data) external {(bool _zeroForOne, address _tokenIn, uint24 _swapFee) = abi.decode(data, (bool, address, uint24)) uint256 amountOut = _zeroForOne ? amount1 : amount0; (uint112 reserve0, uint112 reserve1) = IUniswapV2Pair(msg.sender).getReserves() uint256 amountIn = OracleLibraryV2.getAmountIn(amountOut, _zeroForOne ? reserve0 : reserve1, _zeroForOne ? reserve1 : reserve0, _swapFee); IERC20(_tokenIn).transfer(msg.sender, amountIn)} 
 ```

 <span style="color: green;">File: </span> BabySwapCallback.sol 13-13 
 ```solidity 
 function babyCall(address, uint256 amount0, uint256 amount1, bytes calldata data) external {(bool _zeroForOne, address _tokenIn, uint24 _swapFee) = abi.decode(data, (bool, address, uint24)) uint256 amountOut = _zeroForOne ? amount1 : amount0; (uint112 reserve0, uint112 reserve1) = IUniswapV2Pair(msg.sender).getReserves() uint256 amountIn = OracleLibraryV2.getAmountIn(amountOut, _zeroForOne ? reserve0 : reserve1, _zeroForOne ? reserve1 : reserve0, _swapFee); IERC20(_tokenIn).transfer(msg.sender, amountIn)} 
 ```

 <span style="color: green;">File: </span> SakeSwapCallback.sol 13-13 
 ```solidity 
 function SakeSwapCall(address, uint256 amount0, uint256 amount1, bytes calldata data) external {(bool _zeroForOne, address _tokenIn, uint24 _swapFee) = abi.decode(data, (bool, address, uint24)) uint256 amountOut = _zeroForOne ? amount1 : amount0; (uint112 reserve0, uint112 reserve1) = IUniswapV2Pair(msg.sender).getReserves() uint256 amountIn = OracleLibraryV2.getAmountIn(amountOut, _zeroForOne ? reserve0 : reserve1, _zeroForOne ? reserve1 : reserve0, _swapFee); IERC20(_tokenIn).transfer(msg.sender, amountIn)} 
 ```

 <span style="color: green;">File: </span> ConvergenceXCallback.sol 13-13 
 ```solidity 
 function swapCall(address, uint256 amount0, uint256 amount1, bytes calldata data) external {(bool _zeroForOne, address _tokenIn, uint24 _swapFee) = abi.decode(data, (bool, address, uint24)) uint256 amountOut = _zeroForOne ? amount1 : amount0; (uint112 reserve0, uint112 reserve1) = IUniswapV2Pair(msg.sender).getReserves() uint256 amountIn = OracleLibraryV2.getAmountIn(amountOut, _zeroForOne ? reserve0 : reserve1, _zeroForOne ? reserve1 : reserve0, _swapFee); IERC20(_tokenIn).transfer(msg.sender, amountIn)} 
 ```

 <span style="color: green;">File: </span> MeerkatCallback.sol 13-13 
 ```solidity 
 function MeerkatCall(address, uint256 amount0, uint256 amount1, bytes calldata data) external {(bool _zeroForOne, address _tokenIn, uint24 _swapFee) = abi.decode(data, (bool, address, uint24)) uint256 amountOut = _zeroForOne ? amount1 : amount0; (uint112 reserve0, uint112 reserve1) = IUniswapV2Pair(msg.sender).getReserves() uint256 amountIn = OracleLibraryV2.getAmountIn(amountOut, _zeroForOne ? reserve0 : reserve1, _zeroForOne ? reserve1 : reserve0, _swapFee); IERC20(_tokenIn).transfer(msg.sender, amountIn)} 
 ```

 <span style="color: green;">File: </span> DeployFantomAggregator.s.sol 11-11 
 ```solidity 
 function run() public returns (ConveyorRouterV1 conveyorRouterV1) {uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY"); vm.startBroadcast(deployerPrivateKey) conveyorRouterV1 = new ConveyorRouterV1(WFTM) vm.stopBroadcast()} 
 ```

 <span style="color: green;">File: </span> LinkSwapCallback.sol 13-13 
 ```solidity 
 function linkswapCall(address, uint256 amount0, uint256 amount1, bytes calldata data) external {(bool _zeroForOne, address _tokenIn, uint24 _swapFee) = abi.decode(data, (bool, address, uint24)) uint256 amountOut = _zeroForOne ? amount1 : amount0; (uint112 reserve0, uint112 reserve1) = IUniswapV2Pair(msg.sender).getReserves() uint256 amountIn = OracleLibraryV2.getAmountIn(amountOut, _zeroForOne ? reserve0 : reserve1, _zeroForOne ? reserve1 : reserve0, _swapFee); IERC20(_tokenIn).transfer(msg.sender, amountIn)} 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 238-238 
 ```solidity 
 function decreaseExecutionCredit(bytes32 orderId, uint128 amount) external nonReentrant {SandboxLimitOrder memory order = orderIdToSandboxLimitOrder[orderId]; if (order.orderId == bytes32(0)) {revert OrderDoesNotExist(order.orderId);} if (order.owner != msg.sender) {revert MsgSenderIsNotOrderOwner();} uint128 executionCreditRemaining = order.executionCreditRemaining; if (executionCreditRemaining < amount) {revert WithdrawAmountExceedsExecutionCredit(amount, executionCreditRemaining);} else {if (executionCreditRemaining - amount < minExecutionCredit) {revert InsufficientExecutionCredit(executionCreditRemaining - amount, minExecutionCredit);}} orderIdToSandboxLimitOrder[orderId].executionCreditRemaining = executionCreditRemaining - amount _safeTransferETH(msg.sender, amount) emit OrderExecutionCreditUpdated(orderId, executionCreditRemaining - amount);} 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 511-511 
 ```solidity 
 function cancelOrders(bytes32[] calldata orderIds) public {for (uint256 i = 0; i < orderIds.length;) {cancelOrder(orderIds[i]) unchecked {++i}}} 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 524-524 
 ```solidity 
 function cancelOrder(bytes32 orderId) public {SandboxLimitOrder memory order = orderIdToSandboxLimitOrder[orderId]; if (order.orderId == bytes32(0)) {revert OrderDoesNotExist(orderId);} if (order.owner != msg.sender) {revert MsgSenderIsNotOrderOwner();} delete orderIdToSandboxLimitOrder[orderId] delete addressToOrderIds[msg.sender][orderId] --totalOrdersPerAddress[msg.sender] decrementTotalOrdersQuantity(order.tokenIn, order.owner, order.amountInRemaining) addressToOrderIds[order.owner][order.orderId] = OrderType.CanceledSandboxLimitOrder bytes32[] memory orderIds = new bytes32[](1); orderIds[0] = order.orderId emit OrderCanceled(orderIds);} 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 560-560 
 ```solidity 
 function validateAndCancelOrder(bytes32 orderId) external nonReentrant returns (bool success) {uint256 lastCheckInTime = IConveyorExecutor(LIMIT_ORDER_EXECUTOR).lastCheckIn(msg.sender); if (block.timestamp - lastCheckInTime > CHECK_IN_INTERVAL) {revert ExecutorNotCheckedIn();} SandboxLimitOrder memory order = getSandboxLimitOrderById(orderId); if (IERC20(order.tokenIn).balanceOf(order.owner) < order.amountInRemaining) {_safeTransferETH(msg.sender, _cancelSandboxLimitOrderViaExecutor(order)) return true;} return false;} 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 620-620 
 ```solidity 
 function refreshOrder(bytes32[] calldata orderIds) external nonReentrant {uint256 lastCheckInTime = IConveyorExecutor(LIMIT_ORDER_EXECUTOR).lastCheckIn(msg.sender); if (block.timestamp - lastCheckInTime > CHECK_IN_INTERVAL) {revert ExecutorNotCheckedIn();} uint256 totalRefreshFees; for (uint256 i = 0; i < orderIds.length;) {bytes32 orderId = orderIds[i]; SandboxLimitOrder memory order = getSandboxLimitOrderById(orderId); totalRefreshFees += _refreshSandboxLimitOrder(order) unchecked {++i}} _safeTransferETH(msg.sender, totalRefreshFees)} 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 703-707 
 ```solidity 
 function executeOrdersViaSandboxMulticall(SandboxLimitOrderRouter.SandboxMulticall calldata sandboxMulticall) external onlySandboxLimitOrderRouter nonReentrant {PreSandboxExecutionState memory preSandboxExecutionState = _initializePreSandboxExecutionState(sandboxMulticall.orderIdBundles, sandboxMulticall.fillAmounts); IConveyorExecutor(LIMIT_ORDER_EXECUTOR).executeSandboxLimitOrders(preSandboxExecutionState.sandboxLimitOrders, sandboxMulticall) uint256 executionGasCompensation = _validateSandboxExecutionAndFillOrders(sandboxMulticall.orderIdBundles, sandboxMulticall.fillAmounts, preSandboxExecutionState); _safeTransferETH(tx.origin, executionGasCompensation)} 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 1178-1178 
 ```solidity 
 function getTotalOrdersValue(address token) public view returns (uint256 totalOrderValue) {bytes32 totalOrdersValueKey = keccak256(abi.encode(msg.sender, token)); return totalOrdersQuantity[totalOrdersValueKey];} 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 1183-1183 
 ```solidity 
 function getAllOrderIdsLength(address orderOwner) public view returns (uint256) {return addressToAllOrderIds[orderOwner].length;} 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 1187-1187 
 ```solidity 
 function getSandboxLimitOrderRouterAddress() public view returns (address) {return SANDBOX_LIMIT_ORDER_ROUTER;} 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 1191-1191 
 ```solidity 
 function getSandboxLimitOrderById(bytes32 orderId) public view returns (SandboxLimitOrder memory) {SandboxLimitOrder memory order = orderIdToSandboxLimitOrder[orderId]; if (order.orderId == bytes32(0)) {revert OrderDoesNotExist(orderId);} return order;} 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 1206-1210 
 ```solidity 
 function getOrderIds(address orderOwner, OrderType targetOrderType, uint256 orderOffset, uint256 length) public view returns (bytes32[] memory) {bytes32[] memory allOrderIds = addressToAllOrderIds[orderOwner]; uint256 orderIdIndex = 0; bytes32[] memory orderIds = new bytes32[](allOrderIds.length); uint256 orderOffsetSlot; assembly {orderOffsetSlot := add(add(allOrderIds, 0x20), mul(orderOffset, 0x20))} for (uint256 i = 0; i < length;) {bytes32 orderId; assembly {orderId := mload(orderOffsetSlot) orderOffsetSlot := add(orderOffsetSlot, 0x20)} OrderType orderType = addressToOrderIds[orderOwner][orderId]; if (orderType == targetOrderType) {orderIds[orderIdIndex] = orderId ++orderIdIndex} unchecked {++i}} assembly {mstore(orderIds, orderIdIndex)} return orderIds;} 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 1251-1251 
 ```solidity 
 function setMinExecutionCredit(uint256 newMinExecutionCredit) external onlyOwner {uint256 oldMinExecutionCredit = minExecutionCredit; minExecutionCredit = newMinExecutionCredit emit MinExecutionCreditUpdated(newMinExecutionCredit, oldMinExecutionCredit);} 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 1258-1258 
 ```solidity 
 function confirmTransferOwnership() external {if (msg.sender != tempOwner) {revert MsgSenderIsNotTempOwner();} owner = msg.sender tempOwner = address(0)} 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 1267-1267 
 ```solidity 
 function transferOwnership(address newOwner) external onlyOwner {if (newOwner == address(0)) {revert InvalidAddress();} tempOwner = newOwner} 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 168-168 
 ```solidity 
 function decreaseExecutionCredit(bytes32 orderId, uint128 amount) external nonReentrant {LimitOrder memory order = orderIdToLimitOrder[orderId]; if (order.orderId == bytes32(0)) {revert OrderDoesNotExist(order.orderId);} if (order.owner != msg.sender) {revert MsgSenderIsNotOrderOwner();} uint128 executionCredit = order.executionCredit; if (executionCredit < amount) {revert WithdrawAmountExceedsExecutionCredit(amount, executionCredit);} if (executionCredit - amount < minExecutionCredit) {revert InsufficientExecutionCredit(executionCredit - amount, minExecutionCredit);} orderIdToLimitOrder[orderId].executionCredit = executionCredit - amount _safeTransferETH(msg.sender, amount) emit OrderExecutionCreditUpdated(orderId, executionCredit - amount);} 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 224-224 
 ```solidity 
 function getLimitOrderById(bytes32 orderId) public view returns (LimitOrder memory) {LimitOrder memory order = orderIdToLimitOrder[orderId]; if (order.orderId == bytes32(0)) {revert OrderDoesNotExist(orderId);} return order;} 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 449-449 
 ```solidity 
 function cancelOrder(bytes32 orderId) public {LimitOrder memory order = orderIdToLimitOrder[orderId]; if (order.orderId == bytes32(0)) {revert OrderDoesNotExist(orderId);} if (order.owner != msg.sender) {revert MsgSenderIsNotOrderOwner();} delete orderIdToLimitOrder[orderId] delete addressToOrderIds[msg.sender][orderId] --totalOrdersPerAddress[msg.sender] _decrementTotalOrdersQuantity(order.tokenIn, order.owner, order.quantity) addressToOrderIds[order.owner][order.orderId] = OrderType.CanceledLimitOrder bytes32[] memory orderIds = new bytes32[](1); orderIds[0] = order.orderId emit OrderCanceled(orderIds);} 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 483-483 
 ```solidity 
 function cancelOrders(bytes32[] calldata orderIds) public {for (uint256 i = 0; i < orderIds.length;) {cancelOrder(orderIds[i]) unchecked {++i}}} 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 537-537 
 ```solidity 
 function getTotalOrdersValue(address token) public view returns (uint256 totalOrderValue) {bytes32 totalOrdersValueKey = keccak256(abi.encode(msg.sender, token)); return totalOrdersQuantity[totalOrdersValueKey];} 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 560-560 
 ```solidity 
 function getAllOrderIdsLength(address _owner) public view returns (uint256) {return addressToAllOrderIds[_owner].length;} 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 570-574 
 ```solidity 
 function getOrderIds(address _owner, OrderType targetOrderType, uint256 orderOffset, uint256 length) public view returns (bytes32[] memory) {bytes32[] memory allOrderIds = addressToAllOrderIds[_owner]; uint256 orderIdIndex = 0; bytes32[] memory orderIds = new bytes32[](allOrderIds.length); uint256 orderOffsetSlot; assembly {orderOffsetSlot := add(add(allOrderIds, 0x20), mul(orderOffset, 0x20))} for (uint256 i = 0; i < length;) {bytes32 orderId; assembly {orderId := mload(orderOffsetSlot) orderOffsetSlot := add(orderOffsetSlot, 0x20)} OrderType orderType = addressToOrderIds[_owner][orderId]; if (orderType == targetOrderType) {orderIds[orderIdIndex] = orderId ++orderIdIndex} unchecked {++i}} assembly {mstore(orderIds, orderIdIndex)} return orderIds;} 
 ```

 <span style="color: green;">File: </span> DeployPolygonAggregator.s.sol 12-12 
 ```solidity 
 function run() public returns (address conveyorRouterV1) {bytes32 salt = bytes32("0x8fbb158"); bytes memory creationCode = abi.encodePacked(type(ConveyorRouterV1).creationCode, abi.encode(WMATIC)); vm.startBroadcast() conveyorRouterV1 = ICREATE3Factory(0x93FEC2C00BfE902F733B57c5a6CeeD7CD1384AE1).deploy(salt, creationCode) vm.stopBroadcast()} 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 172-172 
 ```solidity 
 function calculateFee(uint128 amountIn, address usdc, address weth) public view returns (uint128) {if (amountIn == 0) {revert AmountInIsZero();} (SpotReserve memory _spRes) = _calculateV2SpotPrice(weth, usdc, dexes[0].factoryAddress) uint256 spotPrice = _spRes.spotPrice; uint256 amountInUSDCDollarValue = ConveyorMath.mul128U(spotPrice, amountIn) / uint256(10 ** 18); if (amountInUSDCDollarValue >= 1000000) {return 4611686018427388;} uint128 numerator = 4150517416584649000; uint128 exponent = uint128(ConveyorMath.divUU(amountInUSDCDollarValue, 100000)); if (exponent >= 0x400000000000000000) {return 4611686018427388;} uint128 denominator = ConveyorMath.exp(exponent); uint128 rationalFraction = ConveyorMath.div64x64(numerator, denominator); return ConveyorMath.add64x64(rationalFraction, 461168601842738800) / 10 ** 2;} 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 364-364 
 ```solidity 
 function uniswapV3SwapCallback(int256 amount0Delta, int256 amount1Delta, bytes calldata data) external {(uint256 amountOutMin, bool _zeroForOne, address tokenIn, address tokenOut, uint24 fee, address _sender) = abi.decode(data, (uint256, bool, address, address, uint24, address)) address poolAddress = IUniswapV3Factory(UNISWAP_V3_FACTORY).getPool(tokenIn, tokenOut, fee); if (msg.sender != poolAddress) {revert UnauthorizedUniswapV3CallbackCaller();} if (_zeroForOne) {uniV3AmountOut = uint256(-amount1Delta)} else {uniV3AmountOut = uint256(-amount0Delta)} if (uniV3AmountOut < amountOutMin) {revert InsufficientOutputAmount(uniV3AmountOut, amountOutMin);} uint256 amountIn = _zeroForOne ? uint256(amount0Delta) : uint256(amount1Delta); if (!(_sender == address(this))) {IERC20(tokenIn).safeTransferFrom(_sender, poolAddress, amountIn)} else {IERC20(tokenIn).safeTransfer(poolAddress, amountIn)}} 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 577-581 
 ```solidity 
 function getAllPrices(address token0, address token1, uint24 FEE) public view returns (SpotReserve[] memory prices, address[] memory lps) {if (token0 != token1) {SpotReserve[] memory _spotPrices = new SpotReserve[](dexes.length); address[] memory _lps = new address[](dexes.length); for (uint256 i = 0; i < dexes.length;) {if (dexes[i].isUniV2) {{(SpotReserve memory spotPrice, address poolAddress) = _calculateV2SpotPrice(token0, token1, dexes[i].factoryAddress) if (spotPrice.spotPrice != 0) {_spotPrices[i] = spotPrice _lps[i] = poolAddress}}} else {{{(SpotReserve memory spotPrice, address poolAddress) = _calculateV3SpotPrice(token0, token1, FEE, dexes[i].factoryAddress) if (spotPrice.spotPrice != 0) {_lps[i] = poolAddress _spotPrices[i] = spotPrice}}}} unchecked {++i}} return (_spotPrices, _lps);} else {SpotReserve[] memory _spotPrices = new SpotReserve[](dexes.length); address[] memory _lps = new address[](dexes.length); return (_spotPrices, _lps);}} 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 78-78 
 ```solidity 
 function refreshOrder(bytes32[] calldata orderIds) external nonReentrant {uint256 lastCheckInTime = IConveyorExecutor(LIMIT_ORDER_EXECUTOR).lastCheckIn(msg.sender); if (block.timestamp - lastCheckInTime > CHECK_IN_INTERVAL) {revert ExecutorNotCheckedIn();} uint256 totalRefreshFees; for (uint256 i = 0; i < orderIds.length;) {bytes32 orderId = orderIds[i]; LimitOrder memory order = getLimitOrderById(orderId); totalRefreshFees += _refreshLimitOrder(order) unchecked {++i}} _safeTransferETH(msg.sender, totalRefreshFees)} 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 153-153 
 ```solidity 
 function validateAndCancelOrder(bytes32 orderId) external nonReentrant returns (bool success) {uint256 lastCheckInTime = IConveyorExecutor(LIMIT_ORDER_EXECUTOR).lastCheckIn(msg.sender); if (block.timestamp - lastCheckInTime > CHECK_IN_INTERVAL) {revert ExecutorNotCheckedIn();} LimitOrder memory order = getLimitOrderById(orderId); if (IERC20(order.tokenIn).balanceOf(order.owner) < order.quantity) {_safeTransferETH(msg.sender, _cancelLimitOrderViaExecutor(order)) return true;} return false;} 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 266-266 
 ```solidity 
 function executeLimitOrders(bytes32[] calldata orderIds) external nonReentrant onlyEOA {uint256 lastCheckInTime = IConveyorExecutor(LIMIT_ORDER_EXECUTOR).lastCheckIn(msg.sender); if (block.timestamp - lastCheckInTime > CHECK_IN_INTERVAL) {revert ExecutorNotCheckedIn();} if (orderIds.length == 0) {revert InvalidCalldata();} LimitOrder[] memory orders = new LimitOrder[](orderIds.length); for (uint256 i = 0; i < orderIds.length;) {orders[i] = getLimitOrderById(orderIds[i]) if (orders[i].orderId == bytes32(0)) {revert OrderDoesNotExist(orderIds[i]);} unchecked {++i}} bool isStoplossExecution = orders[0].stoploss; if (isStoplossExecution) {if (msg.sender != tx.origin) {revert NonEOAStoplossExecution();}} if (orders.length > 1) {_validateOrderSequencing(orders)} uint256 totalBeaconReward; uint256 totalConveyorReward; if (orders[0].tokenOut == WETH) {(totalBeaconReward, totalConveyorReward) = IConveyorExecutor(LIMIT_ORDER_EXECUTOR).executeTokenToWethOrders(orders)} else {(totalBeaconReward, totalConveyorReward) = IConveyorExecutor(LIMIT_ORDER_EXECUTOR).executeTokenToTokenOrders(orders)} for (uint256 i = 0; i < orderIds.length;) {bytes32 orderId = orderIds[i]; _resolveCompletedOrder(orderId) unchecked {++i}} emit OrderFilled(orderIds); uint256 executionGasCompensation; for (uint256 i = 0; i < orders.length;) {executionGasCompensation += orders[i].executionCredit unchecked {++i}} _safeTransferETH(tx.origin, executionGasCompensation)} 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 350-350 
 ```solidity 
 function confirmTransferOwnership() external {if (msg.sender != tempOwner) {revert MsgSenderIsNotTempOwner();} owner = msg.sender tempOwner = address(0)} 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 359-359 
 ```solidity 
 function transferOwnership(address newOwner) external onlyOwner {if (newOwner == address(0)) {revert InvalidAddress();} tempOwner = newOwner} 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 366-366 
 ```solidity 
 function setMinExecutionCredit(uint256 newMinExecutionCredit) external onlyOwner {uint256 oldMinExecutionCredit = minExecutionCredit; minExecutionCredit = newMinExecutionCredit emit MinExecutionCreditUpdated(minExecutionCredit, oldMinExecutionCredit);} 
 ```

 <span style="color: green;">File: </span> DeployArbitrumAggregator.s.sol 11-11 
 ```solidity 
 function run() public returns (ConveyorRouterV1 conveyorRouterV1) {uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY"); vm.startBroadcast(deployerPrivateKey) conveyorRouterV1 = new ConveyorRouterV1(WETH) vm.stopBroadcast()} 
 ```

 <span style="color: green;">File: </span> UniswapInterfaceMulticall.sol 19-19 
 ```solidity 
 function getCurrentBlockTimestamp() public view returns (uint256 timestamp) {timestamp = block.timestamp} 
 ```

 <span style="color: green;">File: </span> UniswapInterfaceMulticall.sol 23-23 
 ```solidity 
 function getEthBalance(address addr) public view returns (uint256 balance) {balance = addr.balance} 
 ```

 <span style="color: green;">File: </span> UniswapInterfaceMulticall.sol 27-27 
 ```solidity 
 function multicall(Call[] memory calls) public returns (uint256 blockNumber, Result[] memory returnData) {blockNumber = block.number returnData = new Result[](calls.length) for (uint256 i = 0; i < calls.length; i++) {(address target, uint256 gasLimit, bytes memory callData) = (calls[i].target, calls[i].gasLimit, calls[i].callData) uint256 gasLeftBefore = gasleft(); (bool success, bytes memory ret) = target.call{gas: gasLimit}(callData) uint256 gasUsed = gasLeftBefore - gasleft(); returnData[i] = Result(success, gasUsed, ret)}} 
 ```

 <span style="color: green;">File: </span> MdexSwapCallback.sol 13-13 
 ```solidity 
 function swapV2Call(address, uint256 amount0, uint256 amount1, bytes calldata data) external {(bool _zeroForOne, address _tokenIn, uint24 _swapFee) = abi.decode(data, (bool, address, uint24)) uint256 amountOut = _zeroForOne ? amount1 : amount0; (uint112 reserve0, uint112 reserve1) = IUniswapV2Pair(msg.sender).getReserves() uint256 amountIn = OracleLibraryV2.getAmountIn(amountOut, _zeroForOne ? reserve0 : reserve1, _zeroForOne ? reserve1 : reserve0, _swapFee); IERC20(_tokenIn).transfer(msg.sender, amountIn)} 
 ```

 <span style="color: green;">File: </span> DeployOptimismAggregator.s.sol 11-11 
 ```solidity 
 function run() public returns (ConveyorRouterV1 conveyorRouterV1) {uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY"); vm.startBroadcast(deployerPrivateKey) conveyorRouterV1 = new ConveyorRouterV1(WETH) vm.stopBroadcast()} 
 ```

 <span style="color: green;">File: </span> WaultSwapCallback.sol 13-13 
 ```solidity 
 function waultSwapCall(address, uint256 amount0, uint256 amount1, bytes calldata data) external {(bool _zeroForOne, address _tokenIn, uint24 _swapFee) = abi.decode(data, (bool, address, uint24)) uint256 amountOut = _zeroForOne ? amount1 : amount0; (uint112 reserve0, uint112 reserve1) = IUniswapV2Pair(msg.sender).getReserves() uint256 amountIn = OracleLibraryV2.getAmountIn(amountOut, _zeroForOne ? reserve0 : reserve1, _zeroForOne ? reserve1 : reserve0, _swapFee); IERC20(_tokenIn).transfer(msg.sender, amountIn)} 
 ```

 <span style="color: green;">File: </span> UniswapV3Callback.sol 11-11 
 ```solidity 
 function uniswapV3SwapCallback(int256 amount0Delta, int256 amount1Delta, bytes calldata data) external {assembly {let freeMemoryPointer := mload(0x40) let tokenIn := calldataload(add(data.offset, 0x20)) mstore(freeMemoryPointer, 0xa9059cbb00000000000000000000000000000000000000000000000000000000) mstore(add(freeMemoryPointer, 4), and(caller(), 0xffffffffffffffffffffffffffffffffffffffff)) switch slt(amount0Delta, 0) case 0 {mstore(add(freeMemoryPointer, 36), amount0Delta)} default {mstore(add(freeMemoryPointer, 36), amount1Delta)} if iszero(and(or(and(eq(mload(0), 1), gt(returndatasize(), 31)), iszero(returndatasize())), call(gas(), tokenIn, 0, freeMemoryPointer, 68, 0, 32))) {revert(0, 0)}}} 
 ```

 <span style="color: green;">File: </span> DefiSwapCallback.sol 13-13 
 ```solidity 
 function croDefiSwapCall(address, uint256 amount0, uint256 amount1, bytes calldata data) external {(bool _zeroForOne, address _tokenIn, uint24 _swapFee) = abi.decode(data, (bool, address, uint24)) uint256 amountOut = _zeroForOne ? amount1 : amount0; (uint112 reserve0, uint112 reserve1) = IUniswapV2Pair(msg.sender).getReserves() uint256 amountIn = OracleLibraryV2.getAmountIn(amountOut, _zeroForOne ? reserve0 : reserve1, _zeroForOne ? reserve1 : reserve0, _swapFee); IERC20(_tokenIn).transfer(msg.sender, amountIn)} 
 ```

 <span style="color: green;">File: </span> DXSwapCallback.sol 13-13 
 ```solidity 
 function DXswapCall(address, uint256 amount0, uint256 amount1, bytes calldata data) external {(bool _zeroForOne, address _tokenIn, uint24 _swapFee) = abi.decode(data, (bool, address, uint24)) uint256 amountOut = _zeroForOne ? amount1 : amount0; (uint112 reserve0, uint112 reserve1) = IUniswapV2Pair(msg.sender).getReserves() uint256 amountIn = OracleLibraryV2.getAmountIn(amountOut, _zeroForOne ? reserve0 : reserve1, _zeroForOne ? reserve1 : reserve0, _swapFee); IERC20(_tokenIn).transfer(msg.sender, amountIn)} 
 ```

 <span style="color: green;">File: </span> PancakeV3Callback.sol 11-11 
 ```solidity 
 function pancakeV3SwapCallback(int256 amount0Delta, int256 amount1Delta, bytes calldata data) external {(bool _zeroForOne, address _tokenIn, address _sender) = abi.decode(data, (bool, address, address)) uint256 amountIn = _zeroForOne ? uint256(amount0Delta) : uint256(amount1Delta); if (!(_sender == address(this))) {IERC20(_tokenIn).transferFrom(_sender, msg.sender, amountIn)} else {IERC20(_tokenIn).transfer(msg.sender, amountIn)}} 
 ```

 <span style="color: green;">File: </span> AlgebraCallback.sol 11-11 
 ```solidity 
 function algebraSwapCallback(int256 amount0Delta, int256 amount1Delta, bytes calldata data) external {(bool _zeroForOne, address _tokenIn, address _sender) = abi.decode(data, (bool, address, address)) uint256 amountIn = _zeroForOne ? uint256(amount0Delta) : uint256(amount1Delta); if (!(_sender == address(this))) {IERC20(_tokenIn).transferFrom(_sender, msg.sender, amountIn)} else {IERC20(_tokenIn).transfer(msg.sender, amountIn)}} 
 ```

 <span style="color: green;">File: </span> DeployAvalancheAggregator.s.sol 11-11 
 ```solidity 
 function run() public returns (ConveyorRouterV1 conveyorRouterV1) {uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY"); vm.startBroadcast(deployerPrivateKey) conveyorRouterV1 = new ConveyorRouterV1(WAVAX) vm.stopBroadcast()} 
 ```

 <span style="color: green;">File: </span> CafeSwapCallback.sol 13-13 
 ```solidity 
 function cafeCall(address, uint256 amount0, uint256 amount1, bytes calldata data) external {(bool _zeroForOne, address _tokenIn, uint24 _swapFee) = abi.decode(data, (bool, address, uint24)) uint256 amountOut = _zeroForOne ? amount1 : amount0; (uint112 reserve0, uint112 reserve1) = IUniswapV2Pair(msg.sender).getReserves() uint256 amountIn = OracleLibraryV2.getAmountIn(amountOut, _zeroForOne ? reserve0 : reserve1, _zeroForOne ? reserve1 : reserve0, _swapFee); IERC20(_tokenIn).transfer(msg.sender, amountIn)} 
 ```

 <span style="color: green;">File: </span> DeployTest.s.sol 14-14 
 ```solidity 
 function run() public {bytes32 salt = bytes32("0x3rlk7N6qpQ"); bytes memory creationCode = abi.encodePacked(type(ConveyorRouterV1).creationCode, abi.encode(GOERLI_WETH)); vm.startBroadcast() ICREATE3Factory(0x93FEC2C00BfE902F733B57c5a6CeeD7CD1384AE1).deploy(salt, creationCode) vm.stopBroadcast()} 
 ```

 <span style="color: green;">File: </span> KyberSwapV3Callback.sol 11-11 
 ```solidity 
 function swapCallback(int256 amount0Delta, int256 amount1Delta, bytes calldata data) external {(bool _zeroForOne, address _tokenIn, address _sender) = abi.decode(data, (bool, address, address)) uint256 amountIn = _zeroForOne ? uint256(amount0Delta) : uint256(amount1Delta); if (!(_sender == address(this))) {IERC20(_tokenIn).transferFrom(_sender, msg.sender, amountIn)} else {IERC20(_tokenIn).transfer(msg.sender, amountIn)}} 
 ```

 <span style="color: green;">File: </span> ElkSwapCallback.sol 13-13 
 ```solidity 
 function elkCall(address, uint256 amount0, uint256 amount1, bytes calldata data) external {(bool _zeroForOne, address _tokenIn, uint24 _swapFee) = abi.decode(data, (bool, address, uint24)) uint256 amountOut = _zeroForOne ? amount1 : amount0; (uint112 reserve0, uint112 reserve1) = IUniswapV2Pair(msg.sender).getReserves() uint256 amountIn = OracleLibraryV2.getAmountIn(amountOut, _zeroForOne ? reserve0 : reserve1, _zeroForOne ? reserve1 : reserve0, _swapFee); IERC20(_tokenIn).transfer(msg.sender, amountIn)} 
 ``` 
 </details>

 <details open> 
 <summary> 
 <a name=[G-12]></a> [G-12] 
 <h3> Consider marking constants as private - Instances: 13 </h3> 
 </summary>
 
 &nbsp; Consider marking constant variables in storage as private to save gas (unless a constant variable should be easily accessible by another protocol or offchain logic). 
 &nbsp; <details> &nbsp; 
 <summary> 
 &nbsp; &nbsp; Private Constant - Gas Report Savings: ~22 
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

### Gas Report

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
 

 <span style="color: green;">File: </span> ConveyorFeeMath.sol 9-9 
 ```solidity 
 uint128 constant ZERO_POINT_ZERO_ZERO_FIVE = 92233720368547760; 
 ```

 <span style="color: green;">File: </span> ConveyorFeeMath.sol 10-10 
 ```solidity 
 uint128 constant ZERO_POINT_ZERO_ZERO_ONE = 18446744073709550; 
 ```

 <span style="color: green;">File: </span> ConveyorFeeMath.sol 11-11 
 ```solidity 
 uint128 constant MAX_CONVEYOR_PERCENT = 110680464442257300 * 10 ** 2; 
 ```

 <span style="color: green;">File: </span> ConveyorFeeMath.sol 12-12 
 ```solidity 
 uint128 constant MIN_CONVEYOR_PERCENT = 7378697629483821000; 
 ```

 <span style="color: green;">File: </span> DeployAvalancheAggregator.s.sol 9-9 
 ```solidity 
 address constant WAVAX = 0xB31f66AA3C1e785363F0875A1B74E27b85FD66c7; 
 ```

 <span style="color: green;">File: </span> DeployMainnetAggregator.s.sol 10-10 
 ```solidity 
 address constant WETH = 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2; 
 ```

 <span style="color: green;">File: </span> DeployBSCAggregator.s.sol 12-12 
 ```solidity 
 address constant WBNB = 0xbb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c; 
 ```

 <span style="color: green;">File: </span> DeployFantomAggregator.s.sol 9-9 
 ```solidity 
 address constant WFTM = 0x21be370D5312f44cB42ce377BC9b8a0cEF1A4C83; 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 42-42 
 ```solidity 
 uint256 public constant CHECK_IN_INTERVAL = 1 days; 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderRouter.sol 22-22 
 ```solidity 
 uint256 public constant CHECK_IN_INTERVAL = 1 days; 
 ```

 <span style="color: green;">File: </span> DeployTest.s.sol 12-12 
 ```solidity 
 address constant GOERLI_WETH = 0xdD69DB25F6D620A7baD3023c5d32761D353D3De9; 
 ```

 <span style="color: green;">File: </span> ConveyorTickMath.sol 29-29 
 ```solidity 
 uint256 internal constant Q96 = 0x1000000000000000000000000; 
 ```

 <span style="color: green;">File: </span> DeployOptimismAggregator.s.sol 9-9 
 ```solidity 
 address constant WETH = 0x4200000000000000000000000000000000000006; 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 20-20 
 ```solidity 
 uint256 public constant CHECK_IN_INTERVAL = 1 days; 
 ```

 <span style="color: green;">File: </span> DeployPolygonAggregator.s.sol 10-10 
 ```solidity 
 address constant WMATIC = 0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270; 
 ```

 <span style="color: green;">File: </span> DeployArbitrumAggregator.s.sol 9-9 
 ```solidity 
 address constant WETH = 0x82aF49447D8a07e3bd95BD0d56f35241523fBab1; 
 ``` 
 </details>

 <details open> 
 <summary> 
 <a name=[G-13]></a> [G-13] 
 <h3> Avoid Reading From Storage in a for loop - Instances: 3 </h3> 
 </summary>
 
 &nbsp;  
 &nbsp; <details> &nbsp; 
 <summary> 
 &nbsp; &nbsp;  Savings: ~0 
  </summary> 
  
 </details> 
 

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 128-143 
 ```solidity 
 for (uint256 i = 0; i < _dexFactories.length; ++i) {if (i == 0) {require(_isUniV2[i], "First Dex must be uniswap v2")} require(_dexFactories[i] != address(0), "Zero values in constructor") dexes.push(Dex({factoryAddress: _dexFactories[i], isUniV2: _isUniV2[i]})) address uniswapV3Factory; if (!_isUniV2[i]) {uniswapV3Factory = _dexFactories[i]} UNISWAP_V3_FACTORY = uniswapV3Factory} 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 589-621 
 ```solidity 
 for (uint256 i = 0; i < dexes.length;) {if (dexes[i].isUniV2) {{(SpotReserve memory spotPrice, address poolAddress) = _calculateV2SpotPrice(token0, token1, dexes[i].factoryAddress) if (spotPrice.spotPrice != 0) {_spotPrices[i] = spotPrice _lps[i] = poolAddress}}} else {{{(SpotReserve memory spotPrice, address poolAddress) = _calculateV3SpotPrice(token0, token1, FEE, dexes[i].factoryAddress) if (spotPrice.spotPrice != 0) {_lps[i] = poolAddress _spotPrices[i] = spotPrice}}}} unchecked {++i}} 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 589-621 
 ```solidity 
 for (uint256 i = 0; i < dexes.length;) {if (dexes[i].isUniV2) {{(SpotReserve memory spotPrice, address poolAddress) = _calculateV2SpotPrice(token0, token1, dexes[i].factoryAddress) if (spotPrice.spotPrice != 0) {_spotPrices[i] = spotPrice _lps[i] = poolAddress}}} else {{{(SpotReserve memory spotPrice, address poolAddress) = _calculateV3SpotPrice(token0, token1, FEE, dexes[i].factoryAddress) if (spotPrice.spotPrice != 0) {_lps[i] = poolAddress _spotPrices[i] = spotPrice}}}} unchecked {++i}} 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 589-621 
 ```solidity 
 for (uint256 i = 0; i < dexes.length;) {if (dexes[i].isUniV2) {{(SpotReserve memory spotPrice, address poolAddress) = _calculateV2SpotPrice(token0, token1, dexes[i].factoryAddress) if (spotPrice.spotPrice != 0) {_spotPrices[i] = spotPrice _lps[i] = poolAddress}}} else {{{(SpotReserve memory spotPrice, address poolAddress) = _calculateV3SpotPrice(token0, token1, FEE, dexes[i].factoryAddress) if (spotPrice.spotPrice != 0) {_lps[i] = poolAddress _spotPrices[i] = spotPrice}}}} unchecked {++i}} 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 589-621 
 ```solidity 
 for (uint256 i = 0; i < dexes.length;) {if (dexes[i].isUniV2) {{(SpotReserve memory spotPrice, address poolAddress) = _calculateV2SpotPrice(token0, token1, dexes[i].factoryAddress) if (spotPrice.spotPrice != 0) {_spotPrices[i] = spotPrice _lps[i] = poolAddress}}} else {{{(SpotReserve memory spotPrice, address poolAddress) = _calculateV3SpotPrice(token0, token1, FEE, dexes[i].factoryAddress) if (spotPrice.spotPrice != 0) {_lps[i] = poolAddress _spotPrices[i] = spotPrice}}}} unchecked {++i}} 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 317-428 
 ```solidity 
 for (uint256 i = 0; i < orderGroup.length;) {SandboxLimitOrder memory newOrder = orderGroup[i]; updatedTotalOrdersValue += newOrder.amountInRemaining uint256 relativeWethValue; {if (!(newOrder.tokenIn == WETH)) {(LimitOrderSwapRouter.SpotReserve[] memory spRes) = ILimitOrderSwapRouter(LIMIT_ORDER_EXECUTOR).getAllPrices(newOrder.tokenIn, WETH, 500) uint256 tokenAWethSpotPrice; for (uint256 k = 0; k < spRes.length;) {if (spRes[k].spotPrice != 0) {tokenAWethSpotPrice = spRes[k].spotPrice break;} unchecked {++k}} if (tokenAWethSpotPrice == 0) {revert InvalidInputTokenForOrderPlacement();} if (!(tokenAWethSpotPrice == 0)) {uint8 tokenInDecimals = IERC20(newOrder.tokenIn).decimals(); relativeWethValue = tokenInDecimals <= 18 ? ConveyorMath.mul128U(tokenAWethSpotPrice, newOrder.amountInRemaining) * 10 ** (18 - tokenInDecimals) : ConveyorMath.mul128U(tokenAWethSpotPrice, newOrder.amountInRemaining) / 10 ** (tokenInDecimals - 18)}} else {relativeWethValue = newOrder.amountInRemaining} if (relativeWethValue < MIN_ORDER_VALUE_IN_WETH) {revert InsufficientOrderInputValue();} uint128 minFeeReceived = uint128(ConveyorMath.mul64U(ILimitOrderSwapRouter(LIMIT_ORDER_EXECUTOR).calculateFee(uint128(relativeWethValue), USDC, WETH), relativeWethValue)); newOrder.feeRemaining = minFeeReceived} if ((orderToken != newOrder.tokenIn)) {revert IncongruentInputTokenInOrderGroup(newOrder.tokenIn, orderToken);} if (tokenBalance < updatedTotalOrdersValue) {revert InsufficientWalletBalance(msg.sender, tokenBalance, updatedTotalOrdersValue);} bytes32 orderId = keccak256(abi.encode(orderNonce, block.timestamp)); unchecked {orderNonce += 2} newOrder.owner = msg.sender newOrder.orderId = orderId newOrder.lastRefreshTimestamp = uint32(block.timestamp) cumulativeExecutionCredit += newOrder.executionCreditRemaining orderIdToSandboxLimitOrder[orderId] = newOrder addressToOrderIds[msg.sender][orderId] = OrderType.PendingSandboxLimitOrder ++totalOrdersPerAddress[msg.sender] orderIds[i] = orderId addressToAllOrderIds[msg.sender].push(orderId) unchecked {++i}} 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 317-428 
 ```solidity 
 for (uint256 i = 0; i < orderGroup.length;) {SandboxLimitOrder memory newOrder = orderGroup[i]; updatedTotalOrdersValue += newOrder.amountInRemaining uint256 relativeWethValue; {if (!(newOrder.tokenIn == WETH)) {(LimitOrderSwapRouter.SpotReserve[] memory spRes) = ILimitOrderSwapRouter(LIMIT_ORDER_EXECUTOR).getAllPrices(newOrder.tokenIn, WETH, 500) uint256 tokenAWethSpotPrice; for (uint256 k = 0; k < spRes.length;) {if (spRes[k].spotPrice != 0) {tokenAWethSpotPrice = spRes[k].spotPrice break;} unchecked {++k}} if (tokenAWethSpotPrice == 0) {revert InvalidInputTokenForOrderPlacement();} if (!(tokenAWethSpotPrice == 0)) {uint8 tokenInDecimals = IERC20(newOrder.tokenIn).decimals(); relativeWethValue = tokenInDecimals <= 18 ? ConveyorMath.mul128U(tokenAWethSpotPrice, newOrder.amountInRemaining) * 10 ** (18 - tokenInDecimals) : ConveyorMath.mul128U(tokenAWethSpotPrice, newOrder.amountInRemaining) / 10 ** (tokenInDecimals - 18)}} else {relativeWethValue = newOrder.amountInRemaining} if (relativeWethValue < MIN_ORDER_VALUE_IN_WETH) {revert InsufficientOrderInputValue();} uint128 minFeeReceived = uint128(ConveyorMath.mul64U(ILimitOrderSwapRouter(LIMIT_ORDER_EXECUTOR).calculateFee(uint128(relativeWethValue), USDC, WETH), relativeWethValue)); newOrder.feeRemaining = minFeeReceived} if ((orderToken != newOrder.tokenIn)) {revert IncongruentInputTokenInOrderGroup(newOrder.tokenIn, orderToken);} if (tokenBalance < updatedTotalOrdersValue) {revert InsufficientWalletBalance(msg.sender, tokenBalance, updatedTotalOrdersValue);} bytes32 orderId = keccak256(abi.encode(orderNonce, block.timestamp)); unchecked {orderNonce += 2} newOrder.owner = msg.sender newOrder.orderId = orderId newOrder.lastRefreshTimestamp = uint32(block.timestamp) cumulativeExecutionCredit += newOrder.executionCreditRemaining orderIdToSandboxLimitOrder[orderId] = newOrder addressToOrderIds[msg.sender][orderId] = OrderType.PendingSandboxLimitOrder ++totalOrdersPerAddress[msg.sender] orderIds[i] = orderId addressToAllOrderIds[msg.sender].push(orderId) unchecked {++i}} 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 317-428 
 ```solidity 
 for (uint256 i = 0; i < orderGroup.length;) {SandboxLimitOrder memory newOrder = orderGroup[i]; updatedTotalOrdersValue += newOrder.amountInRemaining uint256 relativeWethValue; {if (!(newOrder.tokenIn == WETH)) {(LimitOrderSwapRouter.SpotReserve[] memory spRes) = ILimitOrderSwapRouter(LIMIT_ORDER_EXECUTOR).getAllPrices(newOrder.tokenIn, WETH, 500) uint256 tokenAWethSpotPrice; for (uint256 k = 0; k < spRes.length;) {if (spRes[k].spotPrice != 0) {tokenAWethSpotPrice = spRes[k].spotPrice break;} unchecked {++k}} if (tokenAWethSpotPrice == 0) {revert InvalidInputTokenForOrderPlacement();} if (!(tokenAWethSpotPrice == 0)) {uint8 tokenInDecimals = IERC20(newOrder.tokenIn).decimals(); relativeWethValue = tokenInDecimals <= 18 ? ConveyorMath.mul128U(tokenAWethSpotPrice, newOrder.amountInRemaining) * 10 ** (18 - tokenInDecimals) : ConveyorMath.mul128U(tokenAWethSpotPrice, newOrder.amountInRemaining) / 10 ** (tokenInDecimals - 18)}} else {relativeWethValue = newOrder.amountInRemaining} if (relativeWethValue < MIN_ORDER_VALUE_IN_WETH) {revert InsufficientOrderInputValue();} uint128 minFeeReceived = uint128(ConveyorMath.mul64U(ILimitOrderSwapRouter(LIMIT_ORDER_EXECUTOR).calculateFee(uint128(relativeWethValue), USDC, WETH), relativeWethValue)); newOrder.feeRemaining = minFeeReceived} if ((orderToken != newOrder.tokenIn)) {revert IncongruentInputTokenInOrderGroup(newOrder.tokenIn, orderToken);} if (tokenBalance < updatedTotalOrdersValue) {revert InsufficientWalletBalance(msg.sender, tokenBalance, updatedTotalOrdersValue);} bytes32 orderId = keccak256(abi.encode(orderNonce, block.timestamp)); unchecked {orderNonce += 2} newOrder.owner = msg.sender newOrder.orderId = orderId newOrder.lastRefreshTimestamp = uint32(block.timestamp) cumulativeExecutionCredit += newOrder.executionCreditRemaining orderIdToSandboxLimitOrder[orderId] = newOrder addressToOrderIds[msg.sender][orderId] = OrderType.PendingSandboxLimitOrder ++totalOrdersPerAddress[msg.sender] orderIds[i] = orderId addressToAllOrderIds[msg.sender].push(orderId) unchecked {++i}} 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 317-428 
 ```solidity 
 for (uint256 i = 0; i < orderGroup.length;) {SandboxLimitOrder memory newOrder = orderGroup[i]; updatedTotalOrdersValue += newOrder.amountInRemaining uint256 relativeWethValue; {if (!(newOrder.tokenIn == WETH)) {(LimitOrderSwapRouter.SpotReserve[] memory spRes) = ILimitOrderSwapRouter(LIMIT_ORDER_EXECUTOR).getAllPrices(newOrder.tokenIn, WETH, 500) uint256 tokenAWethSpotPrice; for (uint256 k = 0; k < spRes.length;) {if (spRes[k].spotPrice != 0) {tokenAWethSpotPrice = spRes[k].spotPrice break;} unchecked {++k}} if (tokenAWethSpotPrice == 0) {revert InvalidInputTokenForOrderPlacement();} if (!(tokenAWethSpotPrice == 0)) {uint8 tokenInDecimals = IERC20(newOrder.tokenIn).decimals(); relativeWethValue = tokenInDecimals <= 18 ? ConveyorMath.mul128U(tokenAWethSpotPrice, newOrder.amountInRemaining) * 10 ** (18 - tokenInDecimals) : ConveyorMath.mul128U(tokenAWethSpotPrice, newOrder.amountInRemaining) / 10 ** (tokenInDecimals - 18)}} else {relativeWethValue = newOrder.amountInRemaining} if (relativeWethValue < MIN_ORDER_VALUE_IN_WETH) {revert InsufficientOrderInputValue();} uint128 minFeeReceived = uint128(ConveyorMath.mul64U(ILimitOrderSwapRouter(LIMIT_ORDER_EXECUTOR).calculateFee(uint128(relativeWethValue), USDC, WETH), relativeWethValue)); newOrder.feeRemaining = minFeeReceived} if ((orderToken != newOrder.tokenIn)) {revert IncongruentInputTokenInOrderGroup(newOrder.tokenIn, orderToken);} if (tokenBalance < updatedTotalOrdersValue) {revert InsufficientWalletBalance(msg.sender, tokenBalance, updatedTotalOrdersValue);} bytes32 orderId = keccak256(abi.encode(orderNonce, block.timestamp)); unchecked {orderNonce += 2} newOrder.owner = msg.sender newOrder.orderId = orderId newOrder.lastRefreshTimestamp = uint32(block.timestamp) cumulativeExecutionCredit += newOrder.executionCreditRemaining orderIdToSandboxLimitOrder[orderId] = newOrder addressToOrderIds[msg.sender][orderId] = OrderType.PendingSandboxLimitOrder ++totalOrdersPerAddress[msg.sender] orderIds[i] = orderId addressToAllOrderIds[msg.sender].push(orderId) unchecked {++i}} 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 317-428 
 ```solidity 
 for (uint256 i = 0; i < orderGroup.length;) {SandboxLimitOrder memory newOrder = orderGroup[i]; updatedTotalOrdersValue += newOrder.amountInRemaining uint256 relativeWethValue; {if (!(newOrder.tokenIn == WETH)) {(LimitOrderSwapRouter.SpotReserve[] memory spRes) = ILimitOrderSwapRouter(LIMIT_ORDER_EXECUTOR).getAllPrices(newOrder.tokenIn, WETH, 500) uint256 tokenAWethSpotPrice; for (uint256 k = 0; k < spRes.length;) {if (spRes[k].spotPrice != 0) {tokenAWethSpotPrice = spRes[k].spotPrice break;} unchecked {++k}} if (tokenAWethSpotPrice == 0) {revert InvalidInputTokenForOrderPlacement();} if (!(tokenAWethSpotPrice == 0)) {uint8 tokenInDecimals = IERC20(newOrder.tokenIn).decimals(); relativeWethValue = tokenInDecimals <= 18 ? ConveyorMath.mul128U(tokenAWethSpotPrice, newOrder.amountInRemaining) * 10 ** (18 - tokenInDecimals) : ConveyorMath.mul128U(tokenAWethSpotPrice, newOrder.amountInRemaining) / 10 ** (tokenInDecimals - 18)}} else {relativeWethValue = newOrder.amountInRemaining} if (relativeWethValue < MIN_ORDER_VALUE_IN_WETH) {revert InsufficientOrderInputValue();} uint128 minFeeReceived = uint128(ConveyorMath.mul64U(ILimitOrderSwapRouter(LIMIT_ORDER_EXECUTOR).calculateFee(uint128(relativeWethValue), USDC, WETH), relativeWethValue)); newOrder.feeRemaining = minFeeReceived} if ((orderToken != newOrder.tokenIn)) {revert IncongruentInputTokenInOrderGroup(newOrder.tokenIn, orderToken);} if (tokenBalance < updatedTotalOrdersValue) {revert InsufficientWalletBalance(msg.sender, tokenBalance, updatedTotalOrdersValue);} bytes32 orderId = keccak256(abi.encode(orderNonce, block.timestamp)); unchecked {orderNonce += 2} newOrder.owner = msg.sender newOrder.orderId = orderId newOrder.lastRefreshTimestamp = uint32(block.timestamp) cumulativeExecutionCredit += newOrder.executionCreditRemaining orderIdToSandboxLimitOrder[orderId] = newOrder addressToOrderIds[msg.sender][orderId] = OrderType.PendingSandboxLimitOrder ++totalOrdersPerAddress[msg.sender] orderIds[i] = orderId addressToAllOrderIds[msg.sender].push(orderId) unchecked {++i}} 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 317-428 
 ```solidity 
 for (uint256 i = 0; i < orderGroup.length;) {SandboxLimitOrder memory newOrder = orderGroup[i]; updatedTotalOrdersValue += newOrder.amountInRemaining uint256 relativeWethValue; {if (!(newOrder.tokenIn == WETH)) {(LimitOrderSwapRouter.SpotReserve[] memory spRes) = ILimitOrderSwapRouter(LIMIT_ORDER_EXECUTOR).getAllPrices(newOrder.tokenIn, WETH, 500) uint256 tokenAWethSpotPrice; for (uint256 k = 0; k < spRes.length;) {if (spRes[k].spotPrice != 0) {tokenAWethSpotPrice = spRes[k].spotPrice break;} unchecked {++k}} if (tokenAWethSpotPrice == 0) {revert InvalidInputTokenForOrderPlacement();} if (!(tokenAWethSpotPrice == 0)) {uint8 tokenInDecimals = IERC20(newOrder.tokenIn).decimals(); relativeWethValue = tokenInDecimals <= 18 ? ConveyorMath.mul128U(tokenAWethSpotPrice, newOrder.amountInRemaining) * 10 ** (18 - tokenInDecimals) : ConveyorMath.mul128U(tokenAWethSpotPrice, newOrder.amountInRemaining) / 10 ** (tokenInDecimals - 18)}} else {relativeWethValue = newOrder.amountInRemaining} if (relativeWethValue < MIN_ORDER_VALUE_IN_WETH) {revert InsufficientOrderInputValue();} uint128 minFeeReceived = uint128(ConveyorMath.mul64U(ILimitOrderSwapRouter(LIMIT_ORDER_EXECUTOR).calculateFee(uint128(relativeWethValue), USDC, WETH), relativeWethValue)); newOrder.feeRemaining = minFeeReceived} if ((orderToken != newOrder.tokenIn)) {revert IncongruentInputTokenInOrderGroup(newOrder.tokenIn, orderToken);} if (tokenBalance < updatedTotalOrdersValue) {revert InsufficientWalletBalance(msg.sender, tokenBalance, updatedTotalOrdersValue);} bytes32 orderId = keccak256(abi.encode(orderNonce, block.timestamp)); unchecked {orderNonce += 2} newOrder.owner = msg.sender newOrder.orderId = orderId newOrder.lastRefreshTimestamp = uint32(block.timestamp) cumulativeExecutionCredit += newOrder.executionCreditRemaining orderIdToSandboxLimitOrder[orderId] = newOrder addressToOrderIds[msg.sender][orderId] = OrderType.PendingSandboxLimitOrder ++totalOrdersPerAddress[msg.sender] orderIds[i] = orderId addressToAllOrderIds[msg.sender].push(orderId) unchecked {++i}} 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 750-799 
 ```solidity 
 for (uint256 i = 0; i < orderIdBundles.length;) {bytes32[] memory orderIdBundle = orderIdBundles[i]; for (uint256 j = 0; j < orderIdBundle.length;) {bytes32 orderId = orderIdBundle[j]; SandboxLimitOrder memory currentOrder = orderIdToSandboxLimitOrder[orderId]; if (currentOrder.orderId == bytes32(0)) {revert OrderDoesNotExist(orderId);} preSandboxExecutionState.orderOwners[arrayIndex] = currentOrder.owner preSandboxExecutionState.sandboxLimitOrders[arrayIndex] = currentOrder uint128 amountSpecifiedToFill = fillAmounts[arrayIndex]; if (amountSpecifiedToFill > currentOrder.amountInRemaining) {revert FillAmountSpecifiedGreaterThanAmountRemaining(amountSpecifiedToFill, currentOrder.amountInRemaining, currentOrder.orderId);} preSandboxExecutionState.initialTokenInBalances[arrayIndex] = IERC20(currentOrder.tokenIn).balanceOf(currentOrder.owner) preSandboxExecutionState.initialTokenOutBalances[arrayIndex] = IERC20(currentOrder.tokenOut).balanceOf(currentOrder.owner) unchecked {++arrayIndex} unchecked {++j}} unchecked {++i}} 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 753-794 
 ```solidity 
 for (uint256 j = 0; j < orderIdBundle.length;) {bytes32 orderId = orderIdBundle[j]; SandboxLimitOrder memory currentOrder = orderIdToSandboxLimitOrder[orderId]; if (currentOrder.orderId == bytes32(0)) {revert OrderDoesNotExist(orderId);} preSandboxExecutionState.orderOwners[arrayIndex] = currentOrder.owner preSandboxExecutionState.sandboxLimitOrders[arrayIndex] = currentOrder uint128 amountSpecifiedToFill = fillAmounts[arrayIndex]; if (amountSpecifiedToFill > currentOrder.amountInRemaining) {revert FillAmountSpecifiedGreaterThanAmountRemaining(amountSpecifiedToFill, currentOrder.amountInRemaining, currentOrder.orderId);} preSandboxExecutionState.initialTokenInBalances[arrayIndex] = IERC20(currentOrder.tokenIn).balanceOf(currentOrder.owner) preSandboxExecutionState.initialTokenOutBalances[arrayIndex] = IERC20(currentOrder.tokenOut).balanceOf(currentOrder.owner) unchecked {++arrayIndex} unchecked {++j}} 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 1222-1242 
 ```solidity 
 for (uint256 i = 0; i < length;) {bytes32 orderId; assembly {orderId := mload(orderOffsetSlot) orderOffsetSlot := add(orderOffsetSlot, 0x20)} OrderType orderType = addressToOrderIds[orderOwner][orderId]; if (orderType == targetOrderType) {orderIds[orderIdIndex] = orderId ++orderIdIndex} unchecked {++i}} 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 281-351 
 ```solidity 
 for (uint256 i = 0; i < orderGroup.length;) {LimitOrder memory newOrder = orderGroup[i]; if (newOrder.quantity == 0) {revert OrderQuantityIsZero();} updatedTotalOrdersValue += newOrder.quantity if (!(orderToken == newOrder.tokenIn)) {revert IncongruentInputTokenInOrderGroup(newOrder.tokenIn, orderToken);} if (newOrder.tokenOut == newOrder.tokenIn) {revert TokenInIsTokenOut();} if (tokenBalance < updatedTotalOrdersValue) {revert InsufficientWalletBalance(msg.sender, tokenBalance, updatedTotalOrdersValue);} bytes32 orderId = keccak256(abi.encode(orderNonce, block.timestamp)); cumulativeExecutionCredit += newOrder.executionCredit unchecked {orderNonce += 2} newOrder.owner = msg.sender newOrder.orderId = orderId newOrder.lastRefreshTimestamp = uint32(block.timestamp) orderIdToLimitOrder[orderId] = newOrder addressToOrderIds[msg.sender][orderId] = OrderType.PendingLimitOrder ++totalOrdersPerAddress[msg.sender] orderIds[i] = orderId addressToAllOrderIds[msg.sender].push(orderId) unchecked {++i}} 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 281-351 
 ```solidity 
 for (uint256 i = 0; i < orderGroup.length;) {LimitOrder memory newOrder = orderGroup[i]; if (newOrder.quantity == 0) {revert OrderQuantityIsZero();} updatedTotalOrdersValue += newOrder.quantity if (!(orderToken == newOrder.tokenIn)) {revert IncongruentInputTokenInOrderGroup(newOrder.tokenIn, orderToken);} if (newOrder.tokenOut == newOrder.tokenIn) {revert TokenInIsTokenOut();} if (tokenBalance < updatedTotalOrdersValue) {revert InsufficientWalletBalance(msg.sender, tokenBalance, updatedTotalOrdersValue);} bytes32 orderId = keccak256(abi.encode(orderNonce, block.timestamp)); cumulativeExecutionCredit += newOrder.executionCredit unchecked {orderNonce += 2} newOrder.owner = msg.sender newOrder.orderId = orderId newOrder.lastRefreshTimestamp = uint32(block.timestamp) orderIdToLimitOrder[orderId] = newOrder addressToOrderIds[msg.sender][orderId] = OrderType.PendingLimitOrder ++totalOrdersPerAddress[msg.sender] orderIds[i] = orderId addressToAllOrderIds[msg.sender].push(orderId) unchecked {++i}} 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 281-351 
 ```solidity 
 for (uint256 i = 0; i < orderGroup.length;) {LimitOrder memory newOrder = orderGroup[i]; if (newOrder.quantity == 0) {revert OrderQuantityIsZero();} updatedTotalOrdersValue += newOrder.quantity if (!(orderToken == newOrder.tokenIn)) {revert IncongruentInputTokenInOrderGroup(newOrder.tokenIn, orderToken);} if (newOrder.tokenOut == newOrder.tokenIn) {revert TokenInIsTokenOut();} if (tokenBalance < updatedTotalOrdersValue) {revert InsufficientWalletBalance(msg.sender, tokenBalance, updatedTotalOrdersValue);} bytes32 orderId = keccak256(abi.encode(orderNonce, block.timestamp)); cumulativeExecutionCredit += newOrder.executionCredit unchecked {orderNonce += 2} newOrder.owner = msg.sender newOrder.orderId = orderId newOrder.lastRefreshTimestamp = uint32(block.timestamp) orderIdToLimitOrder[orderId] = newOrder addressToOrderIds[msg.sender][orderId] = OrderType.PendingLimitOrder ++totalOrdersPerAddress[msg.sender] orderIds[i] = orderId addressToAllOrderIds[msg.sender].push(orderId) unchecked {++i}} 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 281-351 
 ```solidity 
 for (uint256 i = 0; i < orderGroup.length;) {LimitOrder memory newOrder = orderGroup[i]; if (newOrder.quantity == 0) {revert OrderQuantityIsZero();} updatedTotalOrdersValue += newOrder.quantity if (!(orderToken == newOrder.tokenIn)) {revert IncongruentInputTokenInOrderGroup(newOrder.tokenIn, orderToken);} if (newOrder.tokenOut == newOrder.tokenIn) {revert TokenInIsTokenOut();} if (tokenBalance < updatedTotalOrdersValue) {revert InsufficientWalletBalance(msg.sender, tokenBalance, updatedTotalOrdersValue);} bytes32 orderId = keccak256(abi.encode(orderNonce, block.timestamp)); cumulativeExecutionCredit += newOrder.executionCredit unchecked {orderNonce += 2} newOrder.owner = msg.sender newOrder.orderId = orderId newOrder.lastRefreshTimestamp = uint32(block.timestamp) orderIdToLimitOrder[orderId] = newOrder addressToOrderIds[msg.sender][orderId] = OrderType.PendingLimitOrder ++totalOrdersPerAddress[msg.sender] orderIds[i] = orderId addressToAllOrderIds[msg.sender].push(orderId) unchecked {++i}} 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 281-351 
 ```solidity 
 for (uint256 i = 0; i < orderGroup.length;) {LimitOrder memory newOrder = orderGroup[i]; if (newOrder.quantity == 0) {revert OrderQuantityIsZero();} updatedTotalOrdersValue += newOrder.quantity if (!(orderToken == newOrder.tokenIn)) {revert IncongruentInputTokenInOrderGroup(newOrder.tokenIn, orderToken);} if (newOrder.tokenOut == newOrder.tokenIn) {revert TokenInIsTokenOut();} if (tokenBalance < updatedTotalOrdersValue) {revert InsufficientWalletBalance(msg.sender, tokenBalance, updatedTotalOrdersValue);} bytes32 orderId = keccak256(abi.encode(orderNonce, block.timestamp)); cumulativeExecutionCredit += newOrder.executionCredit unchecked {orderNonce += 2} newOrder.owner = msg.sender newOrder.orderId = orderId newOrder.lastRefreshTimestamp = uint32(block.timestamp) orderIdToLimitOrder[orderId] = newOrder addressToOrderIds[msg.sender][orderId] = OrderType.PendingLimitOrder ++totalOrdersPerAddress[msg.sender] orderIds[i] = orderId addressToAllOrderIds[msg.sender].push(orderId) unchecked {++i}} 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 281-351 
 ```solidity 
 for (uint256 i = 0; i < orderGroup.length;) {LimitOrder memory newOrder = orderGroup[i]; if (newOrder.quantity == 0) {revert OrderQuantityIsZero();} updatedTotalOrdersValue += newOrder.quantity if (!(orderToken == newOrder.tokenIn)) {revert IncongruentInputTokenInOrderGroup(newOrder.tokenIn, orderToken);} if (newOrder.tokenOut == newOrder.tokenIn) {revert TokenInIsTokenOut();} if (tokenBalance < updatedTotalOrdersValue) {revert InsufficientWalletBalance(msg.sender, tokenBalance, updatedTotalOrdersValue);} bytes32 orderId = keccak256(abi.encode(orderNonce, block.timestamp)); cumulativeExecutionCredit += newOrder.executionCredit unchecked {orderNonce += 2} newOrder.owner = msg.sender newOrder.orderId = orderId newOrder.lastRefreshTimestamp = uint32(block.timestamp) orderIdToLimitOrder[orderId] = newOrder addressToOrderIds[msg.sender][orderId] = OrderType.PendingLimitOrder ++totalOrdersPerAddress[msg.sender] orderIds[i] = orderId addressToAllOrderIds[msg.sender].push(orderId) unchecked {++i}} 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 586-606 
 ```solidity 
 for (uint256 i = 0; i < length;) {bytes32 orderId; assembly {orderId := mload(orderOffsetSlot) orderOffsetSlot := add(orderOffsetSlot, 0x20)} OrderType orderType = addressToOrderIds[_owner][orderId]; if (orderType == targetOrderType) {orderIds[orderIdIndex] = orderId ++orderIdIndex} unchecked {++i}} 
 ``` 
 </details>

 <details open> 
 <summary> 
 <a name=[G-14]></a> [G-14] 
 <h3> Use assembly to hash instead of Solidity - Instances: 2 </h3> 
 </summary>
 
 &nbsp;  
 &nbsp; <details> &nbsp; 
 <summary> 
 &nbsp; &nbsp; Keccak256 - Gas Report Savings: ~82 
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

### Gas Report

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
 

 <span style="color: green;">File: </span> LimitOrderBook.sol 308-308 
 ```solidity 
 keccak256(abi.encode(orderNonce, block.timestamp)) 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 538-538 
 ```solidity 
 keccak256(abi.encode(msg.sender, token)) 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 547-547 
 ```solidity 
 keccak256(abi.encode(_owner, token)) 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 556-556 
 ```solidity 
 keccak256(abi.encode(_owner, token)) 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 385-385 
 ```solidity 
 keccak256(abi.encode(orderNonce, block.timestamp)) 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 1142-1142 
 ```solidity 
 keccak256(abi.encode(orderOwner, token)) 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 1151-1151 
 ```solidity 
 keccak256(abi.encode(orderOwner, token)) 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 1179-1179 
 ```solidity 
 keccak256(abi.encode(msg.sender, token)) 
 ``` 
 </details>

 <details open> 
 <summary> 
 <a name=[G-15]></a> [G-15] 
 <h3> Use assembly for math (add, sub, mul, div) - Instances: 13 </h3> 
 </summary>
 
 &nbsp; Use assembly for math instead of Solidity. You can check for overflow/underflow in assembly to ensure safety. If using Solidity versions < 0.8.0 and you are using Safemath, you can gain significant gas savings by using assembly to calculate values and checking for overflow/underflow. 
 &nbsp; <details> &nbsp; 
 <summary> 
 &nbsp; &nbsp; Assembly Math - Gas Report Savings: ~60 
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

### Gas Report

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
 

 <span style="color: green;">File: </span> SandboxLimitOrderRouter.sol 65-65 
 ```solidity 
 block.timestamp - lastCheckInTime 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 186-186 
 ```solidity 
 executionCredit - amount 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 187-187 
 ```solidity 
 executionCredit - amount 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 190-190 
 ```solidity 
 executionCredit - amount 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 194-194 
 ```solidity 
 executionCredit - amount 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 215-215 
 ```solidity 
 orderIdToLimitOrder[orderId].executionCredit + uint128(msg.value) 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 255-255 
 ```solidity 
 minExecutionCredit * orderGroup.length 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 409-409 
 ```solidity 
 orderIdToLimitOrder[order.orderId].executionCredit + uint128(msg.value) 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 181-182 
 ```solidity 
 spotReserveAToWeth.length * spotReserveWethToB.length 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 277-277 
 ```solidity 
 alphaX * 10 ** (18 - tokenInDecimals) 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 277-277 
 ```solidity 
 18 - tokenInDecimals 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 278-278 
 ```solidity 
 alphaX / (10 ** (tokenInDecimals - 18)) 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 278-278 
 ```solidity 
 tokenInDecimals - 18 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 401-401 
 ```solidity 
 reserveA + alphaX 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 456-456 
 ```solidity 
 amountIn * 997 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 457-457 
 ```solidity 
 amountInWithFee * reserveOut 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 458-458 
 ```solidity 
 reserveIn * 1000 + (amountInWithFee) 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 458-458 
 ```solidity 
 reserveIn * 1000 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 459-459 
 ```solidity 
 numerator / denominator 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 481-481 
 ```solidity 
 alphaX * 10 ** (18 - tokenInDecimals) 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 481-481 
 ```solidity 
 18 - tokenInDecimals 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 482-482 
 ```solidity 
 alphaX / (10 ** (tokenInDecimals - 18)) 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 482-482 
 ```solidity 
 tokenInDecimals - 18 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 560-560 
 ```solidity 
 (amountInOrder * taxIn) / 10 ** 5 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 560-560 
 ```solidity 
 amountInOrder * taxIn 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 561-561 
 ```solidity 
 amountInOrder - amountInBuffer 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 581-581 
 ```solidity 
 (amountInOrder * taxIn) / 10 ** 5 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 581-581 
 ```solidity 
 amountInOrder * taxIn 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 583-583 
 ```solidity 
 amountInOrder - amountInBuffer 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 586-586 
 ```solidity 
 (amountInOrder * taxIn) / 10 ** 5 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 586-586 
 ```solidity 
 amountInOrder * taxIn 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 588-588 
 ```solidity 
 amountInOrder - amountInBuffer 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 126-126 
 ```solidity 
 balanceBefore + swapData.amountOutMin 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 135-135 
 ```solidity 
 tokenOutAmountRequired - balanceAfter 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 166-166 
 ```solidity 
 msg.value - swapData.protocolFee 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 178-178 
 ```solidity 
 balanceBefore + swapData.amountOutMin 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 188-188 
 ```solidity 
 tokenOutAmountRequired - balanceAfter 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 225-225 
 ```solidity 
 balanceBefore + swapData.amountOutMin 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 241-241 
 ```solidity 
 amountOutRequired - msg.sender.balance 
 ```

 <span style="color: green;">File: </span> UniswapInterfaceMulticall.sol 35-35 
 ```solidity 
 gasLeftBefore - gasleft() 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 184-184 
 ```solidity 
 ConveyorMath.mul128U(spotPrice, amountIn) / uint256(10 ** 18) 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 207-207 
 ```solidity 
 ConveyorMath.add64x64(rationalFraction, 461168601842738800) / 10 ** 2 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 272-272 
 ```solidity 
 IERC20(_tokenOut).balanceOf(_receiver) - balanceBefore 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 348-348 
 ```solidity 
 TickMath.MIN_SQRT_RATIO + 1 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 348-348 
 ```solidity 
 TickMath.MAX_SQRT_RATIO - 1 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 475-475 
 ```solidity 
 reserve0 * (10 ** (18 - token0Decimals)) 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 475-475 
 ```solidity 
 18 - token0Decimals 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 476-476 
 ```solidity 
 reserve0 * (10 ** (token0Decimals - 18)) 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 476-476 
 ```solidity 
 token0Decimals - 18 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 478-478 
 ```solidity 
 reserve1 * (10 ** (18 - token1Decimals)) 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 478-478 
 ```solidity 
 18 - token1Decimals 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 479-479 
 ```solidity 
 reserve1 * (10 ** (token1Decimals - 18)) 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 479-479 
 ```solidity 
 token1Decimals - 18 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 74-74 
 ```solidity 
 uint256(x) + y 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 86-86 
 ```solidity 
 int256(x) - y 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 97-97 
 ```solidity 
 x + y 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 107-107 
 ```solidity 
 x + (uint256(y) << 64) 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 118-118 
 ```solidity 
 uint256(x) * y 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 132-132 
 ```solidity 
 uint256(y) * x 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 147-147 
 ```solidity 
 uint256(x) * (y & 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF) 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 148-148 
 ```solidity 
 uint256(x) * (y >> 128) 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 153-153 
 ```solidity 
 MAX_128x128 - lo 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 154-154 
 ```solidity 
 hi + lo 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 167-167 
 ```solidity 
 x * y 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 187-187 
 ```solidity 
 (uint256(x) << 64) / y 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 205-205 
 ```solidity 
 xInt * (MAX_128x128 / y) 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 205-205 
 ```solidity 
 MAX_128x128 / y 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 206-206 
 ```solidity 
 xDec * (MAX_128x128 / y) 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 206-206 
 ```solidity 
 MAX_128x128 / y 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 208-208 
 ```solidity 
 MAX_128x128 - lo 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 209-209 
 ```solidity 
 hi + lo 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 237-237 
 ```solidity 
 (x << 64) / y 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 263-263 
 ```solidity 
 (x << (255 - msb)) / (((y - 1) >> (msb - 191)) + 1) 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 263-263 
 ```solidity 
 255 - msb 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 263-263 
 ```solidity 
 ((y - 1) >> (msb - 191)) + 1 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 263-263 
 ```solidity 
 y - 1 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 263-263 
 ```solidity 
 msb - 191 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 266-266 
 ```solidity 
 answer * (y >> 128) 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 267-267 
 ```solidity 
 answer * (y & 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF) 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 280-280 
 ```solidity 
 xl / y 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 291-291 
 ```solidity 
 (uint32(integers) << 16) + decimals 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 305-305 
 ```solidity 
 answer * 0x16A09E667F3BCC908B2FB1366EA957D3E 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 308-308 
 ```solidity 
 answer * 0x1306FE0A31B7152DE8D5A46305C85EDEC 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 311-311 
 ```solidity 
 answer * 0x1172B83C7D517ADCDF7C8C50EB14A791F 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 314-314 
 ```solidity 
 answer * 0x10B5586CF9890F6298B92B71842A98363 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 317-317 
 ```solidity 
 answer * 0x1059B0D31585743AE7C548EB68CA417FD 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 320-320 
 ```solidity 
 answer * 0x102C9A3E778060EE6F7CACA4F7A29BDE8 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 323-323 
 ```solidity 
 answer * 0x10163DA9FB33356D84A66AE336DCDFA3F 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 326-326 
 ```solidity 
 answer * 0x100B1AFA5ABCBED6129AB13EC11DC9543 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 329-329 
 ```solidity 
 answer * 0x10058C86DA1C09EA1FF19D294CF2F679B 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 332-332 
 ```solidity 
 answer * 0x1002C605E2E8CEC506D21BFC89A23A00F 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 335-335 
 ```solidity 
 answer * 0x100162F3904051FA128BCA9C55C31E5DF 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 338-338 
 ```solidity 
 answer * 0x1000B175EFFDC76BA38E31671CA939725 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 341-341 
 ```solidity 
 answer * 0x100058BA01FB9F96D6CACD4B180917C3D 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 344-344 
 ```solidity 
 answer * 0x10002C5CC37DA9491D0985C348C68E7B3 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 347-347 
 ```solidity 
 answer * 0x1000162E525EE054754457D5995292026 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 350-350 
 ```solidity 
 answer * 0x10000B17255775C040618BF4A4ADE83FC 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 353-353 
 ```solidity 
 answer * 0x1000058B91B5BC9AE2EED81E9B7D4CFAB 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 356-356 
 ```solidity 
 answer * 0x100002C5C89D5EC6CA4D7C8ACC017B7C9 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 359-359 
 ```solidity 
 answer * 0x10000162E43F4F831060E02D839A9D16D 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 362-362 
 ```solidity 
 answer * 0x100000B1721BCFC99D9F890EA06911763 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 365-365 
 ```solidity 
 answer * 0x10000058B90CF1E6D97F9CA14DBCC1628 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 368-368 
 ```solidity 
 answer * 0x1000002C5C863B73F016468F6BAC5CA2B 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 371-371 
 ```solidity 
 answer * 0x100000162E430E5A18F6119E3C02282A5 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 374-374 
 ```solidity 
 answer * 0x1000000B1721835514B86E6D96EFD1BFE 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 377-377 
 ```solidity 
 answer * 0x100000058B90C0B48C6BE5DF846C5B2EF 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 380-380 
 ```solidity 
 answer * 0x10000002C5C8601CC6B9E94213C72737A 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 383-383 
 ```solidity 
 answer * 0x1000000162E42FFF037DF38AA2B219F06 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 386-386 
 ```solidity 
 answer * 0x10000000B17217FBA9C739AA5819F44F9 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 389-389 
 ```solidity 
 answer * 0x1000000058B90BFCDEE5ACD3C1CEDC823 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 392-392 
 ```solidity 
 answer * 0x100000002C5C85FE31F35A6A30DA1BE50 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 395-395 
 ```solidity 
 answer * 0x10000000162E42FF0999CE3541B9FFFCF 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 398-398 
 ```solidity 
 answer * 0x100000000B17217F80F4EF5AADDA45554 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 401-401 
 ```solidity 
 answer * 0x10000000058B90BFBF8479BD5A81B51AD 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 404-404 
 ```solidity 
 answer * 0x1000000002C5C85FDF84BD62AE30A74CC 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 407-407 
 ```solidity 
 answer * 0x100000000162E42FEFB2FED257559BDAA 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 410-410 
 ```solidity 
 answer * 0x1000000000B17217F7D5A7716BBA4A9AE 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 413-413 
 ```solidity 
 answer * 0x100000000058B90BFBE9DDBAC5E109CCE 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 416-416 
 ```solidity 
 answer * 0x10000000002C5C85FDF4B15DE6F17EB0D 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 419-419 
 ```solidity 
 answer * 0x1000000000162E42FEFA494F1478FDE05 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 422-422 
 ```solidity 
 answer * 0x10000000000B17217F7D20CF927C8E94C 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 425-425 
 ```solidity 
 answer * 0x1000000000058B90BFBE8F71CB4E4B33D 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 428-428 
 ```solidity 
 answer * 0x100000000002C5C85FDF477B662B26945 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 431-431 
 ```solidity 
 answer * 0x10000000000162E42FEFA3AE53369388C 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 434-434 
 ```solidity 
 answer * 0x100000000000B17217F7D1D351A389D40 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 437-437 
 ```solidity 
 answer * 0x10000000000058B90BFBE8E8B2D3D4EDE 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 440-440 
 ```solidity 
 answer * 0x1000000000002C5C85FDF4741BEA6E77E 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 443-443 
 ```solidity 
 answer * 0x100000000000162E42FEFA39FE95583C2 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 446-446 
 ```solidity 
 answer * 0x1000000000000B17217F7D1CFB72B45E1 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 449-449 
 ```solidity 
 answer * 0x100000000000058B90BFBE8E7CC35C3F0 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 452-452 
 ```solidity 
 answer * 0x10000000000002C5C85FDF473E242EA38 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 455-455 
 ```solidity 
 answer * 0x1000000000000162E42FEFA39F02B772C 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 458-458 
 ```solidity 
 answer * 0x10000000000000B17217F7D1CF7D83C1A 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 461-461 
 ```solidity 
 answer * 0x1000000000000058B90BFBE8E7BDCBE2E 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 464-464 
 ```solidity 
 answer * 0x100000000000002C5C85FDF473DEA871F 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 467-467 
 ```solidity 
 answer * 0x10000000000000162E42FEFA39EF44D91 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 470-470 
 ```solidity 
 answer * 0x100000000000000B17217F7D1CF79E949 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 473-473 
 ```solidity 
 answer * 0x10000000000000058B90BFBE8E7BCE544 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 476-476 
 ```solidity 
 answer * 0x1000000000000002C5C85FDF473DE6ECA 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 479-479 
 ```solidity 
 answer * 0x100000000000000162E42FEFA39EF366F 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 482-482 
 ```solidity 
 answer * 0x1000000000000000B17217F7D1CF79AFA 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 485-485 
 ```solidity 
 answer * 0x100000000000000058B90BFBE8E7BCD6D 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 488-488 
 ```solidity 
 answer * 0x10000000000000002C5C85FDF473DE6B2 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 491-491 
 ```solidity 
 answer * 0x1000000000000000162E42FEFA39EF358 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 494-494 
 ```solidity 
 answer * 0x10000000000000000B17217F7D1CF79AB 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 497-497 
 ```solidity 
 63 - (x >> 64) 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 511-511 
 ```solidity 
 uint256(x) * 0x171547652B82FE1777D0FFDA0D23A7D12 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 552-552 
 ```solidity 
 r + x / r 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 552-552 
 ```solidity 
 x / r 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 553-553 
 ```solidity 
 r + x / r 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 553-553 
 ```solidity 
 x / r 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 554-554 
 ```solidity 
 r + x / r 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 554-554 
 ```solidity 
 x / r 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 555-555 
 ```solidity 
 r + x / r 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 555-555 
 ```solidity 
 x / r 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 556-556 
 ```solidity 
 r + x / r 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 556-556 
 ```solidity 
 x / r 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 557-557 
 ```solidity 
 r + x / r 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 557-557 
 ```solidity 
 x / r 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 558-558 
 ```solidity 
 r + x / r 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 558-558 
 ```solidity 
 x / r 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 559-559 
 ```solidity 
 x / r 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 252-252 
 ```solidity 
 executionCreditRemaining - amount 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 253-253 
 ```solidity 
 executionCreditRemaining - amount 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 257-257 
 ```solidity 
 executionCreditRemaining - amount 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 260-260 
 ```solidity 
 executionCreditRemaining - amount 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 276-276 
 ```solidity 
 orderIdToSandboxLimitOrder[orderId].executionCreditRemaining + uint128(msg.value) 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 292-292 
 ```solidity 
 minExecutionCredit * orderGroup.length 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 350-352 
 ```solidity 
 ConveyorMath.mul128U(tokenAWethSpotPrice, newOrder.amountInRemaining) * 10 ** (18 - tokenInDecimals) 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 351-351 
 ```solidity 
 18 - tokenInDecimals 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 352-353 
 ```solidity 
 ConveyorMath.mul128U(tokenAWethSpotPrice, newOrder.amountInRemaining) / 10 ** (tokenInDecimals - 18) 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 353-353 
 ```solidity 
 tokenInDecimals - 18 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 472-472 
 ```solidity 
 orderIdToSandboxLimitOrder[order.orderId].executionCreditRemaining + uint128(msg.value) 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 565-565 
 ```solidity 
 block.timestamp - lastCheckInTime 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 603-603 
 ```solidity 
 executionCreditRemaining - uint128(REFRESH_FEE) 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 605-605 
 ```solidity 
 executionCreditRemaining - REFRESH_FEE 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 625-625 
 ```solidity 
 block.timestamp - lastCheckInTime 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 666-666 
 ```solidity 
 executionCreditBalance - REFRESH_FEE 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 676-676 
 ```solidity 
 block.timestamp - order.lastRefreshTimestamp 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 681-681 
 ```solidity 
 executionCreditBalance - uint128(REFRESH_FEE) 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 682-682 
 ```solidity 
 executionCreditBalance - REFRESH_FEE 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 822-822 
 ```solidity 
 orderIdBundle.length - 1 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 868-868 
 ```solidity 
 initialTokenInBalance - currentTokenInBalance 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 870-870 
 ```solidity 
 initialTokenInBalance - currentTokenInBalance 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 875-875 
 ```solidity 
 currentTokenOutBalance - initialTokenOutBalance 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 877-877 
 ```solidity 
 currentTokenOutBalance - initialTokenOutBalance 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 888-888 
 ```solidity 
 initialTokenInBalance - currentTokenInBalance 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 889-889 
 ```solidity 
 currentTokenOutBalance - initialTokenOutBalance 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 933-933 
 ```solidity 
 offset + 1 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 944-944 
 ```solidity 
 offset + 1 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 956-957 
 ```solidity 
 preSandboxExecutionState.initialTokenInBalances[offset] - currentTokenInBalance 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 961-961 
 ```solidity 
 preSandboxExecutionState.initialTokenInBalances[offset] - currentTokenInBalance 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 966-966 
 ```solidity 
 offset + 1 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 969-969 
 ```solidity 
 offset + 1 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 975-976 
 ```solidity 
 currentTokenOutBalance - preSandboxExecutionState.initialTokenOutBalances[offset] 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 980-980 
 ```solidity 
 currentTokenOutBalance - preSandboxExecutionState.initialTokenOutBalances[offset] 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 999-999 
 ```solidity 
 offset - 1 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 1058-1058 
 ```solidity 
 order.amountInRemaining - amountInFilled 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 1060-1060 
 ```solidity 
 order.amountOutRemaining - amountOutFilled 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 1065-1066 
 ```solidity 
 feeRemaining - uint128(ConveyorMath.mul64U(ConveyorMath.divUU(amountInFilled, amountInRemaining), feeRemaining)) 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 1073-1073 
 ```solidity 
 executionCreditRemaining - executionCreditCompensation 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 1080-1080 
 ```solidity 
 order.amountInRemaining - amountInFilled 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 1081-1081 
 ```solidity 
 order.amountOutRemaining - amountOutFilled 
 ```

 <span style="color: green;">File: </span> ConveyorFeeMath.sol 11-11 
 ```solidity 
 110680464442257300 * 10 ** 2 
 ```

 <span style="color: green;">File: </span> ConveyorFeeMath.sol 32-32 
 ```solidity 
 int256(uint256(ZERO_POINT_ZERO_ZERO_FIVE)) - int128(percentFee) 
 ```

 <span style="color: green;">File: </span> ConveyorFeeMath.sol 34-37 
 ```solidity 
 (percentFee + ConveyorMath.div64x64(uint128(uint256(innerPartial)), uint128(2) << 64) + uint128(ZERO_POINT_ZERO_ZERO_ONE)) * 10 ** 2 
 ```

 <span style="color: green;">File: </span> ConveyorFeeMath.sol 35-37 
 ```solidity 
 percentFee + ConveyorMath.div64x64(uint128(uint256(innerPartial)), uint128(2) << 64) + uint128(ZERO_POINT_ZERO_ZERO_ONE) 
 ```

 <span style="color: green;">File: </span> ConveyorFeeMath.sol 35-36 
 ```solidity 
 percentFee + ConveyorMath.div64x64(uint128(uint256(innerPartial)), uint128(2) << 64) 
 ```

 <span style="color: green;">File: </span> ConveyorFeeMath.sol 49-49 
 ```solidity 
 uint128(totalWethReward) - conveyorReward 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 83-83 
 ```solidity 
 block.timestamp - lastCheckInTime 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 123-123 
 ```solidity 
 executionCreditBalance - REFRESH_FEE 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 133-133 
 ```solidity 
 block.timestamp - order.lastRefreshTimestamp 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 137-137 
 ```solidity 
 executionCreditBalance - uint128(REFRESH_FEE) 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 138-138 
 ```solidity 
 executionCreditBalance - REFRESH_FEE 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 141-141 
 ```solidity 
 2 ** 32 - 1 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 158-158 
 ```solidity 
 block.timestamp - lastCheckInTime 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 190-190 
 ```solidity 
 executionCredit - uint128(REFRESH_FEE) 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 192-192 
 ```solidity 
 executionCredit - REFRESH_FEE 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 211-211 
 ```solidity 
 orders.length - 1 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 214-214 
 ```solidity 
 i + 1 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 271-271 
 ```solidity 
 block.timestamp - lastCheckInTime 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 272-272 
 ```solidity 
 amountOutWeth - (beaconReward + conveyorReward) 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 272-272 
 ```solidity 
 beaconReward + conveyorReward 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 384-384 
 ```solidity 
 amountIn - (beaconReward + conveyorReward) 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 384-384 
 ```solidity 
 beaconReward + conveyorReward 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 500-500 
 ```solidity 
 contractBalancePostExecution - contractBalancePreExecution 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 501-502 
 ```solidity 
 expectedAccumulatedFees - (contractBalancePostExecution - contractBalancePreExecution) 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 501-501 
 ```solidity 
 contractBalancePostExecution - contractBalancePreExecution 
 ```

 <span style="color: green;">File: </span> OracleLibraryV2.sol 12-12 
 ```solidity 
 reserveIn * amountOut * 100000 
 ```

 <span style="color: green;">File: </span> OracleLibraryV2.sol 12-12 
 ```solidity 
 reserveIn * amountOut 
 ```

 <span style="color: green;">File: </span> OracleLibraryV2.sol 13-13 
 ```solidity 
 (reserveOut - amountOut) * (100000 - swapFee) 
 ```

 <span style="color: green;">File: </span> OracleLibraryV2.sol 13-13 
 ```solidity 
 reserveOut - amountOut 
 ```

 <span style="color: green;">File: </span> OracleLibraryV2.sol 13-13 
 ```solidity 
 100000 - swapFee 
 ```

 <span style="color: green;">File: </span> OracleLibraryV2.sol 14-14 
 ```solidity 
 (numerator / denominator) + 1 
 ```

 <span style="color: green;">File: </span> OracleLibraryV2.sol 14-14 
 ```solidity 
 numerator / denominator 
 ```

 <span style="color: green;">File: </span> ConveyorTickMath.sol 76-76 
 ```solidity 
 int8(IERC20(token0).decimals()) - int8(IERC20(token1).decimals()) 
 ```

 <span style="color: green;">File: </span> ConveyorTickMath.sol 79-80 
 ```solidity 
 uint256(sqrtPriceX96) ** 2 / uint256(10) ** (uint8(-decimalShift)) 
 ```

 <span style="color: green;">File: </span> ConveyorTickMath.sol 80-80 
 ```solidity 
 uint256(sqrtPriceX96) ** 2 * 10 ** uint8(decimalShift) 
 ```

 <span style="color: green;">File: </span> ConveyorTickMath.sol 84-85 
 ```solidity 
 priceSquaredX96 / Q96 
 ```

 <span style="color: green;">File: </span> ConveyorTickMath.sol 85-85 
 ```solidity 
 (Q96 * 0xffffffffffffffffffffffffffffffff) / (priceSquaredX96 / Q96) 
 ```

 <span style="color: green;">File: </span> ConveyorTickMath.sol 85-85 
 ```solidity 
 Q96 * 0xffffffffffffffffffffffffffffffff 
 ```

 <span style="color: green;">File: </span> ConveyorTickMath.sol 85-85 
 ```solidity 
 priceSquaredX96 / Q96 
 ```

 <span style="color: green;">File: </span> ConveyorTickMath.sol 89-90 
 ```solidity 
 (uint256(priceSquaredShiftQ96) * 0xffffffffffffffffffffffffffffffff) / Q96 
 ```

 <span style="color: green;">File: </span> ConveyorTickMath.sol 89-89 
 ```solidity 
 uint256(priceSquaredShiftQ96) * 0xffffffffffffffffffffffffffffffff 
 ```

 <span style="color: green;">File: </span> ConveyorTickMath.sol 120-120 
 ```solidity 
 TickMath.MIN_SQRT_RATIO + 1 
 ```

 <span style="color: green;">File: </span> ConveyorTickMath.sol 120-120 
 ```solidity 
 TickMath.MAX_SQRT_RATIO - 1 
 ```

 <span style="color: green;">File: </span> ConveyorTickMath.sol 163-163 
 ```solidity 
 step.amountIn + step.feeAmount 
 ```

 <span style="color: green;">File: </span> ConveyorTickMath.sol 177-177 
 ```solidity 
 step.tickNext - 1 
 ``` 
 </details>

 <details open> 
 <summary> 
 <a name=[G-16]></a> [G-16] 
 <h3> Use assembly to write storage values - Instances: 6 </h3> 
 </summary>
 
 &nbsp;  
 &nbsp; <details> &nbsp; 
 <summary> 
 &nbsp; &nbsp; sstore - Gas Report Savings: ~66 
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

### Gas Report
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
 

 <span style="color: green;">File: </span> LimitOrderBook.sol 32-32 
 ```solidity 
 reentrancyStatus = true 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 34-34 
 ```solidity 
 reentrancyStatus = false 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 47-47 
 ```solidity 
 minExecutionCredit = _minExecutionCredit 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 66-66 
 ```solidity 
 reentrancyStatus = true 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 68-68 
 ```solidity 
 reentrancyStatus = false 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 96-96 
 ```solidity 
 minExecutionCredit = _minExecutionCredit 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 103-103 
 ```solidity 
 owner = tx.origin 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 1253-1253 
 ```solidity 
 minExecutionCredit = newMinExecutionCredit 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 1262-1262 
 ```solidity 
 owner = msg.sender 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 1263-1263 
 ```solidity 
 tempOwner = address(0) 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 1271-1271 
 ```solidity 
 tempOwner = newOwner 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 66-66 
 ```solidity 
 CONVEYOR_MULTICALL = address(new ConveyorMulticall()) 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 68-68 
 ```solidity 
 owner = tx.origin 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 375-375 
 ```solidity 
 tempOwner = address(0) 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 376-376 
 ```solidity 
 owner = msg.sender 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 385-385 
 ```solidity 
 tempOwner = newOwner 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 409-409 
 ```solidity 
 affiliateNonce = tempAffiliateNonce 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 425-425 
 ```solidity 
 referrerNonce = tempReferrerNonce 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 446-446 
 ```solidity 
 locked = true 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 448-448 
 ```solidity 
 locked = false 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 355-355 
 ```solidity 
 uniV3AmountOut = 0 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 378-378 
 ```solidity 
 uniV3AmountOut = uint256(-amount1Delta) 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 383-383 
 ```solidity 
 uniV3AmountOut = uint256(-amount0Delta) 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 69-69 
 ```solidity 
 reentrancyStatus = true 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 71-71 
 ```solidity 
 reentrancyStatus = false 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 150-150 
 ```solidity 
 owner = msg.sender 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 513-513 
 ```solidity 
 conveyorBalance = 0 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 524-524 
 ```solidity 
 tempOwner = address(0) 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 525-525 
 ```solidity 
 owner = msg.sender 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 534-534 
 ```solidity 
 tempOwner = newOwner 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 73-73 
 ```solidity 
 owner = tx.origin 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 354-354 
 ```solidity 
 owner = msg.sender 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 355-355 
 ```solidity 
 tempOwner = address(0) 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 363-363 
 ```solidity 
 tempOwner = newOwner 
 ``` 
 </details>

 <details open> 
 <summary> 
 <a name=[G-17]></a> [G-17] 
 <h3> Use custom errors instead of string error messages - Instances: 10 </h3> 
 </summary>
 
 &nbsp;  
 &nbsp; <details> &nbsp; 
 <summary> 
 &nbsp; &nbsp; String Error - Gas Report Savings: ~57 
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

### Gas Report

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
 

 <span style="color: green;">File: </span> ConveyorTickMath.sol 91-91 
 ```solidity 
 require(priceX128 <= type(uint256).max, "Overflow") 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 119-119 
 ```solidity 
 require(_weth != address(0), "Invalid weth address") 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 120-120 
 ```solidity 
 require(_usdc != address(0), "Invalid usdc address") 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 121-121 
 ```solidity 
 require(_limitOrderQuoterAddress != address(0), "Invalid LimitOrderQuoter address") 
 ```

 <span style="color: green;">File: </span> OracleLibraryV2.sol 10-10 
 ```solidity 
 require(amountOut > 0, "UniswapV2Library: INSUFFICIENT_OUTPUT_AMOUNT") 
 ```

 <span style="color: green;">File: </span> OracleLibraryV2.sol 11-11 
 ```solidity 
 require(reserveIn > 0 && reserveOut > 0, "UniswapV2Library: INSUFFICIENT_LIQUIDITY") 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 221-221 
 ```solidity 
 require(answer <= uint128(MAX_64x64), "overflow") 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 264-264 
 ```solidity 
 require(answer <= 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, "overflow in divuu") 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 283-283 
 ```solidity 
 require(answer <= 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, "overflow in divuu last") 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 509-509 
 ```solidity 
 require(x < 0x400000000000000000, "Exponential overflow") 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 17-17 
 ```solidity 
 require(_weth != address(0), "Invalid weth address") 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 65-65 
 ```solidity 
 require(_weth != address(0), "WETH address is zero") 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 408-408 
 ```solidity 
 require(tempAffiliateNonce < type(uint16).max >> 0x1, "Affiliate nonce overflow") 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 424-424 
 ```solidity 
 require(tempReferrerNonce < type(uint16).max >> 0x1, "Referrer nonce overflow") 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 43-43 
 ```solidity 
 require(_limitOrderExecutor != address(0), "limitOrderExecutor address is address(0)") 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 45-45 
 ```solidity 
 require(_minExecutionCredit != 0, "Minimum Execution Credit is 0") 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 130-130 
 ```solidity 
 require(_isUniV2[i], "First Dex must be uniswap v2") 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 132-132 
 ```solidity 
 require(_dexFactories[i] != address(0), "Zero values in constructor") 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 94-94 
 ```solidity 
 require(_limitOrderExecutor != address(0), "limitOrderExecutor address is address(0)") 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 95-95 
 ```solidity 
 require(_minExecutionCredit != 0, "Minimum Execution Credit is 0") 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 70-70 
 ```solidity 
 require(_limitOrderExecutor != address(0), "Invalid ConveyorExecutor address") 
 ``` 
 </details> 
 </details>

 <details open> 
 <summary> 
 <h3>Quality Assurance - Instances: 9 </h3> 
 </summary> 
  


 <details open> 
 <summary> 
 <a name=[NC-0]></a> [NC-0] 
 <h3> Constructor should be listed before any other function - Instances: 1 </h3> 
 </summary>
 Description of the qa pattern goes here 

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 64-64 
 ```solidity 
 constructor(address _weth) payable {require(_weth != address(0), "WETH address is zero") CONVEYOR_MULTICALL = address(new ConveyorMulticall()) WETH = _weth owner = tx.origin} 
 ``` 
 </details>

 <details open> 
 <summary> 
 <a name=[NC-1]></a> [NC-1] 
 <h3> Private variables should contain a leading underscore - Instances: 1 </h3> 
 </summary>
 Description of the qa pattern goes here 

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 439-439 
 ```solidity 
 bool private locked; 
 ``` 
 </details>

 <details open> 
 <summary> 
 <a name=[NC-2]></a> [NC-2] 
 <h3> Constructor should initialize all variables - Instances: 13 </h3> 
 </summary>
 Description of the qa pattern goes here 

 <span style="color: green;">File: </span> LimitOrderBook.sol 42-42 
 ```solidity 
 constructor(address _limitOrderExecutor, address _weth, address _usdc, uint256 _minExecutionCredit) {require(_limitOrderExecutor != address(0), "limitOrderExecutor address is address(0)") require(_minExecutionCredit != 0, "Minimum Execution Credit is 0") minExecutionCredit = _minExecutionCredit WETH = _weth USDC = _usdc LIMIT_ORDER_EXECUTOR = _limitOrderExecutor} 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 42-42 
 ```solidity 
 constructor(address _limitOrderExecutor, address _weth, address _usdc, uint256 _minExecutionCredit) {require(_limitOrderExecutor != address(0), "limitOrderExecutor address is address(0)") require(_minExecutionCredit != 0, "Minimum Execution Credit is 0") minExecutionCredit = _minExecutionCredit WETH = _weth USDC = _usdc LIMIT_ORDER_EXECUTOR = _limitOrderExecutor} 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderRouter.sol 54-54 
 ```solidity 
 constructor(address _limitOrderExecutor, address _sandboxLimitOrderBook) {LIMIT_ORDER_EXECUTOR = _limitOrderExecutor SANDBOX_LIMIT_ORDER_BOOK = _sandboxLimitOrderBook} 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderRouter.sol 54-54 
 ```solidity 
 constructor(address _limitOrderExecutor, address _sandboxLimitOrderBook) {LIMIT_ORDER_EXECUTOR = _limitOrderExecutor SANDBOX_LIMIT_ORDER_BOOK = _sandboxLimitOrderBook} 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 65-67 
 ```solidity 
 constructor(address _weth, address _usdc, address _limitOrderExecutor, uint256 _minExecutionCredit) LimitOrderBook(_limitOrderExecutor, _weth, _usdc, _minExecutionCredit) {require(_limitOrderExecutor != address(0), "Invalid ConveyorExecutor address") owner = tx.origin} 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 65-67 
 ```solidity 
 constructor(address _weth, address _usdc, address _limitOrderExecutor, uint256 _minExecutionCredit) LimitOrderBook(_limitOrderExecutor, _weth, _usdc, _minExecutionCredit) {require(_limitOrderExecutor != address(0), "Invalid ConveyorExecutor address") owner = tx.origin} 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 65-67 
 ```solidity 
 constructor(address _weth, address _usdc, address _limitOrderExecutor, uint256 _minExecutionCredit) LimitOrderBook(_limitOrderExecutor, _weth, _usdc, _minExecutionCredit) {require(_limitOrderExecutor != address(0), "Invalid ConveyorExecutor address") owner = tx.origin} 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 93-93 
 ```solidity 
 constructor(address _limitOrderExecutor, address _weth, address _usdc, uint256 _minExecutionCredit) {require(_limitOrderExecutor != address(0), "limitOrderExecutor address is address(0)") require(_minExecutionCredit != 0, "Minimum Execution Credit is 0") minExecutionCredit = _minExecutionCredit WETH = _weth USDC = _usdc LIMIT_ORDER_EXECUTOR = _limitOrderExecutor SANDBOX_LIMIT_ORDER_ROUTER = address(new SandboxLimitOrderRouter(_limitOrderExecutor, address(this))) owner = tx.origin} 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 93-93 
 ```solidity 
 constructor(address _limitOrderExecutor, address _weth, address _usdc, uint256 _minExecutionCredit) {require(_limitOrderExecutor != address(0), "limitOrderExecutor address is address(0)") require(_minExecutionCredit != 0, "Minimum Execution Credit is 0") minExecutionCredit = _minExecutionCredit WETH = _weth USDC = _usdc LIMIT_ORDER_EXECUTOR = _limitOrderExecutor SANDBOX_LIMIT_ORDER_ROUTER = address(new SandboxLimitOrderRouter(_limitOrderExecutor, address(this))) owner = tx.origin} 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 126-126 
 ```solidity 
 constructor(address[] memory _dexFactories, bool[] memory _isUniV2) {for (uint256 i = 0; i < _dexFactories.length; ++i) {if (i == 0) {require(_isUniV2[i], "First Dex must be uniswap v2")} require(_dexFactories[i] != address(0), "Zero values in constructor") dexes.push(Dex({factoryAddress: _dexFactories[i], isUniV2: _isUniV2[i]})) address uniswapV3Factory; if (!_isUniV2[i]) {uniswapV3Factory = _dexFactories[i]} UNISWAP_V3_FACTORY = uniswapV3Factory}} 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 111-118 
 ```solidity 
 constructor(address _weth, address _usdc, address _limitOrderQuoterAddress, address[] memory _dexFactories, bool[] memory _isUniV2, uint256 _minExecutionCredit) LimitOrderSwapRouter(_dexFactories, _isUniV2) {require(_weth != address(0), "Invalid weth address") require(_usdc != address(0), "Invalid usdc address") require(_limitOrderQuoterAddress != address(0), "Invalid LimitOrderQuoter address") USDC = _usdc WETH = _weth LIMIT_ORDER_QUOTER = _limitOrderQuoterAddress SANDBOX_LIMIT_ORDER_BOOK = address(new SandboxLimitOrderBook(address(this), _weth, _usdc, _minExecutionCredit)) SANDBOX_LIMIT_ORDER_ROUTER = ISandboxLimitOrderBook(SANDBOX_LIMIT_ORDER_BOOK).getSandboxLimitOrderRouterAddress() LIMIT_ORDER_ROUTER = address(new LimitOrderRouter(_weth, _usdc, address(this), _minExecutionCredit)) owner = msg.sender} 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 111-118 
 ```solidity 
 constructor(address _weth, address _usdc, address _limitOrderQuoterAddress, address[] memory _dexFactories, bool[] memory _isUniV2, uint256 _minExecutionCredit) LimitOrderSwapRouter(_dexFactories, _isUniV2) {require(_weth != address(0), "Invalid weth address") require(_usdc != address(0), "Invalid usdc address") require(_limitOrderQuoterAddress != address(0), "Invalid LimitOrderQuoter address") USDC = _usdc WETH = _weth LIMIT_ORDER_QUOTER = _limitOrderQuoterAddress SANDBOX_LIMIT_ORDER_BOOK = address(new SandboxLimitOrderBook(address(this), _weth, _usdc, _minExecutionCredit)) SANDBOX_LIMIT_ORDER_ROUTER = ISandboxLimitOrderBook(SANDBOX_LIMIT_ORDER_BOOK).getSandboxLimitOrderRouterAddress() LIMIT_ORDER_ROUTER = address(new LimitOrderRouter(_weth, _usdc, address(this), _minExecutionCredit)) owner = msg.sender} 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 111-118 
 ```solidity 
 constructor(address _weth, address _usdc, address _limitOrderQuoterAddress, address[] memory _dexFactories, bool[] memory _isUniV2, uint256 _minExecutionCredit) LimitOrderSwapRouter(_dexFactories, _isUniV2) {require(_weth != address(0), "Invalid weth address") require(_usdc != address(0), "Invalid usdc address") require(_limitOrderQuoterAddress != address(0), "Invalid LimitOrderQuoter address") USDC = _usdc WETH = _weth LIMIT_ORDER_QUOTER = _limitOrderQuoterAddress SANDBOX_LIMIT_ORDER_BOOK = address(new SandboxLimitOrderBook(address(this), _weth, _usdc, _minExecutionCredit)) SANDBOX_LIMIT_ORDER_ROUTER = ISandboxLimitOrderBook(SANDBOX_LIMIT_ORDER_BOOK).getSandboxLimitOrderRouterAddress() LIMIT_ORDER_ROUTER = address(new LimitOrderRouter(_weth, _usdc, address(this), _minExecutionCredit)) owner = msg.sender} 
 ``` 
 </details>

 <details open> 
 <summary> 
 <a name=[NC-3]></a> [NC-3] 
 <h3> Consider importing specific identifiers instead of the whole file - Instances: 157 </h3> 
 </summary>
 This will minimize compiled code size and help with readability 

 <span style="color: green;">File: </span> UniFiCallback.sol 4-4 
 ```solidity 
 import "../../lib/interfaces/token/IERC20.sol"; 
 ```

 <span style="color: green;">File: </span> UniFiCallback.sol 5-5 
 ```solidity 
 import "../../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol"; 
 ```

 <span style="color: green;">File: </span> UniFiCallback.sol 6-6 
 ```solidity 
 import "../lib/OracleLibraryV2.sol"; 
 ```

 <span style="color: green;">File: </span> MeerkatCallback.sol 4-4 
 ```solidity 
 import "../../lib/interfaces/token/IERC20.sol"; 
 ```

 <span style="color: green;">File: </span> MeerkatCallback.sol 5-5 
 ```solidity 
 import "../../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol"; 
 ```

 <span style="color: green;">File: </span> MeerkatCallback.sol 6-6 
 ```solidity 
 import "../lib/OracleLibraryV2.sol"; 
 ```

 <span style="color: green;">File: </span> AlgebraCallback.sol 4-4 
 ```solidity 
 import "../../lib/interfaces/token/IERC20.sol"; 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 4-4 
 ```solidity 
 import "./LimitOrderSwapRouter.sol"; 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 5-5 
 ```solidity 
 import "./lib/ConveyorTickMath.sol"; 
 ```

 <span style="color: green;">File: </span> LimitOrderQuoter.sol 6-6 
 ```solidity 
 import "./interfaces/ILimitOrderQuoter.sol"; 
 ```

 <span style="color: green;">File: </span> DeployBSCAggregator.s.sol 7-7 
 ```solidity 
 import "../../test/utils/Console.sol"; 
 ```

 <span style="color: green;">File: </span> ApeSwapCallback.sol 4-4 
 ```solidity 
 import "../../lib/interfaces/token/IERC20.sol"; 
 ```

 <span style="color: green;">File: </span> ApeSwapCallback.sol 5-5 
 ```solidity 
 import "../../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol"; 
 ```

 <span style="color: green;">File: </span> ApeSwapCallback.sol 6-6 
 ```solidity 
 import "../lib/OracleLibraryV2.sol"; 
 ```

 <span style="color: green;">File: </span> UniswapV3Callback.sol 4-4 
 ```solidity 
 import "../../lib/interfaces/token/IERC20.sol"; 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 4-4 
 ```solidity 
 import "../lib/interfaces/token/IERC20.sol"; 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 5-5 
 ```solidity 
 import "./LimitOrderBook.sol"; 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 6-6 
 ```solidity 
 import "./ConveyorErrors.sol"; 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 7-7 
 ```solidity 
 import "../lib/interfaces/token/IWETH.sol"; 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 8-8 
 ```solidity 
 import "./LimitOrderSwapRouter.sol"; 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 9-9 
 ```solidity 
 import "./interfaces/ILimitOrderQuoter.sol"; 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 10-10 
 ```solidity 
 import "./interfaces/IConveyorExecutor.sol"; 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 11-11 
 ```solidity 
 import "./interfaces/ILimitOrderRouter.sol"; 
 ```

 <span style="color: green;">File: </span> KyberSwapV3Callback.sol 4-4 
 ```solidity 
 import "../../lib/interfaces/token/IERC20.sol"; 
 ```

 <span style="color: green;">File: </span> PancakeV2Callback.sol 4-4 
 ```solidity 
 import "../../lib/interfaces/token/IERC20.sol"; 
 ```

 <span style="color: green;">File: </span> PancakeV2Callback.sol 5-5 
 ```solidity 
 import "../../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol"; 
 ```

 <span style="color: green;">File: </span> PancakeV2Callback.sol 6-6 
 ```solidity 
 import "../lib/OracleLibraryV2.sol"; 
 ```

 <span style="color: green;">File: </span> LinkSwapCallback.sol 4-4 
 ```solidity 
 import "../../lib/interfaces/token/IERC20.sol"; 
 ```

 <span style="color: green;">File: </span> LinkSwapCallback.sol 5-5 
 ```solidity 
 import "../../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol"; 
 ```

 <span style="color: green;">File: </span> LinkSwapCallback.sol 6-6 
 ```solidity 
 import "../lib/OracleLibraryV2.sol"; 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 4-4 
 ```solidity 
 import "./LimitOrderSwapRouter.sol"; 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 5-5 
 ```solidity 
 import "./interfaces/ILimitOrderQuoter.sol"; 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 6-6 
 ```solidity 
 import "./lib/ConveyorFeeMath.sol"; 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 7-7 
 ```solidity 
 import "./LimitOrderRouter.sol"; 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 8-8 
 ```solidity 
 import "./interfaces/ILimitOrderSwapRouter.sol"; 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 9-9 
 ```solidity 
 import "./interfaces/ISandboxLimitOrderRouter.sol"; 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 10-10 
 ```solidity 
 import "./interfaces/ISandboxLimitOrderBook.sol"; 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 11-11 
 ```solidity 
 import "./interfaces/ILimitOrderBook.sol"; 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 12-12 
 ```solidity 
 import "./interfaces/IConveyorExecutor.sol"; 
 ```

 <span style="color: green;">File: </span> ILimitOrderQuoter.sol 4-4 
 ```solidity 
 import "../LimitOrderSwapRouter.sol"; 
 ```

 <span style="color: green;">File: </span> DXSwapCallback.sol 4-4 
 ```solidity 
 import "../../lib/interfaces/token/IERC20.sol"; 
 ```

 <span style="color: green;">File: </span> DXSwapCallback.sol 5-5 
 ```solidity 
 import "../../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol"; 
 ```

 <span style="color: green;">File: </span> DXSwapCallback.sol 6-6 
 ```solidity 
 import "../lib/OracleLibraryV2.sol"; 
 ```

 <span style="color: green;">File: </span> ElkSwapCallback.sol 4-4 
 ```solidity 
 import "../../lib/interfaces/token/IERC20.sol"; 
 ```

 <span style="color: green;">File: </span> ElkSwapCallback.sol 5-5 
 ```solidity 
 import "../../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol"; 
 ```

 <span style="color: green;">File: </span> ElkSwapCallback.sol 6-6 
 ```solidity 
 import "../lib/OracleLibraryV2.sol"; 
 ```

 <span style="color: green;">File: </span> IConveyorRouterV1.sol 4-4 
 ```solidity 
 import "../ConveyorRouterV1.sol"; 
 ```

 <span style="color: green;">File: </span> ConvergenceXCallback.sol 4-4 
 ```solidity 
 import "../../lib/interfaces/token/IERC20.sol"; 
 ```

 <span style="color: green;">File: </span> ConvergenceXCallback.sol 5-5 
 ```solidity 
 import "../../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol"; 
 ```

 <span style="color: green;">File: </span> ConvergenceXCallback.sol 6-6 
 ```solidity 
 import "../lib/OracleLibraryV2.sol"; 
 ```

 <span style="color: green;">File: </span> ISandboxLimitOrderRouter.sol 4-4 
 ```solidity 
 import "../SandboxLimitOrderRouter.sol"; 
 ```

 <span style="color: green;">File: </span> IConveyorExecutor.sol 4-4 
 ```solidity 
 import "../LimitOrderBook.sol"; 
 ```

 <span style="color: green;">File: </span> IConveyorExecutor.sol 5-5 
 ```solidity 
 import "../SandboxLimitOrderBook.sol"; 
 ```

 <span style="color: green;">File: </span> IConveyorExecutor.sol 6-6 
 ```solidity 
 import "../SandboxLimitOrderRouter.sol"; 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 4-4 
 ```solidity 
 import "../lib/interfaces/token/IERC20.sol"; 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 5-5 
 ```solidity 
 import "./ConveyorErrors.sol"; 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 6-6 
 ```solidity 
 import "./interfaces/ILimitOrderSwapRouter.sol"; 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 7-7 
 ```solidity 
 import "./lib/ConveyorMath.sol"; 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 8-8 
 ```solidity 
 import "./interfaces/IConveyorExecutor.sol"; 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 4-4 
 ```solidity 
 import "./ConveyorErrors.sol"; 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 10-10 
 ```solidity 
 import "../test/utils/Console.sol"; 
 ```

 <span style="color: green;">File: </span> BiswapCallback.sol 4-4 
 ```solidity 
 import "../../lib/interfaces/token/IERC20.sol"; 
 ```

 <span style="color: green;">File: </span> BiswapCallback.sol 5-5 
 ```solidity 
 import "../../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol"; 
 ```

 <span style="color: green;">File: </span> BiswapCallback.sol 6-6 
 ```solidity 
 import "../lib/OracleLibraryV2.sol"; 
 ```

 <span style="color: green;">File: </span> ILimitOrderSwapRouter.sol 4-4 
 ```solidity 
 import "../LimitOrderSwapRouter.sol"; 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 4-4 
 ```solidity 
 import "./ConveyorErrors.sol"; 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 5-5 
 ```solidity 
 import "../lib/interfaces/token/IERC20.sol"; 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 6-6 
 ```solidity 
 import "./interfaces/ILimitOrderBook.sol"; 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 7-7 
 ```solidity 
 import "./interfaces/ILimitOrderSwapRouter.sol"; 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 8-8 
 ```solidity 
 import "./LimitOrderSwapRouter.sol"; 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 9-9 
 ```solidity 
 import "./lib/ConveyorMath.sol"; 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 10-10 
 ```solidity 
 import "./interfaces/IConveyorExecutor.sol"; 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 11-11 
 ```solidity 
 import "./SandboxLimitOrderRouter.sol"; 
 ```

 <span style="color: green;">File: </span> VerseCallback.sol 4-4 
 ```solidity 
 import "../../lib/interfaces/token/IERC20.sol"; 
 ```

 <span style="color: green;">File: </span> VerseCallback.sol 5-5 
 ```solidity 
 import "../../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol"; 
 ```

 <span style="color: green;">File: </span> VerseCallback.sol 6-6 
 ```solidity 
 import "../lib/OracleLibraryV2.sol"; 
 ```

 <span style="color: green;">File: </span> BabyDogeCallback.sol 4-4 
 ```solidity 
 import "../../lib/interfaces/token/IERC20.sol"; 
 ```

 <span style="color: green;">File: </span> BabyDogeCallback.sol 5-5 
 ```solidity 
 import "../../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol"; 
 ```

 <span style="color: green;">File: </span> BabyDogeCallback.sol 6-6 
 ```solidity 
 import "../lib/OracleLibraryV2.sol"; 
 ```

 <span style="color: green;">File: </span> UniswapV2Callback.sol 4-4 
 ```solidity 
 import "../../lib/interfaces/token/IERC20.sol"; 
 ```

 <span style="color: green;">File: </span> UniswapV2Callback.sol 5-5 
 ```solidity 
 import "../../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol"; 
 ```

 <span style="color: green;">File: </span> UniswapV2Callback.sol 6-6 
 ```solidity 
 import "../lib/OracleLibraryV2.sol"; 
 ```

 <span style="color: green;">File: </span> DefiSwapCallback.sol 4-4 
 ```solidity 
 import "../../lib/interfaces/token/IERC20.sol"; 
 ```

 <span style="color: green;">File: </span> DefiSwapCallback.sol 5-5 
 ```solidity 
 import "../../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol"; 
 ```

 <span style="color: green;">File: </span> DefiSwapCallback.sol 6-6 
 ```solidity 
 import "../lib/OracleLibraryV2.sol"; 
 ```

 <span style="color: green;">File: </span> MdexSwapCallback.sol 4-4 
 ```solidity 
 import "../../lib/interfaces/token/IERC20.sol"; 
 ```

 <span style="color: green;">File: </span> MdexSwapCallback.sol 5-5 
 ```solidity 
 import "../../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol"; 
 ```

 <span style="color: green;">File: </span> MdexSwapCallback.sol 6-6 
 ```solidity 
 import "../lib/OracleLibraryV2.sol"; 
 ```

 <span style="color: green;">File: </span> SakeSwapCallback.sol 4-4 
 ```solidity 
 import "../../lib/interfaces/token/IERC20.sol"; 
 ```

 <span style="color: green;">File: </span> SakeSwapCallback.sol 5-5 
 ```solidity 
 import "../../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol"; 
 ```

 <span style="color: green;">File: </span> SakeSwapCallback.sol 6-6 
 ```solidity 
 import "../lib/OracleLibraryV2.sol"; 
 ```

 <span style="color: green;">File: </span> WaultSwapCallback.sol 4-4 
 ```solidity 
 import "../../lib/interfaces/token/IERC20.sol"; 
 ```

 <span style="color: green;">File: </span> WaultSwapCallback.sol 5-5 
 ```solidity 
 import "../../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol"; 
 ```

 <span style="color: green;">File: </span> WaultSwapCallback.sol 6-6 
 ```solidity 
 import "../lib/OracleLibraryV2.sol"; 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 4-4 
 ```solidity 
 import "../lib/interfaces/token/IERC20.sol"; 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 5-5 
 ```solidity 
 import "../lib/interfaces/uniswap-v2/IUniswapV2Factory.sol"; 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 6-6 
 ```solidity 
 import "../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol"; 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 7-7 
 ```solidity 
 import "../lib/interfaces/uniswap-v3/IUniswapV3Factory.sol"; 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 8-8 
 ```solidity 
 import "../lib/interfaces/uniswap-v3/IUniswapV3Pool.sol"; 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 9-9 
 ```solidity 
 import "./lib/ConveyorMath.sol"; 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 10-10 
 ```solidity 
 import "./LimitOrderBook.sol"; 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 11-11 
 ```solidity 
 import "./lib/ConveyorTickMath.sol"; 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 12-12 
 ```solidity 
 import "../lib/libraries/Uniswap/FullMath.sol"; 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 13-13 
 ```solidity 
 import "../lib/libraries/Uniswap/FixedPoint96.sol"; 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 14-14 
 ```solidity 
 import "../lib/libraries/Uniswap/TickMath.sol"; 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 15-15 
 ```solidity 
 import "../lib/interfaces/token/IWETH.sol"; 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 16-16 
 ```solidity 
 import "./lib/ConveyorFeeMath.sol"; 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 17-17 
 ```solidity 
 import "../lib/libraries/Uniswap/SqrtPriceMath.sol"; 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 18-18 
 ```solidity 
 import "../lib/interfaces/uniswap-v3/IQuoter.sol"; 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 19-19 
 ```solidity 
 import "../lib/libraries/token/SafeERC20.sol"; 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 20-20 
 ```solidity 
 import "./ConveyorErrors.sol"; 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 21-21 
 ```solidity 
 import "./interfaces/ILimitOrderSwapRouter.sol"; 
 ```

 <span style="color: green;">File: </span> BabySwapCallback.sol 4-4 
 ```solidity 
 import "../../lib/interfaces/token/IERC20.sol"; 
 ```

 <span style="color: green;">File: </span> BabySwapCallback.sol 5-5 
 ```solidity 
 import "../../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol"; 
 ```

 <span style="color: green;">File: </span> BabySwapCallback.sol 6-6 
 ```solidity 
 import "../lib/OracleLibraryV2.sol"; 
 ```

 <span style="color: green;">File: </span> ISandboxLimitOrderBook.sol 4-4 
 ```solidity 
 import "../SandboxLimitOrderRouter.sol"; 
 ```

 <span style="color: green;">File: </span> ISandboxLimitOrderBook.sol 5-5 
 ```solidity 
 import "../SandboxLimitOrderBook.sol"; 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderRouter.sol 4-4 
 ```solidity 
 import "../lib/interfaces/token/IERC20.sol"; 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderRouter.sol 5-5 
 ```solidity 
 import "./ConveyorErrors.sol"; 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderRouter.sol 6-6 
 ```solidity 
 import "./interfaces/ISandboxLimitOrderBook.sol"; 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderRouter.sol 7-7 
 ```solidity 
 import "../lib/libraries/token/SafeERC20.sol"; 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderRouter.sol 8-8 
 ```solidity 
 import "./interfaces/ISandboxLimitOrderRouter.sol"; 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderRouter.sol 9-9 
 ```solidity 
 import "./interfaces/IConveyorExecutor.sol"; 
 ```

 <span style="color: green;">File: </span> ILimitOrderBook.sol 4-4 
 ```solidity 
 import "../LimitOrderBook.sol"; 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 4-4 
 ```solidity 
 import "../../lib/libraries/Uniswap/FullMath.sol"; 
 ```

 <span style="color: green;">File: </span> PancakeV3Callback.sol 4-4 
 ```solidity 
 import "../../lib/interfaces/token/IERC20.sol"; 
 ```

 <span style="color: green;">File: </span> DeployTest.s.sol 7-7 
 ```solidity 
 import "../../test/utils/Console.sol"; 
 ```

 <span style="color: green;">File: </span> DystopiaCallback.sol 4-4 
 ```solidity 
 import "../../lib/interfaces/token/IERC20.sol"; 
 ```

 <span style="color: green;">File: </span> DystopiaCallback.sol 5-5 
 ```solidity 
 import "../../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol"; 
 ```

 <span style="color: green;">File: </span> DystopiaCallback.sol 6-6 
 ```solidity 
 import "../lib/OracleLibraryV2.sol"; 
 ```

 <span style="color: green;">File: </span> CafeSwapCallback.sol 4-4 
 ```solidity 
 import "../../lib/interfaces/token/IERC20.sol"; 
 ```

 <span style="color: green;">File: </span> CafeSwapCallback.sol 5-5 
 ```solidity 
 import "../../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol"; 
 ```

 <span style="color: green;">File: </span> CafeSwapCallback.sol 6-6 
 ```solidity 
 import "../lib/OracleLibraryV2.sol"; 
 ```

 <span style="color: green;">File: </span> ConveyorFeeMath.sol 4-4 
 ```solidity 
 import "./ConveyorMath.sol"; 
 ```

 <span style="color: green;">File: </span> ConveyorFeeMath.sol 5-5 
 ```solidity 
 import "../../lib/libraries/QuadruplePrecision.sol"; 
 ```

 <span style="color: green;">File: </span> JetSwapCallback.sol 4-4 
 ```solidity 
 import "../../lib/interfaces/token/IERC20.sol"; 
 ```

 <span style="color: green;">File: </span> JetSwapCallback.sol 5-5 
 ```solidity 
 import "../../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol"; 
 ```

 <span style="color: green;">File: </span> JetSwapCallback.sol 6-6 
 ```solidity 
 import "../lib/OracleLibraryV2.sol"; 
 ```

 <span style="color: green;">File: </span> ConveyorTickMath.sol 4-4 
 ```solidity 
 import "../../lib/libraries/Uniswap/FullMath.sol"; 
 ```

 <span style="color: green;">File: </span> ConveyorTickMath.sol 5-5 
 ```solidity 
 import "../../lib/libraries/Uniswap/LowGasSafeMath.sol"; 
 ```

 <span style="color: green;">File: </span> ConveyorTickMath.sol 6-6 
 ```solidity 
 import "../../lib/libraries/Uniswap/SafeCast.sol"; 
 ```

 <span style="color: green;">File: </span> ConveyorTickMath.sol 7-7 
 ```solidity 
 import "../../lib/libraries/Uniswap/SqrtPriceMath.sol"; 
 ```

 <span style="color: green;">File: </span> ConveyorTickMath.sol 8-8 
 ```solidity 
 import "../../lib/libraries/Uniswap/TickMath.sol"; 
 ```

 <span style="color: green;">File: </span> ConveyorTickMath.sol 9-9 
 ```solidity 
 import "../../lib/libraries/Uniswap/TickBitmap.sol"; 
 ```

 <span style="color: green;">File: </span> ConveyorTickMath.sol 10-10 
 ```solidity 
 import "../../lib/libraries/Uniswap/SwapMath.sol"; 
 ```

 <span style="color: green;">File: </span> ConveyorTickMath.sol 11-11 
 ```solidity 
 import "../../lib/interfaces/uniswap-v3/IUniswapV3Pool.sol"; 
 ```

 <span style="color: green;">File: </span> ConveyorTickMath.sol 12-12 
 ```solidity 
 import "../../lib/libraries/Uniswap/LowGasSafeMath.sol"; 
 ```

 <span style="color: green;">File: </span> ConveyorTickMath.sol 13-13 
 ```solidity 
 import "../../lib/libraries/Uniswap/LiquidityMath.sol"; 
 ```

 <span style="color: green;">File: </span> ConveyorTickMath.sol 14-14 
 ```solidity 
 import "../../lib/libraries/Uniswap/Tick.sol"; 
 ```

 <span style="color: green;">File: </span> ConveyorTickMath.sol 15-15 
 ```solidity 
 import "../../lib/libraries/Uniswap/SafeCast.sol"; 
 ```

 <span style="color: green;">File: </span> ConveyorTickMath.sol 16-16 
 ```solidity 
 import "../../lib/interfaces/token/IERC20.sol"; 
 ```

 <span style="color: green;">File: </span> NomiswapCallback.sol 4-4 
 ```solidity 
 import "../../lib/interfaces/token/IERC20.sol"; 
 ```

 <span style="color: green;">File: </span> NomiswapCallback.sol 5-5 
 ```solidity 
 import "../../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol"; 
 ```

 <span style="color: green;">File: </span> NomiswapCallback.sol 6-6 
 ```solidity 
 import "../lib/OracleLibraryV2.sol"; 
 ```

 <span style="color: green;">File: </span> TraderJoeCallback.sol 4-4 
 ```solidity 
 import "../../lib/interfaces/token/IERC20.sol"; 
 ```

 <span style="color: green;">File: </span> TraderJoeCallback.sol 5-5 
 ```solidity 
 import "../../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol"; 
 ```

 <span style="color: green;">File: </span> TraderJoeCallback.sol 6-6 
 ```solidity 
 import "../lib/OracleLibraryV2.sol"; 
 ``` 
 </details>

 <details open> 
 <summary> 
 <a name=[NC-4]></a> [NC-4] 
 <h3> Constants & Immutables should be named with screaming snake case - Instances: 6 </h3> 
 </summary>
 Consider renaming to follow convention 

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 101-101 
 ```solidity 
 uint128 private constant MIN_FEE_64x64 = 18446744073709552; 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 105-105 
 ```solidity 
 uint256 private constant ONE_128x128 = uint256(1) << 128; 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 8-8 
 ```solidity 
 uint128 private constant MAX_64x64 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF; 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 13-13 
 ```solidity 
 int128 private constant MIN_64x64 = -0x80000000000000000000000000000000; 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 16-16 
 ```solidity 
 uint256 private constant MAX_128x128 = 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff; 
 ```

 <span style="color: green;">File: </span> ConveyorTickMath.sol 28-28 
 ```solidity 
 uint128 private constant MAX_64x64 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF; 
 ``` 
 </details>

 <details open> 
 <summary> 
 <a name=[NC-5]></a> [NC-5] 
 <h3> Consider using scientific notation for large multiples of 10 - Instances: 17 </h3> 
 </summary>
 For example 100000 can be written as 1e5 

 <span style="color: green;">File: </span> ConveyorFeeMath.sol 9-9 
 ```solidity 
 uint128 constant ZERO_POINT_ZERO_ZERO_FIVE = 92233720368547760; 
 ```

 <span style="color: green;">File: </span> ConveyorFeeMath.sol 10-10 
 ```solidity 
 uint128 constant ZERO_POINT_ZERO_ZERO_ONE = 18446744073709550; 
 ```

 <span style="color: green;">File: </span> ConveyorFeeMath.sol 11-11 
 ```solidity 
 uint128 constant MAX_CONVEYOR_PERCENT = 110680464442257300 * 10 ** 2; 
 ```

 <span style="color: green;">File: </span> ConveyorFeeMath.sol 12-12 
 ```solidity 
 uint128 constant MIN_CONVEYOR_PERCENT = 7378697629483821000; 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 33-33 
 ```solidity 
 uint256 private constant REFRESH_INTERVAL = 2592000; 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 39-39 
 ```solidity 
 uint256 private constant REFRESH_FEE = 20000000000000000; 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 102-102 
 ```solidity 
 uint128 private constant BASE_SWAP_FEE = 55340232221128660; 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 107-107 
 ```solidity 
 uint256 private constant ZERO_POINT_NINE = 16602069666338597000 << 64; 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 108-108 
 ```solidity 
 uint256 private constant ONE_POINT_TWO_FIVE = 23058430092136940000 << 64; 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 109-109 
 ```solidity 
 uint128 private constant ZERO_POINT_ONE = 1844674407370955300; 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 110-110 
 ```solidity 
 uint128 private constant ZERO_POINT_ZERO_ZERO_FIVE = 92233720368547760; 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 111-111 
 ```solidity 
 uint128 private constant ZERO_POINT_ZERO_ZERO_ONE = 18446744073709550; 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 28-28 
 ```solidity 
 uint128 private constant AFFILIATE_PERCENT = 5534023222112865000; 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 29-29 
 ```solidity 
 uint128 private constant REFERRAL_PERCENT = 5534023222112865000; 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 49-49 
 ```solidity 
 uint256 private constant REFRESH_INTERVAL = 2592000; 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 53-53 
 ```solidity 
 uint256 private constant REFRESH_FEE = 20000000000000000; 
 ```

 <span style="color: green;">File: </span> ConveyorExecutor.sol 35-35 
 ```solidity 
 uint128 private constant STOP_LOSS_MAX_BEACON_REWARD = 50000000000000000; 
 ``` 
 </details>

 <details open> 
 <summary> 
 <a name=[NC-6]></a> [NC-6] 
 <h3> Remove any unused functions - Instances: 28 </h3> 
 </summary>
  

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 222-222 
 ```solidity 
 function _transferBeaconReward(uint256 totalBeaconReward, address executorAddress, address weth) internal {IWETH(weth).withdraw(totalBeaconReward) _safeTransferETH(executorAddress, totalBeaconReward)} 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 295-304 
 ```solidity 
 function _swap(address _tokenIn, address _tokenOut, address _lp, uint24 _fee, uint256 _amountIn, uint256 _amountOutMin, address _receiver, address _sender) internal returns (uint256 amountReceived) {if (_lpIsNotUniV3(_lp)) {amountReceived = _swapV2(_tokenIn, _tokenOut, _lp, _amountIn, _amountOutMin, _receiver, _sender)} else {amountReceived = _swapV3(_lp, _tokenIn, _tokenOut, _fee, _amountIn, _amountOutMin, _receiver, _sender)}} 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 214-214 
 ```solidity 
 function _transferTokensOutToOwner(address orderOwner, uint256 amount, address tokenOut) internal {IERC20(tokenOut).safeTransfer(orderOwner, amount)} 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 128-128 
 ```solidity 
 function mul128x64(uint256 x, uint128 y) internal pure returns (uint256) {if (x == 0 || y == 0) {return 0;} uint256 answer = (uint256(y) * x) >> 64; return answer;} 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 162-162 
 ```solidity 
 function mul128U(uint256 x, uint256 y) internal pure returns (uint256) {if (y == 0 || x == 0) {return 0;} return (x * y) >> 128;} 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 31-31 
 ```solidity 
 function toUInt64(uint128 x) internal pure returns (uint64) {unchecked {return uint64(x >> 64);}} 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 84-84 
 ```solidity 
 function sub(int128 x, int128 y) internal pure returns (int128) {unchecked {int256 result = int256(x) - y; require(result >= MIN_64x64 && result <= type(int128).max) return int128(result);}} 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 62-62 
 ```solidity 
 function to128x128(uint128 x) internal pure returns (uint256) {unchecked {return uint256(x) << 64;}} 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 173-173 
 ```solidity 
 function abs(int256 x) internal pure returns (int256) {unchecked {return x < 0 ? -x : x;}} 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 183-183 
 ```solidity 
 function div64x64(uint128 x, uint128 y) internal pure returns (uint128) {unchecked {require(y != 0) uint256 answer = (uint256(x) << 64) / y; require(answer <= 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF) return uint128(answer);}} 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 288-288 
 ```solidity 
 function fromX64ToX16(uint128 x) internal pure returns (uint32) {uint16 decimals = uint16(uint64(x & 0xFFFFFFFFFFFFFFFF) >> 48); uint16 integers = uint16(uint64(x >> 64) >> 48); uint32 result = (uint32(integers) << 16) + decimals; return result;} 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 141-141 
 ```solidity 
 function mul64U(uint128 x, uint256 y) internal pure returns (uint256) {unchecked {if (y == 0 || x == 0) {return 0;} uint256 lo = (uint256(x) * (y & 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF)) >> 64; uint256 hi = uint256(x) * (y >> 128); require(hi <= 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF) hi <<= 64 require(hi <= MAX_128x128 - lo) return hi + lo;}} 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 518-518 
 ```solidity 
 function sqrtu(uint256 x) internal pure returns (uint128) {unchecked {if (x == 0) {return 0;} else {uint256 xx = x; uint256 r = 1; if (xx >= 0x100000000000000000000000000000000) {xx >>= 128 r <<= 64} if (xx >= 0x10000000000000000) {xx >>= 64 r <<= 32} if (xx >= 0x100000000) {xx >>= 32 r <<= 16} if (xx >= 0x10000) {xx >>= 16 r <<= 8} if (xx >= 0x100) {xx >>= 8 r <<= 4} if (xx >= 0x10) {xx >>= 4 r <<= 2} if (xx >= 0x8) {r <<= 1} r = (r + x / r) >> 1 r = (r + x / r) >> 1 r = (r + x / r) >> 1 r = (r + x / r) >> 1 r = (r + x / r) >> 1 r = (r + x / r) >> 1 r = (r + x / r) >> 1 uint256 r1 = x / r; return uint128(r < r1 ? r : r1);}}} 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 51-51 
 ```solidity 
 function from128x128(uint256 x) internal pure returns (uint128) {unchecked {uint256 answer = x >> 64; require(answer >= 0x0 && answer <= MAX_64x64) return uint128(answer);}} 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 72-72 
 ```solidity 
 function add64x64(uint128 x, uint128 y) internal pure returns (uint128) {unchecked {uint256 answer = uint256(x) + y; require(answer <= MAX_64x64) return uint128(answer);}} 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 96-96 
 ```solidity 
 function add128x128(uint256 x, uint256 y) internal pure returns (uint256) {uint256 answer = x + y; return answer;} 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 106-106 
 ```solidity 
 function add128x64(uint256 x, uint128 y) internal pure returns (uint256) {uint256 answer = x + (uint256(y) << 64); return answer;} 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 116-116 
 ```solidity 
 function mul64x64(uint128 x, uint128 y) internal pure returns (uint128) {unchecked {uint256 answer = (uint256(x) * y) >> 64; require(answer <= MAX_64x64) return uint128(answer);}} 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 507-507 
 ```solidity 
 function exp(uint128 x) internal pure returns (uint128) {unchecked {require(x < 0x400000000000000000, "Exponential overflow") return exp_2(uint128((uint256(x) * 0x171547652B82FE1777D0FFDA0D23A7D12) >> 128));}} 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 21-21 
 ```solidity 
 function fromUInt256(uint256 x) internal pure returns (uint128) {unchecked {require(x <= MAX_UINT64) return uint128(x << 64);}} 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 198-198 
 ```solidity 
 function div128x128(uint256 x, uint256 y) internal pure returns (uint256) {unchecked {require(y != 0) uint256 xDec = x & 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF; uint256 xInt = x >> 128; uint256 hi = xInt * (MAX_128x128 / y); uint256 lo = (xDec * (MAX_128x128 / y)) >> 128; require(hi <= MAX_128x128 - lo) return hi + lo;}} 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 217-217 
 ```solidity 
 function divUU(uint256 x, uint256 y) internal pure returns (uint128) {unchecked {require(y != 0) uint128 answer = divuu(x, y); require(answer <= uint128(MAX_64x64), "overflow") return answer;}} 
 ```

 <span style="color: green;">File: </span> ConveyorMath.sol 40-40 
 ```solidity 
 function fromUInt128(uint128 x) internal pure returns (uint256) {unchecked {require(x <= 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF) return uint256(x) << 128;}} 
 ```

 <span style="color: green;">File: </span> ConveyorTickMath.sol 69-73 
 ```solidity 
 function fromSqrtX96(uint160 sqrtPriceX96, bool token0IsReserve0, address token0, address token1) internal view returns (uint256 priceX128) {unchecked {int8 decimalShift = int8(IERC20(token0).decimals()) - int8(IERC20(token1).decimals()); uint256 priceSquaredX96 = decimalShift < 0 ? uint256(sqrtPriceX96) ** 2 / uint256(10) ** (uint8(-decimalShift)) : uint256(sqrtPriceX96) ** 2 * 10 ** uint8(decimalShift); uint256 priceSquaredShiftQ96 = token0IsReserve0 ? priceSquaredX96 / Q96 : (Q96 * 0xffffffffffffffffffffffffffffffff) / (priceSquaredX96 / Q96); priceX128 = token0IsReserve0 ? (uint256(priceSquaredShiftQ96) * 0xffffffffffffffffffffffffffffffff) / Q96 : priceSquaredShiftQ96 require(priceX128 <= type(uint256).max, "Overflow")}} 
 ```

 <span style="color: green;">File: </span> ConveyorTickMath.sol 103-111 
 ```solidity 
 function simulateAmountOutOnSqrtPriceX96(address token0, address tokenIn, address pool, uint256 amountIn, int24 tickSpacing, uint128 liquidity, uint24 fee) internal view returns (uint128 amountOut, uint160 sqrtPriceX96) {bool zeroForOne = token0 == tokenIn ? true : false; int24 initialTick; {(sqrtPriceX96, initialTick) = IUniswapV3Pool(pool).slot0()} uint160 sqrtPriceLimitX96 = zeroForOne ? TickMath.MIN_SQRT_RATIO + 1 : TickMath.MAX_SQRT_RATIO - 1; CurrentState memory currentState = CurrentState({sqrtPriceX96: sqrtPriceX96, amountCalculated: 0, amountSpecifiedRemaining: int256(amountIn), tick: initialTick, liquidity: liquidity}); while (currentState.amountSpecifiedRemaining > 0 && currentState.sqrtPriceX96 != sqrtPriceLimitX96) {StepComputations memory step; step.sqrtPriceStartX96 = currentState.sqrtPriceX96 (step.tickNext, step.initialized) = TickBitmap.nextInitializedTickWithinOneWord(currentState.tick, tickSpacing, zeroForOne, pool) if (step.tickNext < TickMath.MIN_TICK) {step.tickNext = TickMath.MIN_TICK} else if (step.tickNext > TickMath.MAX_TICK) {step.tickNext = TickMath.MAX_TICK} step.sqrtPriceNextX96 = TickMath.getSqrtRatioAtTick(step.tickNext) (currentState.sqrtPriceX96, step.amountIn, step.amountOut, step.feeAmount) = SwapMath.computeSwapStep(currentState.sqrtPriceX96, (zeroForOne ? step.sqrtPriceNextX96 < sqrtPriceLimitX96 : step.sqrtPriceNextX96 > sqrtPriceLimitX96) ? sqrtPriceLimitX96 : step.sqrtPriceNextX96, currentState.liquidity, currentState.amountSpecifiedRemaining, fee) currentState.amountSpecifiedRemaining -= (step.amountIn + step.feeAmount).toInt256() currentState.amountCalculated -= step.amountOut.toInt256() if (currentState.sqrtPriceX96 == step.sqrtPriceNextX96) {if (step.initialized) {int128 liquidityNet = Tick.cross(step.tickNext, pool); if (zeroForOne) liquidityNet = -liquidityNet currentState.liquidity = LiquidityMath.addDelta(currentState.liquidity, liquidityNet)} unchecked {currentState.tick = zeroForOne ? step.tickNext - 1 : step.tickNext}} else if (currentState.sqrtPriceX96 != step.sqrtPriceStartX96) {currentState.tick = TickMath.getTickAtSqrtRatio(currentState.sqrtPriceX96)}} {amountOut = uint128(SafeCast.toInt128(-currentState.amountCalculated)) sqrtPriceX96 = currentState.sqrtPriceX96}} 
 ```

 <span style="color: green;">File: </span> OracleLibraryV2.sol 5-9 
 ```solidity 
 function getAmountIn(uint256 amountOut, uint256 reserveIn, uint256 reserveOut, uint24 swapFee) internal pure returns (uint256 amountIn) {require(amountOut > 0, "UniswapV2Library: INSUFFICIENT_OUTPUT_AMOUNT") require(reserveIn > 0 && reserveOut > 0, "UniswapV2Library: INSUFFICIENT_LIQUIDITY") uint256 numerator = reserveIn * amountOut * 100000; uint256 denominator = (reserveOut - amountOut) * (100000 - swapFee); amountIn = (numerator / denominator) + 1} 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 511-511 
 ```solidity 
 function _resolveCompletedOrder(bytes32 orderId) internal {LimitOrder memory order = orderIdToLimitOrder[orderId]; if (order.orderId == bytes32(0)) {revert DuplicateOrderIdsInOrderGroup();} delete orderIdToLimitOrder[orderId] delete addressToOrderIds[order.owner][orderId] --totalOrdersPerAddress[order.owner] _decrementTotalOrdersQuantity(order.tokenIn, order.owner, order.quantity) addressToOrderIds[order.owner][order.orderId] = OrderType.FilledLimitOrder} 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 496-496 
 ```solidity 
 function _removeOrderFromSystem(bytes32 orderId) internal {LimitOrder memory order = orderIdToLimitOrder[orderId]; delete orderIdToLimitOrder[orderId] --totalOrdersPerAddress[order.owner] _decrementTotalOrdersQuantity(order.tokenIn, order.owner, order.quantity)} 
 ``` 
 </details>

 <details open> 
 <summary> 
 <a name=[NC-7]></a> [NC-7] 
 <h3> Storage variables should be named with camel case - Instances: 1 </h3> 
 </summary>
 Consider renaming to follow convention 

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 22-22 
 ```solidity 
 address public CONVEYOR_MULTICALL; 
 ``` 
 </details>

 <details open> 
 <summary> 
 <a name=[NC-8]></a> [NC-8] 
 <h3> Remove any unused returns - Instances: 11 </h3> 
 </summary>
  

 <span style="color: green;">File: </span> LimitOrderRouter.sol 111-111 
 ```solidity 
 function _refreshLimitOrder(LimitOrder memory order) internal returns (uint256 executorFee) {uint128 executionCreditBalance = order.executionCredit; if (block.timestamp > order.expirationTimestamp) {return _cancelLimitOrderViaExecutor(order);} if (executionCreditBalance < REFRESH_FEE) {return _cancelLimitOrderViaExecutor(order);} else {if (executionCreditBalance - REFRESH_FEE < minExecutionCredit) {return _cancelLimitOrderViaExecutor(order);}} if (IERC20(order.tokenIn).balanceOf(order.owner) < order.quantity) {return _cancelLimitOrderViaExecutor(order);} if (block.timestamp - order.lastRefreshTimestamp < REFRESH_INTERVAL) {revert OrderNotEligibleForRefresh(order.orderId);} orderIdToLimitOrder[order.orderId].executionCredit = executionCreditBalance - uint128(REFRESH_FEE) emit OrderExecutionCreditUpdated(order.orderId, executionCreditBalance - REFRESH_FEE); orderIdToLimitOrder[order.orderId].lastRefreshTimestamp = uint32(block.timestamp % (2 ** 32 - 1)) emit OrderRefreshed(order.orderId, order.lastRefreshTimestamp, order.expirationTimestamp); return REFRESH_FEE;} 
 ```

 <span style="color: green;">File: </span> LimitOrderRouter.sol 153-153 
 ```solidity 
 function validateAndCancelOrder(bytes32 orderId) external nonReentrant returns (bool success) {uint256 lastCheckInTime = IConveyorExecutor(LIMIT_ORDER_EXECUTOR).lastCheckIn(msg.sender); if (block.timestamp - lastCheckInTime > CHECK_IN_INTERVAL) {revert ExecutorNotCheckedIn();} LimitOrder memory order = getLimitOrderById(orderId); if (IERC20(order.tokenIn).balanceOf(order.owner) < order.quantity) {_safeTransferETH(msg.sender, _cancelLimitOrderViaExecutor(order)) return true;} return false;} 
 ```

 <span style="color: green;">File: </span> LimitOrderBook.sol 537-537 
 ```solidity 
 function getTotalOrdersValue(address token) public view returns (uint256 totalOrderValue) {bytes32 totalOrdersValueKey = keccak256(abi.encode(msg.sender, token)); return totalOrdersQuantity[totalOrdersValueKey];} 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 560-560 
 ```solidity 
 function validateAndCancelOrder(bytes32 orderId) external nonReentrant returns (bool success) {uint256 lastCheckInTime = IConveyorExecutor(LIMIT_ORDER_EXECUTOR).lastCheckIn(msg.sender); if (block.timestamp - lastCheckInTime > CHECK_IN_INTERVAL) {revert ExecutorNotCheckedIn();} SandboxLimitOrder memory order = getSandboxLimitOrderById(orderId); if (IERC20(order.tokenIn).balanceOf(order.owner) < order.amountInRemaining) {_safeTransferETH(msg.sender, _cancelSandboxLimitOrderViaExecutor(order)) return true;} return false;} 
 ```

 <span style="color: green;">File: </span> SandboxLimitOrderBook.sol 1178-1178 
 ```solidity 
 function getTotalOrdersValue(address token) public view returns (uint256 totalOrderValue) {bytes32 totalOrdersValueKey = keccak256(abi.encode(msg.sender, token)); return totalOrdersQuantity[totalOrdersValueKey];} 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 321-330 
 ```solidity 
 function _swapV3(address _lp, address _tokenIn, address _tokenOut, uint24 _fee, uint256 _amountIn, uint256 _amountOutMin, address _receiver, address _sender) internal returns (uint256 amountReceived) {bool _zeroForOne; {(address token0) = _sortTokens(_tokenIn, _tokenOut) _zeroForOne = token0 == _tokenIn ? true : false} bytes memory data = abi.encode(_amountOutMin, _zeroForOne, _tokenIn, _tokenOut, _fee, _sender); IUniswapV3Pool(_lp).swap(_receiver, _zeroForOne, int256(_amountIn), _zeroForOne ? TickMath.MIN_SQRT_RATIO + 1 : TickMath.MAX_SQRT_RATIO - 1, data) uint256 amountOut = uniV3AmountOut; uniV3AmountOut = 0 return amountOut;} 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 577-581 
 ```solidity 
 function getAllPrices(address token0, address token1, uint24 FEE) public view returns (SpotReserve[] memory prices, address[] memory lps) {if (token0 != token1) {SpotReserve[] memory _spotPrices = new SpotReserve[](dexes.length); address[] memory _lps = new address[](dexes.length); for (uint256 i = 0; i < dexes.length;) {if (dexes[i].isUniV2) {{(SpotReserve memory spotPrice, address poolAddress) = _calculateV2SpotPrice(token0, token1, dexes[i].factoryAddress) if (spotPrice.spotPrice != 0) {_spotPrices[i] = spotPrice _lps[i] = poolAddress}}} else {{{(SpotReserve memory spotPrice, address poolAddress) = _calculateV3SpotPrice(token0, token1, FEE, dexes[i].factoryAddress) if (spotPrice.spotPrice != 0) {_lps[i] = poolAddress _spotPrices[i] = spotPrice}}}} unchecked {++i}} return (_spotPrices, _lps);} else {SpotReserve[] memory _spotPrices = new SpotReserve[](dexes.length); address[] memory _lps = new address[](dexes.length); return (_spotPrices, _lps);}} 
 ```

 <span style="color: green;">File: </span> LimitOrderSwapRouter.sol 577-581 
 ```solidity 
 function getAllPrices(address token0, address token1, uint24 FEE) public view returns (SpotReserve[] memory prices, address[] memory lps) {if (token0 != token1) {SpotReserve[] memory _spotPrices = new SpotReserve[](dexes.length); address[] memory _lps = new address[](dexes.length); for (uint256 i = 0; i < dexes.length;) {if (dexes[i].isUniV2) {{(SpotReserve memory spotPrice, address poolAddress) = _calculateV2SpotPrice(token0, token1, dexes[i].factoryAddress) if (spotPrice.spotPrice != 0) {_spotPrices[i] = spotPrice _lps[i] = poolAddress}}} else {{{(SpotReserve memory spotPrice, address poolAddress) = _calculateV3SpotPrice(token0, token1, FEE, dexes[i].factoryAddress) if (spotPrice.spotPrice != 0) {_lps[i] = poolAddress _spotPrices[i] = spotPrice}}}} unchecked {++i}} return (_spotPrices, _lps);} else {SpotReserve[] memory _spotPrices = new SpotReserve[](dexes.length); address[] memory _lps = new address[](dexes.length); return (_spotPrices, _lps);}} 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 262-265 
 ```solidity 
 function quoteSwapExactTokenForToken(TokenToTokenSwapData calldata swapData, SwapAggregatorMulticall calldata swapAggregatorMulticall) external payable returns (uint256 gasConsumed) {uint256 gasBefore; assembly {gasBefore := gas()} swapExactTokenForToken(swapData, swapAggregatorMulticall) assembly {gasConsumed := sub(gasBefore, gas())}} 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 278-281 
 ```solidity 
 function quoteSwapExactEthForToken(EthToTokenSwapData calldata swapData, SwapAggregatorMulticall calldata swapAggregatorMulticall) external payable returns (uint256 gasConsumed) {uint256 gasBefore; assembly {gasBefore := gas()} swapExactEthForToken(swapData, swapAggregatorMulticall) assembly {gasConsumed := sub(gasBefore, gas())}} 
 ```

 <span style="color: green;">File: </span> ConveyorRouterV1.sol 294-297 
 ```solidity 
 function quoteSwapExactTokenForEth(TokenToEthSwapData calldata swapData, SwapAggregatorMulticall calldata swapAggregatorMulticall) external payable returns (uint256 gasConsumed) {uint256 gasBefore; assembly {gasBefore := gas()} swapExactTokenForEth(swapData, swapAggregatorMulticall) assembly {gasConsumed := sub(gasBefore, gas())}} 
 ``` 
 </details> 
 </details>
