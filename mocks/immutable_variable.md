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
 | [[G-0]](#[G-0]) | Mark storage variables as `immutable` if they never change after contract initialization. | 113 |
## Quality Assurance 

 | Classification | Title | Instances | 
 |:-------:|:---------|:-------:| 

## Vulnerabilities - Total: 0 




## Optimizations - Total: 113 

<a name=[G-0]></a>
### [G-0] Mark storage variables as `immutable` if they never change after contract initialization. - Instances: 113 

 
 
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

[File:ReturnsTooLittleToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/lib/solmate/src/test/utils/weird-tokens/ReturnsTooLittleToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:ERC20.sol#L21](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solmate/src/tokens/ERC20.sol#L21) 
```solidity
20:    string public name;
``` 



[File:ERC20.sol#L23](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solmate/src/tokens/ERC20.sol#L23) 
```solidity
22:    string public symbol;
``` 



[File:Governor.sol#L50](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/governance/Governor.sol#L50) 
```solidity
49:    string private _name;
``` 



[File:RevertingToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/test/utils/weird-tokens/RevertingToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:ERC20Mock.sol#L54](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/ext/woke/ERC20Mock.sol#L54) 
```solidity
53:    uint8 private $decimals;
``` 



[File:ERC20Mock.sol#L52](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/ext/woke/ERC20Mock.sol#L52) 
```solidity
51:    string private $name;
``` 



[File:ERC20Mock.sol#L53](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/ext/woke/ERC20Mock.sol#L53) 
```solidity
52:    string private $symbol;
``` 



[File:ERC20.sol#L45](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/token/ERC20/ERC20.sol#L45) 
```solidity
44:    string private _name;
``` 



[File:ERC20.sol#L46](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/token/ERC20/ERC20.sol#L46) 
```solidity
45:    string private _symbol;
``` 



[File:ERC20.sol#L21](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/lib/solmate/src/tokens/ERC20.sol#L21) 
```solidity
20:    string public name;
``` 



[File:ERC20.sol#L23](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/lib/solmate/src/tokens/ERC20.sol#L23) 
```solidity
22:    string public symbol;
``` 



[File:DaiPermit.sol#L21](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/ext/woke/weird/DaiPermit.sol#L21) 
```solidity
20:    uint256 public totalSupply;
``` 



[File:MissingReturnToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/lib/solmate/src/test/utils/weird-tokens/MissingReturnToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:ReturnsTooMuchToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solady/lib/solmate/src/test/utils/weird-tokens/ReturnsTooMuchToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:ReturnsTooMuchToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/test/utils/weird-tokens/ReturnsTooMuchToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:ReturnsGarbageToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solmate/src/test/utils/weird-tokens/ReturnsGarbageToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:MockERC20.sol#L7](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/test/mocks/MockERC20.sol#L7) 
```solidity
6:    string private _name;
``` 



[File:MockERC20.sol#L8](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/test/mocks/MockERC20.sol#L8) 
```solidity
7:    string private _symbol;
``` 



[File:ReturnsTooLittleToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solady/lib/solmate/src/test/utils/weird-tokens/ReturnsTooLittleToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:RevertingToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solmate/src/test/utils/weird-tokens/RevertingToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:ATokensAndRatesHelper.sol#L18](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/deployments/ATokensAndRatesHelper.sol#L18) 
```solidity
17:  address payable private pool;
``` 



[File:ATokensAndRatesHelper.sol#L19](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/deployments/ATokensAndRatesHelper.sol#L19) 
```solidity
18:  address private addressesProvider;
``` 



[File:ATokensAndRatesHelper.sol#L20](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/deployments/ATokensAndRatesHelper.sol#L20) 
```solidity
19:  address private poolConfigurator;
``` 



[File:ReturnsTwoToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solady/test/utils/weird-tokens/ReturnsTwoToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:ERC1155ReceiverMock.sol#L9](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/token/ERC1155ReceiverMock.sol#L9) 
```solidity
8:    bytes4 private _recRetval;
``` 



[File:ERC1155ReceiverMock.sol#L12](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/token/ERC1155ReceiverMock.sol#L12) 
```solidity
11:    bool private _batReverts;
``` 



[File:ERC1155ReceiverMock.sol#L11](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/token/ERC1155ReceiverMock.sol#L11) 
```solidity
10:    bytes4 private _batRetval;
``` 



[File:ERC1155ReceiverMock.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/token/ERC1155ReceiverMock.sol#L10) 
```solidity
9:    bool private _recReverts;
``` 



[File:ERC20.sol#L46](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/token/ERC20/ERC20.sol#L46) 
```solidity
45:    string private _symbol;
``` 



[File:ERC20.sol#L45](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/token/ERC20/ERC20.sol#L45) 
```solidity
44:    string private _name;
``` 



[File:SwapPair.sol#L16](https://github.com/0xKitsune/sstan/blob/main/bin/scope/3xcalibur/scope/SwapPair.sol#L16) 
```solidity
15:    string public symbol;
``` 



[File:SwapPair.sol#L15](https://github.com/0xKitsune/sstan/blob/main/bin/scope/3xcalibur/scope/SwapPair.sol#L15) 
```solidity
14:    string public name;
``` 



[File:ArraysMock.sol#L42](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/ArraysMock.sol#L42) 
```solidity
41:    bytes32[] private _array;
``` 



[File:GovernorPreventLateQuorumMock.sol#L16](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/governance/GovernorPreventLateQuorumMock.sol#L16) 
```solidity
15:    uint256 private _quorum;
``` 



[File:ReturnsGarbageToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solmate/src/test/utils/weird-tokens/ReturnsGarbageToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:RevertingToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solady/test/utils/weird-tokens/RevertingToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:ReturnsTooLittleToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/test/utils/weird-tokens/ReturnsTooLittleToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:Pausable.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/ext/woke/weird/Pausable.sol#L10) 
```solidity
9:    address owner;
``` 



[File:ReturnsRawBytesToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/test/utils/weird-tokens/ReturnsRawBytesToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:BaseHandler.sol#L22](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/test/handlers/BaseHandler.sol#L22) 
```solidity
21:  string internal label;
``` 



[File:ERC20.sol#L44](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/dependencies/openzeppelin/contracts/ERC20.sol#L44) 
```solidity
43:  string private _name;
``` 



[File:ERC20.sol#L45](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/dependencies/openzeppelin/contracts/ERC20.sol#L45) 
```solidity
44:  string private _symbol;
``` 



[File:ReturnsFalseToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solmate/src/test/utils/weird-tokens/ReturnsFalseToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:ERC777.sol#L41](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/token/ERC777/ERC777.sol#L41) 
```solidity
40:    string private _symbol;
``` 



[File:ERC777.sol#L47](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/token/ERC777/ERC777.sol#L47) 
```solidity
46:    address[] private _defaultOperatorsArray;
``` 



[File:ERC777.sol#L40](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/token/ERC777/ERC777.sol#L40) 
```solidity
39:    string private _name;
``` 



[File:RevertingToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solmate/src/test/utils/weird-tokens/RevertingToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:MockERC20.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/test/utils/mocks/MockERC20.sol#L10) 
```solidity
9:    string internal _symbol;
``` 



[File:MockERC20.sol#L11](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/test/utils/mocks/MockERC20.sol#L11) 
```solidity
10:    uint8 internal _decimals;
``` 



[File:MockERC20.sol#L9](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/test/utils/mocks/MockERC20.sol#L9) 
```solidity
8:    string internal _name;
``` 



[File:ERC20.sol#L46](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/token/ERC20/ERC20.sol#L46) 
```solidity
45:    string private _symbol;
``` 



[File:ERC20.sol#L45](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/token/ERC20/ERC20.sol#L45) 
```solidity
44:    string private _name;
``` 



[File:AccessControlDefaultAdminRulesHarness.sol#L8](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/certora/harnesses/AccessControlDefaultAdminRulesHarness.sol#L8) 
```solidity
7:    uint48 private _delayIncreaseWait;
``` 



[File:MissingReturnToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solady/test/utils/weird-tokens/MissingReturnToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:RevertingToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solady/lib/solmate/src/test/utils/weird-tokens/RevertingToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:ERC20Mock.sol#L53](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/ext/woke/ERC20Mock.sol#L53) 
```solidity
52:    string private $symbol;
``` 



[File:ERC20Mock.sol#L54](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/ext/woke/ERC20Mock.sol#L54) 
```solidity
53:    uint8 private $decimals;
``` 



[File:ERC20Mock.sol#L52](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/ext/woke/ERC20Mock.sol#L52) 
```solidity
51:    string private $name;
``` 



[File:Pausable.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/ext/woke/weird/Pausable.sol#L10) 
```solidity
9:    address owner;
``` 



[File:ReturnsFalseToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solady/test/utils/weird-tokens/ReturnsFalseToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:MockAggregator.sol#L5](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/mocks/oracle/CLAggregators/MockAggregator.sol#L5) 
```solidity
4:  int256 private _latestAnswer;
``` 



[File:MockERC20.sol#L11](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/test/utils/mocks/MockERC20.sol#L11) 
```solidity
10:    uint8 internal _decimals;
``` 



[File:MockERC20.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/test/utils/mocks/MockERC20.sol#L10) 
```solidity
9:    string internal _symbol;
``` 



[File:MockERC20.sol#L9](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/test/utils/mocks/MockERC20.sol#L9) 
```solidity
8:    string internal _name;
``` 



[File:ERC721.sol#L24](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/token/ERC721/ERC721.sol#L24) 
```solidity
23:    string private _name;
``` 



[File:ERC721.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/token/ERC721/ERC721.sol#L27) 
```solidity
26:    string private _symbol;
``` 



[File:ERC721.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/token/ERC721/ERC721.sol#L27) 
```solidity
26:    string private _symbol;
``` 



[File:ERC721.sol#L24](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/token/ERC721/ERC721.sol#L24) 
```solidity
23:    string private _name;
``` 



[File:GovernorPreventLateQuorumMock.sol#L16](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/governance/GovernorPreventLateQuorumMock.sol#L16) 
```solidity
15:    uint256 private _quorum;
``` 



[File:ERC721PresetMinterPauserAutoId.sol#L45](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/token/ERC721/presets/ERC721PresetMinterPauserAutoId.sol#L45) 
```solidity
44:    string private _baseTokenURI;
``` 



[File:StableAndVariableTokensHelper.sol#L12](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/deployments/StableAndVariableTokensHelper.sol#L12) 
```solidity
11:  address payable private pool;
``` 



[File:StableAndVariableTokensHelper.sol#L13](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/deployments/StableAndVariableTokensHelper.sol#L13) 
```solidity
12:  address private addressesProvider;
``` 



[File:GovernorPreventLateQuorumMock.sol#L16](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/governance/GovernorPreventLateQuorumMock.sol#L16) 
```solidity
15:    uint256 private _quorum;
``` 



[File:ReturnsTwoToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/test/utils/weird-tokens/ReturnsTwoToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:Governor.sol#L50](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/governance/Governor.sol#L50) 
```solidity
49:    string private _name;
``` 



[File:MockERC1271Wallet.sol#L43](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/test/utils/mocks/MockERC1271Wallet.sol#L43) 
```solidity
42:    address signer;
``` 



[File:ExpectedBalances.sol#L258](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/test/helpers/ExpectedBalances.sol#L258) 
```solidity
257:  string internal tokenKind;
``` 



[File:GnosisSafeProxy.sol#L16](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/safe-contracts/contracts/proxies/GnosisSafeProxy.sol#L16) 
```solidity
15:    address internal singleton;
``` 



[File:ReturnsGarbageToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solady/test/utils/weird-tokens/ReturnsGarbageToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:ERC721.sol#L21](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solmate/src/tokens/ERC721.sol#L21) 
```solidity
20:    string public name;
``` 



[File:ERC721.sol#L23](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solmate/src/tokens/ERC721.sol#L23) 
```solidity
22:    string public symbol;
``` 



[File:MockERC4626.sol#L16](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/test/utils/mocks/MockERC4626.sol#L16) 
```solidity
15:    string internal _symbol;
``` 



[File:MockERC4626.sol#L15](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/test/utils/mocks/MockERC4626.sol#L15) 
```solidity
14:    string internal _name;
``` 



[File:Governor.sol#L50](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/governance/Governor.sol#L50) 
```solidity
49:    string private _name;
``` 



[File:RevertingToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/test/utils/weird-tokens/RevertingToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:ReturnsTooMuchToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solady/test/utils/weird-tokens/ReturnsTooMuchToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:ERC721PresetMinterPauserAutoId.sol#L45](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/token/ERC721/presets/ERC721PresetMinterPauserAutoId.sol#L45) 
```solidity
44:    string private _baseTokenURI;
``` 



[File:BlockList.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/ext/woke/weird/BlockList.sol#L10) 
```solidity
9:    address owner;
``` 



[File:MockERC4626.sol#L16](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/test/utils/mocks/MockERC4626.sol#L16) 
```solidity
15:    string internal _symbol;
``` 



[File:MockERC4626.sol#L15](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/test/utils/mocks/MockERC4626.sol#L15) 
```solidity
14:    string internal _name;
``` 



[File:ReturnsGarbageToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solady/lib/solmate/src/test/utils/weird-tokens/ReturnsGarbageToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:ReturnsFalseToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/lib/solmate/src/test/utils/weird-tokens/ReturnsFalseToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:MockERC1271Wallet.sol#L7](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solady/test/utils/mocks/MockERC1271Wallet.sol#L7) 
```solidity
6:    address signer;
``` 



[File:RevertingToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/lib/solmate/src/test/utils/weird-tokens/RevertingToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:ReturnsTooMuchToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solmate/src/test/utils/weird-tokens/ReturnsTooMuchToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:ReturnsFalseToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/test/utils/weird-tokens/ReturnsFalseToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:Voter.sol#L21](https://github.com/0xKitsune/sstan/blob/main/bin/scope/3xcalibur/scope/Voter.sol#L21) 
```solidity
20:    address public gaugeFactory;
``` 



[File:ReturnsTwoToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solady/lib/solmate/src/test/utils/weird-tokens/ReturnsTwoToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:ReturnsGarbageToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/lib/solmate/src/test/utils/weird-tokens/ReturnsGarbageToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:ERC777.sol#L40](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/token/ERC777/ERC777.sol#L40) 
```solidity
39:    string private _name;
``` 



[File:ERC777.sol#L47](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/token/ERC777/ERC777.sol#L47) 
```solidity
46:    address[] private _defaultOperatorsArray;
``` 



[File:ERC777.sol#L41](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/token/ERC777/ERC777.sol#L41) 
```solidity
40:    string private _symbol;
``` 



[File:ReturnsTwoToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solmate/src/test/utils/weird-tokens/ReturnsTwoToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:WildcatMarketBase.sol#L60](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/src/market/WildcatMarketBase.sol#L60) 
```solidity
59:  string public symbol;
``` 



[File:WildcatMarketBase.sol#L57](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/src/market/WildcatMarketBase.sol#L57) 
```solidity
56:  string public name;
``` 



[File:ReturnsTooMuchToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solmate/src/test/utils/weird-tokens/ReturnsTooMuchToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:AccessControlDefaultAdminRulesHarness.sol#L8](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/certora/harnesses/AccessControlDefaultAdminRulesHarness.sol#L8) 
```solidity
7:    uint48 private _delayIncreaseWait;
``` 



[File:MissingReturnToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solmate/src/test/utils/weird-tokens/MissingReturnToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:ERC20.sol#L21](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solady/lib/solmate/src/tokens/ERC20.sol#L21) 
```solidity
20:    string public name;
``` 



[File:ERC20.sol#L23](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solady/lib/solmate/src/tokens/ERC20.sol#L23) 
```solidity
22:    string public symbol;
``` 



[File:ReturnsTooLittleToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solmate/src/test/utils/weird-tokens/ReturnsTooLittleToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:ERC721PresetMinterPauserAutoId.sol#L45](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/token/ERC721/presets/ERC721PresetMinterPauserAutoId.sol#L45) 
```solidity
44:    string private _baseTokenURI;
``` 



[File:MissingReturnToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solmate/src/test/utils/weird-tokens/MissingReturnToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:MissingReturnToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/test/utils/weird-tokens/MissingReturnToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:ERC20.sol#L21](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solmate/src/tokens/ERC20.sol#L21) 
```solidity
20:    string public name;
``` 



[File:ERC20.sol#L23](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solmate/src/tokens/ERC20.sol#L23) 
```solidity
22:    string public symbol;
``` 



[File:ReturnsTooLittleToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/test/utils/weird-tokens/ReturnsTooLittleToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:MissingReturnToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/test/utils/weird-tokens/MissingReturnToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:MockERC1271Wallet.sol#L43](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/test/utils/mocks/MockERC1271Wallet.sol#L43) 
```solidity
42:    address signer;
``` 



[File:ReturnsTooLittleToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solady/test/utils/weird-tokens/ReturnsTooLittleToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:MissingReturnToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solady/lib/solmate/src/test/utils/weird-tokens/MissingReturnToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:BlockList.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/ext/woke/weird/BlockList.sol#L10) 
```solidity
9:    address owner;
``` 



[File:ReturnsFalseToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solmate/src/test/utils/weird-tokens/ReturnsFalseToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:ReturnsTooLittleToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solmate/src/test/utils/weird-tokens/ReturnsTooLittleToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:ReturnsRawBytesToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/test/utils/weird-tokens/ReturnsRawBytesToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:ERC777.sol#L47](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/token/ERC777/ERC777.sol#L47) 
```solidity
46:    address[] private _defaultOperatorsArray;
``` 



[File:ERC777.sol#L40](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/token/ERC777/ERC777.sol#L40) 
```solidity
39:    string private _name;
``` 



[File:ERC777.sol#L41](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/token/ERC777/ERC777.sol#L41) 
```solidity
40:    string private _symbol;
``` 



[File:ArraysMock.sol#L42](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/ArraysMock.sol#L42) 
```solidity
41:    bytes32[] private _array;
``` 



[File:ArraysMock.sol#L42](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/ArraysMock.sol#L42) 
```solidity
41:    bytes32[] private _array;
``` 



[File:ReturnsTwoToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/test/utils/weird-tokens/ReturnsTwoToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:ERC721.sol#L21](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/lib/solmate/src/tokens/ERC721.sol#L21) 
```solidity
20:    string public name;
``` 



[File:ERC721.sol#L23](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/lib/solmate/src/tokens/ERC721.sol#L23) 
```solidity
22:    string public symbol;
``` 



[File:ERC721.sol#L23](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solmate/src/tokens/ERC721.sol#L23) 
```solidity
22:    string public symbol;
``` 



[File:ERC721.sol#L21](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solmate/src/tokens/ERC721.sol#L21) 
```solidity
20:    string public name;
``` 



[File:ReturnsFalseToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solady/lib/solmate/src/test/utils/weird-tokens/ReturnsFalseToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:ERC721.sol#L24](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/token/ERC721/ERC721.sol#L24) 
```solidity
23:    string private _name;
``` 



[File:ERC721.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/token/ERC721/ERC721.sol#L27) 
```solidity
26:    string private _symbol;
``` 



[File:AddressRegistry.sol#L8](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/test/commons/AddressRegistry.sol#L8) 
```solidity
7:    string addressManagerPath;
``` 



[File:ERC721.sol#L23](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solady/lib/solmate/src/tokens/ERC721.sol#L23) 
```solidity
22:    string public symbol;
``` 



[File:ERC721.sol#L21](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solady/lib/solmate/src/tokens/ERC721.sol#L21) 
```solidity
20:    string public name;
``` 



[File:ReturnsTooMuchToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/test/utils/weird-tokens/ReturnsTooMuchToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:ReturnsTwoToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/lib/solmate/src/test/utils/weird-tokens/ReturnsTwoToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:ERC1155ReceiverMock.sol#L12](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/token/ERC1155ReceiverMock.sol#L12) 
```solidity
11:    bool private _batReverts;
``` 



[File:ERC1155ReceiverMock.sol#L9](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/token/ERC1155ReceiverMock.sol#L9) 
```solidity
8:    bytes4 private _recRetval;
``` 



[File:ERC1155ReceiverMock.sol#L11](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/token/ERC1155ReceiverMock.sol#L11) 
```solidity
10:    bytes4 private _batRetval;
``` 



[File:ERC1155ReceiverMock.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/token/ERC1155ReceiverMock.sol#L10) 
```solidity
9:    bool private _recReverts;
``` 



[File:ReturnsTwoToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solmate/src/test/utils/weird-tokens/ReturnsTwoToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:ERC1155ReceiverMock.sol#L9](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/token/ERC1155ReceiverMock.sol#L9) 
```solidity
8:    bytes4 private _recRetval;
``` 



[File:ERC1155ReceiverMock.sol#L12](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/token/ERC1155ReceiverMock.sol#L12) 
```solidity
11:    bool private _batReverts;
``` 



[File:ERC1155ReceiverMock.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/token/ERC1155ReceiverMock.sol#L10) 
```solidity
9:    bool private _recReverts;
``` 



[File:ERC1155ReceiverMock.sol#L11](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/token/ERC1155ReceiverMock.sol#L11) 
```solidity
10:    bytes4 private _batRetval;
``` 



[File:Minter.sol#L30](https://github.com/0xKitsune/sstan/blob/main/bin/scope/3xcalibur/scope/Minter.sol#L30) 
```solidity
29:    address internal admin;
``` 



[File:AccessControlDefaultAdminRulesHarness.sol#L8](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/certora/harnesses/AccessControlDefaultAdminRulesHarness.sol#L8) 
```solidity
7:    uint48 private _delayIncreaseWait;
``` 



[File:ReturnsFalseToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/test/utils/weird-tokens/ReturnsFalseToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



[File:DaiPermit.sol#L21](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/ext/woke/weird/DaiPermit.sol#L21) 
```solidity
20:    uint256 public totalSupply;
``` 



[File:ERC20Handler.sol#L12](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/test/handlers/ERC20Handler.sol#L12) 
```solidity
11:  IERC20 token;
``` 



[File:ReturnsTooMuchToken.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/lib/solmate/src/test/utils/weird-tokens/ReturnsTooMuchToken.sol#L27) 
```solidity
26:    uint256 public totalSupply;
``` 



 --- 



## Quality Assurance - Total: 0 



