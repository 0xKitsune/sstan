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
 | [[G-0]](#[G-0]) | Mark storage variables as `constant` if they never change. | 77 |
## Quality Assurance 

 | Classification | Title | Instances | 
 |:-------:|:---------|:-------:| 

## Vulnerabilities - Total: 0 




## Optimizations - Total: 77 

<a name=[G-0]></a>
### [G-0] Mark storage variables as `constant` if they never change. - Instances: 77 

 
 
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

[File:test.sol#L38](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/lib/forge-std/lib/ds-test/src/test.sol#L38) 
```solidity
37:    bool public IS_TEST = true;
``` 



[File:MockETHRecipient.sol#L13](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/test/utils/mocks/MockETHRecipient.sol#L13) 
```solidity
12:    uint256 public garbage;
``` 



[File:bridges.sol#L41](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/crosschain/bridges.sol#L41) 
```solidity
40:    address public immutable inbox = address(new BridgeArbitrumL1Inbox());
``` 



[File:bridges.sol#L43](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/crosschain/bridges.sol#L43) 
```solidity
42:    address public immutable outbox = address(new BridgeArbitrumL1Outbox());
``` 



[File:bridges.sol#L61](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/crosschain/bridges.sol#L61) 
```solidity
60:    address public immutable bridge = msg.sender;
``` 



[File:ERC20.sol#L20](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/ext/woke/weird/ERC20.sol#L20) 
```solidity
19:    uint8   public decimals = 18;
``` 



[File:test.sol#L38](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solmate/lib/ds-test/src/test.sol#L38) 
```solidity
37:    bool public IS_TEST = true;
``` 



[File:NoRevert.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/ext/woke/weird/NoRevert.sol#L10) 
```solidity
9:    uint8   public decimals = 18;
``` 



[File:Script.sol#L26](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/forge-std/src/Script.sol#L26) 
```solidity
25:    bool public IS_SCRIPT = true;
``` 



[File:Script.sol#L25](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/lib/forge-std/src/Script.sol#L25) 
```solidity
24:    bool public IS_SCRIPT = true;
``` 



[File:DaiPermit.sol#L20](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/ext/woke/weird/DaiPermit.sol#L20) 
```solidity
19:    uint8   public decimals = 18;
``` 



[File:DaiPermit.sol#L23](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/ext/woke/weird/DaiPermit.sol#L23) 
```solidity
22:	bytes32 public immutable DOMAIN_SEPARATOR = keccak256(
23:        abi.encode(
24:            keccak256('EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)'),
25:            keccak256(bytes(name)),
26:            keccak256(bytes('1')),
27:            block.chainid,
28:            address(this)
29:        )
30:    );
``` 



[File:EIP712.sol#L52](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/utils/cryptography/EIP712.sol#L52) 
```solidity
51:    string private _nameFallback;
``` 



[File:EIP712.sol#L53](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/utils/cryptography/EIP712.sol#L53) 
```solidity
52:    string private _versionFallback;
``` 



[File:NoRevert.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/ext/woke/weird/NoRevert.sol#L10) 
```solidity
9:    uint8   public decimals = 18;
``` 



[File:Script.sol#L25](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/lib/forge-std/src/Script.sol#L25) 
```solidity
24:    bool public IS_SCRIPT = true;
``` 



[File:ERC20FlashMintHarness.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/certora/harnesses/ERC20FlashMintHarness.sol#L10) 
```solidity
9:    uint256 someFee;
``` 



[File:ERC20FlashMintHarness.sol#L11](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/certora/harnesses/ERC20FlashMintHarness.sol#L11) 
```solidity
10:    address someFeeReceiver;
``` 



[File:Migrate_1_3_0_to_1_2_0.sol#L21](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/safe-contracts/contracts/examples/libraries/Migrate_1_3_0_to_1_2_0.sol#L21) 
```solidity
20:    bytes32 private guard;
``` 



[File:EIP712.sol#L53](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/utils/cryptography/EIP712.sol#L53) 
```solidity
52:    string private _versionFallback;
``` 



[File:EIP712.sol#L52](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/utils/cryptography/EIP712.sol#L52) 
```solidity
51:    string private _nameFallback;
``` 



[File:receivers.sol#L15](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/crosschain/receivers.sol#L15) 
```solidity
14:    address public immutable owner = msg.sender;
``` 



[File:ERC20Permit.sol#L37](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/token/ERC20/extensions/ERC20Permit.sol#L37) 
```solidity
36:    bytes32 private _PERMIT_TYPEHASH_DEPRECATED_SLOT;
``` 



[File:ConsoleFactory.s.sol#L49](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/script/utils/ConsoleFactory.s.sol#L49) 
```solidity
48:    bytes32 consoleFallbackHandlerSalt = salt("ConsoleFallbackHandler");
``` 



[File:ConsoleFactory.s.sol#L58](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/script/utils/ConsoleFactory.s.sol#L58) 
```solidity
57:    bytes32 safeEnablerSalt = salt("SafeEnabler");
``` 



[File:ConsoleFactory.s.sol#L57](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/script/utils/ConsoleFactory.s.sol#L57) 
```solidity
56:    bytes32 executorPluginSalt = salt("ExecutorPlugin");
``` 



[File:ConsoleFactory.s.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/script/utils/ConsoleFactory.s.sol#L27) 
```solidity
26:    string private suffix = "beta_v2";
``` 



[File:ConsoleFactory.s.sol#L53](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/script/utils/ConsoleFactory.s.sol#L53) 
```solidity
52:    bytes32 safeDeployerSalt = salt("SafeDeployer");
``` 



[File:ConsoleFactory.s.sol#L48](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/script/utils/ConsoleFactory.s.sol#L48) 
```solidity
47:    bytes32 addressProviderSalt = salt("AddressProvider");
``` 



[File:ConsoleFactory.s.sol#L51](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/script/utils/ConsoleFactory.s.sol#L51) 
```solidity
50:    bytes32 policyRegistrySalt = salt("PolicyRegistry");
``` 



[File:ConsoleFactory.s.sol#L59](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/script/utils/ConsoleFactory.s.sol#L59) 
```solidity
58:    bytes32 transactionValidatorSalt = salt("TransactionValidator");
``` 



[File:ConsoleFactory.s.sol#L56](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/script/utils/ConsoleFactory.s.sol#L56) 
```solidity
55:    bytes32 policyValidatorSalt = salt("PolicyValidator");
``` 



[File:ConsoleFactory.s.sol#L50](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/script/utils/ConsoleFactory.s.sol#L50) 
```solidity
49:    bytes32 walletRegistrySalt = salt("WalletRegistry");
``` 



[File:ConsoleFactory.s.sol#L55](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/script/utils/ConsoleFactory.s.sol#L55) 
```solidity
54:    bytes32 safeModeratorOverridableSalt = salt("SafeModeratorOverridable");
``` 



[File:ConsoleFactory.s.sol#L60](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/script/utils/ConsoleFactory.s.sol#L60) 
```solidity
59:    bytes32 consoleOpBuilderSalt = salt("ConsoleOpBuilder");
``` 



[File:ConsoleFactory.s.sol#L52](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/script/utils/ConsoleFactory.s.sol#L52) 
```solidity
51:    bytes32 executorRegistrySalt = salt("ExecutorRegistry");
``` 



[File:ConsoleFactory.s.sol#L54](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/script/utils/ConsoleFactory.s.sol#L54) 
```solidity
53:    bytes32 safeModeratorSalt = salt("SafeModerator");
``` 



[File:ConsoleFactory.s.sol#L46](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/script/utils/ConsoleFactory.s.sol#L46) 
```solidity
45:    address trustedValidator = makeAddr("TrustedValidator");
``` 



[File:receivers.sol#L15](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/crosschain/receivers.sol#L15) 
```solidity
14:    address public immutable owner = msg.sender;
``` 



[File:Uint96.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/ext/woke/weird/Uint96.sol#L10) 
```solidity
9:    uint8   public decimals = 18;
``` 



[File:SwapPair.sol#L38](https://github.com/0xKitsune/sstan/blob/main/bin/scope/3xcalibur/scope/SwapPair.sol#L38) 
```solidity
37:    uint public immutable fee;
``` 



[File:SwapPair.sol#L35](https://github.com/0xKitsune/sstan/blob/main/bin/scope/3xcalibur/scope/SwapPair.sol#L35) 
```solidity
34:    address public immutable token1;
``` 



[File:SwapPair.sol#L20](https://github.com/0xKitsune/sstan/blob/main/bin/scope/3xcalibur/scope/SwapPair.sol#L20) 
```solidity
19:    bool public immutable stable;
``` 



[File:SwapPair.sol#L34](https://github.com/0xKitsune/sstan/blob/main/bin/scope/3xcalibur/scope/SwapPair.sol#L34) 
```solidity
33:    address public immutable token0;
``` 



[File:BaseERC20Test.sol#L22](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/test/helpers/BaseERC20Test.sol#L22) 
```solidity
21:  string _name;
``` 



[File:BaseERC20Test.sol#L23](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/test/helpers/BaseERC20Test.sol#L23) 
```solidity
22:  string _symbol;
``` 



[File:BaseERC20Test.sol#L24](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/test/helpers/BaseERC20Test.sol#L24) 
```solidity
23:  uint8 _decimals;
``` 



[File:MockETHRecipient.sol#L11](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solady/test/utils/mocks/MockETHRecipient.sol#L11) 
```solidity
10:    uint256 public garbage;
``` 



[File:Script.sol#L25](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/lib/forge-std/src/Script.sol#L25) 
```solidity
24:    bool public IS_SCRIPT = true;
``` 



[File:DaiPermit.sol#L20](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/ext/woke/weird/DaiPermit.sol#L20) 
```solidity
19:    uint8   public decimals = 18;
``` 



[File:DaiPermit.sol#L23](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/ext/woke/weird/DaiPermit.sol#L23) 
```solidity
22:	bytes32 public immutable DOMAIN_SEPARATOR = keccak256(
23:        abi.encode(
24:            keccak256('EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)'),
25:            keccak256(bytes(name)),
26:            keccak256(bytes('1')),
27:            block.chainid,
28:            address(this)
29:        )
30:    );
``` 



[File:LibStoredInitCodeExternal.sol#L7](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/test/libraries/wrappers/LibStoredInitCodeExternal.sol#L7) 
```solidity
6:  uint256 public immutable getContractParameters = 123;
``` 



[File:Voter.sol#L19](https://github.com/0xKitsune/sstan/blob/main/bin/scope/3xcalibur/scope/Voter.sol#L19) 
```solidity
18:    address public convenience;
``` 



[File:GnosisSafe.sol#L52](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/safe-contracts/contracts/GnosisSafe.sol#L52) 
```solidity
51:    bytes32 private _deprecatedDomainSeparator;
``` 



[File:ERC3156FlashBorrowerHarness.sol#L8](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/certora/harnesses/ERC3156FlashBorrowerHarness.sol#L8) 
```solidity
7:    bytes32 somethingToReturn;
``` 



[File:ERC20FlashMintHarness.sol#L11](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/certora/harnesses/ERC20FlashMintHarness.sol#L11) 
```solidity
10:    address someFeeReceiver;
``` 



[File:ERC20FlashMintHarness.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/certora/harnesses/ERC20FlashMintHarness.sol#L10) 
```solidity
9:    uint256 someFee;
``` 



[File:ERC3156FlashBorrowerHarness.sol#L8](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/certora/harnesses/ERC3156FlashBorrowerHarness.sol#L8) 
```solidity
7:    bytes32 somethingToReturn;
``` 



[File:Uint96.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/ext/woke/weird/Uint96.sol#L10) 
```solidity
9:    uint8   public decimals = 18;
``` 



[File:MockETHRecipient.sol#L13](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/test/utils/mocks/MockETHRecipient.sol#L13) 
```solidity
12:    uint256 public garbage;
``` 



[File:ERC20.sol#L20](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/ext/woke/weird/ERC20.sol#L20) 
```solidity
19:    uint8   public decimals = 18;
``` 



[File:Script.sol#L25](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/forge-std/src/Script.sol#L25) 
```solidity
24:    bool public IS_SCRIPT = true;
``` 



[File:ERC4626.prop.sol#L49](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/lib/erc4626-tests/ERC4626.prop.sol#L49) 
```solidity
48:    bool internal _unlimitedAmount;
``` 



[File:ERC4626.prop.sol#L46](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/lib/erc4626-tests/ERC4626.prop.sol#L46) 
```solidity
45:    address internal _vault_;
``` 



[File:ERC4626.prop.sol#L48](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/lib/erc4626-tests/ERC4626.prop.sol#L48) 
```solidity
47:    bool internal _vaultMayBeEmpty;
``` 



[File:ERC4626.prop.sol#L43](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/lib/erc4626-tests/ERC4626.prop.sol#L43) 
```solidity
42:    uint internal _delta_;
``` 



[File:ERC4626.prop.sol#L45](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/lib/erc4626-tests/ERC4626.prop.sol#L45) 
```solidity
44:    address internal _underlying_;
``` 



[File:ERC4626.prop.sol#L45](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/lib/erc4626-tests/ERC4626.prop.sol#L45) 
```solidity
44:    address internal _underlying_;
``` 



[File:ERC4626.prop.sol#L49](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/lib/erc4626-tests/ERC4626.prop.sol#L49) 
```solidity
48:    bool internal _unlimitedAmount;
``` 



[File:ERC4626.prop.sol#L43](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/lib/erc4626-tests/ERC4626.prop.sol#L43) 
```solidity
42:    uint internal _delta_;
``` 



[File:ERC4626.prop.sol#L46](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/lib/erc4626-tests/ERC4626.prop.sol#L46) 
```solidity
45:    address internal _vault_;
``` 



[File:ERC4626.prop.sol#L48](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/lib/erc4626-tests/ERC4626.prop.sol#L48) 
```solidity
47:    bool internal _vaultMayBeEmpty;
``` 



[File:Script.sol#L8](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/test/utils/forge-std/Script.sol#L8) 
```solidity
7:    bool public IS_SCRIPT = true;
``` 



[File:test.sol#L38](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solmate/lib/ds-test/src/test.sol#L38) 
```solidity
37:    bool public IS_TEST = true;
``` 



[File:GovernorVotesQuorumFraction.sol#L19](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/governance/extensions/GovernorVotesQuorumFraction.sol#L19) 
```solidity
18:    uint256 private _quorumNumerator; // DEPRECATED in favor of _quorumNumeratorHistory
``` 



[File:BaseERC20Test.sol#L21](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/test/helpers/BaseERC20Test.sol#L21) 
```solidity
20:  string _name;
``` 



[File:BaseERC20Test.sol#L23](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/test/helpers/BaseERC20Test.sol#L23) 
```solidity
22:  uint8 _decimals;
``` 



[File:BaseERC20Test.sol#L22](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/test/helpers/BaseERC20Test.sol#L22) 
```solidity
21:  string _symbol;
``` 



[File:GovernorVotesQuorumFraction.sol#L19](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/governance/extensions/GovernorVotesQuorumFraction.sol#L19) 
```solidity
18:    uint256 private _quorumNumerator; // DEPRECATED in favor of _quorumNumeratorHistory
``` 



[File:Script.sol#L25](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/forge-std/src/Script.sol#L25) 
```solidity
24:    bool public IS_SCRIPT = true;
``` 



[File:GovernorVotesQuorumFraction.sol#L19](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/governance/extensions/GovernorVotesQuorumFraction.sol#L19) 
```solidity
18:    uint256 private _quorumNumerator; // DEPRECATED in favor of _quorumNumeratorHistory
``` 



[File:test.sol#L38](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/lib/forge-std/lib/ds-test/src/test.sol#L38) 
```solidity
37:    bool public IS_TEST = true;
``` 



[File:Script.sol#L8](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solady/test/utils/forge-std/Script.sol#L8) 
```solidity
7:    bool public IS_SCRIPT = true;
``` 



[File:WildcatMarketController.sol#L53](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/src/WildcatMarketController.sol#L53) 
```solidity
52:  uint256 internal immutable ownCreate2Prefix = LibStoredInitCode.getCreate2Prefix(address(this));
``` 



[File:test.sol#L38](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/forge-std/lib/ds-test/src/test.sol#L38) 
```solidity
37:    bool public IS_TEST = true;
``` 



[File:WETH9.sol#L19](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/mocks/dependencies/weth/WETH9.sol#L19) 
```solidity
18:  string public name = 'Wrapped Ether';
``` 



[File:WETH9.sol#L21](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/mocks/dependencies/weth/WETH9.sol#L21) 
```solidity
20:  uint8 public decimals = 18;
``` 



[File:WETH9.sol#L20](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/mocks/dependencies/weth/WETH9.sol#L20) 
```solidity
19:  string public symbol = 'WETH';
``` 



[File:WildcatSanctionsEscrow.sol#L12](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/src/WildcatSanctionsEscrow.sol#L12) 
```solidity
11:  address public immutable override borrower;
``` 



[File:WildcatSanctionsEscrow.sol#L13](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/src/WildcatSanctionsEscrow.sol#L13) 
```solidity
12:  address public immutable override account;
``` 



[File:WildcatSanctionsEscrow.sol#L14](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/src/WildcatSanctionsEscrow.sol#L14) 
```solidity
13:  address internal immutable asset;
``` 



[File:test.sol#L38](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/forge-std/lib/ds-test/src/test.sol#L38) 
```solidity
37:    bool public IS_TEST = true;
``` 



[File:Script.sol#L8](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/test/utils/forge-std/Script.sol#L8) 
```solidity
7:    bool public IS_SCRIPT = true;
``` 



[File:ERC20Permit.sol#L37](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/token/ERC20/extensions/ERC20Permit.sol#L37) 
```solidity
36:    bytes32 private _PERMIT_TYPEHASH_DEPRECATED_SLOT;
``` 



[File:UUPSUpgradeable.sol#L23](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/proxy/utils/UUPSUpgradeable.sol#L23) 
```solidity
22:    address private immutable __self = address(this);
``` 



[File:ERC4626.prop.sol#L46](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/lib/erc4626-tests/ERC4626.prop.sol#L46) 
```solidity
45:    address internal _vault_;
``` 



[File:ERC4626.prop.sol#L48](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/lib/erc4626-tests/ERC4626.prop.sol#L48) 
```solidity
47:    bool internal _vaultMayBeEmpty;
``` 



[File:ERC4626.prop.sol#L45](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/lib/erc4626-tests/ERC4626.prop.sol#L45) 
```solidity
44:    address internal _underlying_;
``` 



[File:ERC4626.prop.sol#L43](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/lib/erc4626-tests/ERC4626.prop.sol#L43) 
```solidity
42:    uint internal _delta_;
``` 



[File:ERC4626.prop.sol#L49](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/lib/erc4626-tests/ERC4626.prop.sol#L49) 
```solidity
48:    bool internal _unlimitedAmount;
``` 



[File:test.sol#L38](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/forge-std/lib/ds-test/src/test.sol#L38) 
```solidity
37:    bool public IS_TEST = true;
``` 



[File:test.sol#L38](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/lib/forge-std/lib/ds-test/src/test.sol#L38) 
```solidity
37:    bool public IS_TEST = true;
``` 



[File:test.sol#L38](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/lib/ds-test/src/test.sol#L38) 
```solidity
37:    bool public IS_TEST = true;
``` 



[File:bridges.sol#L61](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/crosschain/bridges.sol#L61) 
```solidity
60:    address public immutable bridge = msg.sender;
``` 



[File:bridges.sol#L41](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/crosschain/bridges.sol#L41) 
```solidity
40:    address public immutable inbox = address(new BridgeArbitrumL1Inbox());
``` 



[File:bridges.sol#L43](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/crosschain/bridges.sol#L43) 
```solidity
42:    address public immutable outbox = address(new BridgeArbitrumL1Outbox());
``` 



[File:BaseMarketTest.sol#L20](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/test/BaseMarketTest.sol#L20) 
```solidity
19:  address internal wlUser = address(0x42);
``` 



[File:BaseMarketTest.sol#L18](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/test/BaseMarketTest.sol#L18) 
```solidity
17:  address internal wildcatController = address(0x69);
``` 



[File:BaseMarketTest.sol#L21](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/test/BaseMarketTest.sol#L21) 
```solidity
20:  address internal nonwlUser = address(0x43);
``` 



[File:BaseMarketTest.sol#L19](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/test/BaseMarketTest.sol#L19) 
```solidity
18:  address internal wintermuteController = address(0x70);
``` 



[File:test.sol#L38](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/lib/solmate/lib/ds-test/src/test.sol#L38) 
```solidity
37:    bool public IS_TEST = true;
``` 



[File:ERC20FlashMintHarness.sol#L11](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/certora/harnesses/ERC20FlashMintHarness.sol#L11) 
```solidity
10:    address someFeeReceiver;
``` 



[File:ERC20FlashMintHarness.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/certora/harnesses/ERC20FlashMintHarness.sol#L10) 
```solidity
9:    uint256 someFee;
``` 



[File:test.sol#L38](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solady/lib/solmate/lib/ds-test/src/test.sol#L38) 
```solidity
37:    bool public IS_TEST = true;
``` 



[File:LendingPoolStorage.sol#L31](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/protocol/lendingpool/LendingPoolStorage.sol#L31) 
```solidity
30:  uint256 internal _maxNumberOfReserves;
``` 



[File:LendingPoolStorage.sol#L23](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/protocol/lendingpool/LendingPoolStorage.sol#L23) 
```solidity
22:  uint256 internal _reservesCount;
``` 



[File:LendingPoolStorage.sol#L25](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/protocol/lendingpool/LendingPoolStorage.sol#L25) 
```solidity
24:  bool internal _paused;
``` 



[File:LendingPoolStorage.sol#L29](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/protocol/lendingpool/LendingPoolStorage.sol#L29) 
```solidity
28:  uint256 internal _flashLoanPremiumTotal;
``` 



[File:LendingPoolStorage.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/protocol/lendingpool/LendingPoolStorage.sol#L27) 
```solidity
26:  uint256 internal _maxStableRateBorrowSizePercent;
``` 



[File:ExpectedStateTracker.sol#L33](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/test/helpers/ExpectedStateTracker.sol#L33) 
```solidity
32:  uint256 internal lastTotalAssets;
``` 



[File:test.sol#L38](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/ds-test/src/test.sol#L38) 
```solidity
37:    bool public IS_TEST = true;
``` 



[File:UUPSUpgradeable.sol#L23](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/proxy/utils/UUPSUpgradeable.sol#L23) 
```solidity
22:    address private immutable __self = address(this);
``` 



[File:receivers.sol#L15](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/crosschain/receivers.sol#L15) 
```solidity
14:    address public immutable owner = msg.sender;
``` 



[File:ERC20Permit.sol#L37](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/token/ERC20/extensions/ERC20Permit.sol#L37) 
```solidity
36:    bytes32 private _PERMIT_TYPEHASH_DEPRECATED_SLOT;
``` 



[File:ERC3156FlashBorrowerHarness.sol#L8](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/certora/harnesses/ERC3156FlashBorrowerHarness.sol#L8) 
```solidity
7:    bytes32 somethingToReturn;
``` 



[File:WildcatMarketControllerFactory.sol#L38](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/src/WildcatMarketControllerFactory.sol#L38) 
```solidity
37:  uint256 public immutable marketInitCodeHash;
``` 



[File:WildcatMarketControllerFactory.sol#L42](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/src/WildcatMarketControllerFactory.sol#L42) 
```solidity
41:  uint256 public immutable controllerInitCodeHash;
``` 



[File:WildcatMarketControllerFactory.sol#L36](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/src/WildcatMarketControllerFactory.sol#L36) 
```solidity
35:  address public immutable marketInitCodeStorage;
``` 



[File:WildcatMarketControllerFactory.sol#L40](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/src/WildcatMarketControllerFactory.sol#L40) 
```solidity
39:  address public immutable controllerInitCodeStorage;
``` 



[File:WildcatMarketControllerFactory.sol#L44](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/src/WildcatMarketControllerFactory.sol#L44) 
```solidity
43:  uint256 internal immutable ownCreate2Prefix = LibStoredInitCode.getCreate2Prefix(address(this));
``` 



[File:UUPSUpgradeable.sol#L23](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/proxy/utils/UUPSUpgradeable.sol#L23) 
```solidity
22:    address private immutable __self = address(this);
``` 



[File:SwapFactory.sol#L19](https://github.com/0xKitsune/sstan/blob/main/bin/scope/3xcalibur/scope/SwapFactory.sol#L19) 
```solidity
18:    address internal _temp0;
``` 



[File:SwapFactory.sol#L20](https://github.com/0xKitsune/sstan/blob/main/bin/scope/3xcalibur/scope/SwapFactory.sol#L20) 
```solidity
19:    address internal _temp1;
``` 



[File:SwapFactory.sol#L22](https://github.com/0xKitsune/sstan/blob/main/bin/scope/3xcalibur/scope/SwapFactory.sol#L22) 
```solidity
21:    uint internal _temp3;
``` 



[File:SwapFactory.sol#L21](https://github.com/0xKitsune/sstan/blob/main/bin/scope/3xcalibur/scope/SwapFactory.sol#L21) 
```solidity
20:    bool internal _temp2;
``` 



[File:EIP712.sol#L53](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/utils/cryptography/EIP712.sol#L53) 
```solidity
52:    string private _versionFallback;
``` 



[File:EIP712.sol#L52](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/utils/cryptography/EIP712.sol#L52) 
```solidity
51:    string private _nameFallback;
``` 



[File:Singleton.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/safe-contracts/contracts/common/Singleton.sol#L10) 
```solidity
9:    address private singleton;
``` 



[File:bridges.sol#L61](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/crosschain/bridges.sol#L61) 
```solidity
60:    address public immutable bridge = msg.sender;
``` 



[File:bridges.sol#L41](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/crosschain/bridges.sol#L41) 
```solidity
40:    address public immutable inbox = address(new BridgeArbitrumL1Inbox());
``` 



[File:bridges.sol#L43](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/crosschain/bridges.sol#L43) 
```solidity
42:    address public immutable outbox = address(new BridgeArbitrumL1Outbox());
``` 



[File:GnosisSafeStorage.sol#L18](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/safe-contracts/contracts/examples/libraries/GnosisSafeStorage.sol#L18) 
```solidity
17:    bytes32 internal domainSeparator;
``` 



[File:GnosisSafeStorage.sol#L14](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/safe-contracts/contracts/examples/libraries/GnosisSafeStorage.sol#L14) 
```solidity
13:    uint256 internal threshold;
``` 



[File:GnosisSafeStorage.sol#L8](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/safe-contracts/contracts/examples/libraries/GnosisSafeStorage.sol#L8) 
```solidity
7:    address internal singleton;
``` 



[File:GnosisSafeStorage.sol#L13](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/safe-contracts/contracts/examples/libraries/GnosisSafeStorage.sol#L13) 
```solidity
12:    uint256 internal ownerCount;
``` 



[File:GnosisSafeStorage.sol#L17](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/safe-contracts/contracts/examples/libraries/GnosisSafeStorage.sol#L17) 
```solidity
16:    bytes32 internal nonce;
``` 



 --- 



## Quality Assurance - Total: 0 



