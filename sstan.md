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
## Quality Assurance 

 | Classification | Title | Instances | 
 |:-------:|:---------|:-------:| 
 | [[NC-0]](#[NC-0]) | This variables default value is the same as the value it is initialized with | 12 |

## Vulnerabilities - Total: 0 




## Optimizations - Total: 0 




## Quality Assurance - Total: 12 

<a name=[NC-0]></a>
### [NC-0] This variables default value is the same as the value it is initialized with - Instances: 12 

 
> This is unnecessary and will have some overhead on Gas
     

 --- 

[File:ERC4626.sol#L21](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/src/tokens/ERC4626.sol#L21) 
```solidity
20:    uint8 internal constant _DEFAULT_DECIMALS_OFFSET = 0;
``` 



[File:RedBlackTreeLib.sol#L74](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/src/utils/RedBlackTreeLib.sol#L74) 
```solidity
73:    uint256 private constant _BITPOS_LEFT = 0;
``` 



[File:JSONParserLib.sol#L23](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/src/utils/JSONParserLib.sol#L23) 
```solidity
22:    uint8 internal constant TYPE_UNDEFINED = 0;
``` 



[File:VersionedInitializable.sol#L22](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/protocol/libraries/aave-upgradeability/VersionedInitializable.sol#L22) 
```solidity
21:  uint256 private lastInitializedRevision = 0;
``` 



[File:UniswapConstants.sol#L89](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/src/uniswap/UniswapConstants.sol#L89) 
```solidity
88:uint256 constant UniswapV2_GetReserves_Selector_Pointer = 0;
``` 



[File:SafeModeratorOverridable.sol#L21](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/src/core/SafeModeratorOverridable.sol#L21) 
```solidity
20:    uint8 public constant DIFFER_SAFE_MOD = 0;
``` 



[File:RedBlackTreeLib.sol#L74](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solady/src/utils/RedBlackTreeLib.sol#L74) 
```solidity
73:    uint256 private constant _BITPOS_LEFT = 0;
``` 



[File:ERC4626.sol#L21](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/src/tokens/ERC4626.sol#L21) 
```solidity
20:    uint8 internal constant _DEFAULT_DECIMALS_OFFSET = 0;
``` 



[File:SwapPair.sol#L22](https://github.com/0xKitsune/sstan/blob/main/bin/scope/3xcalibur/scope/SwapPair.sol#L22) 
```solidity
21:    uint public totalSupply = 0;
``` 



[File:SwapPair.sol#L64](https://github.com/0xKitsune/sstan/blob/main/bin/scope/3xcalibur/scope/SwapPair.sol#L64) 
```solidity
63:    uint public index0 = 0;
``` 



[File:SwapPair.sol#L65](https://github.com/0xKitsune/sstan/blob/main/bin/scope/3xcalibur/scope/SwapPair.sol#L65) 
```solidity
64:    uint public index1 = 0;
``` 



[File:RedBlackTreeLib.sol#L74](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/src/utils/RedBlackTreeLib.sol#L74) 
```solidity
73:    uint256 private constant _BITPOS_LEFT = 0;
``` 



 --- 


