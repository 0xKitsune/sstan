# Sstan - v0.1.0 

 --- 
 ## Authors: 0x00face, 0xOsiris 
 --- 
 TODO: add description

# Summary




# <h3>Vulnerabilities</h3> 

 | Classification | Title | Instances | 
 |:-------:|:---------|:-------:| 
 | [[H-0]](#[H-0]) | <Strong>Uninitialized storage variables</Strong> | 2 |
 | [[L-1]](#[L-1]) | <Strong>Unsafe ERC20 Operation</Strong> | 27 |
# <h3>Optimizations</h3> 

 | Classification | Title | Instances | 
 |:-------:|:---------|:-------:| 
 | [[G-0]](#[G-0]) | <Strong>Tightly pack storage variables</Strong> | 4 |
 | [[G-1]](#[G-1]) | <Strong>Avoid Reading From Storage in a for loop</Strong> | 3 |
 | [[G-2]](#[G-2]) | <Strong>Mark storage variables as `immutable` if they never change after contract initialization.</Strong> | 2 |
 | [[G-3]](#[G-3]) | <Strong>`unchecked{++i}` instead of `i++` (or use assembly when applicable)</Strong> | 4 |
 | [[G-4]](#[G-4]) | <Strong>Cache Storage Variables in Memory</Strong> | 6 |
 | [[G-5]](#[G-5]) | <Strong>Use `calldata` instead of `memory` for function arguments that do not get mutated.</Strong> | 5 |
 | [[G-6]](#[G-6]) | <Strong>Use assembly to hash instead of Solidity</Strong> | 2 |
 | [[G-7]](#[G-7]) | <Strong>Use custom errors instead of string error messages</Strong> | 10 |
 | [[G-8]](#[G-8]) | <Strong>Use assembly for math (add, sub, mul, div)</Strong> | 12 |
 | [[G-9]](#[G-9]) | <Strong>Use assembly to write storage values</Strong> | 6 |
 | [[G-10]](#[G-10]) | <Strong>Event is not properly indexed.</Strong> | 4 |
 | [[G-11]](#[G-11]) | <Strong>Use multiple require() statments insted of require(expression && expression && ...)</Strong> | 2 |
 | [[G-12]](#[G-12]) | <Strong>Optimal Comparison</Strong> | 6 |
 | [[G-13]](#[G-13]) | <Strong>Mark functions as payable (with discretion)</Strong> | 44 |
 | [[G-14]](#[G-14]) | <Strong>Consider marking constants as private</Strong> | 14 |
 | [[G-15]](#[G-15]) | <Strong>Use assembly when getting a contract's balance of ETH</Strong> | 1 |
 | [[G-16]](#[G-16]) | <Strong>Use assembly to check for address(0)</Strong> | 7 |
 | [[G-17]](#[G-17]) | <Strong>Cache array length during for loop definition.</Strong> | 8 |
# <h3>Quality Assurance</h3> 

 | Classification | Title | Instances | 
 |:-------:|:---------|:-------:| 
 | [[NC-0]](#[NC-0]) | <Strong>Constructor should be listed before any other function</Strong> | 1 |
 | [[NC-1]](#[NC-1]) | <Strong>Private variables should contain a leading underscore</Strong> | 1 |
 | [[NC-2]](#[NC-2]) | <Strong>Constructor should initialize all variables</Strong> | 13 |
 | [[NC-3]](#[NC-3]) | <Strong>Consider importing specific identifiers instead of the whole file</Strong> | 156 |
 | [[NC-4]](#[NC-4]) | <Strong>Constants & Immutables should be named with screaming snake case</Strong> | 6 |
 | [[NC-5]](#[NC-5]) | <Strong>Consider using scientific notation for large multiples of 10</Strong> | 17 |
 | [[NC-6]](#[NC-6]) | <Strong>Remove any unused functions</Strong> | 28 |
 | [[NC-7]](#[NC-7]) | <Strong>Storage variables should be named with camel case</Strong> | 1 |
 | [[NC-8]](#[NC-8]) | <Strong>Remove any unused returns</Strong> | 11 |
 | [[NC-9]](#[NC-9]) | <Strong>Consider marking public function External</Strong> | 23 |

## Vulnerabilities - Total: 2 


 ### <a name=[H-0]></a> [H-0] Uninitialized storage variables - Instances: 2 

 
> A storage variable that is declared but not initialized will have a default value of zero (or the equivalent, such as an empty array for array types or zero-address for address types). Failing to initialize a storage variable can pose risks if the contract logic assumes that the variable has been explicitly set to a particular value. 
--- 

[File:LimitOrderSwapRouter.sol#L92](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L92) 
```solidity
91:    mapping(address => uint256) dexToIndex;
``` 



[File:ConveyorTickMath.sol#L25](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorTickMath.sol#L25) 
```solidity
24:    mapping(int24 => Tick.Info) public ticks;
``` 



 --- 


 ### <a name=[L-1]></a> [L-1] Unsafe ERC20 Operation - Instances: 27 

 
> ERC20 operations can be unsafe due to different implementations and vulnerabilities in the standard. To account for this, either use OpenZeppelin's SafeERC20 library or wrap each operation in a require statement.
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

[File:VerseCallback.sol#L23](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/VerseCallback.sol#L23) 
```solidity
22:        IERC20(_tokenIn).transfer(msg.sender, amountIn);
``` 



[File:MdexSwapCallback.sol#L23](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/MdexSwapCallback.sol#L23) 
```solidity
22:        IERC20(_tokenIn).transfer(msg.sender, amountIn);
``` 



[File:DefiSwapCallback.sol#L23](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/DefiSwapCallback.sol#L23) 
```solidity
22:        IERC20(_tokenIn).transfer(msg.sender, amountIn);
``` 



[File:UniswapV2Callback.sol#L23](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/UniswapV2Callback.sol#L23) 
```solidity
22:        IERC20(_tokenIn).transfer(msg.sender, amountIn);
``` 



[File:KyberSwapV3Callback.sol#L20](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/KyberSwapV3Callback.sol#L20) 
```solidity
19:            IERC20(_tokenIn).transferFrom(_sender, msg.sender, amountIn);
``` 



[File:KyberSwapV3Callback.sol#L22](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/KyberSwapV3Callback.sol#L22) 
```solidity
21:            IERC20(_tokenIn).transfer(msg.sender, amountIn);
``` 



[File:MeerkatCallback.sol#L23](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/MeerkatCallback.sol#L23) 
```solidity
22:        IERC20(_tokenIn).transfer(msg.sender, amountIn);
``` 



[File:CafeSwapCallback.sol#L23](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/CafeSwapCallback.sol#L23) 
```solidity
22:        IERC20(_tokenIn).transfer(msg.sender, amountIn);
``` 



[File:DXSwapCallback.sol#L23](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/DXSwapCallback.sol#L23) 
```solidity
22:        IERC20(_tokenIn).transfer(msg.sender, amountIn);
``` 



[File:ConveyorRouterV1.sol#L120](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L120) 
```solidity
119:        IERC20(swapData.tokenIn).transferFrom(msg.sender, genericMulticall.tokenInDestination, swapData.amountIn);
``` 



[File:ConveyorRouterV1.sol#L170](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L170) 
```solidity
169:        IERC20(WETH).transfer(swapAggregatorMulticall.tokenInDestination, amountIn);
``` 



[File:ConveyorRouterV1.sol#L215](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L215) 
```solidity
214:            IERC20(swapData.tokenIn).transferFrom(
``` 



[File:ApeSwapCallback.sol#L23](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/ApeSwapCallback.sol#L23) 
```solidity
22:        IERC20(_tokenIn).transfer(msg.sender, amountIn);
``` 



[File:BabyDogeCallback.sol#L23](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/BabyDogeCallback.sol#L23) 
```solidity
22:        IERC20(_tokenIn).transfer(msg.sender, amountIn);
``` 



[File:WaultSwapCallback.sol#L23](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/WaultSwapCallback.sol#L23) 
```solidity
22:        IERC20(_tokenIn).transfer(msg.sender, amountIn);
``` 



[File:DystopiaCallback.sol#L23](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/DystopiaCallback.sol#L23) 
```solidity
22:        IERC20(_tokenIn).transfer(msg.sender, amountIn);
``` 



[File:NomiswapCallback.sol#L23](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/NomiswapCallback.sol#L23) 
```solidity
22:        IERC20(_tokenIn).transfer(msg.sender, amountIn);
``` 



[File:UniFiCallback.sol#L23](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/UniFiCallback.sol#L23) 
```solidity
22:        IERC20(_tokenIn).transfer(msg.sender, amountIn);
``` 



[File:PancakeV2Callback.sol#L23](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/PancakeV2Callback.sol#L23) 
```solidity
22:        IERC20(_tokenIn).transfer(msg.sender, amountIn);
``` 



[File:LinkSwapCallback.sol#L23](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/LinkSwapCallback.sol#L23) 
```solidity
22:        IERC20(_tokenIn).transfer(msg.sender, amountIn);
``` 



[File:ConvergenceXCallback.sol#L23](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/ConvergenceXCallback.sol#L23) 
```solidity
22:        IERC20(_tokenIn).transfer(msg.sender, amountIn);
``` 



[File:SakeSwapCallback.sol#L23](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/SakeSwapCallback.sol#L23) 
```solidity
22:        IERC20(_tokenIn).transfer(msg.sender, amountIn);
``` 



[File:ElkSwapCallback.sol#L23](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/ElkSwapCallback.sol#L23) 
```solidity
22:        IERC20(_tokenIn).transfer(msg.sender, amountIn);
``` 



[File:BabySwapCallback.sol#L23](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/BabySwapCallback.sol#L23) 
```solidity
22:        IERC20(_tokenIn).transfer(msg.sender, amountIn);
``` 



[File:TraderJoeCallback.sol#L23](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/TraderJoeCallback.sol#L23) 
```solidity
22:        IERC20(_tokenIn).transfer(msg.sender, amountIn);
``` 



[File:JetSwapCallback.sol#L23](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/JetSwapCallback.sol#L23) 
```solidity
22:        IERC20(_tokenIn).transfer(msg.sender, amountIn);
``` 



[File:BiswapCallback.sol#L23](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/BiswapCallback.sol#L23) 
```solidity
22:        IERC20(_tokenIn).transfer(msg.sender, amountIn);
``` 



 --- 



## Optimizations - Total: 18 


 ### <a name=[G-0]></a> [G-0] Tightly pack storage variables - Instances: 4 

 
 
> When defining storage variables, make sure to declare them in ascending order, according to size. When multiple variables are able to fit into one 256 bit slot, this will save storage size and gas during runtime. For example, if you have a `bool`, `uint256` and a `bool`, instead of defining the variables in the previously mentioned order, defining the two boolean variables first will pack them both into one storage slot since they only take up one byte of storage. 
 
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

[File:LimitOrderSwapRouter.sol#L84](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L84) 
```solidity
83:    uint256 uniV3AmountOut;
``` 



[File:ConveyorExecutor.sol#L21](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L21) 
```solidity
20:    address immutable WETH;
``` 



[File:LimitOrderBook.sol#L14](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L14) 
```solidity
13:    address immutable LIMIT_ORDER_EXECUTOR;
``` 



[File:ConveyorMath.sol#L8](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L8) 
```solidity
7:    uint128 private constant MAX_64x64 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF;
``` 



 --- 


 ### <a name=[G-1]></a> [G-1] Avoid Reading From Storage in a for loop - Instances: 3 

 
  
  - Savings: ~0 
 <details>  
 <summary>  
  </summary> 
  
 </details> 
 
--- 

[File:LimitOrderSwapRouter.sol#L128](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L128) 
```solidity
127:        for (uint256 i = 0; i < _dexFactories.length; ++i) {
128:            if (i == 0) {
129:                require(_isUniV2[i], "First Dex must be uniswap v2");
130:            }
131:            require(_dexFactories[i] != address(0), "Zero values in constructor");
132:            dexes.push(Dex({factoryAddress: _dexFactories[i], isUniV2: _isUniV2[i]}));
133:
134:            address uniswapV3Factory;
135:            ///@notice If the dex is a univ3 variant, then set the uniswapV3Factory storage address.
136:            if (!_isUniV2[i]) {
137:                uniswapV3Factory = _dexFactories[i];
138:            }
139:
140:            UNISWAP_V3_FACTORY = uniswapV3Factory;
141:        }
142:    }
``` 



[File:LimitOrderSwapRouter.sol#L589](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L589) 
```solidity
588:            for (uint256 i = 0; i < dexes.length;) {
589:                if (dexes[i].isUniV2) {
590:                    {
591:                        ///@notice Get the Uniswap v2 spot price and lp address.
592:                        (SpotReserve memory spotPrice, address poolAddress) =
593:                            _calculateV2SpotPrice(token0, token1, dexes[i].factoryAddress);
594:                        ///@notice Set SpotReserve and lp values if the returned values are not null.
595:                        if (spotPrice.spotPrice != 0) {
596:                            _spotPrices[i] = spotPrice;
597:                            _lps[i] = poolAddress;
598:                        }
599:                    }
600:                } else {
601:                    {
602:                        {
603:                            ///@notice Get the Uniswap v2 spot price and lp address.
604:                            (SpotReserve memory spotPrice, address poolAddress) =
605:                                _calculateV3SpotPrice(token0, token1, FEE, dexes[i].factoryAddress);
606:
607:                            ///@notice Set SpotReserve and lp values if the returned values are not null.
608:                            if (spotPrice.spotPrice != 0) {
609:                                _lps[i] = poolAddress;
610:                                _spotPrices[i] = spotPrice;
611:                            }
612:                        }
613:                    }
614:                }
615:
616:                unchecked {
617:                    ++i;
618:                }
619:            }
620:
``` 



[File:LimitOrderSwapRouter.sol#L589](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L589) 
```solidity
588:            for (uint256 i = 0; i < dexes.length;) {
589:                if (dexes[i].isUniV2) {
590:                    {
591:                        ///@notice Get the Uniswap v2 spot price and lp address.
592:                        (SpotReserve memory spotPrice, address poolAddress) =
593:                            _calculateV2SpotPrice(token0, token1, dexes[i].factoryAddress);
594:                        ///@notice Set SpotReserve and lp values if the returned values are not null.
595:                        if (spotPrice.spotPrice != 0) {
596:                            _spotPrices[i] = spotPrice;
597:                            _lps[i] = poolAddress;
598:                        }
599:                    }
600:                } else {
601:                    {
602:                        {
603:                            ///@notice Get the Uniswap v2 spot price and lp address.
604:                            (SpotReserve memory spotPrice, address poolAddress) =
605:                                _calculateV3SpotPrice(token0, token1, FEE, dexes[i].factoryAddress);
606:
607:                            ///@notice Set SpotReserve and lp values if the returned values are not null.
608:                            if (spotPrice.spotPrice != 0) {
609:                                _lps[i] = poolAddress;
610:                                _spotPrices[i] = spotPrice;
611:                            }
612:                        }
613:                    }
614:                }
615:
616:                unchecked {
617:                    ++i;
618:                }
619:            }
620:
``` 



[File:LimitOrderSwapRouter.sol#L589](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L589) 
```solidity
588:            for (uint256 i = 0; i < dexes.length;) {
589:                if (dexes[i].isUniV2) {
590:                    {
591:                        ///@notice Get the Uniswap v2 spot price and lp address.
592:                        (SpotReserve memory spotPrice, address poolAddress) =
593:                            _calculateV2SpotPrice(token0, token1, dexes[i].factoryAddress);
594:                        ///@notice Set SpotReserve and lp values if the returned values are not null.
595:                        if (spotPrice.spotPrice != 0) {
596:                            _spotPrices[i] = spotPrice;
597:                            _lps[i] = poolAddress;
598:                        }
599:                    }
600:                } else {
601:                    {
602:                        {
603:                            ///@notice Get the Uniswap v2 spot price and lp address.
604:                            (SpotReserve memory spotPrice, address poolAddress) =
605:                                _calculateV3SpotPrice(token0, token1, FEE, dexes[i].factoryAddress);
606:
607:                            ///@notice Set SpotReserve and lp values if the returned values are not null.
608:                            if (spotPrice.spotPrice != 0) {
609:                                _lps[i] = poolAddress;
610:                                _spotPrices[i] = spotPrice;
611:                            }
612:                        }
613:                    }
614:                }
615:
616:                unchecked {
617:                    ++i;
618:                }
619:            }
620:
``` 



[File:LimitOrderSwapRouter.sol#L589](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L589) 
```solidity
588:            for (uint256 i = 0; i < dexes.length;) {
589:                if (dexes[i].isUniV2) {
590:                    {
591:                        ///@notice Get the Uniswap v2 spot price and lp address.
592:                        (SpotReserve memory spotPrice, address poolAddress) =
593:                            _calculateV2SpotPrice(token0, token1, dexes[i].factoryAddress);
594:                        ///@notice Set SpotReserve and lp values if the returned values are not null.
595:                        if (spotPrice.spotPrice != 0) {
596:                            _spotPrices[i] = spotPrice;
597:                            _lps[i] = poolAddress;
598:                        }
599:                    }
600:                } else {
601:                    {
602:                        {
603:                            ///@notice Get the Uniswap v2 spot price and lp address.
604:                            (SpotReserve memory spotPrice, address poolAddress) =
605:                                _calculateV3SpotPrice(token0, token1, FEE, dexes[i].factoryAddress);
606:
607:                            ///@notice Set SpotReserve and lp values if the returned values are not null.
608:                            if (spotPrice.spotPrice != 0) {
609:                                _lps[i] = poolAddress;
610:                                _spotPrices[i] = spotPrice;
611:                            }
612:                        }
613:                    }
614:                }
615:
616:                unchecked {
617:                    ++i;
618:                }
619:            }
620:
``` 



[File:SandboxLimitOrderBook.sol#L317](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L317) 
```solidity
316:        for (uint256 i = 0; i < orderGroup.length;) {
317:            ///@notice Get the order details from the orderGroup.
318:            SandboxLimitOrder memory newOrder = orderGroup[i];
319:
320:            ///@notice Increment the total value of orders by the quantity of the new order
321:            updatedTotalOrdersValue += newOrder.amountInRemaining;
322:            uint256 relativeWethValue;
323:            {
324:                ///@notice Boolean indicating if user wants to cover the fee from the fee credit balance, or by calling placeOrder with payment.
325:                if (!(newOrder.tokenIn == WETH)) {
326:                    ///@notice Calculate the spot price of the input token to WETH on Uni v2.
327:                    (LimitOrderSwapRouter.SpotReserve[] memory spRes,) =
328:                        ILimitOrderSwapRouter(LIMIT_ORDER_EXECUTOR).getAllPrices(newOrder.tokenIn, WETH, 500);
329:                    uint256 tokenAWethSpotPrice;
330:                    for (uint256 k = 0; k < spRes.length;) {
331:                        if (spRes[k].spotPrice != 0) {
332:                            tokenAWethSpotPrice = spRes[k].spotPrice;
333:                            break;
334:                        }
335:
336:                        unchecked {
337:                            ++k;
338:                        }
339:                    }
340:                    if (tokenAWethSpotPrice == 0) {
341:                        revert InvalidInputTokenForOrderPlacement();
342:                    }
343:
344:                    if (!(tokenAWethSpotPrice == 0)) {
345:                        ///@notice Get the tokenIn decimals to normalize the relativeWethValue.
346:                        uint8 tokenInDecimals = IERC20(newOrder.tokenIn).decimals();
347:                        ///@notice Multiply the amountIn*spotPrice to get the value of the input amount in weth.
348:                        relativeWethValue = tokenInDecimals <= 18
349:                            ? ConveyorMath.mul128U(tokenAWethSpotPrice, newOrder.amountInRemaining)
350:                                * 10 ** (18 - tokenInDecimals)
351:                            : ConveyorMath.mul128U(tokenAWethSpotPrice, newOrder.amountInRemaining)
352:                                / 10 ** (tokenInDecimals - 18);
353:                    }
354:                } else {
355:                    relativeWethValue = newOrder.amountInRemaining;
356:                }
357:
358:                if (relativeWethValue < MIN_ORDER_VALUE_IN_WETH) {
359:                    revert InsufficientOrderInputValue();
360:                }
361:
362:                ///@notice Set the minimum fee to the fee*wethValue*subsidy.
363:                uint128 minFeeReceived = uint128(
364:                    ConveyorMath.mul64U(
365:                        ILimitOrderSwapRouter(LIMIT_ORDER_EXECUTOR).calculateFee(uint128(relativeWethValue), USDC, WETH),
366:                        relativeWethValue
367:                    )
368:                );
369:                ///@notice Set the Orders min fee to be received during execution.
370:                newOrder.feeRemaining = minFeeReceived;
371:            }
372:
373:            ///@notice If the newOrder's tokenIn does not match the orderToken, revert.
374:            if ((orderToken != newOrder.tokenIn)) {
375:                revert IncongruentInputTokenInOrderGroup(newOrder.tokenIn, orderToken);
376:            }
377:
378:            ///@notice If the msg.sender does not have a sufficent balance to cover the order, revert.
379:            if (tokenBalance < updatedTotalOrdersValue) {
380:                revert InsufficientWalletBalance(msg.sender, tokenBalance, updatedTotalOrdersValue);
381:            }
382:
383:            ///@notice Create a new orderId from the orderNonce and current block timestamp
384:            bytes32 orderId = keccak256(abi.encode(orderNonce, block.timestamp));
385:
386:            ///@notice increment the orderNonce
387:            /**
388:             * @dev This is unchecked because the orderNonce and block.timestamp will never be the same, so even if the
389:             *         orderNonce overflows, it will still produce unique orderIds because the timestamp will be different.
390:             */
391:            unchecked {
392:                orderNonce += 2;
393:            }
394:
395:            ///@notice Set the new order's owner to the msg.sender
396:            newOrder.owner = msg.sender;
397:
398:            ///@notice update the newOrder's Id to the orderId generated from the orderNonce
399:            newOrder.orderId = orderId;
400:
401:            ///@notice update the newOrder's last refresh timestamp
402:            ///@dev uint32(block.timestamp % (2**32 - 1)) is used to future proof the contract.
403:            newOrder.lastRefreshTimestamp = uint32(block.timestamp);
404:
405:            ///@notice Increment the cumulative execution credit by the current orders execution.
406:            cumulativeExecutionCredit += newOrder.executionCreditRemaining;
407:
408:            ///@notice Add the newly created order to the orderIdToOrder mapping
409:            orderIdToSandboxLimitOrder[orderId] = newOrder;
410:
411:            ///@notice Add the orderId to the addressToOrderIds mapping
412:            addressToOrderIds[msg.sender][orderId] = OrderType.PendingSandboxLimitOrder;
413:
414:            ///@notice Increment the total orders per address for the msg.sender
415:            ++totalOrdersPerAddress[msg.sender];
416:
417:            ///@notice Add the orderId to the orderIds array for the PlaceOrder event emission and increment the orderIdIndex
418:            orderIds[i] = orderId;
419:
420:            ///@notice Add the orderId to the addressToAllOrderIds structure
421:            addressToAllOrderIds[msg.sender].push(orderId);
422:
423:            unchecked {
424:                ++i;
425:            }
426:        }
427:
``` 



[File:SandboxLimitOrderBook.sol#L317](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L317) 
```solidity
316:        for (uint256 i = 0; i < orderGroup.length;) {
317:            ///@notice Get the order details from the orderGroup.
318:            SandboxLimitOrder memory newOrder = orderGroup[i];
319:
320:            ///@notice Increment the total value of orders by the quantity of the new order
321:            updatedTotalOrdersValue += newOrder.amountInRemaining;
322:            uint256 relativeWethValue;
323:            {
324:                ///@notice Boolean indicating if user wants to cover the fee from the fee credit balance, or by calling placeOrder with payment.
325:                if (!(newOrder.tokenIn == WETH)) {
326:                    ///@notice Calculate the spot price of the input token to WETH on Uni v2.
327:                    (LimitOrderSwapRouter.SpotReserve[] memory spRes,) =
328:                        ILimitOrderSwapRouter(LIMIT_ORDER_EXECUTOR).getAllPrices(newOrder.tokenIn, WETH, 500);
329:                    uint256 tokenAWethSpotPrice;
330:                    for (uint256 k = 0; k < spRes.length;) {
331:                        if (spRes[k].spotPrice != 0) {
332:                            tokenAWethSpotPrice = spRes[k].spotPrice;
333:                            break;
334:                        }
335:
336:                        unchecked {
337:                            ++k;
338:                        }
339:                    }
340:                    if (tokenAWethSpotPrice == 0) {
341:                        revert InvalidInputTokenForOrderPlacement();
342:                    }
343:
344:                    if (!(tokenAWethSpotPrice == 0)) {
345:                        ///@notice Get the tokenIn decimals to normalize the relativeWethValue.
346:                        uint8 tokenInDecimals = IERC20(newOrder.tokenIn).decimals();
347:                        ///@notice Multiply the amountIn*spotPrice to get the value of the input amount in weth.
348:                        relativeWethValue = tokenInDecimals <= 18
349:                            ? ConveyorMath.mul128U(tokenAWethSpotPrice, newOrder.amountInRemaining)
350:                                * 10 ** (18 - tokenInDecimals)
351:                            : ConveyorMath.mul128U(tokenAWethSpotPrice, newOrder.amountInRemaining)
352:                                / 10 ** (tokenInDecimals - 18);
353:                    }
354:                } else {
355:                    relativeWethValue = newOrder.amountInRemaining;
356:                }
357:
358:                if (relativeWethValue < MIN_ORDER_VALUE_IN_WETH) {
359:                    revert InsufficientOrderInputValue();
360:                }
361:
362:                ///@notice Set the minimum fee to the fee*wethValue*subsidy.
363:                uint128 minFeeReceived = uint128(
364:                    ConveyorMath.mul64U(
365:                        ILimitOrderSwapRouter(LIMIT_ORDER_EXECUTOR).calculateFee(uint128(relativeWethValue), USDC, WETH),
366:                        relativeWethValue
367:                    )
368:                );
369:                ///@notice Set the Orders min fee to be received during execution.
370:                newOrder.feeRemaining = minFeeReceived;
371:            }
372:
373:            ///@notice If the newOrder's tokenIn does not match the orderToken, revert.
374:            if ((orderToken != newOrder.tokenIn)) {
375:                revert IncongruentInputTokenInOrderGroup(newOrder.tokenIn, orderToken);
376:            }
377:
378:            ///@notice If the msg.sender does not have a sufficent balance to cover the order, revert.
379:            if (tokenBalance < updatedTotalOrdersValue) {
380:                revert InsufficientWalletBalance(msg.sender, tokenBalance, updatedTotalOrdersValue);
381:            }
382:
383:            ///@notice Create a new orderId from the orderNonce and current block timestamp
384:            bytes32 orderId = keccak256(abi.encode(orderNonce, block.timestamp));
385:
386:            ///@notice increment the orderNonce
387:            /**
388:             * @dev This is unchecked because the orderNonce and block.timestamp will never be the same, so even if the
389:             *         orderNonce overflows, it will still produce unique orderIds because the timestamp will be different.
390:             */
391:            unchecked {
392:                orderNonce += 2;
393:            }
394:
395:            ///@notice Set the new order's owner to the msg.sender
396:            newOrder.owner = msg.sender;
397:
398:            ///@notice update the newOrder's Id to the orderId generated from the orderNonce
399:            newOrder.orderId = orderId;
400:
401:            ///@notice update the newOrder's last refresh timestamp
402:            ///@dev uint32(block.timestamp % (2**32 - 1)) is used to future proof the contract.
403:            newOrder.lastRefreshTimestamp = uint32(block.timestamp);
404:
405:            ///@notice Increment the cumulative execution credit by the current orders execution.
406:            cumulativeExecutionCredit += newOrder.executionCreditRemaining;
407:
408:            ///@notice Add the newly created order to the orderIdToOrder mapping
409:            orderIdToSandboxLimitOrder[orderId] = newOrder;
410:
411:            ///@notice Add the orderId to the addressToOrderIds mapping
412:            addressToOrderIds[msg.sender][orderId] = OrderType.PendingSandboxLimitOrder;
413:
414:            ///@notice Increment the total orders per address for the msg.sender
415:            ++totalOrdersPerAddress[msg.sender];
416:
417:            ///@notice Add the orderId to the orderIds array for the PlaceOrder event emission and increment the orderIdIndex
418:            orderIds[i] = orderId;
419:
420:            ///@notice Add the orderId to the addressToAllOrderIds structure
421:            addressToAllOrderIds[msg.sender].push(orderId);
422:
423:            unchecked {
424:                ++i;
425:            }
426:        }
427:
``` 



[File:SandboxLimitOrderBook.sol#L317](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L317) 
```solidity
316:        for (uint256 i = 0; i < orderGroup.length;) {
317:            ///@notice Get the order details from the orderGroup.
318:            SandboxLimitOrder memory newOrder = orderGroup[i];
319:
320:            ///@notice Increment the total value of orders by the quantity of the new order
321:            updatedTotalOrdersValue += newOrder.amountInRemaining;
322:            uint256 relativeWethValue;
323:            {
324:                ///@notice Boolean indicating if user wants to cover the fee from the fee credit balance, or by calling placeOrder with payment.
325:                if (!(newOrder.tokenIn == WETH)) {
326:                    ///@notice Calculate the spot price of the input token to WETH on Uni v2.
327:                    (LimitOrderSwapRouter.SpotReserve[] memory spRes,) =
328:                        ILimitOrderSwapRouter(LIMIT_ORDER_EXECUTOR).getAllPrices(newOrder.tokenIn, WETH, 500);
329:                    uint256 tokenAWethSpotPrice;
330:                    for (uint256 k = 0; k < spRes.length;) {
331:                        if (spRes[k].spotPrice != 0) {
332:                            tokenAWethSpotPrice = spRes[k].spotPrice;
333:                            break;
334:                        }
335:
336:                        unchecked {
337:                            ++k;
338:                        }
339:                    }
340:                    if (tokenAWethSpotPrice == 0) {
341:                        revert InvalidInputTokenForOrderPlacement();
342:                    }
343:
344:                    if (!(tokenAWethSpotPrice == 0)) {
345:                        ///@notice Get the tokenIn decimals to normalize the relativeWethValue.
346:                        uint8 tokenInDecimals = IERC20(newOrder.tokenIn).decimals();
347:                        ///@notice Multiply the amountIn*spotPrice to get the value of the input amount in weth.
348:                        relativeWethValue = tokenInDecimals <= 18
349:                            ? ConveyorMath.mul128U(tokenAWethSpotPrice, newOrder.amountInRemaining)
350:                                * 10 ** (18 - tokenInDecimals)
351:                            : ConveyorMath.mul128U(tokenAWethSpotPrice, newOrder.amountInRemaining)
352:                                / 10 ** (tokenInDecimals - 18);
353:                    }
354:                } else {
355:                    relativeWethValue = newOrder.amountInRemaining;
356:                }
357:
358:                if (relativeWethValue < MIN_ORDER_VALUE_IN_WETH) {
359:                    revert InsufficientOrderInputValue();
360:                }
361:
362:                ///@notice Set the minimum fee to the fee*wethValue*subsidy.
363:                uint128 minFeeReceived = uint128(
364:                    ConveyorMath.mul64U(
365:                        ILimitOrderSwapRouter(LIMIT_ORDER_EXECUTOR).calculateFee(uint128(relativeWethValue), USDC, WETH),
366:                        relativeWethValue
367:                    )
368:                );
369:                ///@notice Set the Orders min fee to be received during execution.
370:                newOrder.feeRemaining = minFeeReceived;
371:            }
372:
373:            ///@notice If the newOrder's tokenIn does not match the orderToken, revert.
374:            if ((orderToken != newOrder.tokenIn)) {
375:                revert IncongruentInputTokenInOrderGroup(newOrder.tokenIn, orderToken);
376:            }
377:
378:            ///@notice If the msg.sender does not have a sufficent balance to cover the order, revert.
379:            if (tokenBalance < updatedTotalOrdersValue) {
380:                revert InsufficientWalletBalance(msg.sender, tokenBalance, updatedTotalOrdersValue);
381:            }
382:
383:            ///@notice Create a new orderId from the orderNonce and current block timestamp
384:            bytes32 orderId = keccak256(abi.encode(orderNonce, block.timestamp));
385:
386:            ///@notice increment the orderNonce
387:            /**
388:             * @dev This is unchecked because the orderNonce and block.timestamp will never be the same, so even if the
389:             *         orderNonce overflows, it will still produce unique orderIds because the timestamp will be different.
390:             */
391:            unchecked {
392:                orderNonce += 2;
393:            }
394:
395:            ///@notice Set the new order's owner to the msg.sender
396:            newOrder.owner = msg.sender;
397:
398:            ///@notice update the newOrder's Id to the orderId generated from the orderNonce
399:            newOrder.orderId = orderId;
400:
401:            ///@notice update the newOrder's last refresh timestamp
402:            ///@dev uint32(block.timestamp % (2**32 - 1)) is used to future proof the contract.
403:            newOrder.lastRefreshTimestamp = uint32(block.timestamp);
404:
405:            ///@notice Increment the cumulative execution credit by the current orders execution.
406:            cumulativeExecutionCredit += newOrder.executionCreditRemaining;
407:
408:            ///@notice Add the newly created order to the orderIdToOrder mapping
409:            orderIdToSandboxLimitOrder[orderId] = newOrder;
410:
411:            ///@notice Add the orderId to the addressToOrderIds mapping
412:            addressToOrderIds[msg.sender][orderId] = OrderType.PendingSandboxLimitOrder;
413:
414:            ///@notice Increment the total orders per address for the msg.sender
415:            ++totalOrdersPerAddress[msg.sender];
416:
417:            ///@notice Add the orderId to the orderIds array for the PlaceOrder event emission and increment the orderIdIndex
418:            orderIds[i] = orderId;
419:
420:            ///@notice Add the orderId to the addressToAllOrderIds structure
421:            addressToAllOrderIds[msg.sender].push(orderId);
422:
423:            unchecked {
424:                ++i;
425:            }
426:        }
427:
``` 



[File:SandboxLimitOrderBook.sol#L317](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L317) 
```solidity
316:        for (uint256 i = 0; i < orderGroup.length;) {
317:            ///@notice Get the order details from the orderGroup.
318:            SandboxLimitOrder memory newOrder = orderGroup[i];
319:
320:            ///@notice Increment the total value of orders by the quantity of the new order
321:            updatedTotalOrdersValue += newOrder.amountInRemaining;
322:            uint256 relativeWethValue;
323:            {
324:                ///@notice Boolean indicating if user wants to cover the fee from the fee credit balance, or by calling placeOrder with payment.
325:                if (!(newOrder.tokenIn == WETH)) {
326:                    ///@notice Calculate the spot price of the input token to WETH on Uni v2.
327:                    (LimitOrderSwapRouter.SpotReserve[] memory spRes,) =
328:                        ILimitOrderSwapRouter(LIMIT_ORDER_EXECUTOR).getAllPrices(newOrder.tokenIn, WETH, 500);
329:                    uint256 tokenAWethSpotPrice;
330:                    for (uint256 k = 0; k < spRes.length;) {
331:                        if (spRes[k].spotPrice != 0) {
332:                            tokenAWethSpotPrice = spRes[k].spotPrice;
333:                            break;
334:                        }
335:
336:                        unchecked {
337:                            ++k;
338:                        }
339:                    }
340:                    if (tokenAWethSpotPrice == 0) {
341:                        revert InvalidInputTokenForOrderPlacement();
342:                    }
343:
344:                    if (!(tokenAWethSpotPrice == 0)) {
345:                        ///@notice Get the tokenIn decimals to normalize the relativeWethValue.
346:                        uint8 tokenInDecimals = IERC20(newOrder.tokenIn).decimals();
347:                        ///@notice Multiply the amountIn*spotPrice to get the value of the input amount in weth.
348:                        relativeWethValue = tokenInDecimals <= 18
349:                            ? ConveyorMath.mul128U(tokenAWethSpotPrice, newOrder.amountInRemaining)
350:                                * 10 ** (18 - tokenInDecimals)
351:                            : ConveyorMath.mul128U(tokenAWethSpotPrice, newOrder.amountInRemaining)
352:                                / 10 ** (tokenInDecimals - 18);
353:                    }
354:                } else {
355:                    relativeWethValue = newOrder.amountInRemaining;
356:                }
357:
358:                if (relativeWethValue < MIN_ORDER_VALUE_IN_WETH) {
359:                    revert InsufficientOrderInputValue();
360:                }
361:
362:                ///@notice Set the minimum fee to the fee*wethValue*subsidy.
363:                uint128 minFeeReceived = uint128(
364:                    ConveyorMath.mul64U(
365:                        ILimitOrderSwapRouter(LIMIT_ORDER_EXECUTOR).calculateFee(uint128(relativeWethValue), USDC, WETH),
366:                        relativeWethValue
367:                    )
368:                );
369:                ///@notice Set the Orders min fee to be received during execution.
370:                newOrder.feeRemaining = minFeeReceived;
371:            }
372:
373:            ///@notice If the newOrder's tokenIn does not match the orderToken, revert.
374:            if ((orderToken != newOrder.tokenIn)) {
375:                revert IncongruentInputTokenInOrderGroup(newOrder.tokenIn, orderToken);
376:            }
377:
378:            ///@notice If the msg.sender does not have a sufficent balance to cover the order, revert.
379:            if (tokenBalance < updatedTotalOrdersValue) {
380:                revert InsufficientWalletBalance(msg.sender, tokenBalance, updatedTotalOrdersValue);
381:            }
382:
383:            ///@notice Create a new orderId from the orderNonce and current block timestamp
384:            bytes32 orderId = keccak256(abi.encode(orderNonce, block.timestamp));
385:
386:            ///@notice increment the orderNonce
387:            /**
388:             * @dev This is unchecked because the orderNonce and block.timestamp will never be the same, so even if the
389:             *         orderNonce overflows, it will still produce unique orderIds because the timestamp will be different.
390:             */
391:            unchecked {
392:                orderNonce += 2;
393:            }
394:
395:            ///@notice Set the new order's owner to the msg.sender
396:            newOrder.owner = msg.sender;
397:
398:            ///@notice update the newOrder's Id to the orderId generated from the orderNonce
399:            newOrder.orderId = orderId;
400:
401:            ///@notice update the newOrder's last refresh timestamp
402:            ///@dev uint32(block.timestamp % (2**32 - 1)) is used to future proof the contract.
403:            newOrder.lastRefreshTimestamp = uint32(block.timestamp);
404:
405:            ///@notice Increment the cumulative execution credit by the current orders execution.
406:            cumulativeExecutionCredit += newOrder.executionCreditRemaining;
407:
408:            ///@notice Add the newly created order to the orderIdToOrder mapping
409:            orderIdToSandboxLimitOrder[orderId] = newOrder;
410:
411:            ///@notice Add the orderId to the addressToOrderIds mapping
412:            addressToOrderIds[msg.sender][orderId] = OrderType.PendingSandboxLimitOrder;
413:
414:            ///@notice Increment the total orders per address for the msg.sender
415:            ++totalOrdersPerAddress[msg.sender];
416:
417:            ///@notice Add the orderId to the orderIds array for the PlaceOrder event emission and increment the orderIdIndex
418:            orderIds[i] = orderId;
419:
420:            ///@notice Add the orderId to the addressToAllOrderIds structure
421:            addressToAllOrderIds[msg.sender].push(orderId);
422:
423:            unchecked {
424:                ++i;
425:            }
426:        }
427:
``` 



[File:SandboxLimitOrderBook.sol#L317](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L317) 
```solidity
316:        for (uint256 i = 0; i < orderGroup.length;) {
317:            ///@notice Get the order details from the orderGroup.
318:            SandboxLimitOrder memory newOrder = orderGroup[i];
319:
320:            ///@notice Increment the total value of orders by the quantity of the new order
321:            updatedTotalOrdersValue += newOrder.amountInRemaining;
322:            uint256 relativeWethValue;
323:            {
324:                ///@notice Boolean indicating if user wants to cover the fee from the fee credit balance, or by calling placeOrder with payment.
325:                if (!(newOrder.tokenIn == WETH)) {
326:                    ///@notice Calculate the spot price of the input token to WETH on Uni v2.
327:                    (LimitOrderSwapRouter.SpotReserve[] memory spRes,) =
328:                        ILimitOrderSwapRouter(LIMIT_ORDER_EXECUTOR).getAllPrices(newOrder.tokenIn, WETH, 500);
329:                    uint256 tokenAWethSpotPrice;
330:                    for (uint256 k = 0; k < spRes.length;) {
331:                        if (spRes[k].spotPrice != 0) {
332:                            tokenAWethSpotPrice = spRes[k].spotPrice;
333:                            break;
334:                        }
335:
336:                        unchecked {
337:                            ++k;
338:                        }
339:                    }
340:                    if (tokenAWethSpotPrice == 0) {
341:                        revert InvalidInputTokenForOrderPlacement();
342:                    }
343:
344:                    if (!(tokenAWethSpotPrice == 0)) {
345:                        ///@notice Get the tokenIn decimals to normalize the relativeWethValue.
346:                        uint8 tokenInDecimals = IERC20(newOrder.tokenIn).decimals();
347:                        ///@notice Multiply the amountIn*spotPrice to get the value of the input amount in weth.
348:                        relativeWethValue = tokenInDecimals <= 18
349:                            ? ConveyorMath.mul128U(tokenAWethSpotPrice, newOrder.amountInRemaining)
350:                                * 10 ** (18 - tokenInDecimals)
351:                            : ConveyorMath.mul128U(tokenAWethSpotPrice, newOrder.amountInRemaining)
352:                                / 10 ** (tokenInDecimals - 18);
353:                    }
354:                } else {
355:                    relativeWethValue = newOrder.amountInRemaining;
356:                }
357:
358:                if (relativeWethValue < MIN_ORDER_VALUE_IN_WETH) {
359:                    revert InsufficientOrderInputValue();
360:                }
361:
362:                ///@notice Set the minimum fee to the fee*wethValue*subsidy.
363:                uint128 minFeeReceived = uint128(
364:                    ConveyorMath.mul64U(
365:                        ILimitOrderSwapRouter(LIMIT_ORDER_EXECUTOR).calculateFee(uint128(relativeWethValue), USDC, WETH),
366:                        relativeWethValue
367:                    )
368:                );
369:                ///@notice Set the Orders min fee to be received during execution.
370:                newOrder.feeRemaining = minFeeReceived;
371:            }
372:
373:            ///@notice If the newOrder's tokenIn does not match the orderToken, revert.
374:            if ((orderToken != newOrder.tokenIn)) {
375:                revert IncongruentInputTokenInOrderGroup(newOrder.tokenIn, orderToken);
376:            }
377:
378:            ///@notice If the msg.sender does not have a sufficent balance to cover the order, revert.
379:            if (tokenBalance < updatedTotalOrdersValue) {
380:                revert InsufficientWalletBalance(msg.sender, tokenBalance, updatedTotalOrdersValue);
381:            }
382:
383:            ///@notice Create a new orderId from the orderNonce and current block timestamp
384:            bytes32 orderId = keccak256(abi.encode(orderNonce, block.timestamp));
385:
386:            ///@notice increment the orderNonce
387:            /**
388:             * @dev This is unchecked because the orderNonce and block.timestamp will never be the same, so even if the
389:             *         orderNonce overflows, it will still produce unique orderIds because the timestamp will be different.
390:             */
391:            unchecked {
392:                orderNonce += 2;
393:            }
394:
395:            ///@notice Set the new order's owner to the msg.sender
396:            newOrder.owner = msg.sender;
397:
398:            ///@notice update the newOrder's Id to the orderId generated from the orderNonce
399:            newOrder.orderId = orderId;
400:
401:            ///@notice update the newOrder's last refresh timestamp
402:            ///@dev uint32(block.timestamp % (2**32 - 1)) is used to future proof the contract.
403:            newOrder.lastRefreshTimestamp = uint32(block.timestamp);
404:
405:            ///@notice Increment the cumulative execution credit by the current orders execution.
406:            cumulativeExecutionCredit += newOrder.executionCreditRemaining;
407:
408:            ///@notice Add the newly created order to the orderIdToOrder mapping
409:            orderIdToSandboxLimitOrder[orderId] = newOrder;
410:
411:            ///@notice Add the orderId to the addressToOrderIds mapping
412:            addressToOrderIds[msg.sender][orderId] = OrderType.PendingSandboxLimitOrder;
413:
414:            ///@notice Increment the total orders per address for the msg.sender
415:            ++totalOrdersPerAddress[msg.sender];
416:
417:            ///@notice Add the orderId to the orderIds array for the PlaceOrder event emission and increment the orderIdIndex
418:            orderIds[i] = orderId;
419:
420:            ///@notice Add the orderId to the addressToAllOrderIds structure
421:            addressToAllOrderIds[msg.sender].push(orderId);
422:
423:            unchecked {
424:                ++i;
425:            }
426:        }
427:
``` 



[File:SandboxLimitOrderBook.sol#L317](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L317) 
```solidity
316:        for (uint256 i = 0; i < orderGroup.length;) {
317:            ///@notice Get the order details from the orderGroup.
318:            SandboxLimitOrder memory newOrder = orderGroup[i];
319:
320:            ///@notice Increment the total value of orders by the quantity of the new order
321:            updatedTotalOrdersValue += newOrder.amountInRemaining;
322:            uint256 relativeWethValue;
323:            {
324:                ///@notice Boolean indicating if user wants to cover the fee from the fee credit balance, or by calling placeOrder with payment.
325:                if (!(newOrder.tokenIn == WETH)) {
326:                    ///@notice Calculate the spot price of the input token to WETH on Uni v2.
327:                    (LimitOrderSwapRouter.SpotReserve[] memory spRes,) =
328:                        ILimitOrderSwapRouter(LIMIT_ORDER_EXECUTOR).getAllPrices(newOrder.tokenIn, WETH, 500);
329:                    uint256 tokenAWethSpotPrice;
330:                    for (uint256 k = 0; k < spRes.length;) {
331:                        if (spRes[k].spotPrice != 0) {
332:                            tokenAWethSpotPrice = spRes[k].spotPrice;
333:                            break;
334:                        }
335:
336:                        unchecked {
337:                            ++k;
338:                        }
339:                    }
340:                    if (tokenAWethSpotPrice == 0) {
341:                        revert InvalidInputTokenForOrderPlacement();
342:                    }
343:
344:                    if (!(tokenAWethSpotPrice == 0)) {
345:                        ///@notice Get the tokenIn decimals to normalize the relativeWethValue.
346:                        uint8 tokenInDecimals = IERC20(newOrder.tokenIn).decimals();
347:                        ///@notice Multiply the amountIn*spotPrice to get the value of the input amount in weth.
348:                        relativeWethValue = tokenInDecimals <= 18
349:                            ? ConveyorMath.mul128U(tokenAWethSpotPrice, newOrder.amountInRemaining)
350:                                * 10 ** (18 - tokenInDecimals)
351:                            : ConveyorMath.mul128U(tokenAWethSpotPrice, newOrder.amountInRemaining)
352:                                / 10 ** (tokenInDecimals - 18);
353:                    }
354:                } else {
355:                    relativeWethValue = newOrder.amountInRemaining;
356:                }
357:
358:                if (relativeWethValue < MIN_ORDER_VALUE_IN_WETH) {
359:                    revert InsufficientOrderInputValue();
360:                }
361:
362:                ///@notice Set the minimum fee to the fee*wethValue*subsidy.
363:                uint128 minFeeReceived = uint128(
364:                    ConveyorMath.mul64U(
365:                        ILimitOrderSwapRouter(LIMIT_ORDER_EXECUTOR).calculateFee(uint128(relativeWethValue), USDC, WETH),
366:                        relativeWethValue
367:                    )
368:                );
369:                ///@notice Set the Orders min fee to be received during execution.
370:                newOrder.feeRemaining = minFeeReceived;
371:            }
372:
373:            ///@notice If the newOrder's tokenIn does not match the orderToken, revert.
374:            if ((orderToken != newOrder.tokenIn)) {
375:                revert IncongruentInputTokenInOrderGroup(newOrder.tokenIn, orderToken);
376:            }
377:
378:            ///@notice If the msg.sender does not have a sufficent balance to cover the order, revert.
379:            if (tokenBalance < updatedTotalOrdersValue) {
380:                revert InsufficientWalletBalance(msg.sender, tokenBalance, updatedTotalOrdersValue);
381:            }
382:
383:            ///@notice Create a new orderId from the orderNonce and current block timestamp
384:            bytes32 orderId = keccak256(abi.encode(orderNonce, block.timestamp));
385:
386:            ///@notice increment the orderNonce
387:            /**
388:             * @dev This is unchecked because the orderNonce and block.timestamp will never be the same, so even if the
389:             *         orderNonce overflows, it will still produce unique orderIds because the timestamp will be different.
390:             */
391:            unchecked {
392:                orderNonce += 2;
393:            }
394:
395:            ///@notice Set the new order's owner to the msg.sender
396:            newOrder.owner = msg.sender;
397:
398:            ///@notice update the newOrder's Id to the orderId generated from the orderNonce
399:            newOrder.orderId = orderId;
400:
401:            ///@notice update the newOrder's last refresh timestamp
402:            ///@dev uint32(block.timestamp % (2**32 - 1)) is used to future proof the contract.
403:            newOrder.lastRefreshTimestamp = uint32(block.timestamp);
404:
405:            ///@notice Increment the cumulative execution credit by the current orders execution.
406:            cumulativeExecutionCredit += newOrder.executionCreditRemaining;
407:
408:            ///@notice Add the newly created order to the orderIdToOrder mapping
409:            orderIdToSandboxLimitOrder[orderId] = newOrder;
410:
411:            ///@notice Add the orderId to the addressToOrderIds mapping
412:            addressToOrderIds[msg.sender][orderId] = OrderType.PendingSandboxLimitOrder;
413:
414:            ///@notice Increment the total orders per address for the msg.sender
415:            ++totalOrdersPerAddress[msg.sender];
416:
417:            ///@notice Add the orderId to the orderIds array for the PlaceOrder event emission and increment the orderIdIndex
418:            orderIds[i] = orderId;
419:
420:            ///@notice Add the orderId to the addressToAllOrderIds structure
421:            addressToAllOrderIds[msg.sender].push(orderId);
422:
423:            unchecked {
424:                ++i;
425:            }
426:        }
427:
``` 



[File:SandboxLimitOrderBook.sol#L750](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L750) 
```solidity
749:            for (uint256 i = 0; i < orderIdBundles.length;) {
750:                bytes32[] memory orderIdBundle = orderIdBundles[i];
751:
752:                for (uint256 j = 0; j < orderIdBundle.length;) {
753:                    bytes32 orderId = orderIdBundle[j];
754:
755:                    ///@notice Transfer the tokens from the order owners to the sandbox router contract.
756:                    ///@dev This function is executed in the context of ConveyorExecutor as a delegatecall.
757:
758:                    ///@notice Get the current order
759:                    SandboxLimitOrder memory currentOrder = orderIdToSandboxLimitOrder[orderId];
760:
761:                    if (currentOrder.orderId == bytes32(0)) {
762:                        revert OrderDoesNotExist(orderId);
763:                    }
764:
765:                    preSandboxExecutionState.orderOwners[arrayIndex] = currentOrder.owner;
766:
767:                    preSandboxExecutionState.sandboxLimitOrders[arrayIndex] = currentOrder;
768:
769:                    ///@notice Cache amountSpecifiedToFill for intermediate calculations
770:                    uint128 amountSpecifiedToFill = fillAmounts[arrayIndex];
771:                    ///@notice Require the amountSpecifiedToFill is less than or equal to the amountInRemaining of the order.
772:                    if (amountSpecifiedToFill > currentOrder.amountInRemaining) {
773:                        revert FillAmountSpecifiedGreaterThanAmountRemaining(
774:                            amountSpecifiedToFill, currentOrder.amountInRemaining, currentOrder.orderId
775:                        );
776:                    }
777:
778:                    ///@notice Cache the the pre execution state of the order details
779:                    preSandboxExecutionState.initialTokenInBalances[arrayIndex] =
780:                        IERC20(currentOrder.tokenIn).balanceOf(currentOrder.owner);
781:
782:                    preSandboxExecutionState.initialTokenOutBalances[arrayIndex] =
783:                        IERC20(currentOrder.tokenOut).balanceOf(currentOrder.owner);
784:
785:                    unchecked {
786:                        ++arrayIndex;
787:                    }
788:
789:                    unchecked {
790:                        ++j;
791:                    }
792:                }
793:
794:                unchecked {
795:                    ++i;
796:                }
797:            }
798:        }
``` 



[File:SandboxLimitOrderBook.sol#L753](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L753) 
```solidity
752:                for (uint256 j = 0; j < orderIdBundle.length;) {
753:                    bytes32 orderId = orderIdBundle[j];
754:
755:                    ///@notice Transfer the tokens from the order owners to the sandbox router contract.
756:                    ///@dev This function is executed in the context of ConveyorExecutor as a delegatecall.
757:
758:                    ///@notice Get the current order
759:                    SandboxLimitOrder memory currentOrder = orderIdToSandboxLimitOrder[orderId];
760:
761:                    if (currentOrder.orderId == bytes32(0)) {
762:                        revert OrderDoesNotExist(orderId);
763:                    }
764:
765:                    preSandboxExecutionState.orderOwners[arrayIndex] = currentOrder.owner;
766:
767:                    preSandboxExecutionState.sandboxLimitOrders[arrayIndex] = currentOrder;
768:
769:                    ///@notice Cache amountSpecifiedToFill for intermediate calculations
770:                    uint128 amountSpecifiedToFill = fillAmounts[arrayIndex];
771:                    ///@notice Require the amountSpecifiedToFill is less than or equal to the amountInRemaining of the order.
772:                    if (amountSpecifiedToFill > currentOrder.amountInRemaining) {
773:                        revert FillAmountSpecifiedGreaterThanAmountRemaining(
774:                            amountSpecifiedToFill, currentOrder.amountInRemaining, currentOrder.orderId
775:                        );
776:                    }
777:
778:                    ///@notice Cache the the pre execution state of the order details
779:                    preSandboxExecutionState.initialTokenInBalances[arrayIndex] =
780:                        IERC20(currentOrder.tokenIn).balanceOf(currentOrder.owner);
781:
782:                    preSandboxExecutionState.initialTokenOutBalances[arrayIndex] =
783:                        IERC20(currentOrder.tokenOut).balanceOf(currentOrder.owner);
784:
785:                    unchecked {
786:                        ++arrayIndex;
787:                    }
788:
789:                    unchecked {
790:                        ++j;
791:                    }
792:                }
793:
``` 



[File:SandboxLimitOrderBook.sol#L1222](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L1222) 
```solidity
1221:        for (uint256 i = 0; i < length;) {
1222:            bytes32 orderId;
1223:            assembly {
1224:                //Get the orderId at the orderOffsetSlot
1225:                orderId := mload(orderOffsetSlot)
1226:                //Update the orderOffsetSlot
1227:                orderOffsetSlot := add(orderOffsetSlot, 0x20)
1228:            }
1229:
1230:            OrderType orderType = addressToOrderIds[orderOwner][orderId];
1231:
1232:            if (orderType == targetOrderType) {
1233:                orderIds[orderIdIndex] = orderId;
1234:                ++orderIdIndex;
1235:            }
1236:
1237:            unchecked {
1238:                ++i;
1239:            }
1240:        }
1241:
``` 



[File:LimitOrderBook.sol#L281](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L281) 
```solidity
280:        for (uint256 i = 0; i < orderGroup.length;) {
281:            ///@notice Get the order details from the orderGroup.
282:            LimitOrder memory newOrder = orderGroup[i];
283:
284:            if (newOrder.quantity == 0) {
285:                revert OrderQuantityIsZero();
286:            }
287:
288:            ///@notice Increment the total value of orders by the quantity of the new order
289:            updatedTotalOrdersValue += newOrder.quantity;
290:
291:            ///@notice If the newOrder's tokenIn does not match the orderToken, revert.
292:            if (!(orderToken == newOrder.tokenIn)) {
293:                revert IncongruentInputTokenInOrderGroup(newOrder.tokenIn, orderToken);
294:            }
295:
296:            ///@notice If the newOrder's tokenIn does not match the orderToken, revert.
297:            if (newOrder.tokenOut == newOrder.tokenIn) {
298:                revert TokenInIsTokenOut();
299:            }
300:
301:            ///@notice If the msg.sender does not have a sufficent balance to cover the order, revert.
302:            if (tokenBalance < updatedTotalOrdersValue) {
303:                revert InsufficientWalletBalance(msg.sender, tokenBalance, updatedTotalOrdersValue);
304:            }
305:
306:            ///@notice Create a new orderId from the orderNonce and current block timestamp
307:            bytes32 orderId = keccak256(abi.encode(orderNonce, block.timestamp));
308:
309:            ///@notice Increment the cumulative execution credit by the current orders execution.
310:            cumulativeExecutionCredit += newOrder.executionCredit;
311:
312:            ///@notice increment the orderNonce
313:            /**
314:             * @dev This is unchecked because the orderNonce and block.timestamp will never be the same, so even if the
315:             *         orderNonce overflows, it will still produce unique orderIds because the timestamp will be different.
316:             */
317:            unchecked {
318:                orderNonce += 2;
319:            }
320:
321:            ///@notice Set the new order's owner to the msg.sender
322:            newOrder.owner = msg.sender;
323:
324:            ///@notice update the newOrder's Id to the orderId generated from the orderNonce
325:            newOrder.orderId = orderId;
326:
327:            ///@notice update the newOrder's last refresh timestamp
328:            ///@dev uint32(block.timestamp % (2**32 - 1)) is used to future proof the contract.
329:            newOrder.lastRefreshTimestamp = uint32(block.timestamp);
330:
331:            ///@notice Add the newly created order to the orderIdToOrder mapping
332:            orderIdToLimitOrder[orderId] = newOrder;
333:
334:            ///@notice Add the orderId to the addressToOrderIds mapping
335:            addressToOrderIds[msg.sender][orderId] = OrderType.PendingLimitOrder;
336:
337:            ///@notice Increment the total orders per address for the msg.sender
338:            ++totalOrdersPerAddress[msg.sender];
339:
340:            ///@notice Add the orderId to the orderIds array for the PlaceOrder event emission and increment the orderIdIndex
341:            orderIds[i] = orderId;
342:
343:            ///@notice Add the orderId to the addressToAllOrderIds structure
344:            addressToAllOrderIds[msg.sender].push(orderId);
345:
346:            unchecked {
347:                ++i;
348:            }
349:        }
350:
``` 



[File:LimitOrderBook.sol#L281](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L281) 
```solidity
280:        for (uint256 i = 0; i < orderGroup.length;) {
281:            ///@notice Get the order details from the orderGroup.
282:            LimitOrder memory newOrder = orderGroup[i];
283:
284:            if (newOrder.quantity == 0) {
285:                revert OrderQuantityIsZero();
286:            }
287:
288:            ///@notice Increment the total value of orders by the quantity of the new order
289:            updatedTotalOrdersValue += newOrder.quantity;
290:
291:            ///@notice If the newOrder's tokenIn does not match the orderToken, revert.
292:            if (!(orderToken == newOrder.tokenIn)) {
293:                revert IncongruentInputTokenInOrderGroup(newOrder.tokenIn, orderToken);
294:            }
295:
296:            ///@notice If the newOrder's tokenIn does not match the orderToken, revert.
297:            if (newOrder.tokenOut == newOrder.tokenIn) {
298:                revert TokenInIsTokenOut();
299:            }
300:
301:            ///@notice If the msg.sender does not have a sufficent balance to cover the order, revert.
302:            if (tokenBalance < updatedTotalOrdersValue) {
303:                revert InsufficientWalletBalance(msg.sender, tokenBalance, updatedTotalOrdersValue);
304:            }
305:
306:            ///@notice Create a new orderId from the orderNonce and current block timestamp
307:            bytes32 orderId = keccak256(abi.encode(orderNonce, block.timestamp));
308:
309:            ///@notice Increment the cumulative execution credit by the current orders execution.
310:            cumulativeExecutionCredit += newOrder.executionCredit;
311:
312:            ///@notice increment the orderNonce
313:            /**
314:             * @dev This is unchecked because the orderNonce and block.timestamp will never be the same, so even if the
315:             *         orderNonce overflows, it will still produce unique orderIds because the timestamp will be different.
316:             */
317:            unchecked {
318:                orderNonce += 2;
319:            }
320:
321:            ///@notice Set the new order's owner to the msg.sender
322:            newOrder.owner = msg.sender;
323:
324:            ///@notice update the newOrder's Id to the orderId generated from the orderNonce
325:            newOrder.orderId = orderId;
326:
327:            ///@notice update the newOrder's last refresh timestamp
328:            ///@dev uint32(block.timestamp % (2**32 - 1)) is used to future proof the contract.
329:            newOrder.lastRefreshTimestamp = uint32(block.timestamp);
330:
331:            ///@notice Add the newly created order to the orderIdToOrder mapping
332:            orderIdToLimitOrder[orderId] = newOrder;
333:
334:            ///@notice Add the orderId to the addressToOrderIds mapping
335:            addressToOrderIds[msg.sender][orderId] = OrderType.PendingLimitOrder;
336:
337:            ///@notice Increment the total orders per address for the msg.sender
338:            ++totalOrdersPerAddress[msg.sender];
339:
340:            ///@notice Add the orderId to the orderIds array for the PlaceOrder event emission and increment the orderIdIndex
341:            orderIds[i] = orderId;
342:
343:            ///@notice Add the orderId to the addressToAllOrderIds structure
344:            addressToAllOrderIds[msg.sender].push(orderId);
345:
346:            unchecked {
347:                ++i;
348:            }
349:        }
350:
``` 



[File:LimitOrderBook.sol#L281](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L281) 
```solidity
280:        for (uint256 i = 0; i < orderGroup.length;) {
281:            ///@notice Get the order details from the orderGroup.
282:            LimitOrder memory newOrder = orderGroup[i];
283:
284:            if (newOrder.quantity == 0) {
285:                revert OrderQuantityIsZero();
286:            }
287:
288:            ///@notice Increment the total value of orders by the quantity of the new order
289:            updatedTotalOrdersValue += newOrder.quantity;
290:
291:            ///@notice If the newOrder's tokenIn does not match the orderToken, revert.
292:            if (!(orderToken == newOrder.tokenIn)) {
293:                revert IncongruentInputTokenInOrderGroup(newOrder.tokenIn, orderToken);
294:            }
295:
296:            ///@notice If the newOrder's tokenIn does not match the orderToken, revert.
297:            if (newOrder.tokenOut == newOrder.tokenIn) {
298:                revert TokenInIsTokenOut();
299:            }
300:
301:            ///@notice If the msg.sender does not have a sufficent balance to cover the order, revert.
302:            if (tokenBalance < updatedTotalOrdersValue) {
303:                revert InsufficientWalletBalance(msg.sender, tokenBalance, updatedTotalOrdersValue);
304:            }
305:
306:            ///@notice Create a new orderId from the orderNonce and current block timestamp
307:            bytes32 orderId = keccak256(abi.encode(orderNonce, block.timestamp));
308:
309:            ///@notice Increment the cumulative execution credit by the current orders execution.
310:            cumulativeExecutionCredit += newOrder.executionCredit;
311:
312:            ///@notice increment the orderNonce
313:            /**
314:             * @dev This is unchecked because the orderNonce and block.timestamp will never be the same, so even if the
315:             *         orderNonce overflows, it will still produce unique orderIds because the timestamp will be different.
316:             */
317:            unchecked {
318:                orderNonce += 2;
319:            }
320:
321:            ///@notice Set the new order's owner to the msg.sender
322:            newOrder.owner = msg.sender;
323:
324:            ///@notice update the newOrder's Id to the orderId generated from the orderNonce
325:            newOrder.orderId = orderId;
326:
327:            ///@notice update the newOrder's last refresh timestamp
328:            ///@dev uint32(block.timestamp % (2**32 - 1)) is used to future proof the contract.
329:            newOrder.lastRefreshTimestamp = uint32(block.timestamp);
330:
331:            ///@notice Add the newly created order to the orderIdToOrder mapping
332:            orderIdToLimitOrder[orderId] = newOrder;
333:
334:            ///@notice Add the orderId to the addressToOrderIds mapping
335:            addressToOrderIds[msg.sender][orderId] = OrderType.PendingLimitOrder;
336:
337:            ///@notice Increment the total orders per address for the msg.sender
338:            ++totalOrdersPerAddress[msg.sender];
339:
340:            ///@notice Add the orderId to the orderIds array for the PlaceOrder event emission and increment the orderIdIndex
341:            orderIds[i] = orderId;
342:
343:            ///@notice Add the orderId to the addressToAllOrderIds structure
344:            addressToAllOrderIds[msg.sender].push(orderId);
345:
346:            unchecked {
347:                ++i;
348:            }
349:        }
350:
``` 



[File:LimitOrderBook.sol#L281](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L281) 
```solidity
280:        for (uint256 i = 0; i < orderGroup.length;) {
281:            ///@notice Get the order details from the orderGroup.
282:            LimitOrder memory newOrder = orderGroup[i];
283:
284:            if (newOrder.quantity == 0) {
285:                revert OrderQuantityIsZero();
286:            }
287:
288:            ///@notice Increment the total value of orders by the quantity of the new order
289:            updatedTotalOrdersValue += newOrder.quantity;
290:
291:            ///@notice If the newOrder's tokenIn does not match the orderToken, revert.
292:            if (!(orderToken == newOrder.tokenIn)) {
293:                revert IncongruentInputTokenInOrderGroup(newOrder.tokenIn, orderToken);
294:            }
295:
296:            ///@notice If the newOrder's tokenIn does not match the orderToken, revert.
297:            if (newOrder.tokenOut == newOrder.tokenIn) {
298:                revert TokenInIsTokenOut();
299:            }
300:
301:            ///@notice If the msg.sender does not have a sufficent balance to cover the order, revert.
302:            if (tokenBalance < updatedTotalOrdersValue) {
303:                revert InsufficientWalletBalance(msg.sender, tokenBalance, updatedTotalOrdersValue);
304:            }
305:
306:            ///@notice Create a new orderId from the orderNonce and current block timestamp
307:            bytes32 orderId = keccak256(abi.encode(orderNonce, block.timestamp));
308:
309:            ///@notice Increment the cumulative execution credit by the current orders execution.
310:            cumulativeExecutionCredit += newOrder.executionCredit;
311:
312:            ///@notice increment the orderNonce
313:            /**
314:             * @dev This is unchecked because the orderNonce and block.timestamp will never be the same, so even if the
315:             *         orderNonce overflows, it will still produce unique orderIds because the timestamp will be different.
316:             */
317:            unchecked {
318:                orderNonce += 2;
319:            }
320:
321:            ///@notice Set the new order's owner to the msg.sender
322:            newOrder.owner = msg.sender;
323:
324:            ///@notice update the newOrder's Id to the orderId generated from the orderNonce
325:            newOrder.orderId = orderId;
326:
327:            ///@notice update the newOrder's last refresh timestamp
328:            ///@dev uint32(block.timestamp % (2**32 - 1)) is used to future proof the contract.
329:            newOrder.lastRefreshTimestamp = uint32(block.timestamp);
330:
331:            ///@notice Add the newly created order to the orderIdToOrder mapping
332:            orderIdToLimitOrder[orderId] = newOrder;
333:
334:            ///@notice Add the orderId to the addressToOrderIds mapping
335:            addressToOrderIds[msg.sender][orderId] = OrderType.PendingLimitOrder;
336:
337:            ///@notice Increment the total orders per address for the msg.sender
338:            ++totalOrdersPerAddress[msg.sender];
339:
340:            ///@notice Add the orderId to the orderIds array for the PlaceOrder event emission and increment the orderIdIndex
341:            orderIds[i] = orderId;
342:
343:            ///@notice Add the orderId to the addressToAllOrderIds structure
344:            addressToAllOrderIds[msg.sender].push(orderId);
345:
346:            unchecked {
347:                ++i;
348:            }
349:        }
350:
``` 



[File:LimitOrderBook.sol#L281](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L281) 
```solidity
280:        for (uint256 i = 0; i < orderGroup.length;) {
281:            ///@notice Get the order details from the orderGroup.
282:            LimitOrder memory newOrder = orderGroup[i];
283:
284:            if (newOrder.quantity == 0) {
285:                revert OrderQuantityIsZero();
286:            }
287:
288:            ///@notice Increment the total value of orders by the quantity of the new order
289:            updatedTotalOrdersValue += newOrder.quantity;
290:
291:            ///@notice If the newOrder's tokenIn does not match the orderToken, revert.
292:            if (!(orderToken == newOrder.tokenIn)) {
293:                revert IncongruentInputTokenInOrderGroup(newOrder.tokenIn, orderToken);
294:            }
295:
296:            ///@notice If the newOrder's tokenIn does not match the orderToken, revert.
297:            if (newOrder.tokenOut == newOrder.tokenIn) {
298:                revert TokenInIsTokenOut();
299:            }
300:
301:            ///@notice If the msg.sender does not have a sufficent balance to cover the order, revert.
302:            if (tokenBalance < updatedTotalOrdersValue) {
303:                revert InsufficientWalletBalance(msg.sender, tokenBalance, updatedTotalOrdersValue);
304:            }
305:
306:            ///@notice Create a new orderId from the orderNonce and current block timestamp
307:            bytes32 orderId = keccak256(abi.encode(orderNonce, block.timestamp));
308:
309:            ///@notice Increment the cumulative execution credit by the current orders execution.
310:            cumulativeExecutionCredit += newOrder.executionCredit;
311:
312:            ///@notice increment the orderNonce
313:            /**
314:             * @dev This is unchecked because the orderNonce and block.timestamp will never be the same, so even if the
315:             *         orderNonce overflows, it will still produce unique orderIds because the timestamp will be different.
316:             */
317:            unchecked {
318:                orderNonce += 2;
319:            }
320:
321:            ///@notice Set the new order's owner to the msg.sender
322:            newOrder.owner = msg.sender;
323:
324:            ///@notice update the newOrder's Id to the orderId generated from the orderNonce
325:            newOrder.orderId = orderId;
326:
327:            ///@notice update the newOrder's last refresh timestamp
328:            ///@dev uint32(block.timestamp % (2**32 - 1)) is used to future proof the contract.
329:            newOrder.lastRefreshTimestamp = uint32(block.timestamp);
330:
331:            ///@notice Add the newly created order to the orderIdToOrder mapping
332:            orderIdToLimitOrder[orderId] = newOrder;
333:
334:            ///@notice Add the orderId to the addressToOrderIds mapping
335:            addressToOrderIds[msg.sender][orderId] = OrderType.PendingLimitOrder;
336:
337:            ///@notice Increment the total orders per address for the msg.sender
338:            ++totalOrdersPerAddress[msg.sender];
339:
340:            ///@notice Add the orderId to the orderIds array for the PlaceOrder event emission and increment the orderIdIndex
341:            orderIds[i] = orderId;
342:
343:            ///@notice Add the orderId to the addressToAllOrderIds structure
344:            addressToAllOrderIds[msg.sender].push(orderId);
345:
346:            unchecked {
347:                ++i;
348:            }
349:        }
350:
``` 



[File:LimitOrderBook.sol#L281](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L281) 
```solidity
280:        for (uint256 i = 0; i < orderGroup.length;) {
281:            ///@notice Get the order details from the orderGroup.
282:            LimitOrder memory newOrder = orderGroup[i];
283:
284:            if (newOrder.quantity == 0) {
285:                revert OrderQuantityIsZero();
286:            }
287:
288:            ///@notice Increment the total value of orders by the quantity of the new order
289:            updatedTotalOrdersValue += newOrder.quantity;
290:
291:            ///@notice If the newOrder's tokenIn does not match the orderToken, revert.
292:            if (!(orderToken == newOrder.tokenIn)) {
293:                revert IncongruentInputTokenInOrderGroup(newOrder.tokenIn, orderToken);
294:            }
295:
296:            ///@notice If the newOrder's tokenIn does not match the orderToken, revert.
297:            if (newOrder.tokenOut == newOrder.tokenIn) {
298:                revert TokenInIsTokenOut();
299:            }
300:
301:            ///@notice If the msg.sender does not have a sufficent balance to cover the order, revert.
302:            if (tokenBalance < updatedTotalOrdersValue) {
303:                revert InsufficientWalletBalance(msg.sender, tokenBalance, updatedTotalOrdersValue);
304:            }
305:
306:            ///@notice Create a new orderId from the orderNonce and current block timestamp
307:            bytes32 orderId = keccak256(abi.encode(orderNonce, block.timestamp));
308:
309:            ///@notice Increment the cumulative execution credit by the current orders execution.
310:            cumulativeExecutionCredit += newOrder.executionCredit;
311:
312:            ///@notice increment the orderNonce
313:            /**
314:             * @dev This is unchecked because the orderNonce and block.timestamp will never be the same, so even if the
315:             *         orderNonce overflows, it will still produce unique orderIds because the timestamp will be different.
316:             */
317:            unchecked {
318:                orderNonce += 2;
319:            }
320:
321:            ///@notice Set the new order's owner to the msg.sender
322:            newOrder.owner = msg.sender;
323:
324:            ///@notice update the newOrder's Id to the orderId generated from the orderNonce
325:            newOrder.orderId = orderId;
326:
327:            ///@notice update the newOrder's last refresh timestamp
328:            ///@dev uint32(block.timestamp % (2**32 - 1)) is used to future proof the contract.
329:            newOrder.lastRefreshTimestamp = uint32(block.timestamp);
330:
331:            ///@notice Add the newly created order to the orderIdToOrder mapping
332:            orderIdToLimitOrder[orderId] = newOrder;
333:
334:            ///@notice Add the orderId to the addressToOrderIds mapping
335:            addressToOrderIds[msg.sender][orderId] = OrderType.PendingLimitOrder;
336:
337:            ///@notice Increment the total orders per address for the msg.sender
338:            ++totalOrdersPerAddress[msg.sender];
339:
340:            ///@notice Add the orderId to the orderIds array for the PlaceOrder event emission and increment the orderIdIndex
341:            orderIds[i] = orderId;
342:
343:            ///@notice Add the orderId to the addressToAllOrderIds structure
344:            addressToAllOrderIds[msg.sender].push(orderId);
345:
346:            unchecked {
347:                ++i;
348:            }
349:        }
350:
``` 



[File:LimitOrderBook.sol#L586](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L586) 
```solidity
585:        for (uint256 i = 0; i < length;) {
586:            bytes32 orderId;
587:            assembly {
588:                //Get the orderId at the orderOffsetSlot.
589:                orderId := mload(orderOffsetSlot)
590:                //Update the orderOffsetSlot.
591:                orderOffsetSlot := add(orderOffsetSlot, 0x20)
592:            }
593:
594:            OrderType orderType = addressToOrderIds[_owner][orderId];
595:
596:            if (orderType == targetOrderType) {
597:                orderIds[orderIdIndex] = orderId;
598:                ++orderIdIndex;
599:            }
600:
601:            unchecked {
602:                ++i;
603:            }
604:        }
605:
``` 



 --- 


 ### <a name=[G-2]></a> [G-2] Mark storage variables as `immutable` if they never change after contract initialization. - Instances: 2 

 
 
> State variables can be declared as constant or immutable. In both cases, the variables cannot be modified after the contract has been constructed. For constant variables, the value has to be fixed at compile-time, while for immutable, it can still be assigned at construction time. 
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

[File:ConveyorRouterV1.sol#L21](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L21) 
```solidity
20:    address public CONVEYOR_MULTICALL;
``` 



[File:LimitOrderBook.sol#L22](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L22) 
```solidity
21:    uint256 minExecutionCredit;
``` 



 --- 


 ### <a name=[G-3]></a> [G-3] `unchecked{++i}` instead of `i++` (or use assembly when applicable) - Instances: 4 

 
 
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

[File:LimitOrderBook.sol#L599](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L599) 
```solidity
598:                ++orderIdIndex;
``` 



[File:LimitOrderBook.sol#L525](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L525) 
```solidity
524:        --totalOrdersPerAddress[order.owner];
``` 



[File:LimitOrderBook.sol#L339](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L339) 
```solidity
338:            ++totalOrdersPerAddress[msg.sender];
``` 



[File:LimitOrderBook.sol#L468](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L468) 
```solidity
467:        --totalOrdersPerAddress[msg.sender];
``` 



[File:LimitOrderBook.sol#L503](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L503) 
```solidity
502:        --totalOrdersPerAddress[order.owner];
``` 



[File:LimitOrderSwapRouter.sol#L128](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L128) 
```solidity
127:        for (uint256 i = 0; i < _dexFactories.length; ++i) {
``` 



[File:SandboxLimitOrderBook.sol#L832](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L832) 
```solidity
831:                ++orderIdIndex;
``` 



[File:SandboxLimitOrderBook.sol#L1099](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L1099) 
```solidity
1098:        --totalOrdersPerAddress[order.owner];
``` 



[File:SandboxLimitOrderBook.sol#L416](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L416) 
```solidity
415:            ++totalOrdersPerAddress[msg.sender];
``` 



[File:SandboxLimitOrderBook.sol#L993](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L993) 
```solidity
992:                ++offset;
``` 



[File:SandboxLimitOrderBook.sol#L1235](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L1235) 
```solidity
1234:                ++orderIdIndex;
``` 



[File:SandboxLimitOrderBook.sol#L1122](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L1122) 
```solidity
1121:        --totalOrdersPerAddress[order.owner];
``` 



[File:SandboxLimitOrderBook.sol#L543](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L543) 
```solidity
542:        --totalOrdersPerAddress[msg.sender];
``` 



[File:ConveyorRouterV1.sol#L405](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L405) 
```solidity
404:            tempAffiliateNonce++;
``` 



[File:ConveyorRouterV1.sol#L421](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L421) 
```solidity
420:            tempReferrerNonce++;
``` 



 --- 


 ### <a name=[G-4]></a> [G-4] Cache Storage Variables in Memory - Instances: 6 

 
  
 Cache Array Length - Gas Report - Savings: ~0 
 <details>  
 <summary>  
  </summary> 
  
 </details> 
 
--- 

[File:SandboxLimitOrderBook.sol#L66](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L66) 
```solidity
65:        reentrancyStatus = true;
``` 



[File:SandboxLimitOrderBook.sol#L68](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L68) 
```solidity
67:        reentrancyStatus = false;
``` 



[File:SandboxLimitOrderBook.sol#L253](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L253) 
```solidity
252:                revert InsufficientExecutionCredit(executionCreditRemaining - amount, minExecutionCredit);
``` 



[File:SandboxLimitOrderBook.sol#L257](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L257) 
```solidity
256:        orderIdToSandboxLimitOrder[orderId].executionCreditRemaining = executionCreditRemaining - amount;
``` 



[File:SandboxLimitOrderBook.sol#L276](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L276) 
```solidity
275:            orderIdToSandboxLimitOrder[orderId].executionCreditRemaining + uint128(msg.value);
``` 



[File:SandboxLimitOrderBook.sol#L278](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L278) 
```solidity
277:        orderIdToSandboxLimitOrder[orderId].executionCreditRemaining = newExecutionCreditBalance;
``` 



[File:SandboxLimitOrderBook.sol#L393](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L393) 
```solidity
392:                orderNonce += 2;
``` 



[File:SandboxLimitOrderBook.sol#L472](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L472) 
```solidity
471:                orderIdToSandboxLimitOrder[order.orderId].executionCreditRemaining + uint128(msg.value);
``` 



[File:SandboxLimitOrderBook.sol#L473](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L473) 
```solidity
472:            orderIdToSandboxLimitOrder[order.orderId].executionCreditRemaining = newExecutionCredit;
``` 



[File:SandboxLimitOrderBook.sol#L501](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L501) 
```solidity
500:        orderIdToSandboxLimitOrder[order.orderId].amountInRemaining = amountInRemaining;
``` 



[File:SandboxLimitOrderBook.sol#L502](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L502) 
```solidity
501:        orderIdToSandboxLimitOrder[order.orderId].amountOutRemaining = amountOutRemaining;
``` 



[File:SandboxLimitOrderBook.sol#L537](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L537) 
```solidity
536:        delete orderIdToSandboxLimitOrder[orderId];
``` 



[File:SandboxLimitOrderBook.sol#L549](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L549) 
```solidity
548:        addressToOrderIds[order.owner][order.orderId] = OrderType.CanceledSandboxLimitOrder;
``` 



[File:SandboxLimitOrderBook.sol#L608](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L608) 
```solidity
607:            orderIdToSandboxLimitOrder[order.orderId].executionCreditRemaining = 0;
``` 



[File:SandboxLimitOrderBook.sol#L685](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L685) 
```solidity
684:        orderIdToSandboxLimitOrder[order.orderId].lastRefreshTimestamp = uint32(block.timestamp);
``` 



[File:SandboxLimitOrderBook.sol#L1055](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L1055) 
```solidity
1054:        orderIdToSandboxLimitOrder[orderId].fillPercent += percentFilled;
``` 



[File:SandboxLimitOrderBook.sol#L1058](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L1058) 
```solidity
1057:        orderIdToSandboxLimitOrder[orderId].amountInRemaining = order.amountInRemaining - amountInFilled;
``` 



[File:SandboxLimitOrderBook.sol#L1060](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L1060) 
```solidity
1059:        orderIdToSandboxLimitOrder[orderId].amountOutRemaining = order.amountOutRemaining - amountOutFilled;
``` 



[File:SandboxLimitOrderBook.sol#L1069](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L1069) 
```solidity
1068:        orderIdToSandboxLimitOrder[orderId].feeRemaining = updatedFeeRemaining;
``` 



[File:SandboxLimitOrderBook.sol#L1076](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L1076) 
```solidity
1075:        orderIdToSandboxLimitOrder[order.orderId].executionCreditRemaining = updatedExecutionCreditRemaining;
``` 



[File:SandboxLimitOrderBook.sol#L1096](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L1096) 
```solidity
1095:        delete orderIdToSandboxLimitOrder[order.orderId];
``` 



[File:SandboxLimitOrderBook.sol#L1118](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L1118) 
```solidity
1117:        delete orderIdToSandboxLimitOrder[orderId];
``` 



[File:SandboxLimitOrderBook.sol#L1128](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L1128) 
```solidity
1127:        addressToOrderIds[order.owner][order.orderId] = OrderType.FilledSandboxLimitOrder;
``` 



[File:SandboxLimitOrderBook.sol#L1253](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L1253) 
```solidity
1252:        minExecutionCredit = newMinExecutionCredit;
``` 



[File:SandboxLimitOrderBook.sol#L1263](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L1263) 
```solidity
1262:        tempOwner = address(0);
``` 



[File:ConveyorExecutor.sol#L69](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L69) 
```solidity
68:        reentrancyStatus = true;
``` 



[File:ConveyorExecutor.sol#L71](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L71) 
```solidity
70:        reentrancyStatus = false;
``` 



[File:ConveyorExecutor.sol#L511](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L511) 
```solidity
510:        uint256 withdrawAmount = conveyorBalance;
``` 



[File:ConveyorExecutor.sol#L513](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L513) 
```solidity
512:        conveyorBalance = 0;
``` 



[File:ConveyorExecutor.sol#L524](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L524) 
```solidity
523:        tempOwner = address(0);
``` 



[File:LimitOrderRouter.sol#L355](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L355) 
```solidity
354:        tempOwner = address(0);
``` 



[File:ConveyorRouterV1.sol#L373](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L373) 
```solidity
372:        tempOwner = address(0);
``` 



[File:ConveyorRouterV1.sol#L407](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L407) 
```solidity
406:            affiliateNonce = tempAffiliateNonce;
``` 



[File:ConveyorRouterV1.sol#L418](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L418) 
```solidity
417:        referrerIndex[msg.sender] = uint16(tempReferrerNonce);
``` 



[File:ConveyorRouterV1.sol#L423](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L423) 
```solidity
422:            referrerNonce = tempReferrerNonce;
``` 



[File:ConveyorRouterV1.sol#L444](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L444) 
```solidity
443:        locked = true;
``` 



[File:ConveyorRouterV1.sol#L446](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L446) 
```solidity
445:        locked = false;
``` 



[File:LimitOrderBook.sol#L32](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L32) 
```solidity
31:        reentrancyStatus = true;
``` 



[File:LimitOrderBook.sol#L34](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L34) 
```solidity
33:        reentrancyStatus = false;
``` 



[File:LimitOrderBook.sol#L187](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L187) 
```solidity
186:            revert InsufficientExecutionCredit(executionCredit - amount, minExecutionCredit);
``` 



[File:LimitOrderBook.sol#L190](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L190) 
```solidity
189:        orderIdToLimitOrder[orderId].executionCredit = executionCredit - amount;
``` 



[File:LimitOrderBook.sol#L215](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L215) 
```solidity
214:        uint128 newExecutionCreditBalance = orderIdToLimitOrder[orderId].executionCredit + uint128(msg.value);
``` 



[File:LimitOrderBook.sol#L217](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L217) 
```solidity
216:        orderIdToLimitOrder[orderId].executionCredit = newExecutionCreditBalance;
``` 



[File:LimitOrderBook.sol#L319](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L319) 
```solidity
318:                orderNonce += 2;
``` 



[File:LimitOrderBook.sol#L409](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L409) 
```solidity
408:            uint128 newExecutionCredit = orderIdToLimitOrder[order.orderId].executionCredit + uint128(msg.value);
``` 



[File:LimitOrderBook.sol#L410](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L410) 
```solidity
409:            orderIdToLimitOrder[order.orderId].executionCredit = newExecutionCredit;
``` 



[File:LimitOrderBook.sol#L438](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L438) 
```solidity
437:        orderIdToLimitOrder[order.orderId].price = price;
``` 



[File:LimitOrderBook.sol#L439](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L439) 
```solidity
438:        orderIdToLimitOrder[order.orderId].quantity = quantity;
``` 



[File:LimitOrderBook.sol#L462](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L462) 
```solidity
461:        delete orderIdToLimitOrder[orderId];
``` 



[File:LimitOrderBook.sol#L474](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L474) 
```solidity
473:        addressToOrderIds[order.owner][order.orderId] = OrderType.CanceledLimitOrder;
``` 



[File:LimitOrderBook.sol#L500](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L500) 
```solidity
499:        delete orderIdToLimitOrder[orderId];
``` 



[File:LimitOrderBook.sol#L521](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L521) 
```solidity
520:        delete orderIdToLimitOrder[orderId];
``` 



[File:LimitOrderBook.sol#L531](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L531) 
```solidity
530:        addressToOrderIds[order.owner][order.orderId] = OrderType.FilledLimitOrder;
``` 



[File:LimitOrderSwapRouter.sol#L355](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L355) 
```solidity
354:        uniV3AmountOut = 0;
``` 



[File:LimitOrderSwapRouter.sol#L383](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L383) 
```solidity
382:            uniV3AmountOut = uint256(-amount0Delta);
``` 



[File:LimitOrderSwapRouter.sol#L387](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L387) 
```solidity
386:        if (uniV3AmountOut < amountOutMin) {
``` 



[File:LimitOrderSwapRouter.sol#L388](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L388) 
```solidity
387:            revert InsufficientOutputAmount(uniV3AmountOut, amountOutMin);
``` 



[File:LimitOrderSwapRouter.sol#L586](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L586) 
```solidity
585:            address[] memory _lps = new address[](dexes.length);
``` 



[File:LimitOrderSwapRouter.sol#L589](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L589) 
```solidity
588:            for (uint256 i = 0; i < dexes.length;) {
``` 



[File:LimitOrderSwapRouter.sol#L590](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L590) 
```solidity
589:                if (dexes[i].isUniV2) {
``` 



[File:LimitOrderSwapRouter.sol#L594](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L594) 
```solidity
593:                            _calculateV2SpotPrice(token0, token1, dexes[i].factoryAddress);
``` 



[File:LimitOrderSwapRouter.sol#L606](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L606) 
```solidity
605:                                _calculateV3SpotPrice(token0, token1, FEE, dexes[i].factoryAddress);
``` 



[File:LimitOrderSwapRouter.sol#L624](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L624) 
```solidity
623:            SpotReserve[] memory _spotPrices = new SpotReserve[](dexes.length);
``` 



[File:LimitOrderSwapRouter.sol#L625](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L625) 
```solidity
624:            address[] memory _lps = new address[](dexes.length);
``` 



 --- 


 ### <a name=[G-5]></a> [G-5] Use `calldata` instead of `memory` for function arguments that do not get mutated. - Instances: 5 

 
 
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

[File:ConveyorExecutor.sol#L220](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L220) 
```solidity
219:        LimitOrderSwapRouter.TokenToWethExecutionPrice memory executionPrice
220:    ) internal returns (uint256, uint256) {
``` 



[File:ConveyorExecutor.sol#L344](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L344) 
```solidity
343:        TokenToTokenExecutionPrice memory executionPrice
344:    ) internal returns (uint256, uint256) {
``` 



[File:LimitOrderRouter.sol#L111](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L111) 
```solidity
110:    function _refreshLimitOrder(LimitOrder memory order) internal returns (uint256 executorFee) {
``` 



[File:LimitOrderRouter.sol#L178](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L178) 
```solidity
177:    function _cancelLimitOrderViaExecutor(LimitOrder memory order) internal returns (uint256) {
``` 



[File:LimitOrderRouter.sol#L209](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L209) 
```solidity
208:    function _validateOrderSequencing(LimitOrder[] memory orders) internal pure {
``` 



[File:LimitOrderQuoter.sol#L295](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L295) 
```solidity
294:        LimitOrderSwapRouter.TokenToTokenExecutionPrice memory executionPrice
295:    ) internal returns (LimitOrderSwapRouter.TokenToTokenExecutionPrice memory) {
``` 



[File:LimitOrderQuoter.sol#L326](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L326) 
```solidity
325:        LimitOrderSwapRouter.TokenToTokenExecutionPrice memory executionPrice
326:    ) internal returns (uint256 newSpotPriceA, uint128 newReserveAToken, uint128 newReserveAWeth, uint128 amountOut) {
``` 



[File:LimitOrderQuoter.sol#L343](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L343) 
```solidity
342:        LimitOrderSwapRouter.TokenToTokenExecutionPrice memory executionPrice
343:    ) internal returns (LimitOrderSwapRouter.TokenToTokenExecutionPrice memory) {
``` 



[File:LimitOrderQuoter.sol#L371](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L371) 
```solidity
370:        LimitOrderSwapRouter.TokenToTokenExecutionPrice memory executionPrice
371:    ) internal returns (uint256 newSpotPriceB, uint128 newReserveBWeth, uint128 newReserveBToken) {
``` 



[File:LimitOrderQuoter.sol#L467](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L467) 
```solidity
466:        LimitOrderSwapRouter.TokenToWethExecutionPrice memory executionPrice
467:    ) external returns (LimitOrderSwapRouter.TokenToWethExecutionPrice memory) {
``` 



[File:ConveyorRouterV1.sol#L387](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L387) 
```solidity
386:    function upgradeMulticall(bytes memory bytecode, bytes32 salt) external payable onlyOwner returns (address) {
``` 



[File:SandboxLimitOrderBook.sol#L588](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L588) 
```solidity
587:    function _cancelSandboxLimitOrderViaExecutor(SandboxLimitOrder memory order)
``` 



[File:SandboxLimitOrderBook.sol#L655](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L655) 
```solidity
654:    function _refreshSandboxLimitOrder(SandboxLimitOrder memory order) internal returns (uint256) {
``` 



[File:SandboxLimitOrderBook.sol#L809](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L809) 
```solidity
808:        PreSandboxExecutionState memory preSandboxExecutionState
809:    ) internal returns (uint256 cumulativeExecutionCreditCompensation) {
``` 



[File:SandboxLimitOrderBook.sol#L847](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L847) 
```solidity
846:        SandboxLimitOrder memory currentOrder,
``` 



[File:SandboxLimitOrderBook.sol#L904](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L904) 
```solidity
903:        PreSandboxExecutionState memory preSandboxExecutionState
904:    ) internal returns (uint256 cumulativeExecutionCompensation) {
``` 



[File:SandboxLimitOrderBook.sol#L903](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L903) 
```solidity
902:        uint128[] memory fillAmounts,
``` 



[File:SandboxLimitOrderBook.sol#L1006](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L1006) 
```solidity
1005:        uint128[] memory fillAmounts,
``` 



[File:SandboxLimitOrderBook.sol#L1004](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L1004) 
```solidity
1003:        SandboxLimitOrder memory prevOrder,
``` 



 --- 


 ### <a name=[G-6]></a> [G-6] Use assembly to hash instead of Solidity - Instances: 2 

 
 
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

[File:LimitOrderBook.sol#L308](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L308) 
```solidity
307:            bytes32 orderId = keccak256(abi.encode(orderNonce, block.timestamp));
``` 



[File:LimitOrderBook.sol#L538](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L538) 
```solidity
537:        bytes32 totalOrdersValueKey = keccak256(abi.encode(msg.sender, token));
``` 



[File:LimitOrderBook.sol#L547](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L547) 
```solidity
546:        bytes32 totalOrdersValueKey = keccak256(abi.encode(_owner, token));
``` 



[File:LimitOrderBook.sol#L556](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L556) 
```solidity
555:        bytes32 totalOrdersValueKey = keccak256(abi.encode(_owner, token));
``` 



[File:SandboxLimitOrderBook.sol#L385](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L385) 
```solidity
384:            bytes32 orderId = keccak256(abi.encode(orderNonce, block.timestamp));
``` 



[File:SandboxLimitOrderBook.sol#L1142](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L1142) 
```solidity
1141:        bytes32 totalOrdersValueKey = keccak256(abi.encode(orderOwner, token));
``` 



[File:SandboxLimitOrderBook.sol#L1151](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L1151) 
```solidity
1150:        bytes32 totalOrdersValueKey = keccak256(abi.encode(orderOwner, token));
``` 



[File:SandboxLimitOrderBook.sol#L1179](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L1179) 
```solidity
1178:        bytes32 totalOrdersValueKey = keccak256(abi.encode(msg.sender, token));
``` 



 --- 


 ### <a name=[G-7]></a> [G-7] Use custom errors instead of string error messages - Instances: 10 

 
 
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

[File:LimitOrderQuoter.sol#L17](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L17) 
```solidity
16:        require(_weth != address(0), "Invalid weth address");
``` 



[File:LimitOrderSwapRouter.sol#L130](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L130) 
```solidity
129:                require(_isUniV2[i], "First Dex must be uniswap v2");
``` 



[File:LimitOrderSwapRouter.sol#L132](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L132) 
```solidity
131:            require(_dexFactories[i] != address(0), "Zero values in constructor");
``` 



[File:ConveyorRouterV1.sol#L64](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L64) 
```solidity
63:        require(_weth != address(0), "WETH address is zero");
``` 



[File:ConveyorRouterV1.sol#L406](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L406) 
```solidity
405:            require(tempAffiliateNonce < type(uint16).max >> 0x1, "Affiliate nonce overflow");
``` 



[File:ConveyorRouterV1.sol#L422](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L422) 
```solidity
421:            require(tempReferrerNonce < type(uint16).max >> 0x1, "Referrer nonce overflow");
``` 



[File:ConveyorMath.sol#L221](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L221) 
```solidity
220:            require(answer <= uint128(MAX_64x64), "overflow");
``` 



[File:ConveyorMath.sol#L264](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L264) 
```solidity
263:                require(answer <= 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, "overflow in divuu");
``` 



[File:ConveyorMath.sol#L283](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L283) 
```solidity
282:            require(answer <= 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, "overflow in divuu last");
``` 



[File:ConveyorMath.sol#L509](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L509) 
```solidity
508:            require(x < 0x400000000000000000, "Exponential overflow"); // Overflow
``` 



[File:SandboxLimitOrderBook.sol#L94](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L94) 
```solidity
93:        require(_limitOrderExecutor != address(0), "limitOrderExecutor address is address(0)");
``` 



[File:SandboxLimitOrderBook.sol#L95](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L95) 
```solidity
94:        require(_minExecutionCredit != 0, "Minimum Execution Credit is 0");
``` 



[File:ConveyorExecutor.sol#L119](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L119) 
```solidity
118:        require(_weth != address(0), "Invalid weth address");
``` 



[File:ConveyorExecutor.sol#L120](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L120) 
```solidity
119:        require(_usdc != address(0), "Invalid usdc address");
``` 



[File:ConveyorExecutor.sol#L121](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L121) 
```solidity
120:        require(_limitOrderQuoterAddress != address(0), "Invalid LimitOrderQuoter address");
``` 



[File:LimitOrderBook.sol#L43](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L43) 
```solidity
42:        require(_limitOrderExecutor != address(0), "limitOrderExecutor address is address(0)");
``` 



[File:LimitOrderBook.sol#L45](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L45) 
```solidity
44:        require(_minExecutionCredit != 0, "Minimum Execution Credit is 0");
``` 



[File:ConveyorTickMath.sol#L91](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorTickMath.sol#L91) 
```solidity
90:            require(priceX128 <= type(uint256).max, "Overflow");
``` 



[File:OracleLibraryV2.sol#L10](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/OracleLibraryV2.sol#L10) 
```solidity
9:        require(amountOut > 0, "UniswapV2Library: INSUFFICIENT_OUTPUT_AMOUNT");
``` 



[File:OracleLibraryV2.sol#L11](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/OracleLibraryV2.sol#L11) 
```solidity
10:        require(reserveIn > 0 && reserveOut > 0, "UniswapV2Library: INSUFFICIENT_LIQUIDITY");
``` 



[File:LimitOrderRouter.sol#L70](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L70) 
```solidity
69:        require(_limitOrderExecutor != address(0), "Invalid ConveyorExecutor address");
``` 



 --- 


 ### <a name=[G-8]></a> [G-8] Use assembly for math (add, sub, mul, div) - Instances: 12 

 
 
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

[File:SandboxLimitOrderBook.sol#L252](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L252) 
```solidity
251:            if (executionCreditRemaining - amount < minExecutionCredit) {
``` 



[File:SandboxLimitOrderBook.sol#L253](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L253) 
```solidity
252:                revert InsufficientExecutionCredit(executionCreditRemaining - amount, minExecutionCredit);
``` 



[File:SandboxLimitOrderBook.sol#L257](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L257) 
```solidity
256:        orderIdToSandboxLimitOrder[orderId].executionCreditRemaining = executionCreditRemaining - amount;
``` 



[File:SandboxLimitOrderBook.sol#L260](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L260) 
```solidity
259:        emit OrderExecutionCreditUpdated(orderId, executionCreditRemaining - amount);
``` 



[File:SandboxLimitOrderBook.sol#L276](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L276) 
```solidity
275:            orderIdToSandboxLimitOrder[orderId].executionCreditRemaining + uint128(msg.value);
``` 



[File:SandboxLimitOrderBook.sol#L292](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L292) 
```solidity
291:        uint256 minimumExecutionCreditForOrderGroup = minExecutionCredit * orderGroup.length;
``` 



[File:SandboxLimitOrderBook.sol#L350](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L350) 
```solidity
349:                            ? ConveyorMath.mul128U(tokenAWethSpotPrice, newOrder.amountInRemaining)
350:                                * 10 ** (18 - tokenInDecimals)
351:                            : ConveyorMath.mul128U(tokenAWethSpotPrice, newOrder.amountInRemaining)
``` 



[File:SandboxLimitOrderBook.sol#L351](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L351) 
```solidity
350:                                * 10 ** (18 - tokenInDecimals)
``` 



[File:SandboxLimitOrderBook.sol#L352](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L352) 
```solidity
351:                            : ConveyorMath.mul128U(tokenAWethSpotPrice, newOrder.amountInRemaining)
352:                                / 10 ** (tokenInDecimals - 18);
``` 



[File:SandboxLimitOrderBook.sol#L353](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L353) 
```solidity
352:                                / 10 ** (tokenInDecimals - 18);
``` 



[File:SandboxLimitOrderBook.sol#L472](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L472) 
```solidity
471:                orderIdToSandboxLimitOrder[order.orderId].executionCreditRemaining + uint128(msg.value);
``` 



[File:SandboxLimitOrderBook.sol#L565](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L565) 
```solidity
564:        if (block.timestamp - lastCheckInTime > CHECK_IN_INTERVAL) {
``` 



[File:SandboxLimitOrderBook.sol#L603](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L603) 
```solidity
602:                executionCreditRemaining - uint128(REFRESH_FEE);
``` 



[File:SandboxLimitOrderBook.sol#L605](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L605) 
```solidity
604:            _safeTransferETH(order.owner, executionCreditRemaining - REFRESH_FEE);
``` 



[File:SandboxLimitOrderBook.sol#L625](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L625) 
```solidity
624:        if (block.timestamp - lastCheckInTime > CHECK_IN_INTERVAL) {
``` 



[File:SandboxLimitOrderBook.sol#L666](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L666) 
```solidity
665:            if (executionCreditBalance - REFRESH_FEE < minExecutionCredit) {
``` 



[File:SandboxLimitOrderBook.sol#L676](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L676) 
```solidity
675:        if (block.timestamp - order.lastRefreshTimestamp < REFRESH_INTERVAL) {
``` 



[File:SandboxLimitOrderBook.sol#L681](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L681) 
```solidity
680:            executionCreditBalance - uint128(REFRESH_FEE);
``` 



[File:SandboxLimitOrderBook.sol#L682](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L682) 
```solidity
681:        emit OrderExecutionCreditUpdated(order.orderId, executionCreditBalance - REFRESH_FEE);
``` 



[File:SandboxLimitOrderBook.sol#L822](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L822) 
```solidity
821:                orderIdIndex += orderIdBundle.length - 1;
``` 



[File:SandboxLimitOrderBook.sol#L868](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L868) 
```solidity
867:        if (initialTokenInBalance - currentTokenInBalance > fillAmount) {
``` 



[File:SandboxLimitOrderBook.sol#L870](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L870) 
```solidity
869:                currentOrder.orderId, initialTokenInBalance - currentTokenInBalance, fillAmount
``` 



[File:SandboxLimitOrderBook.sol#L875](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L875) 
```solidity
874:        if (currentTokenOutBalance - initialTokenOutBalance != amountOutRequired) {
``` 



[File:SandboxLimitOrderBook.sol#L877](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L877) 
```solidity
876:                currentOrder.orderId, currentTokenOutBalance - initialTokenOutBalance, amountOutRequired
``` 



[File:SandboxLimitOrderBook.sol#L888](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L888) 
```solidity
887:                uint128(initialTokenInBalance - currentTokenInBalance),
``` 



[File:SandboxLimitOrderBook.sol#L889](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L889) 
```solidity
888:                uint128(currentTokenOutBalance - initialTokenOutBalance),
``` 



[File:SandboxLimitOrderBook.sol#L933](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L933) 
```solidity
932:                SandboxLimitOrder memory currentOrder = preSandboxExecutionState.sandboxLimitOrders[offset + 1];
``` 



[File:SandboxLimitOrderBook.sol#L944](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L944) 
```solidity
943:                        fillAmounts[offset + 1]
``` 



[File:SandboxLimitOrderBook.sol#L956](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L956) 
```solidity
955:                        preSandboxExecutionState.initialTokenInBalances[offset] - currentTokenInBalance
956:                            > cumulativeFillAmount
``` 



[File:SandboxLimitOrderBook.sol#L961](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L961) 
```solidity
960:                            preSandboxExecutionState.initialTokenInBalances[offset] - currentTokenInBalance,
``` 



[File:SandboxLimitOrderBook.sol#L966](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L966) 
```solidity
965:                    cumulativeFillAmount = fillAmounts[offset + 1];
``` 



[File:SandboxLimitOrderBook.sol#L969](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L969) 
```solidity
968:                    cumulativeFillAmount += fillAmounts[offset + 1];
``` 



[File:SandboxLimitOrderBook.sol#L975](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L975) 
```solidity
974:                        currentTokenOutBalance - preSandboxExecutionState.initialTokenOutBalances[offset]
975:                            != cumulativeAmountOutRequired
``` 



[File:SandboxLimitOrderBook.sol#L980](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L980) 
```solidity
979:                            currentTokenOutBalance - preSandboxExecutionState.initialTokenOutBalances[offset],
``` 



[File:SandboxLimitOrderBook.sol#L999](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L999) 
```solidity
998:            _resolveOrPartialFillOrder(prevOrder, offset - 1, fillAmounts, cumulativeExecutionCompensation);
``` 



[File:SandboxLimitOrderBook.sol#L1058](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L1058) 
```solidity
1057:        orderIdToSandboxLimitOrder[orderId].amountInRemaining = order.amountInRemaining - amountInFilled;
``` 



[File:SandboxLimitOrderBook.sol#L1060](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L1060) 
```solidity
1059:        orderIdToSandboxLimitOrder[orderId].amountOutRemaining = order.amountOutRemaining - amountOutFilled;
``` 



[File:SandboxLimitOrderBook.sol#L1065](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L1065) 
```solidity
1064:        uint128 updatedFeeRemaining = feeRemaining
1065:            - uint128(ConveyorMath.mul64U(ConveyorMath.divUU(amountInFilled, amountInRemaining), feeRemaining));
``` 



[File:SandboxLimitOrderBook.sol#L1073](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L1073) 
```solidity
1072:        uint128 updatedExecutionCreditRemaining = executionCreditRemaining - executionCreditCompensation;
``` 



[File:SandboxLimitOrderBook.sol#L1080](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L1080) 
```solidity
1079:            order.amountInRemaining - amountInFilled,
``` 



[File:SandboxLimitOrderBook.sol#L1081](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L1081) 
```solidity
1080:            order.amountOutRemaining - amountOutFilled,
``` 



[File:LimitOrderSwapRouter.sol#L184](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L184) 
```solidity
183:        uint256 amountInUSDCDollarValue = ConveyorMath.mul128U(spotPrice, amountIn) / uint256(10 ** 18);
``` 



[File:LimitOrderSwapRouter.sol#L207](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L207) 
```solidity
206:        return ConveyorMath.add64x64(rationalFraction, 461168601842738800) / 10 ** 2;
``` 



[File:LimitOrderSwapRouter.sol#L272](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L272) 
```solidity
271:        amountReceived = IERC20(_tokenOut).balanceOf(_receiver) - balanceBefore;
``` 



[File:LimitOrderSwapRouter.sol#L348](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L348) 
```solidity
347:            _zeroForOne ? TickMath.MIN_SQRT_RATIO + 1 : TickMath.MAX_SQRT_RATIO - 1,
``` 



[File:LimitOrderSwapRouter.sol#L348](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L348) 
```solidity
347:            _zeroForOne ? TickMath.MIN_SQRT_RATIO + 1 : TickMath.MAX_SQRT_RATIO - 1,
``` 



[File:LimitOrderSwapRouter.sol#L475](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L475) 
```solidity
474:            ? uint128(reserve0 * (10 ** (18 - token0Decimals)))
``` 



[File:LimitOrderSwapRouter.sol#L475](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L475) 
```solidity
474:            ? uint128(reserve0 * (10 ** (18 - token0Decimals)))
``` 



[File:LimitOrderSwapRouter.sol#L476](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L476) 
```solidity
475:            : uint128(reserve0 * (10 ** (token0Decimals - 18)));
``` 



[File:LimitOrderSwapRouter.sol#L476](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L476) 
```solidity
475:            : uint128(reserve0 * (10 ** (token0Decimals - 18)));
``` 



[File:LimitOrderSwapRouter.sol#L478](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L478) 
```solidity
477:            ? uint128(reserve1 * (10 ** (18 - token1Decimals)))
``` 



[File:LimitOrderSwapRouter.sol#L478](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L478) 
```solidity
477:            ? uint128(reserve1 * (10 ** (18 - token1Decimals)))
``` 



[File:LimitOrderSwapRouter.sol#L479](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L479) 
```solidity
478:            : uint128(reserve1 * (10 ** (token1Decimals - 18)));
``` 



[File:LimitOrderSwapRouter.sol#L479](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L479) 
```solidity
478:            : uint128(reserve1 * (10 ** (token1Decimals - 18)));
``` 



[File:ConveyorFeeMath.sol#L11](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorFeeMath.sol#L11) 
```solidity
10:    uint128 constant MAX_CONVEYOR_PERCENT = 110680464442257300 * 10 ** 2;
``` 



[File:ConveyorFeeMath.sol#L32](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorFeeMath.sol#L32) 
```solidity
31:            int256 innerPartial = int256(uint256(ZERO_POINT_ZERO_ZERO_FIVE)) - int128(percentFee);
``` 



[File:ConveyorFeeMath.sol#L34](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorFeeMath.sol#L34) 
```solidity
33:            conveyorPercent = (
34:                percentFee + ConveyorMath.div64x64(uint128(uint256(innerPartial)), uint128(2) << 64)
35:                    + uint128(ZERO_POINT_ZERO_ZERO_ONE)
36:            ) * 10 ** 2;
``` 



[File:ConveyorFeeMath.sol#L35](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorFeeMath.sol#L35) 
```solidity
34:                percentFee + ConveyorMath.div64x64(uint128(uint256(innerPartial)), uint128(2) << 64)
35:                    + uint128(ZERO_POINT_ZERO_ZERO_ONE)
36:            ) * 10 ** 2;
``` 



[File:ConveyorFeeMath.sol#L35](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorFeeMath.sol#L35) 
```solidity
34:                percentFee + ConveyorMath.div64x64(uint128(uint256(innerPartial)), uint128(2) << 64)
35:                    + uint128(ZERO_POINT_ZERO_ZERO_ONE)
``` 



[File:ConveyorFeeMath.sol#L49](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorFeeMath.sol#L49) 
```solidity
48:        beaconReward = uint128(totalWethReward) - conveyorReward;
``` 



[File:ConveyorMath.sol#L74](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L74) 
```solidity
73:            uint256 answer = uint256(x) + y;
``` 



[File:ConveyorMath.sol#L86](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L86) 
```solidity
85:            int256 result = int256(x) - y;
``` 



[File:ConveyorMath.sol#L97](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L97) 
```solidity
96:        uint256 answer = x + y;
``` 



[File:ConveyorMath.sol#L107](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L107) 
```solidity
106:        uint256 answer = x + (uint256(y) << 64);
``` 



[File:ConveyorMath.sol#L118](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L118) 
```solidity
117:            uint256 answer = (uint256(x) * y) >> 64;
``` 



[File:ConveyorMath.sol#L132](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L132) 
```solidity
131:        uint256 answer = (uint256(y) * x) >> 64;
``` 



[File:ConveyorMath.sol#L147](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L147) 
```solidity
146:            uint256 lo = (uint256(x) * (y & 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF)) >> 64;
``` 



[File:ConveyorMath.sol#L148](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L148) 
```solidity
147:            uint256 hi = uint256(x) * (y >> 128);
``` 



[File:ConveyorMath.sol#L153](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L153) 
```solidity
152:            require(hi <= MAX_128x128 - lo);
``` 



[File:ConveyorMath.sol#L154](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L154) 
```solidity
153:            return hi + lo;
``` 



[File:ConveyorMath.sol#L167](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L167) 
```solidity
166:        return (x * y) >> 128;
``` 



[File:ConveyorMath.sol#L187](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L187) 
```solidity
186:            uint256 answer = (uint256(x) << 64) / y;
``` 



[File:ConveyorMath.sol#L205](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L205) 
```solidity
204:            uint256 hi = xInt * (MAX_128x128 / y);
``` 



[File:ConveyorMath.sol#L205](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L205) 
```solidity
204:            uint256 hi = xInt * (MAX_128x128 / y);
``` 



[File:ConveyorMath.sol#L206](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L206) 
```solidity
205:            uint256 lo = (xDec * (MAX_128x128 / y)) >> 128;
``` 



[File:ConveyorMath.sol#L206](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L206) 
```solidity
205:            uint256 lo = (xDec * (MAX_128x128 / y)) >> 128;
``` 



[File:ConveyorMath.sol#L208](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L208) 
```solidity
207:            require(hi <= MAX_128x128 - lo);
``` 



[File:ConveyorMath.sol#L209](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L209) 
```solidity
208:            return hi + lo;
``` 



[File:ConveyorMath.sol#L237](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L237) 
```solidity
236:                answer = (x << 64) / y;
``` 



[File:ConveyorMath.sol#L263](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L263) 
```solidity
262:                answer = (x << (255 - msb)) / (((y - 1) >> (msb - 191)) + 1);
``` 



[File:ConveyorMath.sol#L263](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L263) 
```solidity
262:                answer = (x << (255 - msb)) / (((y - 1) >> (msb - 191)) + 1);
``` 



[File:ConveyorMath.sol#L263](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L263) 
```solidity
262:                answer = (x << (255 - msb)) / (((y - 1) >> (msb - 191)) + 1);
``` 



[File:ConveyorMath.sol#L263](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L263) 
```solidity
262:                answer = (x << (255 - msb)) / (((y - 1) >> (msb - 191)) + 1);
``` 



[File:ConveyorMath.sol#L263](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L263) 
```solidity
262:                answer = (x << (255 - msb)) / (((y - 1) >> (msb - 191)) + 1);
``` 



[File:ConveyorMath.sol#L266](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L266) 
```solidity
265:                uint256 hi = answer * (y >> 128);
``` 



[File:ConveyorMath.sol#L267](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L267) 
```solidity
266:                uint256 lo = answer * (y & 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF);
``` 



[File:ConveyorMath.sol#L280](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L280) 
```solidity
279:                answer += xl / y;
``` 



[File:ConveyorMath.sol#L291](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L291) 
```solidity
290:        uint32 result = (uint32(integers) << 16) + decimals;
``` 



[File:ConveyorMath.sol#L305](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L305) 
```solidity
304:                answer = (answer * 0x16A09E667F3BCC908B2FB1366EA957D3E) >> 128;
``` 



[File:ConveyorMath.sol#L308](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L308) 
```solidity
307:                answer = (answer * 0x1306FE0A31B7152DE8D5A46305C85EDEC) >> 128;
``` 



[File:ConveyorMath.sol#L311](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L311) 
```solidity
310:                answer = (answer * 0x1172B83C7D517ADCDF7C8C50EB14A791F) >> 128;
``` 



[File:ConveyorMath.sol#L314](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L314) 
```solidity
313:                answer = (answer * 0x10B5586CF9890F6298B92B71842A98363) >> 128;
``` 



[File:ConveyorMath.sol#L317](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L317) 
```solidity
316:                answer = (answer * 0x1059B0D31585743AE7C548EB68CA417FD) >> 128;
``` 



[File:ConveyorMath.sol#L320](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L320) 
```solidity
319:                answer = (answer * 0x102C9A3E778060EE6F7CACA4F7A29BDE8) >> 128;
``` 



[File:ConveyorMath.sol#L323](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L323) 
```solidity
322:                answer = (answer * 0x10163DA9FB33356D84A66AE336DCDFA3F) >> 128;
``` 



[File:ConveyorMath.sol#L326](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L326) 
```solidity
325:                answer = (answer * 0x100B1AFA5ABCBED6129AB13EC11DC9543) >> 128;
``` 



[File:ConveyorMath.sol#L329](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L329) 
```solidity
328:                answer = (answer * 0x10058C86DA1C09EA1FF19D294CF2F679B) >> 128;
``` 



[File:ConveyorMath.sol#L332](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L332) 
```solidity
331:                answer = (answer * 0x1002C605E2E8CEC506D21BFC89A23A00F) >> 128;
``` 



[File:ConveyorMath.sol#L335](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L335) 
```solidity
334:                answer = (answer * 0x100162F3904051FA128BCA9C55C31E5DF) >> 128;
``` 



[File:ConveyorMath.sol#L338](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L338) 
```solidity
337:                answer = (answer * 0x1000B175EFFDC76BA38E31671CA939725) >> 128;
``` 



[File:ConveyorMath.sol#L341](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L341) 
```solidity
340:                answer = (answer * 0x100058BA01FB9F96D6CACD4B180917C3D) >> 128;
``` 



[File:ConveyorMath.sol#L344](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L344) 
```solidity
343:                answer = (answer * 0x10002C5CC37DA9491D0985C348C68E7B3) >> 128;
``` 



[File:ConveyorMath.sol#L347](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L347) 
```solidity
346:                answer = (answer * 0x1000162E525EE054754457D5995292026) >> 128;
``` 



[File:ConveyorMath.sol#L350](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L350) 
```solidity
349:                answer = (answer * 0x10000B17255775C040618BF4A4ADE83FC) >> 128;
``` 



[File:ConveyorMath.sol#L353](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L353) 
```solidity
352:                answer = (answer * 0x1000058B91B5BC9AE2EED81E9B7D4CFAB) >> 128;
``` 



[File:ConveyorMath.sol#L356](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L356) 
```solidity
355:                answer = (answer * 0x100002C5C89D5EC6CA4D7C8ACC017B7C9) >> 128;
``` 



[File:ConveyorMath.sol#L359](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L359) 
```solidity
358:                answer = (answer * 0x10000162E43F4F831060E02D839A9D16D) >> 128;
``` 



[File:ConveyorMath.sol#L362](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L362) 
```solidity
361:                answer = (answer * 0x100000B1721BCFC99D9F890EA06911763) >> 128;
``` 



[File:ConveyorMath.sol#L365](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L365) 
```solidity
364:                answer = (answer * 0x10000058B90CF1E6D97F9CA14DBCC1628) >> 128;
``` 



[File:ConveyorMath.sol#L368](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L368) 
```solidity
367:                answer = (answer * 0x1000002C5C863B73F016468F6BAC5CA2B) >> 128;
``` 



[File:ConveyorMath.sol#L371](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L371) 
```solidity
370:                answer = (answer * 0x100000162E430E5A18F6119E3C02282A5) >> 128;
``` 



[File:ConveyorMath.sol#L374](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L374) 
```solidity
373:                answer = (answer * 0x1000000B1721835514B86E6D96EFD1BFE) >> 128;
``` 



[File:ConveyorMath.sol#L377](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L377) 
```solidity
376:                answer = (answer * 0x100000058B90C0B48C6BE5DF846C5B2EF) >> 128;
``` 



[File:ConveyorMath.sol#L380](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L380) 
```solidity
379:                answer = (answer * 0x10000002C5C8601CC6B9E94213C72737A) >> 128;
``` 



[File:ConveyorMath.sol#L383](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L383) 
```solidity
382:                answer = (answer * 0x1000000162E42FFF037DF38AA2B219F06) >> 128;
``` 



[File:ConveyorMath.sol#L386](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L386) 
```solidity
385:                answer = (answer * 0x10000000B17217FBA9C739AA5819F44F9) >> 128;
``` 



[File:ConveyorMath.sol#L389](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L389) 
```solidity
388:                answer = (answer * 0x1000000058B90BFCDEE5ACD3C1CEDC823) >> 128;
``` 



[File:ConveyorMath.sol#L392](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L392) 
```solidity
391:                answer = (answer * 0x100000002C5C85FE31F35A6A30DA1BE50) >> 128;
``` 



[File:ConveyorMath.sol#L395](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L395) 
```solidity
394:                answer = (answer * 0x10000000162E42FF0999CE3541B9FFFCF) >> 128;
``` 



[File:ConveyorMath.sol#L398](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L398) 
```solidity
397:                answer = (answer * 0x100000000B17217F80F4EF5AADDA45554) >> 128;
``` 



[File:ConveyorMath.sol#L401](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L401) 
```solidity
400:                answer = (answer * 0x10000000058B90BFBF8479BD5A81B51AD) >> 128;
``` 



[File:ConveyorMath.sol#L404](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L404) 
```solidity
403:                answer = (answer * 0x1000000002C5C85FDF84BD62AE30A74CC) >> 128;
``` 



[File:ConveyorMath.sol#L407](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L407) 
```solidity
406:                answer = (answer * 0x100000000162E42FEFB2FED257559BDAA) >> 128;
``` 



[File:ConveyorMath.sol#L410](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L410) 
```solidity
409:                answer = (answer * 0x1000000000B17217F7D5A7716BBA4A9AE) >> 128;
``` 



[File:ConveyorMath.sol#L413](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L413) 
```solidity
412:                answer = (answer * 0x100000000058B90BFBE9DDBAC5E109CCE) >> 128;
``` 



[File:ConveyorMath.sol#L416](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L416) 
```solidity
415:                answer = (answer * 0x10000000002C5C85FDF4B15DE6F17EB0D) >> 128;
``` 



[File:ConveyorMath.sol#L419](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L419) 
```solidity
418:                answer = (answer * 0x1000000000162E42FEFA494F1478FDE05) >> 128;
``` 



[File:ConveyorMath.sol#L422](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L422) 
```solidity
421:                answer = (answer * 0x10000000000B17217F7D20CF927C8E94C) >> 128;
``` 



[File:ConveyorMath.sol#L425](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L425) 
```solidity
424:                answer = (answer * 0x1000000000058B90BFBE8F71CB4E4B33D) >> 128;
``` 



[File:ConveyorMath.sol#L428](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L428) 
```solidity
427:                answer = (answer * 0x100000000002C5C85FDF477B662B26945) >> 128;
``` 



[File:ConveyorMath.sol#L431](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L431) 
```solidity
430:                answer = (answer * 0x10000000000162E42FEFA3AE53369388C) >> 128;
``` 



[File:ConveyorMath.sol#L434](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L434) 
```solidity
433:                answer = (answer * 0x100000000000B17217F7D1D351A389D40) >> 128;
``` 



[File:ConveyorMath.sol#L437](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L437) 
```solidity
436:                answer = (answer * 0x10000000000058B90BFBE8E8B2D3D4EDE) >> 128;
``` 



[File:ConveyorMath.sol#L440](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L440) 
```solidity
439:                answer = (answer * 0x1000000000002C5C85FDF4741BEA6E77E) >> 128;
``` 



[File:ConveyorMath.sol#L443](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L443) 
```solidity
442:                answer = (answer * 0x100000000000162E42FEFA39FE95583C2) >> 128;
``` 



[File:ConveyorMath.sol#L446](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L446) 
```solidity
445:                answer = (answer * 0x1000000000000B17217F7D1CFB72B45E1) >> 128;
``` 



[File:ConveyorMath.sol#L449](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L449) 
```solidity
448:                answer = (answer * 0x100000000000058B90BFBE8E7CC35C3F0) >> 128;
``` 



[File:ConveyorMath.sol#L452](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L452) 
```solidity
451:                answer = (answer * 0x10000000000002C5C85FDF473E242EA38) >> 128;
``` 



[File:ConveyorMath.sol#L455](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L455) 
```solidity
454:                answer = (answer * 0x1000000000000162E42FEFA39F02B772C) >> 128;
``` 



[File:ConveyorMath.sol#L458](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L458) 
```solidity
457:                answer = (answer * 0x10000000000000B17217F7D1CF7D83C1A) >> 128;
``` 



[File:ConveyorMath.sol#L461](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L461) 
```solidity
460:                answer = (answer * 0x1000000000000058B90BFBE8E7BDCBE2E) >> 128;
``` 



[File:ConveyorMath.sol#L464](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L464) 
```solidity
463:                answer = (answer * 0x100000000000002C5C85FDF473DEA871F) >> 128;
``` 



[File:ConveyorMath.sol#L467](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L467) 
```solidity
466:                answer = (answer * 0x10000000000000162E42FEFA39EF44D91) >> 128;
``` 



[File:ConveyorMath.sol#L470](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L470) 
```solidity
469:                answer = (answer * 0x100000000000000B17217F7D1CF79E949) >> 128;
``` 



[File:ConveyorMath.sol#L473](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L473) 
```solidity
472:                answer = (answer * 0x10000000000000058B90BFBE8E7BCE544) >> 128;
``` 



[File:ConveyorMath.sol#L476](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L476) 
```solidity
475:                answer = (answer * 0x1000000000000002C5C85FDF473DE6ECA) >> 128;
``` 



[File:ConveyorMath.sol#L479](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L479) 
```solidity
478:                answer = (answer * 0x100000000000000162E42FEFA39EF366F) >> 128;
``` 



[File:ConveyorMath.sol#L482](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L482) 
```solidity
481:                answer = (answer * 0x1000000000000000B17217F7D1CF79AFA) >> 128;
``` 



[File:ConveyorMath.sol#L485](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L485) 
```solidity
484:                answer = (answer * 0x100000000000000058B90BFBE8E7BCD6D) >> 128;
``` 



[File:ConveyorMath.sol#L488](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L488) 
```solidity
487:                answer = (answer * 0x10000000000000002C5C85FDF473DE6B2) >> 128;
``` 



[File:ConveyorMath.sol#L491](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L491) 
```solidity
490:                answer = (answer * 0x1000000000000000162E42FEFA39EF358) >> 128;
``` 



[File:ConveyorMath.sol#L494](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L494) 
```solidity
493:                answer = (answer * 0x10000000000000000B17217F7D1CF79AB) >> 128;
``` 



[File:ConveyorMath.sol#L497](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L497) 
```solidity
496:            answer >>= uint256(63 - (x >> 64));
``` 



[File:ConveyorMath.sol#L511](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L511) 
```solidity
510:            return exp_2(uint128((uint256(x) * 0x171547652B82FE1777D0FFDA0D23A7D12) >> 128));
``` 



[File:ConveyorMath.sol#L552](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L552) 
```solidity
551:                r = (r + x / r) >> 1;
``` 



[File:ConveyorMath.sol#L552](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L552) 
```solidity
551:                r = (r + x / r) >> 1;
``` 



[File:ConveyorMath.sol#L553](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L553) 
```solidity
552:                r = (r + x / r) >> 1;
``` 



[File:ConveyorMath.sol#L553](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L553) 
```solidity
552:                r = (r + x / r) >> 1;
``` 



[File:ConveyorMath.sol#L554](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L554) 
```solidity
553:                r = (r + x / r) >> 1;
``` 



[File:ConveyorMath.sol#L554](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L554) 
```solidity
553:                r = (r + x / r) >> 1;
``` 



[File:ConveyorMath.sol#L555](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L555) 
```solidity
554:                r = (r + x / r) >> 1;
``` 



[File:ConveyorMath.sol#L555](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L555) 
```solidity
554:                r = (r + x / r) >> 1;
``` 



[File:ConveyorMath.sol#L556](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L556) 
```solidity
555:                r = (r + x / r) >> 1;
``` 



[File:ConveyorMath.sol#L556](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L556) 
```solidity
555:                r = (r + x / r) >> 1;
``` 



[File:ConveyorMath.sol#L557](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L557) 
```solidity
556:                r = (r + x / r) >> 1;
``` 



[File:ConveyorMath.sol#L557](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L557) 
```solidity
556:                r = (r + x / r) >> 1;
``` 



[File:ConveyorMath.sol#L558](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L558) 
```solidity
557:                r = (r + x / r) >> 1; // Seven iterations should be enough
``` 



[File:ConveyorMath.sol#L558](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L558) 
```solidity
557:                r = (r + x / r) >> 1; // Seven iterations should be enough
``` 



[File:ConveyorMath.sol#L559](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L559) 
```solidity
558:                uint256 r1 = x / r;
``` 



[File:ConveyorRouterV1.sol#L125](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L125) 
```solidity
124:        uint256 tokenOutAmountRequired = balanceBefore + swapData.amountOutMin;
``` 



[File:ConveyorRouterV1.sol#L133](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L133) 
```solidity
132:            revert InsufficientOutputAmount(tokenOutAmountRequired - balanceAfter, swapData.amountOutMin);
``` 



[File:ConveyorRouterV1.sol#L164](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L164) 
```solidity
163:        uint256 amountIn = msg.value - swapData.protocolFee;
``` 



[File:ConveyorRouterV1.sol#L176](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L176) 
```solidity
175:        uint256 tokenOutAmountRequired = balanceBefore + swapData.amountOutMin;
``` 



[File:ConveyorRouterV1.sol#L186](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L186) 
```solidity
185:            revert InsufficientOutputAmount(tokenOutAmountRequired - balanceAfter, swapData.amountOutMin);
``` 



[File:ConveyorRouterV1.sol#L223](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L223) 
```solidity
222:        uint256 amountOutRequired = balanceBefore + swapData.amountOutMin;
``` 



[File:ConveyorRouterV1.sol#L239](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L239) 
```solidity
238:            revert InsufficientOutputAmount(amountOutRequired - msg.sender.balance, swapData.amountOutMin);
``` 



[File:SandboxLimitOrderRouter.sol#L65](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderRouter.sol#L65) 
```solidity
64:        if (block.timestamp - lastCheckInTime > CHECK_IN_INTERVAL) {
``` 



[File:LimitOrderBook.sol#L186](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L186) 
```solidity
185:        if (executionCredit - amount < minExecutionCredit) {
``` 



[File:LimitOrderBook.sol#L187](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L187) 
```solidity
186:            revert InsufficientExecutionCredit(executionCredit - amount, minExecutionCredit);
``` 



[File:LimitOrderBook.sol#L190](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L190) 
```solidity
189:        orderIdToLimitOrder[orderId].executionCredit = executionCredit - amount;
``` 



[File:LimitOrderBook.sol#L194](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L194) 
```solidity
193:        emit OrderExecutionCreditUpdated(orderId, executionCredit - amount);
``` 



[File:LimitOrderBook.sol#L215](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L215) 
```solidity
214:        uint128 newExecutionCreditBalance = orderIdToLimitOrder[orderId].executionCredit + uint128(msg.value);
``` 



[File:LimitOrderBook.sol#L255](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L255) 
```solidity
254:        uint256 minimumExecutionCreditForOrderGroup = minExecutionCredit * orderGroup.length;
``` 



[File:LimitOrderBook.sol#L409](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L409) 
```solidity
408:            uint128 newExecutionCredit = orderIdToLimitOrder[order.orderId].executionCredit + uint128(msg.value);
``` 



[File:ConveyorTickMath.sol#L76](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorTickMath.sol#L76) 
```solidity
75:            int8 decimalShift = int8(IERC20(token0).decimals()) - int8(IERC20(token1).decimals());
``` 



[File:ConveyorTickMath.sol#L79](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorTickMath.sol#L79) 
```solidity
78:                ? uint256(sqrtPriceX96) ** 2 / uint256(10) ** (uint8(-decimalShift))
79:                : uint256(sqrtPriceX96) ** 2 * 10 ** uint8(decimalShift);
``` 



[File:ConveyorTickMath.sol#L80](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorTickMath.sol#L80) 
```solidity
79:                : uint256(sqrtPriceX96) ** 2 * 10 ** uint8(decimalShift);
``` 



[File:ConveyorTickMath.sol#L84](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorTickMath.sol#L84) 
```solidity
83:                ? priceSquaredX96 / Q96
84:                : (Q96 * 0xffffffffffffffffffffffffffffffff) / (priceSquaredX96 / Q96);
``` 



[File:ConveyorTickMath.sol#L85](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorTickMath.sol#L85) 
```solidity
84:                : (Q96 * 0xffffffffffffffffffffffffffffffff) / (priceSquaredX96 / Q96);
``` 



[File:ConveyorTickMath.sol#L85](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorTickMath.sol#L85) 
```solidity
84:                : (Q96 * 0xffffffffffffffffffffffffffffffff) / (priceSquaredX96 / Q96);
``` 



[File:ConveyorTickMath.sol#L85](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorTickMath.sol#L85) 
```solidity
84:                : (Q96 * 0xffffffffffffffffffffffffffffffff) / (priceSquaredX96 / Q96);
``` 



[File:ConveyorTickMath.sol#L89](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorTickMath.sol#L89) 
```solidity
88:                ? (uint256(priceSquaredShiftQ96) * 0xffffffffffffffffffffffffffffffff) / Q96
89:                : priceSquaredShiftQ96;
``` 



[File:ConveyorTickMath.sol#L89](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorTickMath.sol#L89) 
```solidity
88:                ? (uint256(priceSquaredShiftQ96) * 0xffffffffffffffffffffffffffffffff) / Q96
``` 



[File:ConveyorTickMath.sol#L120](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorTickMath.sol#L120) 
```solidity
119:        uint160 sqrtPriceLimitX96 = zeroForOne ? TickMath.MIN_SQRT_RATIO + 1 : TickMath.MAX_SQRT_RATIO - 1;
``` 



[File:ConveyorTickMath.sol#L120](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorTickMath.sol#L120) 
```solidity
119:        uint160 sqrtPriceLimitX96 = zeroForOne ? TickMath.MIN_SQRT_RATIO + 1 : TickMath.MAX_SQRT_RATIO - 1;
``` 



[File:ConveyorTickMath.sol#L163](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorTickMath.sol#L163) 
```solidity
162:            currentState.amountSpecifiedRemaining -= (step.amountIn + step.feeAmount).toInt256();
``` 



[File:ConveyorTickMath.sol#L177](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorTickMath.sol#L177) 
```solidity
176:                    currentState.tick = zeroForOne ? step.tickNext - 1 : step.tickNext;
``` 



[File:ConveyorExecutor.sol#L272](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L272) 
```solidity
271:        amountOutWeth = amountOutWeth - (beaconReward + conveyorReward);
``` 



[File:ConveyorExecutor.sol#L272](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L272) 
```solidity
271:        amountOutWeth = amountOutWeth - (beaconReward + conveyorReward);
``` 



[File:ConveyorExecutor.sol#L384](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L384) 
```solidity
383:                amountInWethToB = amountIn - (beaconReward + conveyorReward);
``` 



[File:ConveyorExecutor.sol#L384](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L384) 
```solidity
383:                amountInWethToB = amountIn - (beaconReward + conveyorReward);
``` 



[File:ConveyorExecutor.sol#L500](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L500) 
```solidity
499:                contractBalancePostExecution - contractBalancePreExecution,
``` 



[File:ConveyorExecutor.sol#L501](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L501) 
```solidity
500:                expectedAccumulatedFees - (contractBalancePostExecution - contractBalancePreExecution)
501:            );
``` 



[File:ConveyorExecutor.sol#L501](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L501) 
```solidity
500:                expectedAccumulatedFees - (contractBalancePostExecution - contractBalancePreExecution)
``` 



[File:LimitOrderRouter.sol#L83](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L83) 
```solidity
82:        if (block.timestamp - lastCheckInTime > CHECK_IN_INTERVAL) {
``` 



[File:LimitOrderRouter.sol#L123](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L123) 
```solidity
122:            if (executionCreditBalance - REFRESH_FEE < minExecutionCredit) {
``` 



[File:LimitOrderRouter.sol#L133](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L133) 
```solidity
132:        if (block.timestamp - order.lastRefreshTimestamp < REFRESH_INTERVAL) {
``` 



[File:LimitOrderRouter.sol#L137](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L137) 
```solidity
136:        orderIdToLimitOrder[order.orderId].executionCredit = executionCreditBalance - uint128(REFRESH_FEE);
``` 



[File:LimitOrderRouter.sol#L138](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L138) 
```solidity
137:        emit OrderExecutionCreditUpdated(order.orderId, executionCreditBalance - REFRESH_FEE);
``` 



[File:LimitOrderRouter.sol#L141](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L141) 
```solidity
140:        orderIdToLimitOrder[order.orderId].lastRefreshTimestamp = uint32(block.timestamp % (2 ** 32 - 1));
``` 



[File:LimitOrderRouter.sol#L158](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L158) 
```solidity
157:        if (block.timestamp - lastCheckInTime > CHECK_IN_INTERVAL) {
``` 



[File:LimitOrderRouter.sol#L190](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L190) 
```solidity
189:            orderIdToLimitOrder[order.orderId].executionCredit = executionCredit - uint128(REFRESH_FEE);
``` 



[File:LimitOrderRouter.sol#L192](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L192) 
```solidity
191:            _safeTransferETH(order.owner, executionCredit - REFRESH_FEE);
``` 



[File:LimitOrderRouter.sol#L211](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L211) 
```solidity
210:        for (uint256 i = 0; i < orders.length - 1;) {
``` 



[File:LimitOrderRouter.sol#L214](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L214) 
```solidity
213:            LimitOrder memory nextOrder = orders[i + 1];
``` 



[File:LimitOrderRouter.sol#L271](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L271) 
```solidity
270:        if (block.timestamp - lastCheckInTime > CHECK_IN_INTERVAL) {
``` 



[File:LimitOrderQuoter.sol#L181](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L181) 
```solidity
180:                spotReserveAToWeth.length * spotReserveWethToB.length
181:            );
``` 



[File:LimitOrderQuoter.sol#L277](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L277) 
```solidity
276:                ? uint128(alphaX * 10 ** (18 - tokenInDecimals))
``` 



[File:LimitOrderQuoter.sol#L277](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L277) 
```solidity
276:                ? uint128(alphaX * 10 ** (18 - tokenInDecimals))
``` 



[File:LimitOrderQuoter.sol#L278](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L278) 
```solidity
277:                : uint128(alphaX / (10 ** (tokenInDecimals - 18)));
``` 



[File:LimitOrderQuoter.sol#L278](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L278) 
```solidity
277:                : uint128(alphaX / (10 ** (tokenInDecimals - 18)));
``` 



[File:LimitOrderQuoter.sol#L401](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L401) 
```solidity
400:                uint256 denominator = reserveA + alphaX;
``` 



[File:LimitOrderQuoter.sol#L456](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L456) 
```solidity
455:        uint256 amountInWithFee = amountIn * 997;
``` 



[File:LimitOrderQuoter.sol#L457](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L457) 
```solidity
456:        uint256 numerator = amountInWithFee * reserveOut;
``` 



[File:LimitOrderQuoter.sol#L458](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L458) 
```solidity
457:        uint256 denominator = reserveIn * 1000 + (amountInWithFee);
``` 



[File:LimitOrderQuoter.sol#L458](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L458) 
```solidity
457:        uint256 denominator = reserveIn * 1000 + (amountInWithFee);
``` 



[File:LimitOrderQuoter.sol#L459](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L459) 
```solidity
458:        amountOut = numerator / denominator;
``` 



[File:LimitOrderQuoter.sol#L481](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L481) 
```solidity
480:            ? uint128(alphaX * 10 ** (18 - tokenInDecimals))
``` 



[File:LimitOrderQuoter.sol#L481](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L481) 
```solidity
480:            ? uint128(alphaX * 10 ** (18 - tokenInDecimals))
``` 



[File:LimitOrderQuoter.sol#L482](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L482) 
```solidity
481:            : uint128(alphaX / (10 ** (tokenInDecimals - 18)));
``` 



[File:LimitOrderQuoter.sol#L482](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L482) 
```solidity
481:            : uint128(alphaX / (10 ** (tokenInDecimals - 18)));
``` 



[File:LimitOrderQuoter.sol#L560](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L560) 
```solidity
559:            uint256 amountInBuffer = (amountInOrder * taxIn) / 10 ** 5;
``` 



[File:LimitOrderQuoter.sol#L560](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L560) 
```solidity
559:            uint256 amountInBuffer = (amountInOrder * taxIn) / 10 ** 5;
``` 



[File:LimitOrderQuoter.sol#L561](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L561) 
```solidity
560:            uint256 amountIn = amountInOrder - amountInBuffer;
``` 



[File:LimitOrderQuoter.sol#L581](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L581) 
```solidity
580:                uint256 amountInBuffer = (amountInOrder * taxIn) / 10 ** 5;
``` 



[File:LimitOrderQuoter.sol#L581](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L581) 
```solidity
580:                uint256 amountInBuffer = (amountInOrder * taxIn) / 10 ** 5;
``` 



[File:LimitOrderQuoter.sol#L583](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L583) 
```solidity
582:                uint256 amountIn = amountInOrder - amountInBuffer;
``` 



[File:LimitOrderQuoter.sol#L586](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L586) 
```solidity
585:                uint256 amountInBuffer = (amountInOrder * taxIn) / 10 ** 5;
``` 



[File:LimitOrderQuoter.sol#L586](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L586) 
```solidity
585:                uint256 amountInBuffer = (amountInOrder * taxIn) / 10 ** 5;
``` 



[File:LimitOrderQuoter.sol#L588](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L588) 
```solidity
587:                uint256 amountIn = amountInOrder - amountInBuffer;
``` 



[File:OracleLibraryV2.sol#L12](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/OracleLibraryV2.sol#L12) 
```solidity
11:        uint256 numerator = reserveIn * amountOut * 100000;
``` 



[File:OracleLibraryV2.sol#L12](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/OracleLibraryV2.sol#L12) 
```solidity
11:        uint256 numerator = reserveIn * amountOut * 100000;
``` 



[File:OracleLibraryV2.sol#L13](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/OracleLibraryV2.sol#L13) 
```solidity
12:        uint256 denominator = (reserveOut - amountOut) * (100000 - swapFee);
``` 



[File:OracleLibraryV2.sol#L13](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/OracleLibraryV2.sol#L13) 
```solidity
12:        uint256 denominator = (reserveOut - amountOut) * (100000 - swapFee);
``` 



[File:OracleLibraryV2.sol#L13](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/OracleLibraryV2.sol#L13) 
```solidity
12:        uint256 denominator = (reserveOut - amountOut) * (100000 - swapFee);
``` 



[File:OracleLibraryV2.sol#L14](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/OracleLibraryV2.sol#L14) 
```solidity
13:        amountIn = (numerator / denominator) + 1;
``` 



[File:OracleLibraryV2.sol#L14](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/OracleLibraryV2.sol#L14) 
```solidity
13:        amountIn = (numerator / denominator) + 1;
``` 



 --- 


 ### <a name=[G-9]></a> [G-9] Use assembly to write storage values - Instances: 6 

 
 
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

[File:ConveyorRouterV1.sol#L65](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L65) 
```solidity
64:        CONVEYOR_MULTICALL = address(new ConveyorMulticall());
``` 



[File:ConveyorRouterV1.sol#L67](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L67) 
```solidity
66:        owner = tx.origin;
``` 



[File:ConveyorRouterV1.sol#L373](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L373) 
```solidity
372:        tempOwner = address(0);
``` 



[File:ConveyorRouterV1.sol#L374](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L374) 
```solidity
373:        owner = msg.sender;
``` 



[File:ConveyorRouterV1.sol#L383](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L383) 
```solidity
382:        tempOwner = newOwner;
``` 



[File:ConveyorRouterV1.sol#L407](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L407) 
```solidity
406:            affiliateNonce = tempAffiliateNonce;
``` 



[File:ConveyorRouterV1.sol#L423](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L423) 
```solidity
422:            referrerNonce = tempReferrerNonce;
``` 



[File:ConveyorRouterV1.sol#L444](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L444) 
```solidity
443:        locked = true;
``` 



[File:ConveyorRouterV1.sol#L446](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L446) 
```solidity
445:        locked = false;
``` 



[File:LimitOrderSwapRouter.sol#L355](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L355) 
```solidity
354:        uniV3AmountOut = 0;
``` 



[File:LimitOrderSwapRouter.sol#L378](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L378) 
```solidity
377:            uniV3AmountOut = uint256(-amount1Delta);
``` 



[File:LimitOrderSwapRouter.sol#L383](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L383) 
```solidity
382:            uniV3AmountOut = uint256(-amount0Delta);
``` 



[File:ConveyorExecutor.sol#L69](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L69) 
```solidity
68:        reentrancyStatus = true;
``` 



[File:ConveyorExecutor.sol#L71](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L71) 
```solidity
70:        reentrancyStatus = false;
``` 



[File:ConveyorExecutor.sol#L150](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L150) 
```solidity
149:        owner = msg.sender;
``` 



[File:ConveyorExecutor.sol#L513](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L513) 
```solidity
512:        conveyorBalance = 0;
``` 



[File:ConveyorExecutor.sol#L524](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L524) 
```solidity
523:        tempOwner = address(0);
``` 



[File:ConveyorExecutor.sol#L525](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L525) 
```solidity
524:        owner = msg.sender;
``` 



[File:ConveyorExecutor.sol#L534](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L534) 
```solidity
533:        tempOwner = newOwner;
``` 



[File:LimitOrderBook.sol#L32](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L32) 
```solidity
31:        reentrancyStatus = true;
``` 



[File:LimitOrderBook.sol#L34](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L34) 
```solidity
33:        reentrancyStatus = false;
``` 



[File:LimitOrderBook.sol#L47](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L47) 
```solidity
46:        minExecutionCredit = _minExecutionCredit;
``` 



[File:LimitOrderRouter.sol#L73](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L73) 
```solidity
72:        owner = tx.origin;
``` 



[File:LimitOrderRouter.sol#L354](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L354) 
```solidity
353:        owner = msg.sender;
``` 



[File:LimitOrderRouter.sol#L355](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L355) 
```solidity
354:        tempOwner = address(0);
``` 



[File:LimitOrderRouter.sol#L363](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L363) 
```solidity
362:        tempOwner = newOwner;
``` 



[File:SandboxLimitOrderBook.sol#L66](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L66) 
```solidity
65:        reentrancyStatus = true;
``` 



[File:SandboxLimitOrderBook.sol#L68](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L68) 
```solidity
67:        reentrancyStatus = false;
``` 



[File:SandboxLimitOrderBook.sol#L96](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L96) 
```solidity
95:        minExecutionCredit = _minExecutionCredit;
``` 



[File:SandboxLimitOrderBook.sol#L103](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L103) 
```solidity
102:        owner = tx.origin;
``` 



[File:SandboxLimitOrderBook.sol#L1253](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L1253) 
```solidity
1252:        minExecutionCredit = newMinExecutionCredit;
``` 



[File:SandboxLimitOrderBook.sol#L1262](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L1262) 
```solidity
1261:        owner = msg.sender;
``` 



[File:SandboxLimitOrderBook.sol#L1263](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L1263) 
```solidity
1262:        tempOwner = address(0);
``` 



[File:SandboxLimitOrderBook.sol#L1271](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L1271) 
```solidity
1270:        tempOwner = newOwner;
``` 



 --- 


 ### <a name=[G-10]></a> [G-10] Event is not properly indexed. - Instances: 4 

 
 
> When possible, always include a minimum of 3 indexed event topics to save gas 
 
#### Gas Report - Savings: ~0 
 <details>  
 <summary>  
  </summary> 
  
 </details> 
 
--- 

[File:SandboxLimitOrderBook.sol#L152](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L152) 
```solidity
151:    event OrderExecutionCreditUpdated(bytes32 orderId, uint256 newExecutionCredit);
``` 



[File:SandboxLimitOrderBook.sol#L157](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L157) 
```solidity
156:    event MinExecutionCreditUpdated(uint256 newMinExecutionCredit, uint256 oldMinExecutionCredit);
``` 



[File:ConveyorRouterV1.sol#L34](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L34) 
```solidity
33:    event Withdraw(address indexed receiver, uint256 amount);
``` 



[File:LimitOrderBook.sol#L75](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L75) 
```solidity
74:    event OrderExecutionCreditUpdated(bytes32 orderId, uint256 newExecutionCredit);
``` 



[File:LimitOrderBook.sol#L91](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L91) 
```solidity
90:    event MinExecutionCreditUpdated(uint256 newMinExecutionCredit, uint256 oldMinExecutionCredit);
``` 



[File:ConveyorExecutor.sol#L104](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L104) 
```solidity
103:    event ExecutorCheckIn(address executor, uint256 timestamp);
``` 



 --- 


 ### <a name=[G-11]></a> [G-11] Use multiple require() statments insted of require(expression && expression && ...) - Instances: 2 

 
 
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

[File:OracleLibraryV2.sol#L11](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/OracleLibraryV2.sol#L11) 
```solidity
10:        require(reserveIn > 0 && reserveOut > 0, "UniswapV2Library: INSUFFICIENT_LIQUIDITY");
``` 



[File:ConveyorMath.sol#L54](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L54) 
```solidity
53:            require(answer >= 0x0 && answer <= MAX_64x64);
``` 



[File:ConveyorMath.sol#L87](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L87) 
```solidity
86:            require(result >= MIN_64x64 && result <= type(int128).max);
``` 



 --- 


 ### <a name=[G-12]></a> [G-12] Optimal Comparison - Instances: 6 

 
 
> When comparing integers, it is cheaper to use strict `>` & `<` operators over `>=` & `<=` operators, even if you must increment or decrement one of the operands. 
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

[File:LimitOrderSwapRouter.sol#L187](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L187) 
```solidity
186:        if (amountInUSDCDollarValue >= 1000000) {
``` 



[File:LimitOrderSwapRouter.sol#L197](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L197) 
```solidity
196:        if (exponent >= 0x400000000000000000) {
``` 



[File:LimitOrderSwapRouter.sol#L474](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L474) 
```solidity
473:        uint128 commonReserve0 = token0Decimals <= 18
474:            ? uint128(reserve0 * (10 ** (18 - token0Decimals)))
``` 



[File:LimitOrderSwapRouter.sol#L477](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L477) 
```solidity
476:        uint128 commonReserve1 = token1Decimals <= 18
477:            ? uint128(reserve1 * (10 ** (18 - token1Decimals)))
``` 



[File:SandboxLimitOrderBook.sol#L349](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L349) 
```solidity
348:                        relativeWethValue = tokenInDecimals <= 18
349:                            ? ConveyorMath.mul128U(tokenAWethSpotPrice, newOrder.amountInRemaining)
``` 



[File:LimitOrderQuoter.sol#L276](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L276) 
```solidity
275:            uint128 amountIn = tokenInDecimals <= 18
276:                ? uint128(alphaX * 10 ** (18 - tokenInDecimals))
``` 



[File:LimitOrderQuoter.sol#L480](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L480) 
```solidity
479:        uint128 amountIn = tokenInDecimals <= 18
480:            ? uint128(alphaX * 10 ** (18 - tokenInDecimals))
``` 



[File:ConveyorTickMath.sol#L91](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorTickMath.sol#L91) 
```solidity
90:            require(priceX128 <= type(uint256).max, "Overflow");
``` 



[File:ConveyorFeeMath.sol#L31](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorFeeMath.sol#L31) 
```solidity
30:        if (percentFee <= ZERO_POINT_ZERO_ZERO_FIVE) {
``` 



[File:ConveyorMath.sol#L23](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L23) 
```solidity
22:            require(x <= MAX_UINT64);
``` 



[File:ConveyorMath.sol#L42](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L42) 
```solidity
41:            require(x <= 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF);
``` 



[File:ConveyorMath.sol#L54](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L54) 
```solidity
53:            require(answer >= 0x0 && answer <= MAX_64x64);
``` 



[File:ConveyorMath.sol#L54](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L54) 
```solidity
53:            require(answer >= 0x0 && answer <= MAX_64x64);
``` 



[File:ConveyorMath.sol#L75](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L75) 
```solidity
74:            require(answer <= MAX_64x64);
``` 



[File:ConveyorMath.sol#L87](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L87) 
```solidity
86:            require(result >= MIN_64x64 && result <= type(int128).max);
``` 



[File:ConveyorMath.sol#L87](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L87) 
```solidity
86:            require(result >= MIN_64x64 && result <= type(int128).max);
``` 



[File:ConveyorMath.sol#L119](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L119) 
```solidity
118:            require(answer <= MAX_64x64);
``` 



[File:ConveyorMath.sol#L150](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L150) 
```solidity
149:            require(hi <= 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF);
``` 



[File:ConveyorMath.sol#L153](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L153) 
```solidity
152:            require(hi <= MAX_128x128 - lo);
``` 



[File:ConveyorMath.sol#L189](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L189) 
```solidity
188:            require(answer <= 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF);
``` 



[File:ConveyorMath.sol#L208](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L208) 
```solidity
207:            require(hi <= MAX_128x128 - lo);
``` 



[File:ConveyorMath.sol#L221](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L221) 
```solidity
220:            require(answer <= uint128(MAX_64x64), "overflow");
``` 



[File:ConveyorMath.sol#L236](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L236) 
```solidity
235:            if (x <= 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF) {
``` 



[File:ConveyorMath.sol#L241](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L241) 
```solidity
240:                if (xc >= 0x100000000) {
``` 



[File:ConveyorMath.sol#L245](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L245) 
```solidity
244:                if (xc >= 0x10000) {
``` 



[File:ConveyorMath.sol#L249](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L249) 
```solidity
248:                if (xc >= 0x100) {
``` 



[File:ConveyorMath.sol#L253](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L253) 
```solidity
252:                if (xc >= 0x10) {
``` 



[File:ConveyorMath.sol#L257](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L257) 
```solidity
256:                if (xc >= 0x4) {
``` 



[File:ConveyorMath.sol#L261](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L261) 
```solidity
260:                if (xc >= 0x2) msb += 1; // No need to shift xc anymore
``` 



[File:ConveyorMath.sol#L264](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L264) 
```solidity
263:                require(answer <= 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, "overflow in divuu");
``` 



[File:ConveyorMath.sol#L283](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L283) 
```solidity
282:            require(answer <= 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, "overflow in divuu last");
``` 



[File:ConveyorMath.sol#L498](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L498) 
```solidity
497:            require(answer <= uint256(MAX_64x64));
``` 



[File:ConveyorMath.sol#L525](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L525) 
```solidity
524:                if (xx >= 0x100000000000000000000000000000000) {
``` 



[File:ConveyorMath.sol#L529](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L529) 
```solidity
528:                if (xx >= 0x10000000000000000) {
``` 



[File:ConveyorMath.sol#L533](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L533) 
```solidity
532:                if (xx >= 0x100000000) {
``` 



[File:ConveyorMath.sol#L537](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L537) 
```solidity
536:                if (xx >= 0x10000) {
``` 



[File:ConveyorMath.sol#L541](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L541) 
```solidity
540:                if (xx >= 0x100) {
``` 



[File:ConveyorMath.sol#L545](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L545) 
```solidity
544:                if (xx >= 0x10) {
``` 



[File:ConveyorMath.sol#L549](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L549) 
```solidity
548:                if (xx >= 0x8) {
``` 



 --- 


 ### <a name=[G-13]></a> [G-13] Mark functions as payable (with discretion) - Instances: 44 

 
 
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

[File:ConveyorExecutor.sol#L154](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L154) 
```solidity
153:    function checkIn() external {
``` 



[File:ConveyorExecutor.sol#L162](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L162) 
```solidity
161:    function executeTokenToWethOrders(LimitOrderBook.LimitOrder[] calldata orders)
162:        external
163:        onlyLimitOrderRouter
164:        returns (uint256, uint256)
165:    {
``` 



[File:ConveyorExecutor.sol#L277](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L277) 
```solidity
276:    function executeTokenToTokenOrders(LimitOrderBook.LimitOrder[] calldata orders)
277:        external
278:        onlyLimitOrderRouter
279:        returns (uint256, uint256)
280:    {
``` 



[File:ConveyorExecutor.sol#L419](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L419) 
```solidity
418:    function executeSandboxLimitOrders(
419:        SandboxLimitOrderBook.SandboxLimitOrder[] calldata orders,
420:        SandboxLimitOrderRouter.SandboxMulticall calldata sandboxMulticall
421:    ) external onlySandboxLimitOrderBook nonReentrant {
``` 



[File:ConveyorExecutor.sol#L507](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L507) 
```solidity
506:    function withdrawConveyorFees() external nonReentrant onlyOwner {
``` 



[File:ConveyorExecutor.sol#L518](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L518) 
```solidity
517:    function confirmTransferOwnership() external {
``` 



[File:ConveyorExecutor.sol#L529](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L529) 
```solidity
528:    function transferOwnership(address newOwner) external onlyOwner {
``` 



[File:DeployMainnetAggregator.s.sol#L12](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/deploy/DeployMainnetAggregator.s.sol#L12) 
```solidity
11:    function run() public returns (address conveyorRouterV1) {
``` 



[File:BiswapCallback.sol#L13](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/BiswapCallback.sol#L13) 
```solidity
12:    function BiswapCall(address, uint256 amount0, uint256 amount1, bytes calldata data) external {
``` 



[File:CafeSwapCallback.sol#L13](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/CafeSwapCallback.sol#L13) 
```solidity
12:    function cafeCall(address, uint256 amount0, uint256 amount1, bytes calldata data) external {
``` 



[File:AlgebraCallback.sol#L11](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/AlgebraCallback.sol#L11) 
```solidity
10:    function algebraSwapCallback(int256 amount0Delta, int256 amount1Delta, bytes calldata data) external {
``` 



[File:LimitOrderSwapRouter.sol#L172](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L172) 
```solidity
171:    function calculateFee(uint128 amountIn, address usdc, address weth) public view returns (uint128) {
``` 



[File:LimitOrderSwapRouter.sol#L364](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L364) 
```solidity
363:    function uniswapV3SwapCallback(int256 amount0Delta, int256 amount1Delta, bytes calldata data) external {
``` 



[File:LimitOrderSwapRouter.sol#L577](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L577) 
```solidity
576:    function getAllPrices(address token0, address token1, uint24 FEE)
577:        public
578:        view
579:        returns (SpotReserve[] memory prices, address[] memory lps)
580:    {
``` 



[File:BabySwapCallback.sol#L13](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/BabySwapCallback.sol#L13) 
```solidity
12:    function babyCall(address, uint256 amount0, uint256 amount1, bytes calldata data) external {
``` 



[File:WaultSwapCallback.sol#L13](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/WaultSwapCallback.sol#L13) 
```solidity
12:    function waultSwapCall(address, uint256 amount0, uint256 amount1, bytes calldata data) external {
``` 



[File:TraderJoeCallback.sol#L13](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/TraderJoeCallback.sol#L13) 
```solidity
12:    function joeCall(address, uint256 amount0, uint256 amount1, bytes calldata data) external {
``` 



[File:DeployBSCAggregator.s.sol#L14](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/deploy/DeployBSCAggregator.s.sol#L14) 
```solidity
13:    function run() public returns (address conveyorRouterV1) {
``` 



[File:DeployTest.s.sol#L14](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/deploy/DeployTest.s.sol#L14) 
```solidity
13:    function run() public {
``` 



[File:DeployAvalancheAggregator.s.sol#L11](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/deploy/DeployAvalancheAggregator.s.sol#L11) 
```solidity
10:    function run() public returns (ConveyorRouterV1 conveyorRouterV1) {
``` 



[File:DeployArbitrumAggregator.s.sol#L11](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/deploy/DeployArbitrumAggregator.s.sol#L11) 
```solidity
10:    function run() public returns (ConveyorRouterV1 conveyorRouterV1) {
``` 



[File:ConvergenceXCallback.sol#L13](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/ConvergenceXCallback.sol#L13) 
```solidity
12:    function swapCall(address, uint256 amount0, uint256 amount1, bytes calldata data) external {
``` 



[File:UniFiCallback.sol#L13](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/UniFiCallback.sol#L13) 
```solidity
12:    function unifiCall(address, uint256 amount0, uint256 amount1, bytes calldata data) external {
``` 



[File:DeployFantomAggregator.s.sol#L11](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/deploy/DeployFantomAggregator.s.sol#L11) 
```solidity
10:    function run() public returns (ConveyorRouterV1 conveyorRouterV1) {
``` 



[File:LimitOrderQuoter.sol#L50](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L50) 
```solidity
49:    function findBestTokenToWethExecutionPrice(
50:        LimitOrderSwapRouter.TokenToWethExecutionPrice[] calldata executionPrices,
51:        bool buyOrder
52:    ) external pure returns (uint256 bestPriceIndex) {
``` 



[File:LimitOrderQuoter.sol#L95](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L95) 
```solidity
94:    function findBestTokenToTokenExecutionPrice(
95:        LimitOrderSwapRouter.TokenToTokenExecutionPrice[] calldata executionPrices,
96:        bool buyOrder
97:    ) external pure returns (uint256 bestPriceIndex) {
``` 



[File:LimitOrderQuoter.sol#L135](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L135) 
```solidity
134:    function initializeTokenToWethExecutionPrices(
135:        LimitOrderSwapRouter.SpotReserve[] calldata spotReserveAToWeth,
136:        address[] calldata lpAddressesAToWeth
137:    ) external pure returns (LimitOrderSwapRouter.TokenToWethExecutionPrice[] memory) {
``` 



[File:LimitOrderQuoter.sol#L171](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L171) 
```solidity
170:    function initializeTokenToTokenExecutionPrices(
171:        address tokenIn,
172:        LimitOrderSwapRouter.SpotReserve[] calldata spotReserveAToWeth,
173:        address[] calldata lpAddressesAToWeth,
174:        LimitOrderSwapRouter.SpotReserve[] calldata spotReserveWethToB,
175:        address[] calldata lpAddressesWethToB
176:    ) external view returns (LimitOrderSwapRouter.TokenToTokenExecutionPrice[] memory) {
``` 



[File:LimitOrderQuoter.sol#L249](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L249) 
```solidity
248:    function simulateTokenToTokenPriceChange(
249:        uint128 alphaX,
250:        LimitOrderSwapRouter.TokenToTokenExecutionPrice memory executionPrice
251:    ) external returns (LimitOrderSwapRouter.TokenToTokenExecutionPrice memory) {
``` 



[File:LimitOrderQuoter.sol#L465](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L465) 
```solidity
464:    function simulateTokenToWethPriceChange(
465:        uint128 alphaX,
466:        LimitOrderSwapRouter.TokenToWethExecutionPrice memory executionPrice
467:    ) external returns (LimitOrderSwapRouter.TokenToWethExecutionPrice memory) {
``` 



[File:LimitOrderQuoter.sol#L549](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L549) 
```solidity
548:    function calculateAmountOutMinAToWeth(
549:        address lpAddressAToWeth,
550:        uint256 amountInOrder,
551:        uint16 taxIn,
552:        uint24 feeIn,
553:        address tokenIn
554:    ) external returns (uint256 amountOutMinAToWeth) {
``` 



[File:PancakeV2Callback.sol#L13](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/PancakeV2Callback.sol#L13) 
```solidity
12:    function pancakeCall(address, uint256 amount0, uint256 amount1, bytes calldata data) external {
``` 



[File:MdexSwapCallback.sol#L13](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/MdexSwapCallback.sol#L13) 
```solidity
12:    function swapV2Call(address, uint256 amount0, uint256 amount1, bytes calldata data) external {
``` 



[File:ApeSwapCallback.sol#L13](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/ApeSwapCallback.sol#L13) 
```solidity
12:    function apeCall(address, uint256 amount0, uint256 amount1, bytes calldata data) external {
``` 



[File:PancakeV3Callback.sol#L11](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/PancakeV3Callback.sol#L11) 
```solidity
10:    function pancakeV3SwapCallback(int256 amount0Delta, int256 amount1Delta, bytes calldata data) external {
``` 



[File:KyberSwapV3Callback.sol#L11](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/KyberSwapV3Callback.sol#L11) 
```solidity
10:    function swapCallback(int256 amount0Delta, int256 amount1Delta, bytes calldata data) external {
``` 



[File:UniswapV2Callback.sol#L13](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/UniswapV2Callback.sol#L13) 
```solidity
12:    function uniswapV2Call(address, uint256 amount0, uint256 amount1, bytes calldata data) external {
``` 



[File:ConveyorFeeMath.sol#L19](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorFeeMath.sol#L19) 
```solidity
18:    function calculateReward(uint128 percentFee, uint128 wethValue)
19:        public
20:        pure
21:        returns (uint128 conveyorReward, uint128 beaconReward)
22:    {
``` 



[File:NomiswapCallback.sol#L13](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/NomiswapCallback.sol#L13) 
```solidity
12:    function nomiswapCall(address, uint256 amount0, uint256 amount1, bytes calldata data) external {
``` 



[File:DeployOptimismAggregator.s.sol#L11](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/deploy/DeployOptimismAggregator.s.sol#L11) 
```solidity
10:    function run() public returns (ConveyorRouterV1 conveyorRouterV1) {
``` 



[File:UniswapV3Callback.sol#L11](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/UniswapV3Callback.sol#L11) 
```solidity
10:    function uniswapV3SwapCallback(int256 amount0Delta, int256 amount1Delta, bytes calldata data) external {
``` 



[File:SakeSwapCallback.sol#L13](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/SakeSwapCallback.sol#L13) 
```solidity
12:    function SakeSwapCall(address, uint256 amount0, uint256 amount1, bytes calldata data) external {
``` 



[File:DystopiaCallback.sol#L13](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/DystopiaCallback.sol#L13) 
```solidity
12:    function hook(address, uint256 amount0, uint256 amount1, bytes calldata data) external {
``` 



[File:SandboxLimitOrderBook.sol#L238](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L238) 
```solidity
237:    function decreaseExecutionCredit(bytes32 orderId, uint128 amount) external nonReentrant {
``` 



[File:SandboxLimitOrderBook.sol#L511](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L511) 
```solidity
510:    function cancelOrders(bytes32[] calldata orderIds) public {
``` 



[File:SandboxLimitOrderBook.sol#L524](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L524) 
```solidity
523:    function cancelOrder(bytes32 orderId) public {
``` 



[File:SandboxLimitOrderBook.sol#L560](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L560) 
```solidity
559:    function validateAndCancelOrder(bytes32 orderId) external nonReentrant returns (bool success) {
``` 



[File:SandboxLimitOrderBook.sol#L620](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L620) 
```solidity
619:    function refreshOrder(bytes32[] calldata orderIds) external nonReentrant {
``` 



[File:SandboxLimitOrderBook.sol#L703](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L703) 
```solidity
702:    function executeOrdersViaSandboxMulticall(SandboxLimitOrderRouter.SandboxMulticall calldata sandboxMulticall)
703:        external
704:        onlySandboxLimitOrderRouter
705:        nonReentrant
706:    {
``` 



[File:SandboxLimitOrderBook.sol#L1178](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L1178) 
```solidity
1177:    function getTotalOrdersValue(address token) public view returns (uint256 totalOrderValue) {
``` 



[File:SandboxLimitOrderBook.sol#L1183](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L1183) 
```solidity
1182:    function getAllOrderIdsLength(address orderOwner) public view returns (uint256) {
``` 



[File:SandboxLimitOrderBook.sol#L1187](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L1187) 
```solidity
1186:    function getSandboxLimitOrderRouterAddress() public view returns (address) {
``` 



[File:SandboxLimitOrderBook.sol#L1191](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L1191) 
```solidity
1190:    function getSandboxLimitOrderById(bytes32 orderId) public view returns (SandboxLimitOrder memory) {
``` 



[File:SandboxLimitOrderBook.sol#L1206](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L1206) 
```solidity
1205:    function getOrderIds(address orderOwner, OrderType targetOrderType, uint256 orderOffset, uint256 length)
1206:        public
1207:        view
1208:        returns (bytes32[] memory)
1209:    {
``` 



[File:SandboxLimitOrderBook.sol#L1251](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L1251) 
```solidity
1250:    function setMinExecutionCredit(uint256 newMinExecutionCredit) external onlyOwner {
``` 



[File:SandboxLimitOrderBook.sol#L1258](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L1258) 
```solidity
1257:    function confirmTransferOwnership() external {
``` 



[File:SandboxLimitOrderBook.sol#L1267](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L1267) 
```solidity
1266:    function transferOwnership(address newOwner) external onlyOwner {
``` 



[File:MeerkatCallback.sol#L13](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/MeerkatCallback.sol#L13) 
```solidity
12:    function MeerkatCall(address, uint256 amount0, uint256 amount1, bytes calldata data) external {
``` 



[File:LimitOrderBook.sol#L168](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L168) 
```solidity
167:    function decreaseExecutionCredit(bytes32 orderId, uint128 amount) external nonReentrant {
``` 



[File:LimitOrderBook.sol#L224](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L224) 
```solidity
223:    function getLimitOrderById(bytes32 orderId) public view returns (LimitOrder memory) {
``` 



[File:LimitOrderBook.sol#L449](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L449) 
```solidity
448:    function cancelOrder(bytes32 orderId) public {
``` 



[File:LimitOrderBook.sol#L483](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L483) 
```solidity
482:    function cancelOrders(bytes32[] calldata orderIds) public {
``` 



[File:LimitOrderBook.sol#L537](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L537) 
```solidity
536:    function getTotalOrdersValue(address token) public view returns (uint256 totalOrderValue) {
``` 



[File:LimitOrderBook.sol#L560](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L560) 
```solidity
559:    function getAllOrderIdsLength(address _owner) public view returns (uint256) {
``` 



[File:LimitOrderBook.sol#L570](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L570) 
```solidity
569:    function getOrderIds(address _owner, OrderType targetOrderType, uint256 orderOffset, uint256 length)
570:        public
571:        view
572:        returns (bytes32[] memory)
573:    {
``` 



[File:LinkSwapCallback.sol#L13](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/LinkSwapCallback.sol#L13) 
```solidity
12:    function linkswapCall(address, uint256 amount0, uint256 amount1, bytes calldata data) external {
``` 



[File:ConveyorRouterV1.sol#L361](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L361) 
```solidity
360:    function withdraw() external onlyOwner {
``` 



[File:ConveyorRouterV1.sol#L367](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L367) 
```solidity
366:    function confirmTransferOwnership() external {
``` 



[File:ConveyorRouterV1.sol#L378](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L378) 
```solidity
377:    function transferOwnership(address newOwner) external onlyOwner {
``` 



[File:ConveyorRouterV1.sol#L400](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L400) 
```solidity
399:    function initializeAffiliate(address affiliateAddress) external onlyOwner {
``` 



[File:ConveyorRouterV1.sol#L451](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L451) 
```solidity
450:    function executeMulticall(ConveyorRouterV1.SwapAggregatorMulticall calldata multicall) external lock {
``` 



[File:BabyDogeCallback.sol#L13](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/BabyDogeCallback.sol#L13) 
```solidity
12:    function BabyDogeCall(address, uint256 amount0, uint256 amount1, bytes calldata data) external {
``` 



[File:ElkSwapCallback.sol#L13](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/ElkSwapCallback.sol#L13) 
```solidity
12:    function elkCall(address, uint256 amount0, uint256 amount1, bytes calldata data) external {
``` 



[File:JetSwapCallback.sol#L13](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/JetSwapCallback.sol#L13) 
```solidity
12:    function jetswapCall(address, uint256 amount0, uint256 amount1, bytes calldata data) external {
``` 



[File:DeployBaseAggregator.s.sol#L12](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/deploy/DeployBaseAggregator.s.sol#L12) 
```solidity
11:    function run() public returns (address conveyorRouterV1) {
``` 



[File:VerseCallback.sol#L13](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/VerseCallback.sol#L13) 
```solidity
12:    function swapsCall(address, uint256 amount0, uint256 amount1, bytes calldata data) external {
``` 



[File:LimitOrderRouter.sol#L78](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L78) 
```solidity
77:    function refreshOrder(bytes32[] calldata orderIds) external nonReentrant {
``` 



[File:LimitOrderRouter.sol#L153](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L153) 
```solidity
152:    function validateAndCancelOrder(bytes32 orderId) external nonReentrant returns (bool success) {
``` 



[File:LimitOrderRouter.sol#L266](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L266) 
```solidity
265:    function executeLimitOrders(bytes32[] calldata orderIds) external nonReentrant onlyEOA {
``` 



[File:LimitOrderRouter.sol#L350](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L350) 
```solidity
349:    function confirmTransferOwnership() external {
``` 



[File:LimitOrderRouter.sol#L359](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L359) 
```solidity
358:    function transferOwnership(address newOwner) external onlyOwner {
``` 



[File:LimitOrderRouter.sol#L366](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L366) 
```solidity
365:    function setMinExecutionCredit(uint256 newMinExecutionCredit) external onlyOwner {
``` 



[File:DXSwapCallback.sol#L13](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/DXSwapCallback.sol#L13) 
```solidity
12:    function DXswapCall(address, uint256 amount0, uint256 amount1, bytes calldata data) external {
``` 



[File:SandboxLimitOrderRouter.sol#L61](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderRouter.sol#L61) 
```solidity
60:    function executeSandboxMulticall(SandboxMulticall calldata sandboxMultiCall) external {
``` 



[File:SandboxLimitOrderRouter.sol#L75](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderRouter.sol#L75) 
```solidity
74:    function sandboxRouterCallback(SandboxMulticall calldata sandboxMulticall) external onlyLimitOrderExecutor {
``` 



[File:SandboxLimitOrderRouter.sol#L96](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderRouter.sol#L96) 
```solidity
95:    function uniswapV3SwapCallback(int256 amount0Delta, int256 amount1Delta, bytes calldata data) external {
``` 



[File:DefiSwapCallback.sol#L13](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/DefiSwapCallback.sol#L13) 
```solidity
12:    function croDefiSwapCall(address, uint256 amount0, uint256 amount1, bytes calldata data) external {
``` 



[File:DeployPolygonAggregator.s.sol#L12](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/deploy/DeployPolygonAggregator.s.sol#L12) 
```solidity
11:    function run() public returns (address conveyorRouterV1) {
``` 



 --- 


 ### <a name=[G-14]></a> [G-14] Consider marking constants as private - Instances: 14 

 
 
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

[File:LimitOrderBook.sol#L20](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L20) 
```solidity
19:    uint256 public constant CHECK_IN_INTERVAL = 1 days;
``` 



[File:DeployArbitrumAggregator.s.sol#L9](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/deploy/DeployArbitrumAggregator.s.sol#L9) 
```solidity
8:    address constant WETH = 0x82aF49447D8a07e3bd95BD0d56f35241523fBab1;
``` 



[File:DeployBSCAggregator.s.sol#L12](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/deploy/DeployBSCAggregator.s.sol#L12) 
```solidity
11:    address constant WBNB = 0xbb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c;
``` 



[File:DeployAvalancheAggregator.s.sol#L9](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/deploy/DeployAvalancheAggregator.s.sol#L9) 
```solidity
8:    address constant WAVAX = 0xB31f66AA3C1e785363F0875A1B74E27b85FD66c7;
``` 



[File:ConveyorFeeMath.sol#L9](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorFeeMath.sol#L9) 
```solidity
8:    uint128 constant ZERO_POINT_ZERO_ZERO_FIVE = 92233720368547760;
``` 



[File:ConveyorFeeMath.sol#L10](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorFeeMath.sol#L10) 
```solidity
9:    uint128 constant ZERO_POINT_ZERO_ZERO_ONE = 18446744073709550;
``` 



[File:ConveyorFeeMath.sol#L11](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorFeeMath.sol#L11) 
```solidity
10:    uint128 constant MAX_CONVEYOR_PERCENT = 110680464442257300 * 10 ** 2;
``` 



[File:ConveyorFeeMath.sol#L12](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorFeeMath.sol#L12) 
```solidity
11:    uint128 constant MIN_CONVEYOR_PERCENT = 7378697629483821000;
``` 



[File:DeployBaseAggregator.s.sol#L10](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/deploy/DeployBaseAggregator.s.sol#L10) 
```solidity
9:    address constant WETH = 0x4200000000000000000000000000000000000006;
``` 



[File:DeployTest.s.sol#L12](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/deploy/DeployTest.s.sol#L12) 
```solidity
11:    address constant GOERLI_WETH = 0xdD69DB25F6D620A7baD3023c5d32761D353D3De9;
``` 



[File:DeployPolygonAggregator.s.sol#L10](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/deploy/DeployPolygonAggregator.s.sol#L10) 
```solidity
9:    address constant WMATIC = 0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270;
``` 



[File:DeployFantomAggregator.s.sol#L9](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/deploy/DeployFantomAggregator.s.sol#L9) 
```solidity
8:    address constant WFTM = 0x21be370D5312f44cB42ce377BC9b8a0cEF1A4C83;
``` 



[File:ConveyorTickMath.sol#L29](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorTickMath.sol#L29) 
```solidity
28:    uint256 internal constant Q96 = 0x1000000000000000000000000;
``` 



[File:SandboxLimitOrderBook.sol#L42](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L42) 
```solidity
41:    uint256 public constant CHECK_IN_INTERVAL = 1 days;
``` 



[File:DeployOptimismAggregator.s.sol#L9](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/deploy/DeployOptimismAggregator.s.sol#L9) 
```solidity
8:    address constant WETH = 0x4200000000000000000000000000000000000006;
``` 



[File:DeployMainnetAggregator.s.sol#L10](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/deploy/DeployMainnetAggregator.s.sol#L10) 
```solidity
9:    address constant WETH = 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2;
``` 



[File:SandboxLimitOrderRouter.sol#L22](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderRouter.sol#L22) 
```solidity
21:    uint256 public constant CHECK_IN_INTERVAL = 1 days;
``` 



 --- 


 ### <a name=[G-15]></a> [G-15] Use assembly when getting a contract's balance of ETH - Instances: 1 

 
 
> You can use `selfbalance()` instead of `address(this).balance` when getting your contract's balance of ETH to save gas. Additionally, you can use `balance(address)` instead of `address.balance()` when getting an external contract's balance of ETH.
     
 
#### Gas Report  - Savings: ~15 
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
 
--- 

[File:ConveyorRouterV1.sol#L362](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L362) 
```solidity
361:        _safeTransferETH(msg.sender, address(this).balance);
``` 



[File:ConveyorRouterV1.sol#L363](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L363) 
```solidity
362:        emit Withdraw(msg.sender, address(this).balance);
``` 



 --- 


 ### <a name=[G-16]></a> [G-16] Use assembly to check for address(0) - Instances: 7 

 
  
 
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

[File:LimitOrderQuoter.sol#L17](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L17) 
```solidity
16:        require(_weth != address(0), "Invalid weth address");
``` 



[File:SandboxLimitOrderBook.sol#L94](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L94) 
```solidity
93:        require(_limitOrderExecutor != address(0), "limitOrderExecutor address is address(0)");
``` 



[File:SandboxLimitOrderBook.sol#L1268](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L1268) 
```solidity
1267:        if (newOwner == address(0)) {
``` 



[File:ConveyorRouterV1.sol#L64](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L64) 
```solidity
63:        require(_weth != address(0), "WETH address is zero");
``` 



[File:ConveyorRouterV1.sol#L137](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L137) 
```solidity
136:            if (affiliate == address(0)) {
``` 



[File:ConveyorRouterV1.sol#L145](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L145) 
```solidity
144:            if (referrer == address(0)) {
``` 



[File:ConveyorRouterV1.sol#L190](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L190) 
```solidity
189:            if (affiliate == address(0)) {
``` 



[File:ConveyorRouterV1.sol#L198](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L198) 
```solidity
197:            if (referrer == address(0)) {
``` 



[File:ConveyorRouterV1.sol#L213](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L213) 
```solidity
212:        if (swapAggregatorMulticall.tokenInDestination != address(0)) {
``` 



[File:ConveyorRouterV1.sol#L243](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L243) 
```solidity
242:            if (affiliate == address(0)) {
``` 



[File:ConveyorRouterV1.sol#L251](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L251) 
```solidity
250:            if (referrer == address(0)) {
``` 



[File:ConveyorRouterV1.sol#L379](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L379) 
```solidity
378:        if (newOwner == address(0)) {
``` 



[File:LimitOrderBook.sol#L43](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L43) 
```solidity
42:        require(_limitOrderExecutor != address(0), "limitOrderExecutor address is address(0)");
``` 



[File:LimitOrderSwapRouter.sol#L132](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L132) 
```solidity
131:            require(_dexFactories[i] != address(0), "Zero values in constructor");
``` 



[File:LimitOrderSwapRouter.sol#L433](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L433) 
```solidity
432:        if (address(0) == pairAddress) {
``` 



[File:LimitOrderSwapRouter.sol#L500](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L500) 
```solidity
499:        if (pool == address(0)) {
``` 



[File:LimitOrderSwapRouter.sol#L541](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L541) 
```solidity
540:        if (token0 == address(0)) {
``` 



[File:ConveyorExecutor.sol#L119](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L119) 
```solidity
118:        require(_weth != address(0), "Invalid weth address");
``` 



[File:ConveyorExecutor.sol#L120](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L120) 
```solidity
119:        require(_usdc != address(0), "Invalid usdc address");
``` 



[File:ConveyorExecutor.sol#L121](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L121) 
```solidity
120:        require(_limitOrderQuoterAddress != address(0), "Invalid LimitOrderQuoter address");
``` 



[File:ConveyorExecutor.sol#L530](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L530) 
```solidity
529:        if (newOwner == address(0)) {
``` 



[File:LimitOrderRouter.sol#L70](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L70) 
```solidity
69:        require(_limitOrderExecutor != address(0), "Invalid ConveyorExecutor address");
``` 



[File:LimitOrderRouter.sol#L360](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L360) 
```solidity
359:        if (newOwner == address(0)) {
``` 



 --- 


 ### <a name=[G-17]></a> [G-17] Cache array length during for loop definition. - Instances: 8 

 
 
> A typical for loop definition may look like: `for (uint256 i; i < arr.length; i++){}`. Instead of using `array.length`, cache the array length before the loop, and use the cached value to safe gas. This will avoid an `MLOAD` every loop for arrays stored in memory and an `SLOAD` for arrays stored in storage. This can have significant gas savings for arrays with a large length, especially if the array is stored in storage. 
 
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

[File:LimitOrderRouter.sol#L92](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L92) 
```solidity
91:        for (uint256 i = 0; i < orderIds.length;) {
``` 



[File:LimitOrderRouter.sol#L211](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L211) 
```solidity
210:        for (uint256 i = 0; i < orders.length - 1;) {
``` 



[File:LimitOrderRouter.sol#L284](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L284) 
```solidity
283:        for (uint256 i = 0; i < orderIds.length;) {
``` 



[File:LimitOrderRouter.sol#L324](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L324) 
```solidity
323:        for (uint256 i = 0; i < orderIds.length;) {
``` 



[File:LimitOrderRouter.sol#L339](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L339) 
```solidity
338:        for (uint256 i = 0; i < orders.length;) {
``` 



[File:ConveyorExecutor.sol#L183](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L183) 
```solidity
182:        for (uint256 i = 0; i < orders.length;) {
``` 



[File:ConveyorExecutor.sol#L309](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L309) 
```solidity
308:        for (uint256 i = 0; i < orders.length;) {
``` 



[File:ConveyorExecutor.sol#L432](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L432) 
```solidity
431:            for (uint256 i = 0; i < orders.length;) {
``` 



[File:ConveyorExecutor.sol#L453](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L453) 
```solidity
452:            for (uint256 i = 0; i < orders.length;) {
``` 



[File:ConveyorRouterV1.sol#L452](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L452) 
```solidity
451:        for (uint256 i = 0; i < multicall.calls.length;) {
``` 



[File:SandboxLimitOrderRouter.sol#L77](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderRouter.sol#L77) 
```solidity
76:        for (uint256 i = 0; i < sandboxMulticall.calls.length;) {
``` 



[File:LimitOrderQuoter.sol#L59](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L59) 
```solidity
58:            for (uint256 i = 0; i < executionPrices.length;) {
``` 



[File:LimitOrderQuoter.sol#L75](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L75) 
```solidity
74:            for (uint256 i = 0; i < executionPrices.length;) {
``` 



[File:LimitOrderQuoter.sol#L103](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L103) 
```solidity
102:            for (uint256 i = 0; i < executionPrices.length;) {
``` 



[File:LimitOrderQuoter.sol#L117](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L117) 
```solidity
116:            for (uint256 i = 0; i < executionPrices.length;) {
``` 



[File:LimitOrderQuoter.sol#L148](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L148) 
```solidity
147:            for (uint256 i = 0; i < spotReserveAToWeth.length;) {
``` 



[File:LimitOrderQuoter.sol#L187](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L187) 
```solidity
186:            for (uint256 i = 0; i < spotReserveWethToB.length;) {
``` 



[File:LimitOrderQuoter.sol#L207](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L207) 
```solidity
206:            for (uint256 i = 0; i < spotReserveAToWeth.length;) {
``` 



[File:LimitOrderQuoter.sol#L209](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L209) 
```solidity
208:                for (uint256 j = 0; j < spotReserveWethToB.length;) {
``` 



[File:SandboxLimitOrderBook.sol#L317](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L317) 
```solidity
316:        for (uint256 i = 0; i < orderGroup.length;) {
``` 



[File:SandboxLimitOrderBook.sol#L331](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L331) 
```solidity
330:                    for (uint256 k = 0; k < spRes.length;) {
``` 



[File:SandboxLimitOrderBook.sol#L513](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L513) 
```solidity
512:        for (uint256 i = 0; i < orderIds.length;) {
``` 



[File:SandboxLimitOrderBook.sol#L634](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L634) 
```solidity
633:        for (uint256 i = 0; i < orderIds.length;) {
``` 



[File:SandboxLimitOrderBook.sol#L750](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L750) 
```solidity
749:            for (uint256 i = 0; i < orderIdBundles.length;) {
``` 



[File:SandboxLimitOrderBook.sol#L753](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L753) 
```solidity
752:                for (uint256 j = 0; j < orderIdBundle.length;) {
``` 



[File:SandboxLimitOrderBook.sol#L815](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L815) 
```solidity
814:        for (uint256 i = 0; i < orderIdBundles.length;) {
``` 



[File:LimitOrderSwapRouter.sol#L128](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L128) 
```solidity
127:        for (uint256 i = 0; i < _dexFactories.length; ++i) {
``` 



[File:LimitOrderSwapRouter.sol#L589](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L589) 
```solidity
588:            for (uint256 i = 0; i < dexes.length;) {
``` 



[File:LimitOrderBook.sol#L281](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L281) 
```solidity
280:        for (uint256 i = 0; i < orderGroup.length;) {
``` 



[File:LimitOrderBook.sol#L485](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L485) 
```solidity
484:        for (uint256 i = 0; i < orderIds.length;) {
``` 



 --- 



## Quality Assurance - Total: 10 


 ### <a name=[NC-0]></a> [NC-0] Constructor should be listed before any other function - Instances: 1 

 Description of the qa pattern goes here 
--- 

[File:ConveyorRouterV1.sol#L63](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L63) 
```solidity
62:    constructor(address _weth) payable {
``` 



 --- 


 ### <a name=[NC-1]></a> [NC-1] Private variables should contain a leading underscore - Instances: 1 

 Description of the qa pattern goes here 
--- 

[File:ConveyorRouterV1.sol#L437](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L437) 
```solidity
436:    bool private locked;
``` 



 --- 


 ### <a name=[NC-2]></a> [NC-2] Constructor should initialize all variables - Instances: 13 

 Description of the qa pattern goes here 
--- 

[File:SandboxLimitOrderBook.sol#L93](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L93) 
```solidity
92:    constructor(address _limitOrderExecutor, address _weth, address _usdc, uint256 _minExecutionCredit) {
``` 



[File:SandboxLimitOrderBook.sol#L93](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L93) 
```solidity
92:    constructor(address _limitOrderExecutor, address _weth, address _usdc, uint256 _minExecutionCredit) {
``` 



[File:LimitOrderSwapRouter.sol#L126](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L126) 
```solidity
125:    constructor(address[] memory _dexFactories, bool[] memory _isUniV2) {
``` 



[File:ConveyorExecutor.sol#L111](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L111) 
```solidity
110:    constructor(
111:        address _weth,
112:        address _usdc,
113:        address _limitOrderQuoterAddress,
114:        address[] memory _dexFactories,
115:        bool[] memory _isUniV2,
116:        uint256 _minExecutionCredit
117:    ) LimitOrderSwapRouter(_dexFactories, _isUniV2) {
``` 



[File:ConveyorExecutor.sol#L111](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L111) 
```solidity
110:    constructor(
111:        address _weth,
112:        address _usdc,
113:        address _limitOrderQuoterAddress,
114:        address[] memory _dexFactories,
115:        bool[] memory _isUniV2,
116:        uint256 _minExecutionCredit
117:    ) LimitOrderSwapRouter(_dexFactories, _isUniV2) {
``` 



[File:ConveyorExecutor.sol#L111](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L111) 
```solidity
110:    constructor(
111:        address _weth,
112:        address _usdc,
113:        address _limitOrderQuoterAddress,
114:        address[] memory _dexFactories,
115:        bool[] memory _isUniV2,
116:        uint256 _minExecutionCredit
117:    ) LimitOrderSwapRouter(_dexFactories, _isUniV2) {
``` 



[File:LimitOrderBook.sol#L42](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L42) 
```solidity
41:    constructor(address _limitOrderExecutor, address _weth, address _usdc, uint256 _minExecutionCredit) {
``` 



[File:LimitOrderBook.sol#L42](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L42) 
```solidity
41:    constructor(address _limitOrderExecutor, address _weth, address _usdc, uint256 _minExecutionCredit) {
``` 



[File:LimitOrderRouter.sol#L65](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L65) 
```solidity
64:    constructor(address _weth, address _usdc, address _limitOrderExecutor, uint256 _minExecutionCredit)
65:        LimitOrderBook(_limitOrderExecutor, _weth, _usdc, _minExecutionCredit)
66:    {
``` 



[File:LimitOrderRouter.sol#L65](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L65) 
```solidity
64:    constructor(address _weth, address _usdc, address _limitOrderExecutor, uint256 _minExecutionCredit)
65:        LimitOrderBook(_limitOrderExecutor, _weth, _usdc, _minExecutionCredit)
66:    {
``` 



[File:LimitOrderRouter.sol#L65](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L65) 
```solidity
64:    constructor(address _weth, address _usdc, address _limitOrderExecutor, uint256 _minExecutionCredit)
65:        LimitOrderBook(_limitOrderExecutor, _weth, _usdc, _minExecutionCredit)
66:    {
``` 



[File:SandboxLimitOrderRouter.sol#L54](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderRouter.sol#L54) 
```solidity
53:    constructor(address _limitOrderExecutor, address _sandboxLimitOrderBook) {
``` 



[File:SandboxLimitOrderRouter.sol#L54](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderRouter.sol#L54) 
```solidity
53:    constructor(address _limitOrderExecutor, address _sandboxLimitOrderBook) {
``` 



 --- 


 ### <a name=[NC-3]></a> [NC-3] Consider importing specific identifiers instead of the whole file - Instances: 156 

 This will minimize compiled code size and help with readability 
--- 

[File:ApeSwapCallback.sol#L4](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/ApeSwapCallback.sol#L4) 
```solidity
3:import "../../lib/interfaces/token/IERC20.sol";
``` 



[File:ApeSwapCallback.sol#L5](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/ApeSwapCallback.sol#L5) 
```solidity
4:import "../../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol";
``` 



[File:ApeSwapCallback.sol#L6](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/ApeSwapCallback.sol#L6) 
```solidity
5:import "../lib/OracleLibraryV2.sol";
``` 



[File:VerseCallback.sol#L4](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/VerseCallback.sol#L4) 
```solidity
3:import "../../lib/interfaces/token/IERC20.sol";
``` 



[File:VerseCallback.sol#L5](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/VerseCallback.sol#L5) 
```solidity
4:import "../../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol";
``` 



[File:VerseCallback.sol#L6](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/VerseCallback.sol#L6) 
```solidity
5:import "../lib/OracleLibraryV2.sol";
``` 



[File:ILimitOrderQuoter.sol#L4](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/interfaces/ILimitOrderQuoter.sol#L4) 
```solidity
3:import "../LimitOrderSwapRouter.sol";
``` 



[File:SandboxLimitOrderBook.sol#L4](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L4) 
```solidity
3:import "./ConveyorErrors.sol";
``` 



[File:SandboxLimitOrderBook.sol#L5](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L5) 
```solidity
4:import "../lib/interfaces/token/IERC20.sol";
``` 



[File:SandboxLimitOrderBook.sol#L6](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L6) 
```solidity
5:import "./interfaces/ILimitOrderBook.sol";
``` 



[File:SandboxLimitOrderBook.sol#L7](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L7) 
```solidity
6:import "./interfaces/ILimitOrderSwapRouter.sol";
``` 



[File:SandboxLimitOrderBook.sol#L8](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L8) 
```solidity
7:import "./LimitOrderSwapRouter.sol";
``` 



[File:SandboxLimitOrderBook.sol#L9](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L9) 
```solidity
8:import "./lib/ConveyorMath.sol";
``` 



[File:SandboxLimitOrderBook.sol#L10](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L10) 
```solidity
9:import "./interfaces/IConveyorExecutor.sol";
``` 



[File:SandboxLimitOrderBook.sol#L11](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L11) 
```solidity
10:import "./SandboxLimitOrderRouter.sol";
``` 



[File:JetSwapCallback.sol#L4](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/JetSwapCallback.sol#L4) 
```solidity
3:import "../../lib/interfaces/token/IERC20.sol";
``` 



[File:JetSwapCallback.sol#L5](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/JetSwapCallback.sol#L5) 
```solidity
4:import "../../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol";
``` 



[File:JetSwapCallback.sol#L6](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/JetSwapCallback.sol#L6) 
```solidity
5:import "../lib/OracleLibraryV2.sol";
``` 



[File:ConveyorTickMath.sol#L4](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorTickMath.sol#L4) 
```solidity
3:import "../../lib/libraries/Uniswap/FullMath.sol";
``` 



[File:ConveyorTickMath.sol#L5](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorTickMath.sol#L5) 
```solidity
4:import "../../lib/libraries/Uniswap/LowGasSafeMath.sol";
``` 



[File:ConveyorTickMath.sol#L6](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorTickMath.sol#L6) 
```solidity
5:import "../../lib/libraries/Uniswap/SafeCast.sol";
``` 



[File:ConveyorTickMath.sol#L7](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorTickMath.sol#L7) 
```solidity
6:import "../../lib/libraries/Uniswap/SqrtPriceMath.sol";
``` 



[File:ConveyorTickMath.sol#L8](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorTickMath.sol#L8) 
```solidity
7:import "../../lib/libraries/Uniswap/TickMath.sol";
``` 



[File:ConveyorTickMath.sol#L9](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorTickMath.sol#L9) 
```solidity
8:import "../../lib/libraries/Uniswap/TickBitmap.sol";
``` 



[File:ConveyorTickMath.sol#L10](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorTickMath.sol#L10) 
```solidity
9:import "../../lib/libraries/Uniswap/SwapMath.sol";
``` 



[File:ConveyorTickMath.sol#L11](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorTickMath.sol#L11) 
```solidity
10:import "../../lib/interfaces/uniswap-v3/IUniswapV3Pool.sol";
``` 



[File:ConveyorTickMath.sol#L12](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorTickMath.sol#L12) 
```solidity
11:import "../../lib/libraries/Uniswap/LowGasSafeMath.sol";
``` 



[File:ConveyorTickMath.sol#L13](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorTickMath.sol#L13) 
```solidity
12:import "../../lib/libraries/Uniswap/LiquidityMath.sol";
``` 



[File:ConveyorTickMath.sol#L14](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorTickMath.sol#L14) 
```solidity
13:import "../../lib/libraries/Uniswap/Tick.sol";
``` 



[File:ConveyorTickMath.sol#L15](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorTickMath.sol#L15) 
```solidity
14:import "../../lib/libraries/Uniswap/SafeCast.sol";
``` 



[File:ConveyorTickMath.sol#L16](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorTickMath.sol#L16) 
```solidity
15:import "../../lib/interfaces/token/IERC20.sol";
``` 



[File:BabySwapCallback.sol#L4](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/BabySwapCallback.sol#L4) 
```solidity
3:import "../../lib/interfaces/token/IERC20.sol";
``` 



[File:BabySwapCallback.sol#L5](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/BabySwapCallback.sol#L5) 
```solidity
4:import "../../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol";
``` 



[File:BabySwapCallback.sol#L6](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/BabySwapCallback.sol#L6) 
```solidity
5:import "../lib/OracleLibraryV2.sol";
``` 



[File:ISandboxLimitOrderBook.sol#L4](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/interfaces/ISandboxLimitOrderBook.sol#L4) 
```solidity
3:import "../SandboxLimitOrderRouter.sol";
``` 



[File:ISandboxLimitOrderBook.sol#L5](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/interfaces/ISandboxLimitOrderBook.sol#L5) 
```solidity
4:import "../SandboxLimitOrderBook.sol";
``` 



[File:KyberSwapV3Callback.sol#L4](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/KyberSwapV3Callback.sol#L4) 
```solidity
3:import "../../lib/interfaces/token/IERC20.sol";
``` 



[File:CafeSwapCallback.sol#L4](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/CafeSwapCallback.sol#L4) 
```solidity
3:import "../../lib/interfaces/token/IERC20.sol";
``` 



[File:CafeSwapCallback.sol#L5](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/CafeSwapCallback.sol#L5) 
```solidity
4:import "../../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol";
``` 



[File:CafeSwapCallback.sol#L6](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/CafeSwapCallback.sol#L6) 
```solidity
5:import "../lib/OracleLibraryV2.sol";
``` 



[File:UniswapV2Callback.sol#L4](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/UniswapV2Callback.sol#L4) 
```solidity
3:import "../../lib/interfaces/token/IERC20.sol";
``` 



[File:UniswapV2Callback.sol#L5](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/UniswapV2Callback.sol#L5) 
```solidity
4:import "../../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol";
``` 



[File:UniswapV2Callback.sol#L6](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/UniswapV2Callback.sol#L6) 
```solidity
5:import "../lib/OracleLibraryV2.sol";
``` 



[File:PancakeV2Callback.sol#L4](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/PancakeV2Callback.sol#L4) 
```solidity
3:import "../../lib/interfaces/token/IERC20.sol";
``` 



[File:PancakeV2Callback.sol#L5](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/PancakeV2Callback.sol#L5) 
```solidity
4:import "../../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol";
``` 



[File:PancakeV2Callback.sol#L6](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/PancakeV2Callback.sol#L6) 
```solidity
5:import "../lib/OracleLibraryV2.sol";
``` 



[File:MdexSwapCallback.sol#L4](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/MdexSwapCallback.sol#L4) 
```solidity
3:import "../../lib/interfaces/token/IERC20.sol";
``` 



[File:MdexSwapCallback.sol#L5](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/MdexSwapCallback.sol#L5) 
```solidity
4:import "../../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol";
``` 



[File:MdexSwapCallback.sol#L6](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/MdexSwapCallback.sol#L6) 
```solidity
5:import "../lib/OracleLibraryV2.sol";
``` 



[File:SandboxLimitOrderRouter.sol#L4](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderRouter.sol#L4) 
```solidity
3:import "../lib/interfaces/token/IERC20.sol";
``` 



[File:SandboxLimitOrderRouter.sol#L5](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderRouter.sol#L5) 
```solidity
4:import "./ConveyorErrors.sol";
``` 



[File:SandboxLimitOrderRouter.sol#L6](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderRouter.sol#L6) 
```solidity
5:import "./interfaces/ISandboxLimitOrderBook.sol";
``` 



[File:SandboxLimitOrderRouter.sol#L7](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderRouter.sol#L7) 
```solidity
6:import "../lib/libraries/token/SafeERC20.sol";
``` 



[File:SandboxLimitOrderRouter.sol#L8](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderRouter.sol#L8) 
```solidity
7:import "./interfaces/ISandboxLimitOrderRouter.sol";
``` 



[File:SandboxLimitOrderRouter.sol#L9](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderRouter.sol#L9) 
```solidity
8:import "./interfaces/IConveyorExecutor.sol";
``` 



[File:DXSwapCallback.sol#L4](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/DXSwapCallback.sol#L4) 
```solidity
3:import "../../lib/interfaces/token/IERC20.sol";
``` 



[File:DXSwapCallback.sol#L5](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/DXSwapCallback.sol#L5) 
```solidity
4:import "../../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol";
``` 



[File:DXSwapCallback.sol#L6](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/DXSwapCallback.sol#L6) 
```solidity
5:import "../lib/OracleLibraryV2.sol";
``` 



[File:DefiSwapCallback.sol#L4](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/DefiSwapCallback.sol#L4) 
```solidity
3:import "../../lib/interfaces/token/IERC20.sol";
``` 



[File:DefiSwapCallback.sol#L5](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/DefiSwapCallback.sol#L5) 
```solidity
4:import "../../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol";
``` 



[File:DefiSwapCallback.sol#L6](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/DefiSwapCallback.sol#L6) 
```solidity
5:import "../lib/OracleLibraryV2.sol";
``` 



[File:BabyDogeCallback.sol#L4](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/BabyDogeCallback.sol#L4) 
```solidity
3:import "../../lib/interfaces/token/IERC20.sol";
``` 



[File:BabyDogeCallback.sol#L5](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/BabyDogeCallback.sol#L5) 
```solidity
4:import "../../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol";
``` 



[File:BabyDogeCallback.sol#L6](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/BabyDogeCallback.sol#L6) 
```solidity
5:import "../lib/OracleLibraryV2.sol";
``` 



[File:NomiswapCallback.sol#L4](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/NomiswapCallback.sol#L4) 
```solidity
3:import "../../lib/interfaces/token/IERC20.sol";
``` 



[File:NomiswapCallback.sol#L5](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/NomiswapCallback.sol#L5) 
```solidity
4:import "../../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol";
``` 



[File:NomiswapCallback.sol#L6](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/NomiswapCallback.sol#L6) 
```solidity
5:import "../lib/OracleLibraryV2.sol";
``` 



[File:ElkSwapCallback.sol#L4](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/ElkSwapCallback.sol#L4) 
```solidity
3:import "../../lib/interfaces/token/IERC20.sol";
``` 



[File:ElkSwapCallback.sol#L5](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/ElkSwapCallback.sol#L5) 
```solidity
4:import "../../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol";
``` 



[File:ElkSwapCallback.sol#L6](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/ElkSwapCallback.sol#L6) 
```solidity
5:import "../lib/OracleLibraryV2.sol";
``` 



[File:UniFiCallback.sol#L4](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/UniFiCallback.sol#L4) 
```solidity
3:import "../../lib/interfaces/token/IERC20.sol";
``` 



[File:UniFiCallback.sol#L5](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/UniFiCallback.sol#L5) 
```solidity
4:import "../../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol";
``` 



[File:UniFiCallback.sol#L6](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/UniFiCallback.sol#L6) 
```solidity
5:import "../lib/OracleLibraryV2.sol";
``` 



[File:ConveyorMath.sol#L4](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L4) 
```solidity
3:import "../../lib/libraries/Uniswap/FullMath.sol";
``` 



[File:LinkSwapCallback.sol#L4](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/LinkSwapCallback.sol#L4) 
```solidity
3:import "../../lib/interfaces/token/IERC20.sol";
``` 



[File:LinkSwapCallback.sol#L5](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/LinkSwapCallback.sol#L5) 
```solidity
4:import "../../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol";
``` 



[File:LinkSwapCallback.sol#L6](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/LinkSwapCallback.sol#L6) 
```solidity
5:import "../lib/OracleLibraryV2.sol";
``` 



[File:PancakeV3Callback.sol#L4](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/PancakeV3Callback.sol#L4) 
```solidity
3:import "../../lib/interfaces/token/IERC20.sol";
``` 



[File:DeployTest.s.sol#L7](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/deploy/DeployTest.s.sol#L7) 
```solidity
6:import "../../test/utils/Console.sol";
``` 



[File:LimitOrderSwapRouter.sol#L4](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L4) 
```solidity
3:import "../lib/interfaces/token/IERC20.sol";
``` 



[File:LimitOrderSwapRouter.sol#L5](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L5) 
```solidity
4:import "../lib/interfaces/uniswap-v2/IUniswapV2Factory.sol";
``` 



[File:LimitOrderSwapRouter.sol#L6](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L6) 
```solidity
5:import "../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol";
``` 



[File:LimitOrderSwapRouter.sol#L7](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L7) 
```solidity
6:import "../lib/interfaces/uniswap-v3/IUniswapV3Factory.sol";
``` 



[File:LimitOrderSwapRouter.sol#L8](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L8) 
```solidity
7:import "../lib/interfaces/uniswap-v3/IUniswapV3Pool.sol";
``` 



[File:LimitOrderSwapRouter.sol#L9](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L9) 
```solidity
8:import "./lib/ConveyorMath.sol";
``` 



[File:LimitOrderSwapRouter.sol#L10](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L10) 
```solidity
9:import "./LimitOrderBook.sol";
``` 



[File:LimitOrderSwapRouter.sol#L11](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L11) 
```solidity
10:import "./lib/ConveyorTickMath.sol";
``` 



[File:LimitOrderSwapRouter.sol#L12](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L12) 
```solidity
11:import "../lib/libraries/Uniswap/FullMath.sol";
``` 



[File:LimitOrderSwapRouter.sol#L13](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L13) 
```solidity
12:import "../lib/libraries/Uniswap/FixedPoint96.sol";
``` 



[File:LimitOrderSwapRouter.sol#L14](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L14) 
```solidity
13:import "../lib/libraries/Uniswap/TickMath.sol";
``` 



[File:LimitOrderSwapRouter.sol#L15](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L15) 
```solidity
14:import "../lib/interfaces/token/IWETH.sol";
``` 



[File:LimitOrderSwapRouter.sol#L16](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L16) 
```solidity
15:import "./lib/ConveyorFeeMath.sol";
``` 



[File:LimitOrderSwapRouter.sol#L17](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L17) 
```solidity
16:import "../lib/libraries/Uniswap/SqrtPriceMath.sol";
``` 



[File:LimitOrderSwapRouter.sol#L18](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L18) 
```solidity
17:import "../lib/interfaces/uniswap-v3/IQuoter.sol";
``` 



[File:LimitOrderSwapRouter.sol#L19](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L19) 
```solidity
18:import "../lib/libraries/token/SafeERC20.sol";
``` 



[File:LimitOrderSwapRouter.sol#L20](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L20) 
```solidity
19:import "./ConveyorErrors.sol";
``` 



[File:LimitOrderSwapRouter.sol#L21](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L21) 
```solidity
20:import "./interfaces/ILimitOrderSwapRouter.sol";
``` 



[File:IConveyorRouterV1.sol#L4](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/interfaces/IConveyorRouterV1.sol#L4) 
```solidity
3:import "../ConveyorRouterV1.sol";
``` 



[File:MeerkatCallback.sol#L4](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/MeerkatCallback.sol#L4) 
```solidity
3:import "../../lib/interfaces/token/IERC20.sol";
``` 



[File:MeerkatCallback.sol#L5](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/MeerkatCallback.sol#L5) 
```solidity
4:import "../../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol";
``` 



[File:MeerkatCallback.sol#L6](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/MeerkatCallback.sol#L6) 
```solidity
5:import "../lib/OracleLibraryV2.sol";
``` 



[File:DystopiaCallback.sol#L4](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/DystopiaCallback.sol#L4) 
```solidity
3:import "../../lib/interfaces/token/IERC20.sol";
``` 



[File:DystopiaCallback.sol#L5](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/DystopiaCallback.sol#L5) 
```solidity
4:import "../../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol";
``` 



[File:DystopiaCallback.sol#L6](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/DystopiaCallback.sol#L6) 
```solidity
5:import "../lib/OracleLibraryV2.sol";
``` 



[File:BiswapCallback.sol#L4](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/BiswapCallback.sol#L4) 
```solidity
3:import "../../lib/interfaces/token/IERC20.sol";
``` 



[File:BiswapCallback.sol#L5](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/BiswapCallback.sol#L5) 
```solidity
4:import "../../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol";
``` 



[File:BiswapCallback.sol#L6](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/BiswapCallback.sol#L6) 
```solidity
5:import "../lib/OracleLibraryV2.sol";
``` 



[File:LimitOrderRouter.sol#L4](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L4) 
```solidity
3:import "../lib/interfaces/token/IERC20.sol";
``` 



[File:LimitOrderRouter.sol#L5](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L5) 
```solidity
4:import "./LimitOrderBook.sol";
``` 



[File:LimitOrderRouter.sol#L6](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L6) 
```solidity
5:import "./ConveyorErrors.sol";
``` 



[File:LimitOrderRouter.sol#L7](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L7) 
```solidity
6:import "../lib/interfaces/token/IWETH.sol";
``` 



[File:LimitOrderRouter.sol#L8](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L8) 
```solidity
7:import "./LimitOrderSwapRouter.sol";
``` 



[File:LimitOrderRouter.sol#L9](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L9) 
```solidity
8:import "./interfaces/ILimitOrderQuoter.sol";
``` 



[File:LimitOrderRouter.sol#L10](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L10) 
```solidity
9:import "./interfaces/IConveyorExecutor.sol";
``` 



[File:LimitOrderRouter.sol#L11](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L11) 
```solidity
10:import "./interfaces/ILimitOrderRouter.sol";
``` 



[File:DeployBSCAggregator.s.sol#L7](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/deploy/DeployBSCAggregator.s.sol#L7) 
```solidity
6:import "../../test/utils/Console.sol";
``` 



[File:ConveyorExecutor.sol#L4](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L4) 
```solidity
3:import "./LimitOrderSwapRouter.sol";
``` 



[File:ConveyorExecutor.sol#L5](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L5) 
```solidity
4:import "./interfaces/ILimitOrderQuoter.sol";
``` 



[File:ConveyorExecutor.sol#L6](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L6) 
```solidity
5:import "./lib/ConveyorFeeMath.sol";
``` 



[File:ConveyorExecutor.sol#L7](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L7) 
```solidity
6:import "./LimitOrderRouter.sol";
``` 



[File:ConveyorExecutor.sol#L8](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L8) 
```solidity
7:import "./interfaces/ILimitOrderSwapRouter.sol";
``` 



[File:ConveyorExecutor.sol#L9](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L9) 
```solidity
8:import "./interfaces/ISandboxLimitOrderRouter.sol";
``` 



[File:ConveyorExecutor.sol#L10](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L10) 
```solidity
9:import "./interfaces/ISandboxLimitOrderBook.sol";
``` 



[File:ConveyorExecutor.sol#L11](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L11) 
```solidity
10:import "./interfaces/ILimitOrderBook.sol";
``` 



[File:ConveyorExecutor.sol#L12](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L12) 
```solidity
11:import "./interfaces/IConveyorExecutor.sol";
``` 



[File:LimitOrderBook.sol#L4](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L4) 
```solidity
3:import "../lib/interfaces/token/IERC20.sol";
``` 



[File:LimitOrderBook.sol#L5](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L5) 
```solidity
4:import "./ConveyorErrors.sol";
``` 



[File:LimitOrderBook.sol#L6](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L6) 
```solidity
5:import "./interfaces/ILimitOrderSwapRouter.sol";
``` 



[File:LimitOrderBook.sol#L7](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L7) 
```solidity
6:import "./lib/ConveyorMath.sol";
``` 



[File:LimitOrderBook.sol#L8](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L8) 
```solidity
7:import "./interfaces/IConveyorExecutor.sol";
``` 



[File:WaultSwapCallback.sol#L4](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/WaultSwapCallback.sol#L4) 
```solidity
3:import "../../lib/interfaces/token/IERC20.sol";
``` 



[File:WaultSwapCallback.sol#L5](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/WaultSwapCallback.sol#L5) 
```solidity
4:import "../../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol";
``` 



[File:WaultSwapCallback.sol#L6](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/WaultSwapCallback.sol#L6) 
```solidity
5:import "../lib/OracleLibraryV2.sol";
``` 



[File:LimitOrderQuoter.sol#L4](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L4) 
```solidity
3:import "./LimitOrderSwapRouter.sol";
``` 



[File:LimitOrderQuoter.sol#L5](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L5) 
```solidity
4:import "./lib/ConveyorTickMath.sol";
``` 



[File:LimitOrderQuoter.sol#L6](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderQuoter.sol#L6) 
```solidity
5:import "./interfaces/ILimitOrderQuoter.sol";
``` 



[File:AlgebraCallback.sol#L4](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/AlgebraCallback.sol#L4) 
```solidity
3:import "../../lib/interfaces/token/IERC20.sol";
``` 



[File:TraderJoeCallback.sol#L4](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/TraderJoeCallback.sol#L4) 
```solidity
3:import "../../lib/interfaces/token/IERC20.sol";
``` 



[File:TraderJoeCallback.sol#L5](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/TraderJoeCallback.sol#L5) 
```solidity
4:import "../../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol";
``` 



[File:TraderJoeCallback.sol#L6](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/TraderJoeCallback.sol#L6) 
```solidity
5:import "../lib/OracleLibraryV2.sol";
``` 



[File:ConveyorFeeMath.sol#L4](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorFeeMath.sol#L4) 
```solidity
3:import "./ConveyorMath.sol";
``` 



[File:ConveyorFeeMath.sol#L5](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorFeeMath.sol#L5) 
```solidity
4:import "../../lib/libraries/QuadruplePrecision.sol";
``` 



[File:SakeSwapCallback.sol#L4](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/SakeSwapCallback.sol#L4) 
```solidity
3:import "../../lib/interfaces/token/IERC20.sol";
``` 



[File:SakeSwapCallback.sol#L5](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/SakeSwapCallback.sol#L5) 
```solidity
4:import "../../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol";
``` 



[File:SakeSwapCallback.sol#L6](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/SakeSwapCallback.sol#L6) 
```solidity
5:import "../lib/OracleLibraryV2.sol";
``` 



[File:ConveyorRouterV1.sol#L4](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L4) 
```solidity
3:import "./ConveyorErrors.sol";
``` 



[File:ISandboxLimitOrderRouter.sol#L4](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/interfaces/ISandboxLimitOrderRouter.sol#L4) 
```solidity
3:import "../SandboxLimitOrderRouter.sol";
``` 



[File:ILimitOrderSwapRouter.sol#L4](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/interfaces/ILimitOrderSwapRouter.sol#L4) 
```solidity
3:import "../LimitOrderSwapRouter.sol";
``` 



[File:IConveyorExecutor.sol#L4](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/interfaces/IConveyorExecutor.sol#L4) 
```solidity
3:import "../LimitOrderBook.sol";
``` 



[File:IConveyorExecutor.sol#L5](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/interfaces/IConveyorExecutor.sol#L5) 
```solidity
4:import "../SandboxLimitOrderBook.sol";
``` 



[File:IConveyorExecutor.sol#L6](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/interfaces/IConveyorExecutor.sol#L6) 
```solidity
5:import "../SandboxLimitOrderRouter.sol";
``` 



[File:ILimitOrderBook.sol#L4](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/interfaces/ILimitOrderBook.sol#L4) 
```solidity
3:import "../LimitOrderBook.sol";
``` 



[File:ConvergenceXCallback.sol#L4](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/ConvergenceXCallback.sol#L4) 
```solidity
3:import "../../lib/interfaces/token/IERC20.sol";
``` 



[File:ConvergenceXCallback.sol#L5](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/ConvergenceXCallback.sol#L5) 
```solidity
4:import "../../lib/interfaces/uniswap-v2/IUniswapV2Pair.sol";
``` 



[File:ConvergenceXCallback.sol#L6](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/ConvergenceXCallback.sol#L6) 
```solidity
5:import "../lib/OracleLibraryV2.sol";
``` 



[File:UniswapV3Callback.sol#L4](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/callbacks/UniswapV3Callback.sol#L4) 
```solidity
3:import "../../lib/interfaces/token/IERC20.sol";
``` 



 --- 


 ### <a name=[NC-4]></a> [NC-4] Constants & Immutables should be named with screaming snake case - Instances: 6 

 Consider renaming to follow convention 
--- 

[File:ConveyorMath.sol#L8](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L8) 
```solidity
7:    uint128 private constant MAX_64x64 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF;
``` 



[File:ConveyorMath.sol#L13](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L13) 
```solidity
12:    int128 private constant MIN_64x64 = -0x80000000000000000000000000000000;
``` 



[File:ConveyorMath.sol#L16](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L16) 
```solidity
15:    uint256 private constant MAX_128x128 = 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff;
``` 



[File:ConveyorTickMath.sol#L28](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorTickMath.sol#L28) 
```solidity
27:    uint128 private constant MAX_64x64 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF;
``` 



[File:LimitOrderSwapRouter.sol#L101](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L101) 
```solidity
100:    uint128 private constant MIN_FEE_64x64 = 18446744073709552;
``` 



[File:LimitOrderSwapRouter.sol#L105](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L105) 
```solidity
104:    uint256 private constant ONE_128x128 = uint256(1) << 128;
``` 



 --- 


 ### <a name=[NC-5]></a> [NC-5] Consider using scientific notation for large multiples of 10 - Instances: 17 

 For example 100000 can be written as 1e5 
--- 

[File:LimitOrderRouter.sol#L49](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L49) 
```solidity
48:    uint256 private constant REFRESH_INTERVAL = 2592000;
``` 



[File:LimitOrderRouter.sol#L53](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L53) 
```solidity
52:    uint256 private constant REFRESH_FEE = 20000000000000000;
``` 



[File:ConveyorRouterV1.sol#L27](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L27) 
```solidity
26:    uint128 private constant AFFILIATE_PERCENT = 5534023222112865000;
``` 



[File:ConveyorRouterV1.sol#L28](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L28) 
```solidity
27:    uint128 private constant REFERRAL_PERCENT = 5534023222112865000;
``` 



[File:ConveyorFeeMath.sol#L9](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorFeeMath.sol#L9) 
```solidity
8:    uint128 constant ZERO_POINT_ZERO_ZERO_FIVE = 92233720368547760;
``` 



[File:ConveyorFeeMath.sol#L10](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorFeeMath.sol#L10) 
```solidity
9:    uint128 constant ZERO_POINT_ZERO_ZERO_ONE = 18446744073709550;
``` 



[File:ConveyorFeeMath.sol#L11](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorFeeMath.sol#L11) 
```solidity
10:    uint128 constant MAX_CONVEYOR_PERCENT = 110680464442257300 * 10 ** 2;
``` 



[File:ConveyorFeeMath.sol#L12](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorFeeMath.sol#L12) 
```solidity
11:    uint128 constant MIN_CONVEYOR_PERCENT = 7378697629483821000;
``` 



[File:SandboxLimitOrderBook.sol#L33](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L33) 
```solidity
32:    uint256 private constant REFRESH_INTERVAL = 2592000;
``` 



[File:SandboxLimitOrderBook.sol#L39](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L39) 
```solidity
38:    uint256 private constant REFRESH_FEE = 20000000000000000;
``` 



[File:LimitOrderSwapRouter.sol#L102](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L102) 
```solidity
101:    uint128 private constant BASE_SWAP_FEE = 55340232221128660;
``` 



[File:LimitOrderSwapRouter.sol#L107](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L107) 
```solidity
106:    uint256 private constant ZERO_POINT_NINE = 16602069666338597000 << 64;
``` 



[File:LimitOrderSwapRouter.sol#L108](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L108) 
```solidity
107:    uint256 private constant ONE_POINT_TWO_FIVE = 23058430092136940000 << 64;
``` 



[File:LimitOrderSwapRouter.sol#L109](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L109) 
```solidity
108:    uint128 private constant ZERO_POINT_ONE = 1844674407370955300;
``` 



[File:LimitOrderSwapRouter.sol#L110](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L110) 
```solidity
109:    uint128 private constant ZERO_POINT_ZERO_ZERO_FIVE = 92233720368547760;
``` 



[File:LimitOrderSwapRouter.sol#L111](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L111) 
```solidity
110:    uint128 private constant ZERO_POINT_ZERO_ZERO_ONE = 18446744073709550;
``` 



[File:ConveyorExecutor.sol#L35](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorExecutor.sol#L35) 
```solidity
34:    uint128 private constant STOP_LOSS_MAX_BEACON_REWARD = 50000000000000000;
``` 



 --- 


 ### <a name=[NC-6]></a> [NC-6] Remove any unused functions - Instances: 28 

 Any functions not used should be removed as best practice. 
--- 

[File:LimitOrderSwapRouter.sol#L214](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L214) 
```solidity
213:    function _transferTokensOutToOwner(address orderOwner, uint256 amount, address tokenOut) internal {
``` 



[File:LimitOrderSwapRouter.sol#L222](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L222) 
```solidity
221:    function _transferBeaconReward(uint256 totalBeaconReward, address executorAddress, address weth) internal {
``` 



[File:LimitOrderSwapRouter.sol#L295](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L295) 
```solidity
294:    function _swap(
295:        address _tokenIn,
296:        address _tokenOut,
297:        address _lp,
298:        uint24 _fee,
299:        uint256 _amountIn,
300:        uint256 _amountOutMin,
301:        address _receiver,
302:        address _sender
303:    ) internal returns (uint256 amountReceived) {
``` 



[File:LimitOrderBook.sol#L496](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L496) 
```solidity
495:    function _removeOrderFromSystem(bytes32 orderId) internal {
``` 



[File:LimitOrderBook.sol#L511](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L511) 
```solidity
510:    function _resolveCompletedOrder(bytes32 orderId) internal {
``` 



[File:ConveyorTickMath.sol#L103](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorTickMath.sol#L103) 
```solidity
102:    function simulateAmountOutOnSqrtPriceX96(
103:        address token0,
104:        address tokenIn,
105:        address pool,
106:        uint256 amountIn,
107:        int24 tickSpacing,
108:        uint128 liquidity,
109:        uint24 fee
110:    ) internal view returns (uint128 amountOut, uint160 sqrtPriceX96) {
``` 



[File:ConveyorTickMath.sol#L69](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorTickMath.sol#L69) 
```solidity
68:    function fromSqrtX96(uint160 sqrtPriceX96, bool token0IsReserve0, address token0, address token1)
69:        internal
70:        view
71:        returns (uint256 priceX128)
72:    {
``` 



[File:ConveyorMath.sol#L31](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L31) 
```solidity
30:    function toUInt64(uint128 x) internal pure returns (uint64) {
``` 



[File:ConveyorMath.sol#L84](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L84) 
```solidity
83:    function sub(int128 x, int128 y) internal pure returns (int128) {
``` 



[File:ConveyorMath.sol#L217](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L217) 
```solidity
216:    function divUU(uint256 x, uint256 y) internal pure returns (uint128) {
``` 



[File:ConveyorMath.sol#L51](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L51) 
```solidity
50:    function from128x128(uint256 x) internal pure returns (uint128) {
``` 



[File:ConveyorMath.sol#L173](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L173) 
```solidity
172:    function abs(int256 x) internal pure returns (int256) {
``` 



[File:ConveyorMath.sol#L72](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L72) 
```solidity
71:    function add64x64(uint128 x, uint128 y) internal pure returns (uint128) {
``` 



[File:ConveyorMath.sol#L106](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L106) 
```solidity
105:    function add128x64(uint256 x, uint128 y) internal pure returns (uint256) {
``` 



[File:ConveyorMath.sol#L162](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L162) 
```solidity
161:    function mul128U(uint256 x, uint256 y) internal pure returns (uint256) {
``` 



[File:ConveyorMath.sol#L183](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L183) 
```solidity
182:    function div64x64(uint128 x, uint128 y) internal pure returns (uint128) {
``` 



[File:ConveyorMath.sol#L198](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L198) 
```solidity
197:    function div128x128(uint256 x, uint256 y) internal pure returns (uint256) {
``` 



[File:ConveyorMath.sol#L288](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L288) 
```solidity
287:    function fromX64ToX16(uint128 x) internal pure returns (uint32) {
``` 



[File:ConveyorMath.sol#L116](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L116) 
```solidity
115:    function mul64x64(uint128 x, uint128 y) internal pure returns (uint128) {
``` 



[File:ConveyorMath.sol#L21](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L21) 
```solidity
20:    function fromUInt256(uint256 x) internal pure returns (uint128) {
``` 



[File:ConveyorMath.sol#L40](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L40) 
```solidity
39:    function fromUInt128(uint128 x) internal pure returns (uint256) {
``` 



[File:ConveyorMath.sol#L507](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L507) 
```solidity
506:    function exp(uint128 x) internal pure returns (uint128) {
``` 



[File:ConveyorMath.sol#L518](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L518) 
```solidity
517:    function sqrtu(uint256 x) internal pure returns (uint128) {
``` 



[File:ConveyorMath.sol#L128](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L128) 
```solidity
127:    function mul128x64(uint256 x, uint128 y) internal pure returns (uint256) {
``` 



[File:ConveyorMath.sol#L141](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L141) 
```solidity
140:    function mul64U(uint128 x, uint256 y) internal pure returns (uint256) {
``` 



[File:ConveyorMath.sol#L62](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L62) 
```solidity
61:    function to128x128(uint128 x) internal pure returns (uint256) {
``` 



[File:ConveyorMath.sol#L96](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorMath.sol#L96) 
```solidity
95:    function add128x128(uint256 x, uint256 y) internal pure returns (uint256) {
``` 



[File:OracleLibraryV2.sol#L5](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/OracleLibraryV2.sol#L5) 
```solidity
4:    function getAmountIn(uint256 amountOut, uint256 reserveIn, uint256 reserveOut, uint24 swapFee)
5:        internal
6:        pure
7:        returns (uint256 amountIn)
8:    {
``` 



 --- 


 ### <a name=[NC-7]></a> [NC-7] Storage variables should be named with camel case - Instances: 1 

 Consider renaming to follow convention 
--- 

[File:ConveyorRouterV1.sol#L21](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L21) 
```solidity
20:    address public CONVEYOR_MULTICALL;
``` 



 --- 


 ### <a name=[NC-8]></a> [NC-8] Remove any unused returns - Instances: 11 

 Either remove the return parameter names, or use them as the returns of the function. 
--- 

[File:LimitOrderSwapRouter.sol#L321](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L321) 
```solidity
320:    function _swapV3(
321:        address _lp,
322:        address _tokenIn,
323:        address _tokenOut,
324:        uint24 _fee,
325:        uint256 _amountIn,
326:        uint256 _amountOutMin,
327:        address _receiver,
328:        address _sender
329:    ) internal returns (uint256 amountReceived) {
``` 



[File:LimitOrderSwapRouter.sol#L577](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L577) 
```solidity
576:    function getAllPrices(address token0, address token1, uint24 FEE)
577:        public
578:        view
579:        returns (SpotReserve[] memory prices, address[] memory lps)
580:    {
``` 



[File:LimitOrderSwapRouter.sol#L577](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L577) 
```solidity
576:    function getAllPrices(address token0, address token1, uint24 FEE)
577:        public
578:        view
579:        returns (SpotReserve[] memory prices, address[] memory lps)
580:    {
``` 



[File:LimitOrderRouter.sol#L111](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L111) 
```solidity
110:    function _refreshLimitOrder(LimitOrder memory order) internal returns (uint256 executorFee) {
``` 



[File:LimitOrderRouter.sol#L153](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderRouter.sol#L153) 
```solidity
152:    function validateAndCancelOrder(bytes32 orderId) external nonReentrant returns (bool success) {
``` 



[File:LimitOrderBook.sol#L537](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L537) 
```solidity
536:    function getTotalOrdersValue(address token) public view returns (uint256 totalOrderValue) {
``` 



[File:SandboxLimitOrderBook.sol#L560](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L560) 
```solidity
559:    function validateAndCancelOrder(bytes32 orderId) external nonReentrant returns (bool success) {
``` 



[File:SandboxLimitOrderBook.sol#L1178](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L1178) 
```solidity
1177:    function getTotalOrdersValue(address token) public view returns (uint256 totalOrderValue) {
``` 



[File:ConveyorRouterV1.sol#L260](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L260) 
```solidity
259:    function quoteSwapExactTokenForToken(
260:        TokenToTokenSwapData calldata swapData,
261:        SwapAggregatorMulticall calldata swapAggregatorMulticall
262:    ) external payable returns (uint256 gasConsumed) {
``` 



[File:ConveyorRouterV1.sol#L276](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L276) 
```solidity
275:    function quoteSwapExactEthForToken(
276:        EthToTokenSwapData calldata swapData,
277:        SwapAggregatorMulticall calldata swapAggregatorMulticall
278:    ) external payable returns (uint256 gasConsumed) {
``` 



[File:ConveyorRouterV1.sol#L292](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/ConveyorRouterV1.sol#L292) 
```solidity
291:    function quoteSwapExactTokenForEth(
292:        TokenToEthSwapData calldata swapData,
293:        SwapAggregatorMulticall calldata swapAggregatorMulticall
294:    ) external payable returns (uint256 gasConsumed) {
``` 



 --- 


 ### <a name=[NC-9]></a> [NC-9] Consider marking public function External - Instances: 23 

 If a public function is never called internally. It is best practice to mark it as external. 
--- 

[File:DeployBaseAggregator.s.sol#L12](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/deploy/DeployBaseAggregator.s.sol#L12) 
```solidity
11:    function run() public returns (address conveyorRouterV1) {
``` 



[File:DeployArbitrumAggregator.s.sol#L11](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/deploy/DeployArbitrumAggregator.s.sol#L11) 
```solidity
10:    function run() public returns (ConveyorRouterV1 conveyorRouterV1) {
``` 



[File:SandboxLimitOrderBook.sol#L511](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L511) 
```solidity
510:    function cancelOrders(bytes32[] calldata orderIds) public {
``` 



[File:SandboxLimitOrderBook.sol#L286](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L286) 
```solidity
285:    function placeSandboxLimitOrder(SandboxLimitOrder[] calldata orderGroup)
286:        public
287:        payable
288:        returns (bytes32[] memory)
289:    {
``` 



[File:SandboxLimitOrderBook.sol#L1187](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L1187) 
```solidity
1186:    function getSandboxLimitOrderRouterAddress() public view returns (address) {
``` 



[File:SandboxLimitOrderBook.sol#L1183](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L1183) 
```solidity
1182:    function getAllOrderIdsLength(address orderOwner) public view returns (uint256) {
``` 



[File:SandboxLimitOrderBook.sol#L1206](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/SandboxLimitOrderBook.sol#L1206) 
```solidity
1205:    function getOrderIds(address orderOwner, OrderType targetOrderType, uint256 orderOffset, uint256 length)
1206:        public
1207:        view
1208:        returns (bytes32[] memory)
1209:    {
``` 



[File:ConveyorFeeMath.sol#L19](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/lib/ConveyorFeeMath.sol#L19) 
```solidity
18:    function calculateReward(uint128 percentFee, uint128 wethValue)
19:        public
20:        pure
21:        returns (uint128 conveyorReward, uint128 beaconReward)
22:    {
``` 



[File:DeployFantomAggregator.s.sol#L11](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/deploy/DeployFantomAggregator.s.sol#L11) 
```solidity
10:    function run() public returns (ConveyorRouterV1 conveyorRouterV1) {
``` 



[File:DeployOptimismAggregator.s.sol#L11](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/deploy/DeployOptimismAggregator.s.sol#L11) 
```solidity
10:    function run() public returns (ConveyorRouterV1 conveyorRouterV1) {
``` 



[File:LimitOrderSwapRouter.sol#L172](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L172) 
```solidity
171:    function calculateFee(uint128 amountIn, address usdc, address weth) public view returns (uint128) {
``` 



[File:LimitOrderSwapRouter.sol#L577](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderSwapRouter.sol#L577) 
```solidity
576:    function getAllPrices(address token0, address token1, uint24 FEE)
577:        public
578:        view
579:        returns (SpotReserve[] memory prices, address[] memory lps)
580:    {
``` 



[File:DeployTest.s.sol#L14](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/deploy/DeployTest.s.sol#L14) 
```solidity
13:    function run() public {
``` 



[File:DeployBSCAggregator.s.sol#L14](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/deploy/DeployBSCAggregator.s.sol#L14) 
```solidity
13:    function run() public returns (address conveyorRouterV1) {
``` 



[File:DeployAvalancheAggregator.s.sol#L11](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/deploy/DeployAvalancheAggregator.s.sol#L11) 
```solidity
10:    function run() public returns (ConveyorRouterV1 conveyorRouterV1) {
``` 



[File:LimitOrderBook.sol#L224](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L224) 
```solidity
223:    function getLimitOrderById(bytes32 orderId) public view returns (LimitOrder memory) {
``` 



[File:LimitOrderBook.sol#L253](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L253) 
```solidity
252:    function placeLimitOrder(LimitOrder[] calldata orderGroup) public payable returns (bytes32[] memory) {
``` 



[File:LimitOrderBook.sol#L483](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L483) 
```solidity
482:    function cancelOrders(bytes32[] calldata orderIds) public {
``` 



[File:LimitOrderBook.sol#L560](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L560) 
```solidity
559:    function getAllOrderIdsLength(address _owner) public view returns (uint256) {
``` 



[File:LimitOrderBook.sol#L570](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L570) 
```solidity
569:    function getOrderIds(address _owner, OrderType targetOrderType, uint256 orderOffset, uint256 length)
570:        public
571:        view
572:        returns (bytes32[] memory)
573:    {
``` 



[File:LimitOrderBook.sol#L383](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/LimitOrderBook.sol#L383) 
```solidity
382:    function updateOrder(bytes32 orderId, uint128 price, uint128 quantity) public payable {
``` 



[File:DeployPolygonAggregator.s.sol#L12](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/deploy/DeployPolygonAggregator.s.sol#L12) 
```solidity
11:    function run() public returns (address conveyorRouterV1) {
``` 



[File:DeployMainnetAggregator.s.sol#L12](https://github.com/ConveyorLabs/smart-contracts/blob/production/src/deploy/DeployMainnetAggregator.s.sol#L12) 
```solidity
11:    function run() public returns (address conveyorRouterV1) {
``` 



 --- 


