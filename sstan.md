# Sstan - v0.1.0 

 --- 
 ## Authors: 0x00face, 0xOsiris 
 --- 
 TODO: add description

# Summary




## Vulnerabilities 

 | Classification | Title | Instances | 
 |:-------:|:---------|:-------:| 
## Optimizations 

 | Classification | Title | Instances | 
 |:-------:|:---------|:-------:| 
 | [[G-0]](#[G-0]) | Use multiple require() statments insted of require(expression && expression && ...) | 35 |
## Quality Assurance 

 | Classification | Title | Instances | 
 |:-------:|:---------|:-------:| 

## Vulnerabilities - Total: 0 




## Optimizations - Total: 35 

<a name=[G-0]></a>
### [G-0] Use multiple require() statments insted of require(expression && expression && ...) - Instances: 35 

 
 
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

[File:SwapFactory.sol#L68](https://github.com/0xKitsune/sstan/blob/main/bin/scope/3xcalibur/scope/SwapFactory.sol#L68) 
```solidity
67:        require(msg.sender == admin && _admin != address(0), "SwapFactory; wrong input parameters");
``` 



[File:LendingPoolConfigurator.sol#L482](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/protocol/lendingpool/LendingPoolConfigurator.sol#L482) 
```solidity
481:    require(
482:      availableLiquidity == 0 && reserveData.currentLiquidityRate == 0,
483:      Errors.LPC_RESERVE_LIQUIDITY_NOT_0
484:    );
``` 



[File:Initializable.sol#L120](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/proxy/utils/Initializable.sol#L120) 
```solidity
119:        require(!_initializing && _initialized < version, "Initializable: contract is already initialized");
``` 



[File:ERC20.sol#L154](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solady/lib/solmate/src/tokens/ERC20.sol#L154) 
```solidity
153:            require(recoveredAddress != address(0) && recoveredAddress == owner, "INVALID_SIGNER");
``` 



[File:AccessControlDefaultAdminRules.sol#L111](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/access/AccessControlDefaultAdminRules.sol#L111) 
```solidity
110:            require(
111:                newDefaultAdmin == address(0) && _isScheduleSet(schedule) && _hasSchedulePassed(schedule),
112:                "AccessControl: only can renounce in two delayed steps"
113:            );
``` 



[File:AccessControlDefaultAdminRules.sol#L250](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/access/AccessControlDefaultAdminRules.sol#L250) 
```solidity
249:        require(_isScheduleSet(schedule) && _hasSchedulePassed(schedule), "AccessControl: transfer delay not passed");
``` 



[File:Governor.sol#L429](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/governance/Governor.sol#L429) 
```solidity
428:        require(
429:            currentState != ProposalState.Canceled &&
430:                currentState != ProposalState.Expired &&
431:                currentState != ProposalState.Executed,
432:            "Governor: proposal not active"
433:        );
``` 



[File:SafeEnabler.sol#L48](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/src/core/SafeEnabler.sol#L48) 
```solidity
47:        require(module != address(0) && module != _SENTINEL_MODULES, "GS101");
``` 



[File:ERC20.sol#L154](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solmate/src/tokens/ERC20.sol#L154) 
```solidity
153:            require(recoveredAddress != address(0) && recoveredAddress == owner, "INVALID_SIGNER");
``` 



[File:Gauge.sol#L85](https://github.com/0xKitsune/sstan/blob/main/bin/scope/3xcalibur/scope/Gauge.sol#L85) 
```solidity
84:        require(
85:            _stake != address(0) &&
86:            _bribe != address(0) &&
87:            __ve != address(0) &&
88:            _voter != address(0),
89:            "Gauge: zero address provided in constructor"
90:        );
``` 



[File:Gauge.sol#L570](https://github.com/0xKitsune/sstan/blob/main/bin/scope/3xcalibur/scope/Gauge.sol#L570) 
```solidity
569:        require(success && (data.length == 0 || abi.decode(data, (bool))));
``` 



[File:Gauge.sol#L577](https://github.com/0xKitsune/sstan/blob/main/bin/scope/3xcalibur/scope/Gauge.sol#L577) 
```solidity
576:        require(success && (data.length == 0 || abi.decode(data, (bool))));
``` 



[File:Gauge.sol#L584](https://github.com/0xKitsune/sstan/blob/main/bin/scope/3xcalibur/scope/Gauge.sol#L584) 
```solidity
583:        require(success && (data.length == 0 || abi.decode(data, (bool))));
``` 



[File:ERC20.sol#L154](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/lib/solmate/src/tokens/ERC20.sol#L154) 
```solidity
153:            require(recoveredAddress != address(0) && recoveredAddress == owner, "INVALID_SIGNER");
``` 



[File:FlashLiquidationAdapter.sol#L75](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/adapters/FlashLiquidationAdapter.sol#L75) 
```solidity
74:    require(assets.length == 1 && assets[0] == decodedParams.borrowedAsset, 'INCONSISTENT_PARAMS');
``` 



[File:ModuleManager.sol#L34](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/safe-contracts/contracts/base/ModuleManager.sol#L34) 
```solidity
33:        require(module != address(0) && module != SENTINEL_MODULES, "GS101");
``` 



[File:ModuleManager.sol#L49](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/safe-contracts/contracts/base/ModuleManager.sol#L49) 
```solidity
48:        require(module != address(0) && module != SENTINEL_MODULES, "GS101");
``` 



[File:ModuleManager.sol#L68](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/safe-contracts/contracts/base/ModuleManager.sol#L68) 
```solidity
67:        require(msg.sender != SENTINEL_MODULES && modules[msg.sender] != address(0), "GS104");
``` 



[File:CREATE3.sol#L54](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/lib/solmate/src/utils/CREATE3.sol#L54) 
```solidity
53:        require(success && deployed.code.length != 0, "INITIALIZATION_FAILED");
``` 



[File:Multiswap.sol#L37](https://github.com/0xKitsune/sstan/blob/main/bin/scope/3xcalibur/scope/Multiswap.sol#L37) 
```solidity
36:        require(length > 1 && length <= 5 && _weights.length == length, "length");
``` 



[File:ValidationLogic.sol#L327](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/protocol/libraries/logic/ValidationLogic.sol#L327) 
```solidity
326:    require(
327:      usageRatio >= REBALANCE_UP_USAGE_RATIO_THRESHOLD &&
328:        currentLiquidityRate <=
329:        maxVariableBorrowRate.percentMul(REBALANCE_UP_LIQUIDITY_RATE_THRESHOLD),
330:      Errors.LP_INTEREST_RATE_REBALANCE_CONDITIONS_NOT_MET
331:    );
``` 



[File:GnosisSafe.sol#L301](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/safe-contracts/contracts/GnosisSafe.sol#L301) 
```solidity
300:            require(currentOwner > lastOwner && owners[currentOwner] != address(0) && currentOwner != SENTINEL_OWNERS, "GS026");
``` 



[File:Initializable.sol#L120](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/proxy/utils/Initializable.sol#L120) 
```solidity
119:        require(!_initializing && _initialized < version, "Initializable: contract is already initialized");
``` 



[File:MockParaSwapAugustus.sol#L51](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/mocks/swap/MockParaSwapAugustus.sol#L51) 
```solidity
50:    require(fromAmount >= _expectedFromAmountMin && fromAmount <= _expectedFromAmountMax, 'From amount out of range');
``` 



[File:AccessControlDefaultAdminRules.sol#L111](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/access/AccessControlDefaultAdminRules.sol#L111) 
```solidity
110:            require(
111:                newDefaultAdmin == address(0) && _isScheduleSet(schedule) && _hasSchedulePassed(schedule),
112:                "AccessControl: only can renounce in two delayed steps"
113:            );
``` 



[File:AccessControlDefaultAdminRules.sol#L250](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/access/AccessControlDefaultAdminRules.sol#L250) 
```solidity
249:        require(_isScheduleSet(schedule) && _hasSchedulePassed(schedule), "AccessControl: transfer delay not passed");
``` 



[File:CREATE3.sol#L55](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solmate/src/utils/CREATE3.sol#L55) 
```solidity
54:        require(success && deployed.code.length != 0, "INITIALIZATION_FAILED");
``` 



[File:Initializable.sol#L120](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/proxy/utils/Initializable.sol#L120) 
```solidity
119:        require(!_initializing && _initialized < version, "Initializable: contract is already initialized");
``` 



[File:Governor.sol#L429](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/governance/Governor.sol#L429) 
```solidity
428:        require(
429:            currentState != ProposalState.Canceled &&
430:                currentState != ProposalState.Expired &&
431:                currentState != ProposalState.Executed,
432:            "Governor: proposal not active"
433:        );
``` 



[File:ParaSwapLiquiditySwapAdapter.sol#L50](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/adapters/ParaSwapLiquiditySwapAdapter.sol#L50) 
```solidity
49:    require(
50:      assets.length == 1 && amounts.length == 1 && premiums.length == 1,
51:      'FLASHLOAN_MULTIPLE_ASSETS_NOT_SUPPORTED'
52:    );
``` 



[File:Voter.sol#L69](https://github.com/0xKitsune/sstan/blob/main/bin/scope/3xcalibur/scope/Voter.sol#L69) 
```solidity
68:        require(
69:            __ve != address(0) &&
70:            _factory != address(0) &&
71:            _gauges != address(0) &&
72:            _bribes != address(0),
73:            "Voter: zero address provided in constructor"
74:        );
``` 



[File:Voter.sol#L251](https://github.com/0xKitsune/sstan/blob/main/bin/scope/3xcalibur/scope/Voter.sol#L251) 
```solidity
250:        require(isWhitelisted[_tokenA] && isWhitelisted[_tokenB], "!whitelisted");
``` 



[File:Voter.sol#L437](https://github.com/0xKitsune/sstan/blob/main/bin/scope/3xcalibur/scope/Voter.sol#L437) 
```solidity
436:        require(success && (data.length == 0 || abi.decode(data, (bool))));
``` 



[File:AccessControlDefaultAdminRules.sol#L111](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/access/AccessControlDefaultAdminRules.sol#L111) 
```solidity
110:            require(
111:                newDefaultAdmin == address(0) && _isScheduleSet(schedule) && _hasSchedulePassed(schedule),
112:                "AccessControl: only can renounce in two delayed steps"
113:            );
``` 



[File:AccessControlDefaultAdminRules.sol#L250](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/access/AccessControlDefaultAdminRules.sol#L250) 
```solidity
249:        require(_isScheduleSet(schedule) && _hasSchedulePassed(schedule), "AccessControl: transfer delay not passed");
``` 



[File:OwnerManager.sol#L35](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/safe-contracts/contracts/base/OwnerManager.sol#L35) 
```solidity
34:            require(owner != address(0) && owner != SENTINEL_OWNERS && owner != address(this) && currentOwner != owner, "GS203");
``` 



[File:OwnerManager.sol#L53](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/safe-contracts/contracts/base/OwnerManager.sol#L53) 
```solidity
52:        require(owner != address(0) && owner != SENTINEL_OWNERS && owner != address(this), "GS203");
``` 



[File:OwnerManager.sol#L78](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/safe-contracts/contracts/base/OwnerManager.sol#L78) 
```solidity
77:        require(owner != address(0) && owner != SENTINEL_OWNERS, "GS203");
``` 



[File:OwnerManager.sol#L100](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/safe-contracts/contracts/base/OwnerManager.sol#L100) 
```solidity
99:        require(newOwner != address(0) && newOwner != SENTINEL_OWNERS && newOwner != address(this), "GS203");
``` 



[File:OwnerManager.sol#L104](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/safe-contracts/contracts/base/OwnerManager.sol#L104) 
```solidity
103:        require(oldOwner != address(0) && oldOwner != SENTINEL_OWNERS, "GS203");
``` 



[File:Minter.sol#L47](https://github.com/0xKitsune/sstan/blob/main/bin/scope/3xcalibur/scope/Minter.sol#L47) 
```solidity
46:        require(
47:            __voter != address(0) &&
48:            __ve != address(0) &&
49:            __ve_dist != address(0) &&
50:            _admin != address(0),
51:            "Minter: zero address provided in constructor"
52:        );
``` 



[File:UniswapLiquiditySwapAdapter.sol#L68](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/adapters/UniswapLiquiditySwapAdapter.sol#L68) 
```solidity
67:    require(
68:      assets.length == decodedParams.assetToSwapToList.length &&
69:        assets.length == decodedParams.minAmountsToReceive.length &&
70:        assets.length == decodedParams.swapAllBalance.length &&
71:        assets.length == decodedParams.permitParams.amount.length &&
72:        assets.length == decodedParams.permitParams.deadline.length &&
73:        assets.length == decodedParams.permitParams.v.length &&
74:        assets.length == decodedParams.permitParams.r.length &&
75:        assets.length == decodedParams.permitParams.s.length &&
76:        assets.length == decodedParams.useEthPath.length,
77:      'INCONSISTENT_PARAMS'
78:    );
``` 



[File:UniswapLiquiditySwapAdapter.sol#L138](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/adapters/UniswapLiquiditySwapAdapter.sol#L138) 
```solidity
137:    require(
138:      assetToSwapFromList.length == assetToSwapToList.length &&
139:        assetToSwapFromList.length == amountToSwapList.length &&
140:        assetToSwapFromList.length == minAmountsToReceive.length &&
141:        assetToSwapFromList.length == permitParams.length,
142:      'INCONSISTENT_PARAMS'
143:    );
``` 



[File:SwapPair.sol#L333](https://github.com/0xKitsune/sstan/blob/main/bin/scope/3xcalibur/scope/SwapPair.sol#L333) 
```solidity
332:        require(amount0 > 0 && amount1 > 0, 'ILB'); // SwapPair: INSUFFICIENT_LIQUIDITY_BURNED
``` 



[File:SwapPair.sol#L349](https://github.com/0xKitsune/sstan/blob/main/bin/scope/3xcalibur/scope/SwapPair.sol#L349) 
```solidity
348:        require(amount0Out < _reserve0 && amount1Out < _reserve1, 'IL'); // SwapPair: INSUFFICIENT_LIQUIDITY
``` 



[File:SwapPair.sol#L355](https://github.com/0xKitsune/sstan/blob/main/bin/scope/3xcalibur/scope/SwapPair.sol#L355) 
```solidity
354:        require(to != _token0 && to != _token1, 'IT'); // SwapPair: INVALID_TO
``` 



[File:SwapPair.sol#L501](https://github.com/0xKitsune/sstan/blob/main/bin/scope/3xcalibur/scope/SwapPair.sol#L501) 
```solidity
500:        require(recoveredAddress != address(0) && recoveredAddress == owner, 'SwapPair: INVALID_SIGNATURE');
``` 



[File:SwapPair.sol#L541](https://github.com/0xKitsune/sstan/blob/main/bin/scope/3xcalibur/scope/SwapPair.sol#L541) 
```solidity
540:        require(success && (data.length == 0 || abi.decode(data, (bool))));
``` 



[File:CREATE3.sol#L55](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solmate/src/utils/CREATE3.sol#L55) 
```solidity
54:        require(success && deployed.code.length != 0, "INITIALIZATION_FAILED");
``` 



[File:CREATE3.sol#L54](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solady/lib/solmate/src/utils/CREATE3.sol#L54) 
```solidity
53:        require(success && deployed.code.length != 0, "INITIALIZATION_FAILED");
``` 



[File:BaseParaSwapSellAdapter.sol#L80](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/adapters/BaseParaSwapSellAdapter.sol#L80) 
```solidity
79:      require(fromAmountOffset >= 4 &&
80:        fromAmountOffset <= swapCalldata.length.sub(32),
81:        'FROM_AMOUNT_OFFSET_OUT_OF_RANGE');
``` 



[File:Bribe.sol#L457](https://github.com/0xKitsune/sstan/blob/main/bin/scope/3xcalibur/scope/Bribe.sol#L457) 
```solidity
456:        require(success && (data.length == 0 || abi.decode(data, (bool))));
``` 



[File:Bribe.sol#L464](https://github.com/0xKitsune/sstan/blob/main/bin/scope/3xcalibur/scope/Bribe.sol#L464) 
```solidity
463:        require(success && (data.length == 0 || abi.decode(data, (bool))));
``` 



[File:ERC20.sol#L154](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solmate/src/tokens/ERC20.sol#L154) 
```solidity
153:            require(recoveredAddress != address(0) && recoveredAddress == owner, "INVALID_SIGNER");
``` 



[File:Governor.sol#L429](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/governance/Governor.sol#L429) 
```solidity
428:        require(
429:            currentState != ProposalState.Canceled &&
430:                currentState != ProposalState.Expired &&
431:                currentState != ProposalState.Executed,
432:            "Governor: proposal not active"
433:        );
``` 



 --- 



## Quality Assurance - Total: 0 



