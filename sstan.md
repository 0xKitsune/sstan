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
 | [[NC-0]](#[NC-0]) | Constructor should check that all parameters are not 0 | 592 |

## Vulnerabilities - Total: 0 




## Optimizations - Total: 0 




## Quality Assurance - Total: 592 

<a name=[NC-0]></a>
### [NC-0] Constructor should check that all parameters are not 0 - Instances: 592 

 > Consider adding a require statement to check that all parameters are not 0 in the constructor 

 --- 

[File:BaseImmutableAdminUpgradeabilityProxy.sol#L19](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/protocol/libraries/aave-upgradeability/BaseImmutableAdminUpgradeabilityProxy.sol#L19) 
```solidity
18:  constructor(address admin) public {
``` 



[File:ERC20Capped.sol#L18](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/token/ERC20/extensions/ERC20Capped.sol#L18) 
```solidity
17:    constructor(uint256 cap_) {
``` 



[File:ERC20.sol#L51](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solmate/src/tokens/ERC20.sol#L51) 
```solidity
50:    constructor(
51:        string memory _name,
52:        string memory _symbol,
53:        uint8 _decimals
54:    ) {
``` 



[File:ERC20.sol#L51](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solmate/src/tokens/ERC20.sol#L51) 
```solidity
50:    constructor(
51:        string memory _name,
52:        string memory _symbol,
53:        uint8 _decimals
54:    ) {
``` 



[File:ERC20.sol#L51](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solmate/src/tokens/ERC20.sol#L51) 
```solidity
50:    constructor(
51:        string memory _name,
52:        string memory _symbol,
53:        uint8 _decimals
54:    ) {
``` 



[File:GovernorTimelockControl.sol#L38](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/governance/extensions/GovernorTimelockControl.sol#L38) 
```solidity
37:    constructor(TimelockController timelockAddress) {
``` 



[File:ERC721Wrapper.sol#L20](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/token/ERC721/extensions/ERC721Wrapper.sol#L20) 
```solidity
19:    constructor(IERC721 underlyingToken) {
``` 



[File:MultiRolesAuthority.sol#L25](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/lib/solmate/src/auth/authorities/MultiRolesAuthority.sol#L25) 
```solidity
24:    constructor(address _owner, Authority _authority) Auth(_owner, _authority) {}
``` 



[File:MultiRolesAuthority.sol#L25](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/lib/solmate/src/auth/authorities/MultiRolesAuthority.sol#L25) 
```solidity
24:    constructor(address _owner, Authority _authority) Auth(_owner, _authority) {}
``` 



[File:ERC721.sol#L44](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/token/ERC721/ERC721.sol#L44) 
```solidity
43:    constructor(string memory name_, string memory symbol_) {
``` 



[File:ERC721.sol#L44](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/token/ERC721/ERC721.sol#L44) 
```solidity
43:    constructor(string memory name_, string memory symbol_) {
``` 



[File:InitializableImmutableAdminUpgradeabilityProxy.sol#L15](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/protocol/libraries/aave-upgradeability/InitializableImmutableAdminUpgradeabilityProxy.sol#L15) 
```solidity
14:  constructor(address admin) public BaseImmutableAdminUpgradeabilityProxy(admin) {}
``` 



[File:ExecutorPlugin.sol#L60](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/src/core/ExecutorPlugin.sol#L60) 
```solidity
59:    constructor(address _addressProvider) AddressProviderService(_addressProvider) {}
``` 



[File:MultiRolesAuthority.sol#L25](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solmate/src/auth/authorities/MultiRolesAuthority.sol#L25) 
```solidity
24:    constructor(address _owner, Authority _authority) Auth(_owner, _authority) {}
``` 



[File:MultiRolesAuthority.sol#L25](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solmate/src/auth/authorities/MultiRolesAuthority.sol#L25) 
```solidity
24:    constructor(address _owner, Authority _authority) Auth(_owner, _authority) {}
``` 



[File:TimelockController.sol#L81](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/governance/TimelockController.sol#L81) 
```solidity
80:    constructor(uint256 minDelay, address[] memory proposers, address[] memory executors, address admin) {
``` 



[File:TimelockController.sol#L81](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/governance/TimelockController.sol#L81) 
```solidity
80:    constructor(uint256 minDelay, address[] memory proposers, address[] memory executors, address admin) {
``` 



[File:TimelockController.sol#L81](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/governance/TimelockController.sol#L81) 
```solidity
80:    constructor(uint256 minDelay, address[] memory proposers, address[] memory executors, address admin) {
``` 



[File:ERC20Permit.sol#L44](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/token/ERC20/extensions/ERC20Permit.sol#L44) 
```solidity
43:    constructor(string memory name) EIP712(name, "1") {}
``` 



[File:ConsoleOpBuilder.sol#L21](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/src/core/ConsoleOpBuilder.sol#L21) 
```solidity
20:    constructor(address _addressProvider) AddressProviderService(_addressProvider) {}
``` 



[File:MockParaSwapAugustusRegistry.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/mocks/swap/MockParaSwapAugustusRegistry.sol#L10) 
```solidity
9:  constructor(address augustus) public {
``` 



[File:NoRevert.sol#L20](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/ext/woke/weird/NoRevert.sol#L20) 
```solidity
19:    constructor(uint _totalSupply) public {
``` 



[File:ConsoleFactory.s.sol#L62](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/script/utils/ConsoleFactory.s.sol#L62) 
```solidity
61:    constructor(string memory _addressManagerPath) AddressRegistry(_addressManagerPath) {}
``` 



[File:UniswapLiquiditySwapAdapter.sol#L32](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/adapters/UniswapLiquiditySwapAdapter.sol#L32) 
```solidity
31:  constructor(
32:    ILendingPoolAddressesProvider addressesProvider,
33:    IUniswapV2Router02 uniswapRouter,
34:    address wethAddress
35:  ) public BaseUniswapAdapter(addressesProvider, uniswapRouter, wethAddress) {}
``` 



[File:UniswapLiquiditySwapAdapter.sol#L32](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/adapters/UniswapLiquiditySwapAdapter.sol#L32) 
```solidity
31:  constructor(
32:    ILendingPoolAddressesProvider addressesProvider,
33:    IUniswapV2Router02 uniswapRouter,
34:    address wethAddress
35:  ) public BaseUniswapAdapter(addressesProvider, uniswapRouter, wethAddress) {}
``` 



[File:UniswapLiquiditySwapAdapter.sol#L32](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/adapters/UniswapLiquiditySwapAdapter.sol#L32) 
```solidity
31:  constructor(
32:    ILendingPoolAddressesProvider addressesProvider,
33:    IUniswapV2Router02 uniswapRouter,
34:    address wethAddress
35:  ) public BaseUniswapAdapter(addressesProvider, uniswapRouter, wethAddress) {}
``` 



[File:receivers.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/crosschain/receivers.sol#L27) 
```solidity
26:    constructor(address bridge) CrossChainEnabledAMB(bridge) {}
``` 



[File:receivers.sol#L35](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/crosschain/receivers.sol#L35) 
```solidity
34:    constructor(address bridge) CrossChainEnabledArbitrumL1(bridge) {}
``` 



[File:receivers.sol#L45](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/crosschain/receivers.sol#L45) 
```solidity
44:    constructor(address bridge) CrossChainEnabledOptimism(bridge) {}
``` 



[File:receivers.sol#L53](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/crosschain/receivers.sol#L53) 
```solidity
52:    constructor(address bridge) CrossChainEnabledPolygonChild(bridge) {}
``` 



[File:ERC721.sol#L57](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solmate/src/tokens/ERC721.sol#L57) 
```solidity
56:    constructor(string memory _name, string memory _symbol) {
``` 



[File:ERC721.sol#L57](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solmate/src/tokens/ERC721.sol#L57) 
```solidity
56:    constructor(string memory _name, string memory _symbol) {
``` 



[File:DelegateCallTransactionGuard.sol#L11](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/safe-contracts/contracts/examples/guards/DelegateCallTransactionGuard.sol#L11) 
```solidity
10:    constructor(address target) {
``` 



[File:ERC3156FlashBorrowerMock.sol#L25](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/ERC3156FlashBorrowerMock.sol#L25) 
```solidity
24:    constructor(bool enableReturn, bool enableApprove) {
``` 



[File:ERC3156FlashBorrowerMock.sol#L25](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/ERC3156FlashBorrowerMock.sol#L25) 
```solidity
24:    constructor(bool enableReturn, bool enableApprove) {
``` 



[File:AddressProvider.sol#L43](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/src/core/AddressProvider.sol#L43) 
```solidity
42:    constructor(address _governance) {
``` 



[File:ATokensAndRatesHelper.sol#L38](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/deployments/ATokensAndRatesHelper.sol#L38) 
```solidity
37:  constructor(
38:    address payable _pool,
39:    address _addressesProvider,
40:    address _poolConfigurator
41:  ) public {
``` 



[File:ATokensAndRatesHelper.sol#L38](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/deployments/ATokensAndRatesHelper.sol#L38) 
```solidity
37:  constructor(
38:    address payable _pool,
39:    address _addressesProvider,
40:    address _poolConfigurator
41:  ) public {
``` 



[File:ATokensAndRatesHelper.sol#L38](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/deployments/ATokensAndRatesHelper.sol#L38) 
```solidity
37:  constructor(
38:    address payable _pool,
39:    address _addressesProvider,
40:    address _poolConfigurator
41:  ) public {
``` 



[File:ERC1967Proxy.sol#L22](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/proxy/ERC1967/ERC1967Proxy.sol#L22) 
```solidity
21:    constructor(address _logic, bytes memory _data) payable {
``` 



[File:ERC1967Proxy.sol#L22](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/proxy/ERC1967/ERC1967Proxy.sol#L22) 
```solidity
21:    constructor(address _logic, bytes memory _data) payable {
``` 



[File:CompTimelock.sol#L68](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/compound/CompTimelock.sol#L68) 
```solidity
67:    constructor(address admin_, uint256 delay_) {
``` 



[File:CompTimelock.sol#L68](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/compound/CompTimelock.sol#L68) 
```solidity
67:    constructor(address admin_, uint256 delay_) {
``` 



[File:MintableERC20.sol#L11](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/mocks/tokens/MintableERC20.sol#L11) 
```solidity
10:  constructor(
11:    string memory name,
12:    string memory symbol,
13:    uint8 decimals
14:  ) public ERC20(name, symbol) {
``` 



[File:MintableERC20.sol#L11](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/mocks/tokens/MintableERC20.sol#L11) 
```solidity
10:  constructor(
11:    string memory name,
12:    string memory symbol,
13:    uint8 decimals
14:  ) public ERC20(name, symbol) {
``` 



[File:MintableERC20.sol#L11](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/mocks/tokens/MintableERC20.sol#L11) 
```solidity
10:  constructor(
11:    string memory name,
12:    string memory symbol,
13:    uint8 decimals
14:  ) public ERC20(name, symbol) {
``` 



[File:ERC4626.sol#L34](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solady/lib/solmate/src/mixins/ERC4626.sol#L34) 
```solidity
33:    constructor(
34:        ERC20 _asset,
35:        string memory _name,
36:        string memory _symbol
37:    ) ERC20(_name, _symbol, _asset.decimals()) {
``` 



[File:ERC4626.sol#L34](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solady/lib/solmate/src/mixins/ERC4626.sol#L34) 
```solidity
33:    constructor(
34:        ERC20 _asset,
35:        string memory _name,
36:        string memory _symbol
37:    ) ERC20(_name, _symbol, _asset.decimals()) {
``` 



[File:ERC4626.sol#L34](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solady/lib/solmate/src/mixins/ERC4626.sol#L34) 
```solidity
33:    constructor(
34:        ERC20 _asset,
35:        string memory _name,
36:        string memory _symbol
37:    ) ERC20(_name, _symbol, _asset.decimals()) {
``` 



[File:ERC721Wrapper.sol#L20](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/token/ERC721/extensions/ERC721Wrapper.sol#L20) 
```solidity
19:    constructor(IERC721 underlyingToken) {
``` 



[File:ERC20DecimalsMock.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/token/ERC20DecimalsMock.sol#L10) 
```solidity
9:    constructor(uint8 decimals_) {
``` 



[File:RolesAuthority.sol#L24](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/lib/solmate/src/auth/authorities/RolesAuthority.sol#L24) 
```solidity
23:    constructor(address _owner, Authority _authority) Auth(_owner, _authority) {}
``` 



[File:RolesAuthority.sol#L24](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/lib/solmate/src/auth/authorities/RolesAuthority.sol#L24) 
```solidity
23:    constructor(address _owner, Authority _authority) Auth(_owner, _authority) {}
``` 



[File:StableDebtTokenHarness.sol#L7](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/specs/harness/StableDebtTokenHarness.sol#L7) 
```solidity
6:  constructor(
7:    address pool,
8:    address underlyingAsset,
9:    string memory name,
10:    string memory symbol,
11:    address incentivesController
12:  ) public StableDebtToken(pool, underlyingAsset, name, symbol, incentivesController) {}
``` 



[File:StableDebtTokenHarness.sol#L7](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/specs/harness/StableDebtTokenHarness.sol#L7) 
```solidity
6:  constructor(
7:    address pool,
8:    address underlyingAsset,
9:    string memory name,
10:    string memory symbol,
11:    address incentivesController
12:  ) public StableDebtToken(pool, underlyingAsset, name, symbol, incentivesController) {}
``` 



[File:StableDebtTokenHarness.sol#L7](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/specs/harness/StableDebtTokenHarness.sol#L7) 
```solidity
6:  constructor(
7:    address pool,
8:    address underlyingAsset,
9:    string memory name,
10:    string memory symbol,
11:    address incentivesController
12:  ) public StableDebtToken(pool, underlyingAsset, name, symbol, incentivesController) {}
``` 



[File:StableDebtTokenHarness.sol#L7](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/specs/harness/StableDebtTokenHarness.sol#L7) 
```solidity
6:  constructor(
7:    address pool,
8:    address underlyingAsset,
9:    string memory name,
10:    string memory symbol,
11:    address incentivesController
12:  ) public StableDebtToken(pool, underlyingAsset, name, symbol, incentivesController) {}
``` 



[File:StableDebtTokenHarness.sol#L7](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/specs/harness/StableDebtTokenHarness.sol#L7) 
```solidity
6:  constructor(
7:    address pool,
8:    address underlyingAsset,
9:    string memory name,
10:    string memory symbol,
11:    address incentivesController
12:  ) public StableDebtToken(pool, underlyingAsset, name, symbol, incentivesController) {}
``` 



[File:ERC1155Mock.sol#L11](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/ext/woke/ERC1155Mock.sol#L11) 
```solidity
10:    constructor(bool enableHooks_) {
``` 



[File:ERC3156FlashBorrowerMock.sol#L25](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/ERC3156FlashBorrowerMock.sol#L25) 
```solidity
24:    constructor(bool enableReturn, bool enableApprove) {
``` 



[File:ERC3156FlashBorrowerMock.sol#L25](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/ERC3156FlashBorrowerMock.sol#L25) 
```solidity
24:    constructor(bool enableReturn, bool enableApprove) {
``` 



[File:FlashLiquidationAdapter.sol#L43](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/adapters/FlashLiquidationAdapter.sol#L43) 
```solidity
42:  constructor(
43:    ILendingPoolAddressesProvider addressesProvider,
44:    IUniswapV2Router02 uniswapRouter,
45:    address wethAddress
46:  ) public BaseUniswapAdapter(addressesProvider, uniswapRouter, wethAddress) {}
``` 



[File:FlashLiquidationAdapter.sol#L43](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/adapters/FlashLiquidationAdapter.sol#L43) 
```solidity
42:  constructor(
43:    ILendingPoolAddressesProvider addressesProvider,
44:    IUniswapV2Router02 uniswapRouter,
45:    address wethAddress
46:  ) public BaseUniswapAdapter(addressesProvider, uniswapRouter, wethAddress) {}
``` 



[File:FlashLiquidationAdapter.sol#L43](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/adapters/FlashLiquidationAdapter.sol#L43) 
```solidity
42:  constructor(
43:    ILendingPoolAddressesProvider addressesProvider,
44:    IUniswapV2Router02 uniswapRouter,
45:    address wethAddress
46:  ) public BaseUniswapAdapter(addressesProvider, uniswapRouter, wethAddress) {}
``` 



[File:ERC20FlashMintHarness.sol#L13](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/certora/harnesses/ERC20FlashMintHarness.sol#L13) 
```solidity
12:    constructor(string memory name, string memory symbol) ERC20(name, symbol) ERC20Permit(name) {}
``` 



[File:ERC20FlashMintHarness.sol#L13](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/certora/harnesses/ERC20FlashMintHarness.sol#L13) 
```solidity
12:    constructor(string memory name, string memory symbol) ERC20(name, symbol) ERC20Permit(name) {}
``` 



[File:TokenTimelock.sol#L32](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/token/ERC20/utils/TokenTimelock.sol#L32) 
```solidity
31:    constructor(IERC20 token_, address beneficiary_, uint256 releaseTime_) {
``` 



[File:TokenTimelock.sol#L32](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/token/ERC20/utils/TokenTimelock.sol#L32) 
```solidity
31:    constructor(IERC20 token_, address beneficiary_, uint256 releaseTime_) {
``` 



[File:TokenTimelock.sol#L32](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/token/ERC20/utils/TokenTimelock.sol#L32) 
```solidity
31:    constructor(IERC20 token_, address beneficiary_, uint256 releaseTime_) {
``` 



[File:ERC20FlashMintHarness.sol#L13](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/certora/harnesses/ERC20FlashMintHarness.sol#L13) 
```solidity
12:    constructor(string memory name, string memory symbol) ERC20(name, symbol) ERC20Permit(name) {}
``` 



[File:ERC20FlashMintHarness.sol#L13](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/certora/harnesses/ERC20FlashMintHarness.sol#L13) 
```solidity
12:    constructor(string memory name, string memory symbol) ERC20(name, symbol) ERC20Permit(name) {}
``` 



[File:ExecutorRegistry.sol#L29](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/src/core/registries/ExecutorRegistry.sol#L29) 
```solidity
28:    constructor(address _addressProvider) AddressProviderService(_addressProvider) {}
``` 



[File:SafeDeployer.sol#L42](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/src/core/SafeDeployer.sol#L42) 
```solidity
41:    constructor(address _addressProvider) AddressProviderService(_addressProvider) {}
``` 



[File:ERC20PresetMinterPauser.sol#L38](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/token/ERC20/presets/ERC20PresetMinterPauser.sol#L38) 
```solidity
37:    constructor(string memory name, string memory symbol) ERC20(name, symbol) {
``` 



[File:ERC20PresetMinterPauser.sol#L38](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/token/ERC20/presets/ERC20PresetMinterPauser.sol#L38) 
```solidity
37:    constructor(string memory name, string memory symbol) ERC20(name, symbol) {
``` 



[File:Bytes32Metadata.sol#L30](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/ext/woke/weird/Bytes32Metadata.sol#L30) 
```solidity
29:    constructor(uint _totalSupply) public {
``` 



[File:ERC721Harness.sol#L8](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/certora/harnesses/ERC721Harness.sol#L8) 
```solidity
7:    constructor(string memory name, string memory symbol) ERC721(name, symbol) {}
``` 



[File:ERC721Harness.sol#L8](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/certora/harnesses/ERC721Harness.sol#L8) 
```solidity
7:    constructor(string memory name, string memory symbol) ERC721(name, symbol) {}
``` 



[File:ERC2771ContextMock.sol#L11](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/ERC2771ContextMock.sol#L11) 
```solidity
10:    constructor(address trustedForwarder) ERC2771Context(trustedForwarder) {
``` 



[File:SafeModerator.sol#L17](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/src/core/SafeModerator.sol#L17) 
```solidity
16:    constructor(address _addressProvider) AddressProviderService(_addressProvider) {}
``` 



[File:TokenTimelock.sol#L32](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/token/ERC20/utils/TokenTimelock.sol#L32) 
```solidity
31:    constructor(IERC20 token_, address beneficiary_, uint256 releaseTime_) {
``` 



[File:TokenTimelock.sol#L32](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/token/ERC20/utils/TokenTimelock.sol#L32) 
```solidity
31:    constructor(IERC20 token_, address beneficiary_, uint256 releaseTime_) {
``` 



[File:TokenTimelock.sol#L32](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/token/ERC20/utils/TokenTimelock.sol#L32) 
```solidity
31:    constructor(IERC20 token_, address beneficiary_, uint256 releaseTime_) {
``` 



[File:BaseParaSwapSellAdapter.sol#L22](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/adapters/BaseParaSwapSellAdapter.sol#L22) 
```solidity
21:  constructor(
22:    ILendingPoolAddressesProvider addressesProvider,
23:    IParaSwapAugustusRegistry augustusRegistry
24:  ) public BaseParaSwapAdapter(addressesProvider) {
``` 



[File:BaseParaSwapSellAdapter.sol#L22](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/adapters/BaseParaSwapSellAdapter.sol#L22) 
```solidity
21:  constructor(
22:    ILendingPoolAddressesProvider addressesProvider,
23:    IParaSwapAugustusRegistry augustusRegistry
24:  ) public BaseParaSwapAdapter(addressesProvider) {
``` 



[File:ERC20.sol#L51](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solmate/src/tokens/ERC20.sol#L51) 
```solidity
50:    constructor(
51:        string memory _name,
52:        string memory _symbol,
53:        uint8 _decimals
54:    ) {
``` 



[File:ERC20.sol#L51](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solmate/src/tokens/ERC20.sol#L51) 
```solidity
50:    constructor(
51:        string memory _name,
52:        string memory _symbol,
53:        uint8 _decimals
54:    ) {
``` 



[File:ERC20.sol#L51](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solmate/src/tokens/ERC20.sol#L51) 
```solidity
50:    constructor(
51:        string memory _name,
52:        string memory _symbol,
53:        uint8 _decimals
54:    ) {
``` 



[File:Bytes32Metadata.sol#L30](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/ext/woke/weird/Bytes32Metadata.sol#L30) 
```solidity
29:    constructor(uint _totalSupply) public {
``` 



[File:SwapPair.sol#L92](https://github.com/0xKitsune/sstan/blob/main/bin/scope/3xcalibur/scope/SwapPair.sol#L92) 
```solidity
91:    constructor() {
``` 



[File:SwapPair.sol#L92](https://github.com/0xKitsune/sstan/blob/main/bin/scope/3xcalibur/scope/SwapPair.sol#L92) 
```solidity
91:    constructor() {
``` 



[File:SwapPair.sol#L92](https://github.com/0xKitsune/sstan/blob/main/bin/scope/3xcalibur/scope/SwapPair.sol#L92) 
```solidity
91:    constructor() {
``` 



[File:SwapPair.sol#L92](https://github.com/0xKitsune/sstan/blob/main/bin/scope/3xcalibur/scope/SwapPair.sol#L92) 
```solidity
91:    constructor() {
``` 



[File:CompTimelock.sol#L68](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/compound/CompTimelock.sol#L68) 
```solidity
67:    constructor(address admin_, uint256 delay_) {
``` 



[File:CompTimelock.sol#L68](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/compound/CompTimelock.sol#L68) 
```solidity
67:    constructor(address admin_, uint256 delay_) {
``` 



[File:ERC4646FeesMock.sol#L13](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/token/ERC4646FeesMock.sol#L13) 
```solidity
12:    constructor(
13:        uint256 entryFeeBasePoint,
14:        address entryFeeRecipient,
15:        uint256 exitFeeBasePoint,
16:        address exitFeeRecipient
17:    ) {
``` 



[File:ERC4646FeesMock.sol#L13](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/token/ERC4646FeesMock.sol#L13) 
```solidity
12:    constructor(
13:        uint256 entryFeeBasePoint,
14:        address entryFeeRecipient,
15:        uint256 exitFeeBasePoint,
16:        address exitFeeRecipient
17:    ) {
``` 



[File:ERC4646FeesMock.sol#L13](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/token/ERC4646FeesMock.sol#L13) 
```solidity
12:    constructor(
13:        uint256 entryFeeBasePoint,
14:        address entryFeeRecipient,
15:        uint256 exitFeeBasePoint,
16:        address exitFeeRecipient
17:    ) {
``` 



[File:ERC4646FeesMock.sol#L13](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/token/ERC4646FeesMock.sol#L13) 
```solidity
12:    constructor(
13:        uint256 entryFeeBasePoint,
14:        address entryFeeRecipient,
15:        uint256 exitFeeBasePoint,
16:        address exitFeeRecipient
17:    ) {
``` 



[File:GovernorVotesComp.sol#L17](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/governance/extensions/GovernorVotesComp.sol#L17) 
```solidity
16:    constructor(ERC20VotesComp token_) {
``` 



[File:ERC20Permit.sol#L44](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/token/ERC20/extensions/ERC20Permit.sol#L44) 
```solidity
43:    constructor(string memory name) EIP712(name, "1") {}
``` 



[File:HighDecimals.sol#L9](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/ext/woke/weird/HighDecimals.sol#L9) 
```solidity
8:    constructor(uint _totalSupply) ERC20(_totalSupply) public {
``` 



[File:RevertToZero.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/ext/woke/weird/RevertToZero.sol#L10) 
```solidity
9:    constructor(uint _totalSupply) ERC20(_totalSupply) public {}
``` 



[File:ERC721ReceiverMock.sol#L20](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/token/ERC721ReceiverMock.sol#L20) 
```solidity
19:    constructor(bytes4 retval, Error error) {
``` 



[File:ERC721ReceiverMock.sol#L20](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/token/ERC721ReceiverMock.sol#L20) 
```solidity
19:    constructor(bytes4 retval, Error error) {
``` 



[File:AaveProtocolDataProvider.sol#L28](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/misc/AaveProtocolDataProvider.sol#L28) 
```solidity
27:  constructor(ILendingPoolAddressesProvider addressesProvider) public {
``` 



[File:ERC4626.sol#L59](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/token/ERC20/extensions/ERC4626.sol#L59) 
```solidity
58:    constructor(IERC20 asset_) {
``` 



[File:ERC4626.sol#L59](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/token/ERC20/extensions/ERC4626.sol#L59) 
```solidity
58:    constructor(IERC20 asset_) {
``` 



[File:ERC4626.sol#L59](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/token/ERC20/extensions/ERC4626.sol#L59) 
```solidity
58:    constructor(IERC20 asset_) {
``` 



[File:MyGovernor.sol#L17](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/docs/governance/MyGovernor.sol#L17) 
```solidity
16:    constructor(
17:        IVotes _token,
18:        TimelockController _timelock
19:    ) Governor("MyGovernor") GovernorVotes(_token) GovernorVotesQuorumFraction(4) GovernorTimelockControl(_timelock) {}
``` 



[File:MyGovernor.sol#L17](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/docs/governance/MyGovernor.sol#L17) 
```solidity
16:    constructor(
17:        IVotes _token,
18:        TimelockController _timelock
19:    ) Governor("MyGovernor") GovernorVotes(_token) GovernorVotesQuorumFraction(4) GovernorTimelockControl(_timelock) {}
``` 



[File:TokenTimelock.sol#L32](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/token/ERC20/utils/TokenTimelock.sol#L32) 
```solidity
31:    constructor(IERC20 token_, address beneficiary_, uint256 releaseTime_) {
``` 



[File:TokenTimelock.sol#L32](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/token/ERC20/utils/TokenTimelock.sol#L32) 
```solidity
31:    constructor(IERC20 token_, address beneficiary_, uint256 releaseTime_) {
``` 



[File:TokenTimelock.sol#L32](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/token/ERC20/utils/TokenTimelock.sol#L32) 
```solidity
31:    constructor(IERC20 token_, address beneficiary_, uint256 releaseTime_) {
``` 



[File:ERC2771ContextMock.sol#L11](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/ERC2771ContextMock.sol#L11) 
```solidity
10:    constructor(address trustedForwarder) ERC2771Context(trustedForwarder) {
``` 



[File:WildcatSanctionsSentinel.sol#L24](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/src/WildcatSanctionsSentinel.sol#L24) 
```solidity
23:  constructor(address _archController, address _chainalysisSanctionsList) {
``` 



[File:WildcatSanctionsSentinel.sol#L24](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/src/WildcatSanctionsSentinel.sol#L24) 
```solidity
23:  constructor(address _archController, address _chainalysisSanctionsList) {
``` 



[File:ERC20.sol#L54](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/token/ERC20/ERC20.sol#L54) 
```solidity
53:    constructor(string memory name_, string memory symbol_) {
``` 



[File:ERC20.sol#L54](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/token/ERC20/ERC20.sol#L54) 
```solidity
53:    constructor(string memory name_, string memory symbol_) {
``` 



[File:RevertZero.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/ext/woke/weird/RevertZero.sol#L10) 
```solidity
9:    constructor(uint _totalSupply) ERC20(_totalSupply) public {}
``` 



[File:DaiPermit.sol#L41](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/ext/woke/weird/DaiPermit.sol#L41) 
```solidity
40:    constructor(uint _totalSupply) public {
``` 



[File:ERC2771ContextMock.sol#L11](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/ERC2771ContextMock.sol#L11) 
```solidity
10:    constructor(address trustedForwarder) ERC2771Context(trustedForwarder) {
``` 



[File:Approval.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/ext/woke/weird/Approval.sol#L10) 
```solidity
9:    constructor(uint _totalSupply) ERC20(_totalSupply) public {}
``` 



[File:CrossChainEnabledPolygonChild.sol#L32](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/crosschain/polygon/CrossChainEnabledPolygonChild.sol#L32) 
```solidity
31:    constructor(address fxChild) {
``` 



[File:ERC1271WalletMock.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/ERC1271WalletMock.sol#L10) 
```solidity
9:    constructor(address originalOwner) {
``` 



[File:ERC4626.sol#L34](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solmate/src/mixins/ERC4626.sol#L34) 
```solidity
33:    constructor(
34:        ERC20 _asset,
35:        string memory _name,
36:        string memory _symbol
37:    ) ERC20(_name, _symbol, _asset.decimals()) {
``` 



[File:ERC4626.sol#L34](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solmate/src/mixins/ERC4626.sol#L34) 
```solidity
33:    constructor(
34:        ERC20 _asset,
35:        string memory _name,
36:        string memory _symbol
37:    ) ERC20(_name, _symbol, _asset.decimals()) {
``` 



[File:ERC4626.sol#L34](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solmate/src/mixins/ERC4626.sol#L34) 
```solidity
33:    constructor(
34:        ERC20 _asset,
35:        string memory _name,
36:        string memory _symbol
37:    ) ERC20(_name, _symbol, _asset.decimals()) {
``` 



[File:GovernorPreventLateQuorum.sol#L37](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/governance/extensions/GovernorPreventLateQuorum.sol#L37) 
```solidity
36:    constructor(uint64 initialVoteExtension) {
``` 



[File:GovernorPreventLateQuorumMock.sol#L18](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/governance/GovernorPreventLateQuorumMock.sol#L18) 
```solidity
17:    constructor(uint256 quorum_) {
``` 



[File:EIP712.sol#L67](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/utils/cryptography/EIP712.sol#L67) 
```solidity
66:    constructor(string memory name, string memory version) {
``` 



[File:EIP712.sol#L67](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/utils/cryptography/EIP712.sol#L67) 
```solidity
66:    constructor(string memory name, string memory version) {
``` 



[File:ERC20.sol#L30](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/ext/woke/weird/ERC20.sol#L30) 
```solidity
29:    constructor(uint _totalSupply) public {
``` 



[File:Owned.sol#L29](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solady/lib/solmate/src/auth/Owned.sol#L29) 
```solidity
28:    constructor(address _owner) {
``` 



[File:receivers.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/crosschain/receivers.sol#L27) 
```solidity
26:    constructor(address bridge) CrossChainEnabledAMB(bridge) {}
``` 



[File:receivers.sol#L35](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/crosschain/receivers.sol#L35) 
```solidity
34:    constructor(address bridge) CrossChainEnabledArbitrumL1(bridge) {}
``` 



[File:receivers.sol#L45](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/crosschain/receivers.sol#L45) 
```solidity
44:    constructor(address bridge) CrossChainEnabledOptimism(bridge) {}
``` 



[File:receivers.sol#L53](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/crosschain/receivers.sol#L53) 
```solidity
52:    constructor(address bridge) CrossChainEnabledPolygonChild(bridge) {}
``` 



[File:BaseParaSwapAdapter.sol#L42](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/adapters/BaseParaSwapAdapter.sol#L42) 
```solidity
41:  constructor(
42:    ILendingPoolAddressesProvider addressesProvider
43:  ) public FlashLoanReceiverBase(addressesProvider) {
``` 



[File:ERC4626OffsetMock.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/token/ERC4626OffsetMock.sol#L10) 
```solidity
9:    constructor(uint8 offset_) {
``` 



[File:MyGovernor1.sol#L17](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/wizard/MyGovernor1.sol#L17) 
```solidity
16:    constructor(
17:        IVotes _token,
18:        TimelockController _timelock
19:    ) Governor("MyGovernor") GovernorVotes(_token) GovernorVotesQuorumFraction(4) GovernorTimelockControl(_timelock) {}
``` 



[File:MyGovernor1.sol#L17](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/wizard/MyGovernor1.sol#L17) 
```solidity
16:    constructor(
17:        IVotes _token,
18:        TimelockController _timelock
19:    ) Governor("MyGovernor") GovernorVotes(_token) GovernorVotesQuorumFraction(4) GovernorTimelockControl(_timelock) {}
``` 



[File:DefaultReserveInterestRateStrategy.sol#L57](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/protocol/lendingpool/DefaultReserveInterestRateStrategy.sol#L57) 
```solidity
56:  constructor(
57:    ILendingPoolAddressesProvider provider,
58:    uint256 optimalUtilizationRate,
59:    uint256 baseVariableBorrowRate,
60:    uint256 variableRateSlope1,
61:    uint256 variableRateSlope2,
62:    uint256 stableRateSlope1,
63:    uint256 stableRateSlope2
64:  ) public {
``` 



[File:DefaultReserveInterestRateStrategy.sol#L57](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/protocol/lendingpool/DefaultReserveInterestRateStrategy.sol#L57) 
```solidity
56:  constructor(
57:    ILendingPoolAddressesProvider provider,
58:    uint256 optimalUtilizationRate,
59:    uint256 baseVariableBorrowRate,
60:    uint256 variableRateSlope1,
61:    uint256 variableRateSlope2,
62:    uint256 stableRateSlope1,
63:    uint256 stableRateSlope2
64:  ) public {
``` 



[File:DefaultReserveInterestRateStrategy.sol#L57](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/protocol/lendingpool/DefaultReserveInterestRateStrategy.sol#L57) 
```solidity
56:  constructor(
57:    ILendingPoolAddressesProvider provider,
58:    uint256 optimalUtilizationRate,
59:    uint256 baseVariableBorrowRate,
60:    uint256 variableRateSlope1,
61:    uint256 variableRateSlope2,
62:    uint256 stableRateSlope1,
63:    uint256 stableRateSlope2
64:  ) public {
``` 



[File:DefaultReserveInterestRateStrategy.sol#L57](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/protocol/lendingpool/DefaultReserveInterestRateStrategy.sol#L57) 
```solidity
56:  constructor(
57:    ILendingPoolAddressesProvider provider,
58:    uint256 optimalUtilizationRate,
59:    uint256 baseVariableBorrowRate,
60:    uint256 variableRateSlope1,
61:    uint256 variableRateSlope2,
62:    uint256 stableRateSlope1,
63:    uint256 stableRateSlope2
64:  ) public {
``` 



[File:DefaultReserveInterestRateStrategy.sol#L57](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/protocol/lendingpool/DefaultReserveInterestRateStrategy.sol#L57) 
```solidity
56:  constructor(
57:    ILendingPoolAddressesProvider provider,
58:    uint256 optimalUtilizationRate,
59:    uint256 baseVariableBorrowRate,
60:    uint256 variableRateSlope1,
61:    uint256 variableRateSlope2,
62:    uint256 stableRateSlope1,
63:    uint256 stableRateSlope2
64:  ) public {
``` 



[File:DefaultReserveInterestRateStrategy.sol#L57](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/protocol/lendingpool/DefaultReserveInterestRateStrategy.sol#L57) 
```solidity
56:  constructor(
57:    ILendingPoolAddressesProvider provider,
58:    uint256 optimalUtilizationRate,
59:    uint256 baseVariableBorrowRate,
60:    uint256 variableRateSlope1,
61:    uint256 variableRateSlope2,
62:    uint256 stableRateSlope1,
63:    uint256 stableRateSlope2
64:  ) public {
``` 



[File:DefaultReserveInterestRateStrategy.sol#L57](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/protocol/lendingpool/DefaultReserveInterestRateStrategy.sol#L57) 
```solidity
56:  constructor(
57:    ILendingPoolAddressesProvider provider,
58:    uint256 optimalUtilizationRate,
59:    uint256 baseVariableBorrowRate,
60:    uint256 variableRateSlope1,
61:    uint256 variableRateSlope2,
62:    uint256 stableRateSlope1,
63:    uint256 stableRateSlope2
64:  ) public {
``` 



[File:AccessControlDefaultAdminRules.sol#L55](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/access/AccessControlDefaultAdminRules.sol#L55) 
```solidity
54:    constructor(uint48 initialDelay, address initialDefaultAdmin) {
``` 



[File:ERC721.sol#L57](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solady/lib/solmate/src/tokens/ERC721.sol#L57) 
```solidity
56:    constructor(string memory _name, string memory _symbol) {
``` 



[File:ERC721.sol#L57](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solady/lib/solmate/src/tokens/ERC721.sol#L57) 
```solidity
56:    constructor(string memory _name, string memory _symbol) {
``` 



[File:LendingPoolAddressesProvider.sol#L31](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/protocol/configuration/LendingPoolAddressesProvider.sol#L31) 
```solidity
30:  constructor(string memory marketId) public {
``` 



[File:ERC20.sol#L54](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/token/ERC20/ERC20.sol#L54) 
```solidity
53:    constructor(string memory name_, string memory symbol_) {
``` 



[File:ERC20.sol#L54](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/token/ERC20/ERC20.sol#L54) 
```solidity
53:    constructor(string memory name_, string memory symbol_) {
``` 



[File:Owned.sol#L29](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solmate/src/auth/Owned.sol#L29) 
```solidity
28:    constructor(address _owner) {
``` 



[File:ERC20Capped.sol#L18](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/token/ERC20/extensions/ERC20Capped.sol#L18) 
```solidity
17:    constructor(uint256 cap_) {
``` 



[File:AccessControlDefaultAdminRules.sol#L55](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/access/AccessControlDefaultAdminRules.sol#L55) 
```solidity
54:    constructor(uint48 initialDelay, address initialDefaultAdmin) {
``` 



[File:TransparentUpgradeableProxy.sol#L63](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/proxy/transparent/TransparentUpgradeableProxy.sol#L63) 
```solidity
62:    constructor(address _logic, address admin_, bytes memory _data) payable ERC1967Proxy(_logic, _data) {
``` 



[File:TransparentUpgradeableProxy.sol#L63](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/proxy/transparent/TransparentUpgradeableProxy.sol#L63) 
```solidity
62:    constructor(address _logic, address admin_, bytes memory _data) payable ERC1967Proxy(_logic, _data) {
``` 



[File:TransparentUpgradeableProxy.sol#L63](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/proxy/transparent/TransparentUpgradeableProxy.sol#L63) 
```solidity
62:    constructor(address _logic, address admin_, bytes memory _data) payable ERC1967Proxy(_logic, _data) {
``` 



[File:AccessControlDefaultAdminRulesHarness.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/certora/harnesses/AccessControlDefaultAdminRulesHarness.sol#L10) 
```solidity
9:    constructor(
10:        uint48 initialDelay,
11:        address initialDefaultAdmin,
12:        uint48 delayIncreaseWait
13:    ) AccessControlDefaultAdminRules(initialDelay, initialDefaultAdmin) {
``` 



[File:AccessControlDefaultAdminRulesHarness.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/certora/harnesses/AccessControlDefaultAdminRulesHarness.sol#L10) 
```solidity
9:    constructor(
10:        uint48 initialDelay,
11:        address initialDefaultAdmin,
12:        uint48 delayIncreaseWait
13:    ) AccessControlDefaultAdminRules(initialDelay, initialDefaultAdmin) {
``` 



[File:AccessControlDefaultAdminRulesHarness.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/certora/harnesses/AccessControlDefaultAdminRulesHarness.sol#L10) 
```solidity
9:    constructor(
10:        uint48 initialDelay,
11:        address initialDefaultAdmin,
12:        uint48 delayIncreaseWait
13:    ) AccessControlDefaultAdminRules(initialDelay, initialDefaultAdmin) {
``` 



[File:ConsoleFallbackHandler.sol#L29](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/src/core/ConsoleFallbackHandler.sol#L29) 
```solidity
28:    constructor(address _addressProvider) AddressProviderService(_addressProvider) {}
``` 



[File:ERC20PermitHarness.sol#L8](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/certora/harnesses/ERC20PermitHarness.sol#L8) 
```solidity
7:    constructor(string memory name, string memory symbol) ERC20(name, symbol) ERC20Permit(name) {}
``` 



[File:ERC20PermitHarness.sol#L8](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/certora/harnesses/ERC20PermitHarness.sol#L8) 
```solidity
7:    constructor(string memory name, string memory symbol) ERC20(name, symbol) ERC20Permit(name) {}
``` 



[File:ERC721ReceiverMock.sol#L20](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/token/ERC721ReceiverMock.sol#L20) 
```solidity
19:    constructor(bytes4 retval, Error error) {
``` 



[File:ERC721ReceiverMock.sol#L20](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/token/ERC721ReceiverMock.sol#L20) 
```solidity
19:    constructor(bytes4 retval, Error error) {
``` 



[File:ERC777PresetFixedSupply.sol#L21](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/token/ERC777/presets/ERC777PresetFixedSupply.sol#L21) 
```solidity
20:    constructor(
21:        string memory name,
22:        string memory symbol,
23:        address[] memory defaultOperators,
24:        uint256 initialSupply,
25:        address owner
26:    ) ERC777(name, symbol, defaultOperators) {
``` 



[File:ERC777PresetFixedSupply.sol#L21](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/token/ERC777/presets/ERC777PresetFixedSupply.sol#L21) 
```solidity
20:    constructor(
21:        string memory name,
22:        string memory symbol,
23:        address[] memory defaultOperators,
24:        uint256 initialSupply,
25:        address owner
26:    ) ERC777(name, symbol, defaultOperators) {
``` 



[File:ERC777PresetFixedSupply.sol#L21](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/token/ERC777/presets/ERC777PresetFixedSupply.sol#L21) 
```solidity
20:    constructor(
21:        string memory name,
22:        string memory symbol,
23:        address[] memory defaultOperators,
24:        uint256 initialSupply,
25:        address owner
26:    ) ERC777(name, symbol, defaultOperators) {
``` 



[File:ERC777PresetFixedSupply.sol#L21](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/token/ERC777/presets/ERC777PresetFixedSupply.sol#L21) 
```solidity
20:    constructor(
21:        string memory name,
22:        string memory symbol,
23:        address[] memory defaultOperators,
24:        uint256 initialSupply,
25:        address owner
26:    ) ERC777(name, symbol, defaultOperators) {
``` 



[File:ERC777PresetFixedSupply.sol#L21](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/token/ERC777/presets/ERC777PresetFixedSupply.sol#L21) 
```solidity
20:    constructor(
21:        string memory name,
22:        string memory symbol,
23:        address[] memory defaultOperators,
24:        uint256 initialSupply,
25:        address owner
26:    ) ERC777(name, symbol, defaultOperators) {
``` 



[File:ERC20Mock.sol#L56](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/ext/woke/ERC20Mock.sol#L56) 
```solidity
55:    constructor(string memory name_, string memory symbol_, uint8 decimals_) {
``` 



[File:ERC20Mock.sol#L56](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/ext/woke/ERC20Mock.sol#L56) 
```solidity
55:    constructor(string memory name_, string memory symbol_, uint8 decimals_) {
``` 



[File:ERC20Mock.sol#L56](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/ext/woke/ERC20Mock.sol#L56) 
```solidity
55:    constructor(string memory name_, string memory symbol_, uint8 decimals_) {
``` 



[File:ERC1155PresetMinterPauser.sol#L36](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/token/ERC1155/presets/ERC1155PresetMinterPauser.sol#L36) 
```solidity
35:    constructor(string memory uri) ERC1155(uri) {
``` 



[File:CrossChainEnabledArbitrumL1.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/crosschain/arbitrum/CrossChainEnabledArbitrumL1.sol#L27) 
```solidity
26:    constructor(address bridge) {
``` 



[File:Reentrant.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/ext/woke/weird/Reentrant.sol#L10) 
```solidity
9:    constructor(uint _totalSupply) ERC20(_totalSupply) public {}
``` 



[File:CrossChainEnabledArbitrumL1.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/crosschain/arbitrum/CrossChainEnabledArbitrumL1.sol#L27) 
```solidity
26:    constructor(address bridge) {
``` 



[File:GovernorTimelockControl.sol#L38](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/governance/extensions/GovernorTimelockControl.sol#L38) 
```solidity
37:    constructor(TimelockController timelockAddress) {
``` 



[File:LowDecimals.sol#L9](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/ext/woke/weird/LowDecimals.sol#L9) 
```solidity
8:    constructor(uint _totalSupply) ERC20(_totalSupply) public {
``` 



[File:ERC721Harness.sol#L8](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/certora/harnesses/ERC721Harness.sol#L8) 
```solidity
7:    constructor(string memory name, string memory symbol) ERC721(name, symbol) {}
``` 



[File:ERC721Harness.sol#L8](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/certora/harnesses/ERC721Harness.sol#L8) 
```solidity
7:    constructor(string memory name, string memory symbol) ERC721(name, symbol) {}
``` 



[File:MissingReturns.sol#L28](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/ext/woke/weird/MissingReturns.sol#L28) 
```solidity
27:    constructor(uint _totalSupply) public {
``` 



[File:Auth.sol#L16](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solady/lib/solmate/src/auth/Auth.sol#L16) 
```solidity
15:    constructor(address _owner, Authority _authority) {
``` 



[File:Auth.sol#L16](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solady/lib/solmate/src/auth/Auth.sol#L16) 
```solidity
15:    constructor(address _owner, Authority _authority) {
``` 



[File:EIP712.sol#L67](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/utils/cryptography/EIP712.sol#L67) 
```solidity
66:    constructor(string memory name, string memory version) {
``` 



[File:EIP712.sol#L67](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/utils/cryptography/EIP712.sol#L67) 
```solidity
66:    constructor(string memory name, string memory version) {
``` 



[File:ERC777.sol#L62](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/token/ERC777/ERC777.sol#L62) 
```solidity
61:    constructor(string memory name_, string memory symbol_, address[] memory defaultOperators_) {
``` 



[File:ERC777.sol#L62](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/token/ERC777/ERC777.sol#L62) 
```solidity
61:    constructor(string memory name_, string memory symbol_, address[] memory defaultOperators_) {
``` 



[File:ERC777.sol#L62](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/token/ERC777/ERC777.sol#L62) 
```solidity
61:    constructor(string memory name_, string memory symbol_, address[] memory defaultOperators_) {
``` 



[File:WETHGateway.sol#L26](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/misc/WETHGateway.sol#L26) 
```solidity
25:  constructor(address weth) public {
``` 



[File:TransparentUpgradeableProxy.sol#L63](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/proxy/transparent/TransparentUpgradeableProxy.sol#L63) 
```solidity
62:    constructor(address _logic, address admin_, bytes memory _data) payable ERC1967Proxy(_logic, _data) {
``` 



[File:TransparentUpgradeableProxy.sol#L63](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/proxy/transparent/TransparentUpgradeableProxy.sol#L63) 
```solidity
62:    constructor(address _logic, address admin_, bytes memory _data) payable ERC1967Proxy(_logic, _data) {
``` 



[File:TransparentUpgradeableProxy.sol#L63](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/proxy/transparent/TransparentUpgradeableProxy.sol#L63) 
```solidity
62:    constructor(address _logic, address admin_, bytes memory _data) payable ERC1967Proxy(_logic, _data) {
``` 



[File:ERC1155ReceiverMock.sol#L17](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/token/ERC1155ReceiverMock.sol#L17) 
```solidity
16:    constructor(bytes4 recRetval, bool recReverts, bytes4 batRetval, bool batReverts) {
``` 



[File:ERC1155ReceiverMock.sol#L17](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/token/ERC1155ReceiverMock.sol#L17) 
```solidity
16:    constructor(bytes4 recRetval, bool recReverts, bytes4 batRetval, bool batReverts) {
``` 



[File:ERC1155ReceiverMock.sol#L17](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/token/ERC1155ReceiverMock.sol#L17) 
```solidity
16:    constructor(bytes4 recRetval, bool recReverts, bytes4 batRetval, bool batReverts) {
``` 



[File:ERC1155ReceiverMock.sol#L17](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/token/ERC1155ReceiverMock.sol#L17) 
```solidity
16:    constructor(bytes4 recRetval, bool recReverts, bytes4 batRetval, bool batReverts) {
``` 



[File:MultiRolesAuthority.sol#L25](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solady/lib/solmate/src/auth/authorities/MultiRolesAuthority.sol#L25) 
```solidity
24:    constructor(address _owner, Authority _authority) Auth(_owner, _authority) {}
``` 



[File:MultiRolesAuthority.sol#L25](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solady/lib/solmate/src/auth/authorities/MultiRolesAuthority.sol#L25) 
```solidity
24:    constructor(address _owner, Authority _authority) Auth(_owner, _authority) {}
``` 



[File:GovernorSettings.sol#L25](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/governance/extensions/GovernorSettings.sol#L25) 
```solidity
24:    constructor(uint256 initialVotingDelay, uint256 initialVotingPeriod, uint256 initialProposalThreshold) {
``` 



[File:GovernorSettings.sol#L25](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/governance/extensions/GovernorSettings.sol#L25) 
```solidity
24:    constructor(uint256 initialVotingDelay, uint256 initialVotingPeriod, uint256 initialProposalThreshold) {
``` 



[File:GovernorSettings.sol#L25](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/governance/extensions/GovernorSettings.sol#L25) 
```solidity
24:    constructor(uint256 initialVotingDelay, uint256 initialVotingPeriod, uint256 initialProposalThreshold) {
``` 



[File:BeaconProxy.sol#L30](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/proxy/beacon/BeaconProxy.sol#L30) 
```solidity
29:    constructor(address beacon, bytes memory data) payable {
``` 



[File:BeaconProxy.sol#L30](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/proxy/beacon/BeaconProxy.sol#L30) 
```solidity
29:    constructor(address beacon, bytes memory data) payable {
``` 



[File:Governor.sol#L84](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/governance/Governor.sol#L84) 
```solidity
83:    constructor(string memory name_) EIP712(name_, version()) {
``` 



[File:EIP712.sol#L37](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/src/utils/EIP712.sol#L37) 
```solidity
36:    constructor() {
``` 



[File:EIP712.sol#L37](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/src/utils/EIP712.sol#L37) 
```solidity
36:    constructor() {
``` 



[File:ERC2771Context.sol#L16](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/metatx/ERC2771Context.sol#L16) 
```solidity
15:    constructor(address trustedForwarder) {
``` 



[File:CrossChainEnabledArbitrumL1.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/crosschain/arbitrum/CrossChainEnabledArbitrumL1.sol#L27) 
```solidity
26:    constructor(address bridge) {
``` 



[File:CrossChainEnabledOptimism.sol#L24](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/crosschain/optimism/CrossChainEnabledOptimism.sol#L24) 
```solidity
23:    constructor(address messenger) {
``` 



[File:BlockList.sol#L19](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/ext/woke/weird/BlockList.sol#L19) 
```solidity
18:    constructor(uint _totalSupply) ERC20(_totalSupply) public {
``` 



[File:TimelockControllerHarness.sol#L6](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/certora/harnesses/TimelockControllerHarness.sol#L6) 
```solidity
5:    constructor(
6:        uint256 minDelay,
7:        address[] memory proposers,
8:        address[] memory executors,
9:        address admin
10:    ) TimelockController(minDelay, proposers, executors, admin) {}
``` 



[File:TimelockControllerHarness.sol#L6](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/certora/harnesses/TimelockControllerHarness.sol#L6) 
```solidity
5:    constructor(
6:        uint256 minDelay,
7:        address[] memory proposers,
8:        address[] memory executors,
9:        address admin
10:    ) TimelockController(minDelay, proposers, executors, admin) {}
``` 



[File:TimelockControllerHarness.sol#L6](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/certora/harnesses/TimelockControllerHarness.sol#L6) 
```solidity
5:    constructor(
6:        uint256 minDelay,
7:        address[] memory proposers,
8:        address[] memory executors,
9:        address admin
10:    ) TimelockController(minDelay, proposers, executors, admin) {}
``` 



[File:TimelockControllerHarness.sol#L6](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/certora/harnesses/TimelockControllerHarness.sol#L6) 
```solidity
5:    constructor(
6:        uint256 minDelay,
7:        address[] memory proposers,
8:        address[] memory executors,
9:        address admin
10:    ) TimelockController(minDelay, proposers, executors, admin) {}
``` 



[File:PaymentSplitter.sol#L51](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/finance/PaymentSplitter.sol#L51) 
```solidity
50:    constructor(address[] memory payees, uint256[] memory shares_) payable {
``` 



[File:PaymentSplitter.sol#L51](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/finance/PaymentSplitter.sol#L51) 
```solidity
50:    constructor(address[] memory payees, uint256[] memory shares_) payable {
``` 



[File:Auth.sol#L16](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solmate/src/auth/Auth.sol#L16) 
```solidity
15:    constructor(address _owner, Authority _authority) {
``` 



[File:Auth.sol#L16](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solmate/src/auth/Auth.sol#L16) 
```solidity
15:    constructor(address _owner, Authority _authority) {
``` 



[File:EIP712.sol#L67](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/utils/cryptography/EIP712.sol#L67) 
```solidity
66:    constructor(string memory name, string memory version) {
``` 



[File:EIP712.sol#L67](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/utils/cryptography/EIP712.sol#L67) 
```solidity
66:    constructor(string memory name, string memory version) {
``` 



[File:ERC721ConsecutiveEnumerableMock.sol#L9](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/token/ERC721ConsecutiveEnumerableMock.sol#L9) 
```solidity
8:    constructor(
9:        string memory name,
10:        string memory symbol,
11:        address[] memory receivers,
12:        uint96[] memory amounts
13:    ) ERC721(name, symbol) {
``` 



[File:ERC721ConsecutiveEnumerableMock.sol#L9](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/token/ERC721ConsecutiveEnumerableMock.sol#L9) 
```solidity
8:    constructor(
9:        string memory name,
10:        string memory symbol,
11:        address[] memory receivers,
12:        uint96[] memory amounts
13:    ) ERC721(name, symbol) {
``` 



[File:ERC721ConsecutiveEnumerableMock.sol#L9](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/token/ERC721ConsecutiveEnumerableMock.sol#L9) 
```solidity
8:    constructor(
9:        string memory name,
10:        string memory symbol,
11:        address[] memory receivers,
12:        uint96[] memory amounts
13:    ) ERC721(name, symbol) {
``` 



[File:ERC721ConsecutiveEnumerableMock.sol#L9](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/token/ERC721ConsecutiveEnumerableMock.sol#L9) 
```solidity
8:    constructor(
9:        string memory name,
10:        string memory symbol,
11:        address[] memory receivers,
12:        uint96[] memory amounts
13:    ) ERC721(name, symbol) {
``` 



[File:ParaSwapLiquiditySwapAdapter.sol#L19](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/adapters/ParaSwapLiquiditySwapAdapter.sol#L19) 
```solidity
18:  constructor(
19:    ILendingPoolAddressesProvider addressesProvider,
20:    IParaSwapAugustusRegistry augustusRegistry
21:  ) public BaseParaSwapSellAdapter(addressesProvider, augustusRegistry) {
``` 



[File:ParaSwapLiquiditySwapAdapter.sol#L19](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/adapters/ParaSwapLiquiditySwapAdapter.sol#L19) 
```solidity
18:  constructor(
19:    ILendingPoolAddressesProvider addressesProvider,
20:    IParaSwapAugustusRegistry augustusRegistry
21:  ) public BaseParaSwapSellAdapter(addressesProvider, augustusRegistry) {
``` 



[File:Pausable.sol#L19](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/ext/woke/weird/Pausable.sol#L19) 
```solidity
18:    constructor(uint _totalSupply) ERC20(_totalSupply) public {
``` 



[File:TimelockController.sol#L81](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/governance/TimelockController.sol#L81) 
```solidity
80:    constructor(uint256 minDelay, address[] memory proposers, address[] memory executors, address admin) {
``` 



[File:TimelockController.sol#L81](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/governance/TimelockController.sol#L81) 
```solidity
80:    constructor(uint256 minDelay, address[] memory proposers, address[] memory executors, address admin) {
``` 



[File:TimelockController.sol#L81](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/governance/TimelockController.sol#L81) 
```solidity
80:    constructor(uint256 minDelay, address[] memory proposers, address[] memory executors, address admin) {
``` 



[File:ERC1155PresetMinterPauser.sol#L36](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/token/ERC1155/presets/ERC1155PresetMinterPauser.sol#L36) 
```solidity
35:    constructor(string memory uri) ERC1155(uri) {
``` 



[File:GovernorVotesQuorumFraction.sol#L33](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/governance/extensions/GovernorVotesQuorumFraction.sol#L33) 
```solidity
32:    constructor(uint256 quorumNumeratorValue) {
``` 



[File:ERC1967Proxy.sol#L22](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/proxy/ERC1967/ERC1967Proxy.sol#L22) 
```solidity
21:    constructor(address _logic, bytes memory _data) payable {
``` 



[File:ERC1967Proxy.sol#L22](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/proxy/ERC1967/ERC1967Proxy.sol#L22) 
```solidity
21:    constructor(address _logic, bytes memory _data) payable {
``` 



[File:ERC721PresetMinterPauserAutoId.sol#L54](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/token/ERC721/presets/ERC721PresetMinterPauserAutoId.sol#L54) 
```solidity
53:    constructor(string memory name, string memory symbol, string memory baseTokenURI) ERC721(name, symbol) {
``` 



[File:ERC721PresetMinterPauserAutoId.sol#L54](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/token/ERC721/presets/ERC721PresetMinterPauserAutoId.sol#L54) 
```solidity
53:    constructor(string memory name, string memory symbol, string memory baseTokenURI) ERC721(name, symbol) {
``` 



[File:ERC721PresetMinterPauserAutoId.sol#L54](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/token/ERC721/presets/ERC721PresetMinterPauserAutoId.sol#L54) 
```solidity
53:    constructor(string memory name, string memory symbol, string memory baseTokenURI) ERC721(name, symbol) {
``` 



[File:ERC4646FeesMock.sol#L13](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/token/ERC4646FeesMock.sol#L13) 
```solidity
12:    constructor(
13:        uint256 entryFeeBasePoint,
14:        address entryFeeRecipient,
15:        uint256 exitFeeBasePoint,
16:        address exitFeeRecipient
17:    ) {
``` 



[File:ERC4646FeesMock.sol#L13](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/token/ERC4646FeesMock.sol#L13) 
```solidity
12:    constructor(
13:        uint256 entryFeeBasePoint,
14:        address entryFeeRecipient,
15:        uint256 exitFeeBasePoint,
16:        address exitFeeRecipient
17:    ) {
``` 



[File:ERC4646FeesMock.sol#L13](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/token/ERC4646FeesMock.sol#L13) 
```solidity
12:    constructor(
13:        uint256 entryFeeBasePoint,
14:        address entryFeeRecipient,
15:        uint256 exitFeeBasePoint,
16:        address exitFeeRecipient
17:    ) {
``` 



[File:ERC4646FeesMock.sol#L13](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/token/ERC4646FeesMock.sol#L13) 
```solidity
12:    constructor(
13:        uint256 entryFeeBasePoint,
14:        address entryFeeRecipient,
15:        uint256 exitFeeBasePoint,
16:        address exitFeeRecipient
17:    ) {
``` 



[File:DaiPermit.sol#L41](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/ext/woke/weird/DaiPermit.sol#L41) 
```solidity
40:    constructor(uint _totalSupply) public {
``` 



[File:MockAggregator.sol#L9](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/mocks/oracle/CLAggregators/MockAggregator.sol#L9) 
```solidity
8:  constructor(int256 _initialAnswer) public {
``` 



[File:AddressProviderService.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/src/core/AddressProviderService.sol#L27) 
```solidity
26:    constructor(address _addressProvider) {
``` 



[File:GovernorVotesQuorumFraction.sol#L33](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/governance/extensions/GovernorVotesQuorumFraction.sol#L33) 
```solidity
32:    constructor(uint256 quorumNumeratorValue) {
``` 



[File:ERC20.sol#L51](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solady/lib/solmate/src/tokens/ERC20.sol#L51) 
```solidity
50:    constructor(
51:        string memory _name,
52:        string memory _symbol,
53:        uint8 _decimals
54:    ) {
``` 



[File:ERC20.sol#L51](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solady/lib/solmate/src/tokens/ERC20.sol#L51) 
```solidity
50:    constructor(
51:        string memory _name,
52:        string memory _symbol,
53:        uint8 _decimals
54:    ) {
``` 



[File:ERC20.sol#L51](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solady/lib/solmate/src/tokens/ERC20.sol#L51) 
```solidity
50:    constructor(
51:        string memory _name,
52:        string memory _symbol,
53:        uint8 _decimals
54:    ) {
``` 



[File:ERC721PresetMinterPauserAutoId.sol#L54](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/token/ERC721/presets/ERC721PresetMinterPauserAutoId.sol#L54) 
```solidity
53:    constructor(string memory name, string memory symbol, string memory baseTokenURI) ERC721(name, symbol) {
``` 



[File:ERC721PresetMinterPauserAutoId.sol#L54](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/token/ERC721/presets/ERC721PresetMinterPauserAutoId.sol#L54) 
```solidity
53:    constructor(string memory name, string memory symbol, string memory baseTokenURI) ERC721(name, symbol) {
``` 



[File:ERC721PresetMinterPauserAutoId.sol#L54](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/token/ERC721/presets/ERC721PresetMinterPauserAutoId.sol#L54) 
```solidity
53:    constructor(string memory name, string memory symbol, string memory baseTokenURI) ERC721(name, symbol) {
``` 



[File:Proxied.sol#L39](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/ext/woke/weird/Proxied.sol#L39) 
```solidity
38:    constructor(uint _totalSupply) public {
``` 



[File:Proxied.sol#L108](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/ext/woke/weird/Proxied.sol#L108) 
```solidity
107:    constructor(address _impl) public {
``` 



[File:PolicyValidator.sol#L30](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/src/core/PolicyValidator.sol#L30) 
```solidity
29:    constructor(address _addressProvider) AddressProviderService(_addressProvider) {}
``` 



[File:CrossChainEnabledOptimism.sol#L24](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/crosschain/optimism/CrossChainEnabledOptimism.sol#L24) 
```solidity
23:    constructor(address messenger) {
``` 



[File:ERC4626Mock.sol#L7](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/ERC4626Mock.sol#L7) 
```solidity
6:    constructor(address underlying) ERC20("ERC4626Mock", "E4626M") ERC4626(IERC20(underlying)) {}
``` 



[File:UpgradeableBeacon.sol#L28](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/proxy/beacon/UpgradeableBeacon.sol#L28) 
```solidity
27:    constructor(address implementation_) {
``` 



[File:ERC1271WalletMock.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/ERC1271WalletMock.sol#L10) 
```solidity
9:    constructor(address originalOwner) {
``` 



[File:RevertToZero.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/ext/woke/weird/RevertToZero.sol#L10) 
```solidity
9:    constructor(uint _totalSupply) ERC20(_totalSupply) public {}
``` 



[File:ERC1155.sol#L35](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/token/ERC1155/ERC1155.sol#L35) 
```solidity
34:    constructor(string memory uri_) {
``` 



[File:UiPoolDataProviderV2.sol#L31](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/misc/UiPoolDataProviderV2.sol#L31) 
```solidity
30:  constructor(
31:    IChainlinkAggregator _networkBaseTokenPriceInUsdProxyAggregator,
32:    IChainlinkAggregator _marketReferenceCurrencyPriceInUsdProxyAggregator
33:  ) public {
``` 



[File:UiPoolDataProviderV2.sol#L31](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/misc/UiPoolDataProviderV2.sol#L31) 
```solidity
30:  constructor(
31:    IChainlinkAggregator _networkBaseTokenPriceInUsdProxyAggregator,
32:    IChainlinkAggregator _marketReferenceCurrencyPriceInUsdProxyAggregator
33:  ) public {
``` 



[File:ERC1155Mock.sol#L11](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/ext/woke/ERC1155Mock.sol#L11) 
```solidity
10:    constructor(bool enableHooks_) {
``` 



[File:MissingReturns.sol#L28](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/ext/woke/weird/MissingReturns.sol#L28) 
```solidity
27:    constructor(uint _totalSupply) public {
``` 



[File:UiPoolDataProviderV2V3.sol#L34](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/misc/UiPoolDataProviderV2V3.sol#L34) 
```solidity
33:  constructor(
34:    IChainlinkAggregator _networkBaseTokenPriceInUsdProxyAggregator,
35:    IChainlinkAggregator _marketReferenceCurrencyPriceInUsdProxyAggregator
36:  ) public {
``` 



[File:UiPoolDataProviderV2V3.sol#L34](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/misc/UiPoolDataProviderV2V3.sol#L34) 
```solidity
33:  constructor(
34:    IChainlinkAggregator _networkBaseTokenPriceInUsdProxyAggregator,
35:    IChainlinkAggregator _marketReferenceCurrencyPriceInUsdProxyAggregator
36:  ) public {
``` 



[File:ERC20WrapperHarness.sol#L8](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/certora/harnesses/ERC20WrapperHarness.sol#L8) 
```solidity
7:    constructor(IERC20 _underlying, string memory _name, string memory _symbol) ERC20(_name, _symbol) ERC20Wrapper(_underlying) {}
``` 



[File:ERC20WrapperHarness.sol#L8](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/certora/harnesses/ERC20WrapperHarness.sol#L8) 
```solidity
7:    constructor(IERC20 _underlying, string memory _name, string memory _symbol) ERC20(_name, _symbol) ERC20Wrapper(_underlying) {}
``` 



[File:ERC20WrapperHarness.sol#L8](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/certora/harnesses/ERC20WrapperHarness.sol#L8) 
```solidity
7:    constructor(IERC20 _underlying, string memory _name, string memory _symbol) ERC20(_name, _symbol) ERC20Wrapper(_underlying) {}
``` 



[File:MyTokenWrapped.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/docs/governance/MyTokenWrapped.sol#L10) 
```solidity
9:    constructor(
10:        IERC20 wrappedToken
11:    ) ERC20("MyTokenWrapped", "MTK") ERC20Permit("MyTokenWrapped") ERC20Wrapper(wrappedToken) {}
``` 



[File:GovernorTimelockCompound.sol#L37](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/governance/extensions/GovernorTimelockCompound.sol#L37) 
```solidity
36:    constructor(ICompoundTimelock timelockAddress) {
``` 



[File:ERC4626OffsetMock.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/token/ERC4626OffsetMock.sol#L10) 
```solidity
9:    constructor(uint8 offset_) {
``` 



[File:BeaconProxy.sol#L30](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/proxy/beacon/BeaconProxy.sol#L30) 
```solidity
29:    constructor(address beacon, bytes memory data) payable {
``` 



[File:BeaconProxy.sol#L30](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/proxy/beacon/BeaconProxy.sol#L30) 
```solidity
29:    constructor(address beacon, bytes memory data) payable {
``` 



[File:ERC1155.sol#L35](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/token/ERC1155/ERC1155.sol#L35) 
```solidity
34:    constructor(string memory uri_) {
``` 



[File:ERC20Permit.sol#L44](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/token/ERC20/extensions/ERC20Permit.sol#L44) 
```solidity
43:    constructor(string memory name) EIP712(name, "1") {}
``` 



[File:RevertZero.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/ext/woke/weird/RevertZero.sol#L10) 
```solidity
9:    constructor(uint _totalSupply) ERC20(_totalSupply) public {}
``` 



[File:AccessControlDefaultAdminRulesHarness.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/certora/harnesses/AccessControlDefaultAdminRulesHarness.sol#L10) 
```solidity
9:    constructor(
10:        uint48 initialDelay,
11:        address initialDefaultAdmin,
12:        uint48 delayIncreaseWait
13:    ) AccessControlDefaultAdminRules(initialDelay, initialDefaultAdmin) {
``` 



[File:AccessControlDefaultAdminRulesHarness.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/certora/harnesses/AccessControlDefaultAdminRulesHarness.sol#L10) 
```solidity
9:    constructor(
10:        uint48 initialDelay,
11:        address initialDefaultAdmin,
12:        uint48 delayIncreaseWait
13:    ) AccessControlDefaultAdminRules(initialDelay, initialDefaultAdmin) {
``` 



[File:AccessControlDefaultAdminRulesHarness.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/certora/harnesses/AccessControlDefaultAdminRulesHarness.sol#L10) 
```solidity
9:    constructor(
10:        uint48 initialDelay,
11:        address initialDefaultAdmin,
12:        uint48 delayIncreaseWait
13:    ) AccessControlDefaultAdminRules(initialDelay, initialDefaultAdmin) {
``` 



[File:NoRevert.sol#L20](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/ext/woke/weird/NoRevert.sol#L20) 
```solidity
19:    constructor(uint _totalSupply) public {
``` 



[File:ERC20.sol#L51](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/lib/solmate/src/tokens/ERC20.sol#L51) 
```solidity
50:    constructor(
51:        string memory _name,
52:        string memory _symbol,
53:        uint8 _decimals
54:    ) {
``` 



[File:ERC20.sol#L51](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/lib/solmate/src/tokens/ERC20.sol#L51) 
```solidity
50:    constructor(
51:        string memory _name,
52:        string memory _symbol,
53:        uint8 _decimals
54:    ) {
``` 



[File:ERC20.sol#L51](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/lib/solmate/src/tokens/ERC20.sol#L51) 
```solidity
50:    constructor(
51:        string memory _name,
52:        string memory _symbol,
53:        uint8 _decimals
54:    ) {
``` 



[File:GovernorTimelockCompound.sol#L37](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/governance/extensions/GovernorTimelockCompound.sol#L37) 
```solidity
36:    constructor(ICompoundTimelock timelockAddress) {
``` 



[File:ERC777PresetFixedSupply.sol#L21](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/token/ERC777/presets/ERC777PresetFixedSupply.sol#L21) 
```solidity
20:    constructor(
21:        string memory name,
22:        string memory symbol,
23:        address[] memory defaultOperators,
24:        uint256 initialSupply,
25:        address owner
26:    ) ERC777(name, symbol, defaultOperators) {
``` 



[File:ERC777PresetFixedSupply.sol#L21](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/token/ERC777/presets/ERC777PresetFixedSupply.sol#L21) 
```solidity
20:    constructor(
21:        string memory name,
22:        string memory symbol,
23:        address[] memory defaultOperators,
24:        uint256 initialSupply,
25:        address owner
26:    ) ERC777(name, symbol, defaultOperators) {
``` 



[File:ERC777PresetFixedSupply.sol#L21](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/token/ERC777/presets/ERC777PresetFixedSupply.sol#L21) 
```solidity
20:    constructor(
21:        string memory name,
22:        string memory symbol,
23:        address[] memory defaultOperators,
24:        uint256 initialSupply,
25:        address owner
26:    ) ERC777(name, symbol, defaultOperators) {
``` 



[File:ERC777PresetFixedSupply.sol#L21](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/token/ERC777/presets/ERC777PresetFixedSupply.sol#L21) 
```solidity
20:    constructor(
21:        string memory name,
22:        string memory symbol,
23:        address[] memory defaultOperators,
24:        uint256 initialSupply,
25:        address owner
26:    ) ERC777(name, symbol, defaultOperators) {
``` 



[File:ERC777PresetFixedSupply.sol#L21](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/token/ERC777/presets/ERC777PresetFixedSupply.sol#L21) 
```solidity
20:    constructor(
21:        string memory name,
22:        string memory symbol,
23:        address[] memory defaultOperators,
24:        uint256 initialSupply,
25:        address owner
26:    ) ERC777(name, symbol, defaultOperators) {
``` 



[File:ERC777PresetFixedSupply.sol#L21](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/token/ERC777/presets/ERC777PresetFixedSupply.sol#L21) 
```solidity
20:    constructor(
21:        string memory name,
22:        string memory symbol,
23:        address[] memory defaultOperators,
24:        uint256 initialSupply,
25:        address owner
26:    ) ERC777(name, symbol, defaultOperators) {
``` 



[File:ERC777PresetFixedSupply.sol#L21](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/token/ERC777/presets/ERC777PresetFixedSupply.sol#L21) 
```solidity
20:    constructor(
21:        string memory name,
22:        string memory symbol,
23:        address[] memory defaultOperators,
24:        uint256 initialSupply,
25:        address owner
26:    ) ERC777(name, symbol, defaultOperators) {
``` 



[File:ERC777PresetFixedSupply.sol#L21](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/token/ERC777/presets/ERC777PresetFixedSupply.sol#L21) 
```solidity
20:    constructor(
21:        string memory name,
22:        string memory symbol,
23:        address[] memory defaultOperators,
24:        uint256 initialSupply,
25:        address owner
26:    ) ERC777(name, symbol, defaultOperators) {
``` 



[File:ERC777PresetFixedSupply.sol#L21](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/token/ERC777/presets/ERC777PresetFixedSupply.sol#L21) 
```solidity
20:    constructor(
21:        string memory name,
22:        string memory symbol,
23:        address[] memory defaultOperators,
24:        uint256 initialSupply,
25:        address owner
26:    ) ERC777(name, symbol, defaultOperators) {
``` 



[File:ERC777PresetFixedSupply.sol#L21](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/token/ERC777/presets/ERC777PresetFixedSupply.sol#L21) 
```solidity
20:    constructor(
21:        string memory name,
22:        string memory symbol,
23:        address[] memory defaultOperators,
24:        uint256 initialSupply,
25:        address owner
26:    ) ERC777(name, symbol, defaultOperators) {
``` 



[File:GovernorVotesComp.sol#L17](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/governance/extensions/GovernorVotesComp.sol#L17) 
```solidity
16:    constructor(ERC20VotesComp token_) {
``` 



[File:UiPoolDataProvider.sol#L31](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/misc/UiPoolDataProvider.sol#L31) 
```solidity
30:  constructor(IAaveIncentivesController _incentivesController, IPriceOracleGetter _oracle) public {
``` 



[File:UiPoolDataProvider.sol#L31](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/misc/UiPoolDataProvider.sol#L31) 
```solidity
30:  constructor(IAaveIncentivesController _incentivesController, IPriceOracleGetter _oracle) public {
``` 



[File:RolesAuthority.sol#L24](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solmate/src/auth/authorities/RolesAuthority.sol#L24) 
```solidity
23:    constructor(address _owner, Authority _authority) Auth(_owner, _authority) {}
``` 



[File:RolesAuthority.sol#L24](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solmate/src/auth/authorities/RolesAuthority.sol#L24) 
```solidity
23:    constructor(address _owner, Authority _authority) Auth(_owner, _authority) {}
``` 



[File:BeaconProxy.sol#L30](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/proxy/beacon/BeaconProxy.sol#L30) 
```solidity
29:    constructor(address beacon, bytes memory data) payable {
``` 



[File:BeaconProxy.sol#L30](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/proxy/beacon/BeaconProxy.sol#L30) 
```solidity
29:    constructor(address beacon, bytes memory data) payable {
``` 



[File:CrossChainEnabledOptimism.sol#L24](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/crosschain/optimism/CrossChainEnabledOptimism.sol#L24) 
```solidity
23:    constructor(address messenger) {
``` 



[File:GovernorVotes.sol#L17](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/governance/extensions/GovernorVotes.sol#L17) 
```solidity
16:    constructor(IVotes tokenAddress) {
``` 



[File:Uint96.sol#L32](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/ext/woke/weird/Uint96.sol#L32) 
```solidity
31:    constructor(uint96 _supply) public {
``` 



[File:VestingWallet.sol#L32](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/finance/VestingWallet.sol#L32) 
```solidity
31:    constructor(address beneficiaryAddress, uint64 startTimestamp, uint64 durationSeconds) payable {
``` 



[File:VestingWallet.sol#L32](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/finance/VestingWallet.sol#L32) 
```solidity
31:    constructor(address beneficiaryAddress, uint64 startTimestamp, uint64 durationSeconds) payable {
``` 



[File:MultiRolesAuthority.sol#L25](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solmate/src/auth/authorities/MultiRolesAuthority.sol#L25) 
```solidity
24:    constructor(address _owner, Authority _authority) Auth(_owner, _authority) {}
``` 



[File:MultiRolesAuthority.sol#L25](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solmate/src/auth/authorities/MultiRolesAuthority.sol#L25) 
```solidity
24:    constructor(address _owner, Authority _authority) Auth(_owner, _authority) {}
``` 



[File:ArraysMock.sol#L12](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/ArraysMock.sol#L12) 
```solidity
11:    constructor(uint256[] memory array) {
``` 



[File:ArraysMock.sol#L30](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/ArraysMock.sol#L30) 
```solidity
29:    constructor(address[] memory array) {
``` 



[File:ArraysMock.sol#L44](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/ArraysMock.sol#L44) 
```solidity
43:    constructor(bytes32[] memory array) {
``` 



[File:ERC721ReceiverMock.sol#L20](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/token/ERC721ReceiverMock.sol#L20) 
```solidity
19:    constructor(bytes4 retval, Error error) {
``` 



[File:ERC721ReceiverMock.sol#L20](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/token/ERC721ReceiverMock.sol#L20) 
```solidity
19:    constructor(bytes4 retval, Error error) {
``` 



[File:Upgradable.sol#L14](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/ext/woke/weird/Upgradable.sol#L14) 
```solidity
13:    constructor(uint totalSupply) public {
``` 



[File:LowDecimals.sol#L9](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/ext/woke/weird/LowDecimals.sol#L9) 
```solidity
8:    constructor(uint _totalSupply) ERC20(_totalSupply) public {
``` 



[File:ERC777.sol#L62](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/token/ERC777/ERC777.sol#L62) 
```solidity
61:    constructor(string memory name_, string memory symbol_, address[] memory defaultOperators_) {
``` 



[File:ERC777.sol#L62](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/token/ERC777/ERC777.sol#L62) 
```solidity
61:    constructor(string memory name_, string memory symbol_, address[] memory defaultOperators_) {
``` 



[File:ERC777.sol#L62](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/token/ERC777/ERC777.sol#L62) 
```solidity
61:    constructor(string memory name_, string memory symbol_, address[] memory defaultOperators_) {
``` 



[File:ERC1155.sol#L35](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/token/ERC1155/ERC1155.sol#L35) 
```solidity
34:    constructor(string memory uri_) {
``` 



[File:ERC1155ReceiverMock.sol#L17](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/token/ERC1155ReceiverMock.sol#L17) 
```solidity
16:    constructor(bytes4 recRetval, bool recReverts, bytes4 batRetval, bool batReverts) {
``` 



[File:ERC1155ReceiverMock.sol#L17](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/token/ERC1155ReceiverMock.sol#L17) 
```solidity
16:    constructor(bytes4 recRetval, bool recReverts, bytes4 batRetval, bool batReverts) {
``` 



[File:ERC1155ReceiverMock.sol#L17](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/token/ERC1155ReceiverMock.sol#L17) 
```solidity
16:    constructor(bytes4 recRetval, bool recReverts, bytes4 batRetval, bool batReverts) {
``` 



[File:ERC1155ReceiverMock.sol#L17](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/token/ERC1155ReceiverMock.sol#L17) 
```solidity
16:    constructor(bytes4 recRetval, bool recReverts, bytes4 batRetval, bool batReverts) {
``` 



[File:ERC20PermitHarness.sol#L8](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/certora/harnesses/ERC20PermitHarness.sol#L8) 
```solidity
7:    constructor(string memory name, string memory symbol) ERC20(name, symbol) ERC20Permit(name) {}
``` 



[File:ERC20PermitHarness.sol#L8](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/certora/harnesses/ERC20PermitHarness.sol#L8) 
```solidity
7:    constructor(string memory name, string memory symbol) ERC20(name, symbol) ERC20Permit(name) {}
``` 



[File:ERC20PresetFixedSupply.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/token/ERC20/presets/ERC20PresetFixedSupply.sol#L27) 
```solidity
26:    constructor(string memory name, string memory symbol, uint256 initialSupply, address owner) ERC20(name, symbol) {
``` 



[File:ERC20PresetFixedSupply.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/token/ERC20/presets/ERC20PresetFixedSupply.sol#L27) 
```solidity
26:    constructor(string memory name, string memory symbol, uint256 initialSupply, address owner) ERC20(name, symbol) {
``` 



[File:ERC20PresetFixedSupply.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/token/ERC20/presets/ERC20PresetFixedSupply.sol#L27) 
```solidity
26:    constructor(string memory name, string memory symbol, uint256 initialSupply, address owner) ERC20(name, symbol) {
``` 



[File:ERC20PresetFixedSupply.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/token/ERC20/presets/ERC20PresetFixedSupply.sol#L27) 
```solidity
26:    constructor(string memory name, string memory symbol, uint256 initialSupply, address owner) ERC20(name, symbol) {
``` 



[File:MockFlashLoanReceiver.sol#L24](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/mocks/flashloan/MockFlashLoanReceiver.sol#L24) 
```solidity
23:  constructor(ILendingPoolAddressesProvider provider) public FlashLoanReceiverBase(provider) {}
``` 



[File:ERC721ConsecutiveEnumerableMock.sol#L9](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/token/ERC721ConsecutiveEnumerableMock.sol#L9) 
```solidity
8:    constructor(
9:        string memory name,
10:        string memory symbol,
11:        address[] memory receivers,
12:        uint96[] memory amounts
13:    ) ERC721(name, symbol) {
``` 



[File:ERC721ConsecutiveEnumerableMock.sol#L9](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/token/ERC721ConsecutiveEnumerableMock.sol#L9) 
```solidity
8:    constructor(
9:        string memory name,
10:        string memory symbol,
11:        address[] memory receivers,
12:        uint96[] memory amounts
13:    ) ERC721(name, symbol) {
``` 



[File:ERC721ConsecutiveEnumerableMock.sol#L9](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/token/ERC721ConsecutiveEnumerableMock.sol#L9) 
```solidity
8:    constructor(
9:        string memory name,
10:        string memory symbol,
11:        address[] memory receivers,
12:        uint96[] memory amounts
13:    ) ERC721(name, symbol) {
``` 



[File:ERC721ConsecutiveEnumerableMock.sol#L9](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/token/ERC721ConsecutiveEnumerableMock.sol#L9) 
```solidity
8:    constructor(
9:        string memory name,
10:        string memory symbol,
11:        address[] memory receivers,
12:        uint96[] memory amounts
13:    ) ERC721(name, symbol) {
``` 



[File:CrossChainEnabledAMB.sol#L32](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/crosschain/amb/CrossChainEnabledAMB.sol#L32) 
```solidity
31:    constructor(address bridge) {
``` 



[File:VestingWallet.sol#L32](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/finance/VestingWallet.sol#L32) 
```solidity
31:    constructor(address beneficiaryAddress, uint64 startTimestamp, uint64 durationSeconds) payable {
``` 



[File:VestingWallet.sol#L32](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/finance/VestingWallet.sol#L32) 
```solidity
31:    constructor(address beneficiaryAddress, uint64 startTimestamp, uint64 durationSeconds) payable {
``` 



[File:AccessControlDefaultAdminRules.sol#L55](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/access/AccessControlDefaultAdminRules.sol#L55) 
```solidity
54:    constructor(uint48 initialDelay, address initialDefaultAdmin) {
``` 



[File:MyGovernor3.sol#L17](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/wizard/MyGovernor3.sol#L17) 
```solidity
16:    constructor(
17:        IVotes _token,
18:        TimelockController _timelock
19:    ) Governor("MyGovernor") GovernorVotes(_token) GovernorVotesQuorumFraction(4) GovernorTimelockControl(_timelock) {}
``` 



[File:MyGovernor3.sol#L17](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/wizard/MyGovernor3.sol#L17) 
```solidity
16:    constructor(
17:        IVotes _token,
18:        TimelockController _timelock
19:    ) Governor("MyGovernor") GovernorVotes(_token) GovernorVotesQuorumFraction(4) GovernorTimelockControl(_timelock) {}
``` 



[File:ReturnsFalse.sol#L28](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/ext/woke/weird/ReturnsFalse.sol#L28) 
```solidity
27:    constructor(uint _totalSupply) public {
``` 



[File:ERC20PresetFixedSupply.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/token/ERC20/presets/ERC20PresetFixedSupply.sol#L27) 
```solidity
26:    constructor(string memory name, string memory symbol, uint256 initialSupply, address owner) ERC20(name, symbol) {
``` 



[File:ERC20PresetFixedSupply.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/token/ERC20/presets/ERC20PresetFixedSupply.sol#L27) 
```solidity
26:    constructor(string memory name, string memory symbol, uint256 initialSupply, address owner) ERC20(name, symbol) {
``` 



[File:ERC20PresetFixedSupply.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/token/ERC20/presets/ERC20PresetFixedSupply.sol#L27) 
```solidity
26:    constructor(string memory name, string memory symbol, uint256 initialSupply, address owner) ERC20(name, symbol) {
``` 



[File:ERC20PresetFixedSupply.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/token/ERC20/presets/ERC20PresetFixedSupply.sol#L27) 
```solidity
26:    constructor(string memory name, string memory symbol, uint256 initialSupply, address owner) ERC20(name, symbol) {
``` 



[File:MyGovernor3.sol#L17](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/wizard/MyGovernor3.sol#L17) 
```solidity
16:    constructor(
17:        IVotes _token,
18:        TimelockController _timelock
19:    ) Governor("MyGovernor") GovernorVotes(_token) GovernorVotesQuorumFraction(4) GovernorTimelockControl(_timelock) {}
``` 



[File:MyGovernor3.sol#L17](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/wizard/MyGovernor3.sol#L17) 
```solidity
16:    constructor(
17:        IVotes _token,
18:        TimelockController _timelock
19:    ) Governor("MyGovernor") GovernorVotes(_token) GovernorVotesQuorumFraction(4) GovernorTimelockControl(_timelock) {}
``` 



[File:Upgradable.sol#L14](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/ext/woke/weird/Upgradable.sol#L14) 
```solidity
13:    constructor(uint totalSupply) public {
``` 



[File:CrossChainEnabledAMB.sol#L32](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/crosschain/amb/CrossChainEnabledAMB.sol#L32) 
```solidity
31:    constructor(address bridge) {
``` 



[File:ERC777.sol#L62](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/token/ERC777/ERC777.sol#L62) 
```solidity
61:    constructor(string memory name_, string memory symbol_, address[] memory defaultOperators_) {
``` 



[File:ERC777.sol#L62](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/token/ERC777/ERC777.sol#L62) 
```solidity
61:    constructor(string memory name_, string memory symbol_, address[] memory defaultOperators_) {
``` 



[File:ERC777.sol#L62](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/token/ERC777/ERC777.sol#L62) 
```solidity
61:    constructor(string memory name_, string memory symbol_, address[] memory defaultOperators_) {
``` 



[File:ApprovalToZero.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/ext/woke/weird/ApprovalToZero.sol#L10) 
```solidity
9:    constructor(uint _totalSupply) ERC20(_totalSupply) public {}
``` 



[File:HighDecimals.sol#L9](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/ext/woke/weird/HighDecimals.sol#L9) 
```solidity
8:    constructor(uint _totalSupply) ERC20(_totalSupply) public {
``` 



[File:Approval.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/ext/woke/weird/Approval.sol#L10) 
```solidity
9:    constructor(uint _totalSupply) ERC20(_totalSupply) public {}
``` 



[File:TransparentUpgradeableProxy.sol#L63](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/proxy/transparent/TransparentUpgradeableProxy.sol#L63) 
```solidity
62:    constructor(address _logic, address admin_, bytes memory _data) payable ERC1967Proxy(_logic, _data) {
``` 



[File:TransparentUpgradeableProxy.sol#L63](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/proxy/transparent/TransparentUpgradeableProxy.sol#L63) 
```solidity
62:    constructor(address _logic, address admin_, bytes memory _data) payable ERC1967Proxy(_logic, _data) {
``` 



[File:TransparentUpgradeableProxy.sol#L63](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/proxy/transparent/TransparentUpgradeableProxy.sol#L63) 
```solidity
62:    constructor(address _logic, address admin_, bytes memory _data) payable ERC1967Proxy(_logic, _data) {
``` 



[File:Proxied.sol#L39](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/ext/woke/weird/Proxied.sol#L39) 
```solidity
38:    constructor(uint _totalSupply) public {
``` 



[File:Proxied.sol#L108](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/ext/woke/weird/Proxied.sol#L108) 
```solidity
107:    constructor(address _impl) public {
``` 



[File:Minter.sol#L39](https://github.com/0xKitsune/sstan/blob/main/bin/scope/3xcalibur/scope/Minter.sol#L39) 
```solidity
38:    constructor(
39:        address __voter, // the voting & distribution system
40:        address  __ve, // the ve(3,3) system that will be locked into
41:        address __ve_dist, // the distribution system that ensures users aren't diluted
42:        address _admin,
43:        uint _weekly
44:    ) {
``` 



[File:Pausable.sol#L19](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/ext/woke/weird/Pausable.sol#L19) 
```solidity
18:    constructor(uint _totalSupply) ERC20(_totalSupply) public {
``` 



[File:Auth.sol#L16](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/lib/solmate/src/auth/Auth.sol#L16) 
```solidity
15:    constructor(address _owner, Authority _authority) {
``` 



[File:Auth.sol#L16](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/lib/solmate/src/auth/Auth.sol#L16) 
```solidity
15:    constructor(address _owner, Authority _authority) {
``` 



[File:ERC721.sol#L44](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/token/ERC721/ERC721.sol#L44) 
```solidity
43:    constructor(string memory name_, string memory symbol_) {
``` 



[File:ERC721.sol#L44](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/token/ERC721/ERC721.sol#L44) 
```solidity
43:    constructor(string memory name_, string memory symbol_) {
``` 



[File:ERC1967Proxy.sol#L22](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/proxy/ERC1967/ERC1967Proxy.sol#L22) 
```solidity
21:    constructor(address _logic, bytes memory _data) payable {
``` 



[File:ERC1967Proxy.sol#L22](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/proxy/ERC1967/ERC1967Proxy.sol#L22) 
```solidity
21:    constructor(address _logic, bytes memory _data) payable {
``` 



[File:ERC2771Context.sol#L16](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/metatx/ERC2771Context.sol#L16) 
```solidity
15:    constructor(address trustedForwarder) {
``` 



[File:Bribe.sol#L70](https://github.com/0xKitsune/sstan/blob/main/bin/scope/3xcalibur/scope/Bribe.sol#L70) 
```solidity
69:    constructor(address _voter) {
``` 



[File:ERC721ConsecutiveEnumerableMock.sol#L9](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/token/ERC721ConsecutiveEnumerableMock.sol#L9) 
```solidity
8:    constructor(
9:        string memory name,
10:        string memory symbol,
11:        address[] memory receivers,
12:        uint96[] memory amounts
13:    ) ERC721(name, symbol) {
``` 



[File:ERC721ConsecutiveEnumerableMock.sol#L9](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/token/ERC721ConsecutiveEnumerableMock.sol#L9) 
```solidity
8:    constructor(
9:        string memory name,
10:        string memory symbol,
11:        address[] memory receivers,
12:        uint96[] memory amounts
13:    ) ERC721(name, symbol) {
``` 



[File:ERC721ConsecutiveEnumerableMock.sol#L9](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/token/ERC721ConsecutiveEnumerableMock.sol#L9) 
```solidity
8:    constructor(
9:        string memory name,
10:        string memory symbol,
11:        address[] memory receivers,
12:        uint96[] memory amounts
13:    ) ERC721(name, symbol) {
``` 



[File:ERC721ConsecutiveEnumerableMock.sol#L9](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/token/ERC721ConsecutiveEnumerableMock.sol#L9) 
```solidity
8:    constructor(
9:        string memory name,
10:        string memory symbol,
11:        address[] memory receivers,
12:        uint96[] memory amounts
13:    ) ERC721(name, symbol) {
``` 



[File:VestingWallet.sol#L32](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/finance/VestingWallet.sol#L32) 
```solidity
31:    constructor(address beneficiaryAddress, uint64 startTimestamp, uint64 durationSeconds) payable {
``` 



[File:VestingWallet.sol#L32](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/finance/VestingWallet.sol#L32) 
```solidity
31:    constructor(address beneficiaryAddress, uint64 startTimestamp, uint64 durationSeconds) payable {
``` 



[File:CompTimelock.sol#L68](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/compound/CompTimelock.sol#L68) 
```solidity
67:    constructor(address admin_, uint256 delay_) {
``` 



[File:CompTimelock.sol#L68](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/compound/CompTimelock.sol#L68) 
```solidity
67:    constructor(address admin_, uint256 delay_) {
``` 



[File:MyGovernor2.sol#L19](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/wizard/MyGovernor2.sol#L19) 
```solidity
18:    constructor(
19:        IVotes _token,
20:        TimelockController _timelock
21:    ) Governor("MyGovernor") GovernorVotes(_token) GovernorVotesQuorumFraction(4) GovernorTimelockControl(_timelock) {}
``` 



[File:MyGovernor2.sol#L19](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/wizard/MyGovernor2.sol#L19) 
```solidity
18:    constructor(
19:        IVotes _token,
20:        TimelockController _timelock
21:    ) Governor("MyGovernor") GovernorVotes(_token) GovernorVotesQuorumFraction(4) GovernorTimelockControl(_timelock) {}
``` 



[File:Reentrant.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/ext/woke/weird/Reentrant.sol#L10) 
```solidity
9:    constructor(uint _totalSupply) ERC20(_totalSupply) public {}
``` 



[File:ERC20DecimalsMock.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/token/ERC20DecimalsMock.sol#L10) 
```solidity
9:    constructor(uint8 decimals_) {
``` 



[File:SafeModeratorOverridable.sol#L23](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/src/core/SafeModeratorOverridable.sol#L23) 
```solidity
22:    constructor(address _addressProvider) AddressProviderService(_addressProvider) {}
``` 



[File:ERC721.sol#L57](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/lib/solmate/src/tokens/ERC721.sol#L57) 
```solidity
56:    constructor(string memory _name, string memory _symbol) {
``` 



[File:ERC721.sol#L57](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/lib/solmate/src/tokens/ERC721.sol#L57) 
```solidity
56:    constructor(string memory _name, string memory _symbol) {
``` 



[File:ERC20.sol#L30](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/ext/woke/weird/ERC20.sol#L30) 
```solidity
29:    constructor(uint _totalSupply) public {
``` 



[File:ERC4646FeesMock.sol#L13](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/token/ERC4646FeesMock.sol#L13) 
```solidity
12:    constructor(
13:        uint256 entryFeeBasePoint,
14:        address entryFeeRecipient,
15:        uint256 exitFeeBasePoint,
16:        address exitFeeRecipient
17:    ) {
``` 



[File:ERC4646FeesMock.sol#L13](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/token/ERC4646FeesMock.sol#L13) 
```solidity
12:    constructor(
13:        uint256 entryFeeBasePoint,
14:        address entryFeeRecipient,
15:        uint256 exitFeeBasePoint,
16:        address exitFeeRecipient
17:    ) {
``` 



[File:ERC4646FeesMock.sol#L13](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/token/ERC4646FeesMock.sol#L13) 
```solidity
12:    constructor(
13:        uint256 entryFeeBasePoint,
14:        address entryFeeRecipient,
15:        uint256 exitFeeBasePoint,
16:        address exitFeeRecipient
17:    ) {
``` 



[File:ERC4646FeesMock.sol#L13](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/token/ERC4646FeesMock.sol#L13) 
```solidity
12:    constructor(
13:        uint256 entryFeeBasePoint,
14:        address entryFeeRecipient,
15:        uint256 exitFeeBasePoint,
16:        address exitFeeRecipient
17:    ) {
``` 



[File:Auth.sol#L16](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solmate/src/auth/Auth.sol#L16) 
```solidity
15:    constructor(address _owner, Authority _authority) {
``` 



[File:Auth.sol#L16](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solmate/src/auth/Auth.sol#L16) 
```solidity
15:    constructor(address _owner, Authority _authority) {
``` 



[File:TimelockController.sol#L81](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/governance/TimelockController.sol#L81) 
```solidity
80:    constructor(uint256 minDelay, address[] memory proposers, address[] memory executors, address admin) {
``` 



[File:TimelockController.sol#L81](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/governance/TimelockController.sol#L81) 
```solidity
80:    constructor(uint256 minDelay, address[] memory proposers, address[] memory executors, address admin) {
``` 



[File:TimelockController.sol#L81](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/governance/TimelockController.sol#L81) 
```solidity
80:    constructor(uint256 minDelay, address[] memory proposers, address[] memory executors, address admin) {
``` 



[File:ERC1155PresetMinterPauser.sol#L36](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/token/ERC1155/presets/ERC1155PresetMinterPauser.sol#L36) 
```solidity
35:    constructor(string memory uri) ERC1155(uri) {
``` 



[File:MyGovernor1.sol#L17](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/wizard/MyGovernor1.sol#L17) 
```solidity
16:    constructor(
17:        IVotes _token,
18:        TimelockController _timelock
19:    ) Governor("MyGovernor") GovernorVotes(_token) GovernorVotesQuorumFraction(4) GovernorTimelockControl(_timelock) {}
``` 



[File:MyGovernor1.sol#L17](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/wizard/MyGovernor1.sol#L17) 
```solidity
16:    constructor(
17:        IVotes _token,
18:        TimelockController _timelock
19:    ) Governor("MyGovernor") GovernorVotes(_token) GovernorVotesQuorumFraction(4) GovernorTimelockControl(_timelock) {}
``` 



[File:ERC721PresetMinterPauserAutoId.sol#L54](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/token/ERC721/presets/ERC721PresetMinterPauserAutoId.sol#L54) 
```solidity
53:    constructor(string memory name, string memory symbol, string memory baseTokenURI) ERC721(name, symbol) {
``` 



[File:ERC721PresetMinterPauserAutoId.sol#L54](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/token/ERC721/presets/ERC721PresetMinterPauserAutoId.sol#L54) 
```solidity
53:    constructor(string memory name, string memory symbol, string memory baseTokenURI) ERC721(name, symbol) {
``` 



[File:ERC721PresetMinterPauserAutoId.sol#L54](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/token/ERC721/presets/ERC721PresetMinterPauserAutoId.sol#L54) 
```solidity
53:    constructor(string memory name, string memory symbol, string memory baseTokenURI) ERC721(name, symbol) {
``` 



[File:MyGovernor.sol#L17](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/docs/governance/MyGovernor.sol#L17) 
```solidity
16:    constructor(
17:        IVotes _token,
18:        TimelockController _timelock
19:    ) Governor("MyGovernor") GovernorVotes(_token) GovernorVotesQuorumFraction(4) GovernorTimelockControl(_timelock) {}
``` 



[File:MyGovernor.sol#L17](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/docs/governance/MyGovernor.sol#L17) 
```solidity
16:    constructor(
17:        IVotes _token,
18:        TimelockController _timelock
19:    ) Governor("MyGovernor") GovernorVotes(_token) GovernorVotesQuorumFraction(4) GovernorTimelockControl(_timelock) {}
``` 



[File:ERC20PermitHarness.sol#L8](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/certora/harnesses/ERC20PermitHarness.sol#L8) 
```solidity
7:    constructor(string memory name, string memory symbol) ERC20(name, symbol) ERC20Permit(name) {}
``` 



[File:ERC20PermitHarness.sol#L8](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/certora/harnesses/ERC20PermitHarness.sol#L8) 
```solidity
7:    constructor(string memory name, string memory symbol) ERC20(name, symbol) ERC20Permit(name) {}
``` 



[File:ERC721Wrapper.sol#L20](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/token/ERC721/extensions/ERC721Wrapper.sol#L20) 
```solidity
19:    constructor(IERC721 underlyingToken) {
``` 



[File:GovernorVotes.sol#L17](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/governance/extensions/GovernorVotes.sol#L17) 
```solidity
16:    constructor(IVotes tokenAddress) {
``` 



[File:ERC20PresetFixedSupply.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/token/ERC20/presets/ERC20PresetFixedSupply.sol#L27) 
```solidity
26:    constructor(string memory name, string memory symbol, uint256 initialSupply, address owner) ERC20(name, symbol) {
``` 



[File:ERC20PresetFixedSupply.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/token/ERC20/presets/ERC20PresetFixedSupply.sol#L27) 
```solidity
26:    constructor(string memory name, string memory symbol, uint256 initialSupply, address owner) ERC20(name, symbol) {
``` 



[File:ERC20PresetFixedSupply.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/token/ERC20/presets/ERC20PresetFixedSupply.sol#L27) 
```solidity
26:    constructor(string memory name, string memory symbol, uint256 initialSupply, address owner) ERC20(name, symbol) {
``` 



[File:ERC20PresetFixedSupply.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/token/ERC20/presets/ERC20PresetFixedSupply.sol#L27) 
```solidity
26:    constructor(string memory name, string memory symbol, uint256 initialSupply, address owner) ERC20(name, symbol) {
``` 



[File:GovernorPreventLateQuorum.sol#L37](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/governance/extensions/GovernorPreventLateQuorum.sol#L37) 
```solidity
36:    constructor(uint64 initialVoteExtension) {
``` 



[File:GovernorPreventLateQuorumMock.sol#L18](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/governance/GovernorPreventLateQuorumMock.sol#L18) 
```solidity
17:    constructor(uint256 quorum_) {
``` 



[File:ERC20PresetMinterPauser.sol#L38](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/token/ERC20/presets/ERC20PresetMinterPauser.sol#L38) 
```solidity
37:    constructor(string memory name, string memory symbol) ERC20(name, symbol) {
``` 



[File:ERC20PresetMinterPauser.sol#L38](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/token/ERC20/presets/ERC20PresetMinterPauser.sol#L38) 
```solidity
37:    constructor(string memory name, string memory symbol) ERC20(name, symbol) {
``` 



[File:GovernorTimelockControl.sol#L38](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/governance/extensions/GovernorTimelockControl.sol#L38) 
```solidity
37:    constructor(TimelockController timelockAddress) {
``` 



[File:ERC721.sol#L44](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/token/ERC721/ERC721.sol#L44) 
```solidity
43:    constructor(string memory name_, string memory symbol_) {
``` 



[File:ERC721.sol#L44](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/token/ERC721/ERC721.sol#L44) 
```solidity
43:    constructor(string memory name_, string memory symbol_) {
``` 



[File:ERC20.sol#L54](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/token/ERC20/ERC20.sol#L54) 
```solidity
53:    constructor(string memory name_, string memory symbol_) {
``` 



[File:ERC20.sol#L54](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/token/ERC20/ERC20.sol#L54) 
```solidity
53:    constructor(string memory name_, string memory symbol_) {
``` 



[File:RolesAuthority.sol#L24](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solmate/src/auth/authorities/RolesAuthority.sol#L24) 
```solidity
23:    constructor(address _owner, Authority _authority) Auth(_owner, _authority) {}
``` 



[File:RolesAuthority.sol#L24](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solmate/src/auth/authorities/RolesAuthority.sol#L24) 
```solidity
23:    constructor(address _owner, Authority _authority) Auth(_owner, _authority) {}
``` 



[File:ERC20DecimalsMock.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/token/ERC20DecimalsMock.sol#L10) 
```solidity
9:    constructor(uint8 decimals_) {
``` 



[File:ERC20FlashMintHarness.sol#L13](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/certora/harnesses/ERC20FlashMintHarness.sol#L13) 
```solidity
12:    constructor(string memory name, string memory symbol) ERC20(name, symbol) ERC20Permit(name) {}
``` 



[File:ERC20FlashMintHarness.sol#L13](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/certora/harnesses/ERC20FlashMintHarness.sol#L13) 
```solidity
12:    constructor(string memory name, string memory symbol) ERC20(name, symbol) ERC20Permit(name) {}
``` 



[File:UniswapRepayAdapter.sol#L25](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/adapters/UniswapRepayAdapter.sol#L25) 
```solidity
24:  constructor(
25:    ILendingPoolAddressesProvider addressesProvider,
26:    IUniswapV2Router02 uniswapRouter,
27:    address wethAddress
28:  ) public BaseUniswapAdapter(addressesProvider, uniswapRouter, wethAddress) {}
``` 



[File:UniswapRepayAdapter.sol#L25](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/adapters/UniswapRepayAdapter.sol#L25) 
```solidity
24:  constructor(
25:    ILendingPoolAddressesProvider addressesProvider,
26:    IUniswapV2Router02 uniswapRouter,
27:    address wethAddress
28:  ) public BaseUniswapAdapter(addressesProvider, uniswapRouter, wethAddress) {}
``` 



[File:UniswapRepayAdapter.sol#L25](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/adapters/UniswapRepayAdapter.sol#L25) 
```solidity
24:  constructor(
25:    ILendingPoolAddressesProvider addressesProvider,
26:    IUniswapV2Router02 uniswapRouter,
27:    address wethAddress
28:  ) public BaseUniswapAdapter(addressesProvider, uniswapRouter, wethAddress) {}
``` 



[File:ERC721ConsecutiveMock.sol#L14](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/token/ERC721ConsecutiveMock.sol#L14) 
```solidity
13:    constructor(
14:        string memory name,
15:        string memory symbol,
16:        address[] memory delegates,
17:        address[] memory receivers,
18:        uint96[] memory amounts
19:    ) ERC721(name, symbol) EIP712(name, "1") {
``` 



[File:ERC721ConsecutiveMock.sol#L14](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/token/ERC721ConsecutiveMock.sol#L14) 
```solidity
13:    constructor(
14:        string memory name,
15:        string memory symbol,
16:        address[] memory delegates,
17:        address[] memory receivers,
18:        uint96[] memory amounts
19:    ) ERC721(name, symbol) EIP712(name, "1") {
``` 



[File:ERC721ConsecutiveMock.sol#L14](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/token/ERC721ConsecutiveMock.sol#L14) 
```solidity
13:    constructor(
14:        string memory name,
15:        string memory symbol,
16:        address[] memory delegates,
17:        address[] memory receivers,
18:        uint96[] memory amounts
19:    ) ERC721(name, symbol) EIP712(name, "1") {
``` 



[File:ERC721ConsecutiveMock.sol#L14](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/token/ERC721ConsecutiveMock.sol#L14) 
```solidity
13:    constructor(
14:        string memory name,
15:        string memory symbol,
16:        address[] memory delegates,
17:        address[] memory receivers,
18:        uint96[] memory amounts
19:    ) ERC721(name, symbol) EIP712(name, "1") {
``` 



[File:ERC721ConsecutiveMock.sol#L14](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/token/ERC721ConsecutiveMock.sol#L14) 
```solidity
13:    constructor(
14:        string memory name,
15:        string memory symbol,
16:        address[] memory delegates,
17:        address[] memory receivers,
18:        uint96[] memory amounts
19:    ) ERC721(name, symbol) EIP712(name, "1") {
``` 



[File:ERC721ConsecutiveMock.sol#L58](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/token/ERC721ConsecutiveMock.sol#L58) 
```solidity
57:    constructor(string memory name, string memory symbol) ERC721(name, symbol) {
``` 



[File:ERC721ConsecutiveMock.sol#L58](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/token/ERC721ConsecutiveMock.sol#L58) 
```solidity
57:    constructor(string memory name, string memory symbol) ERC721(name, symbol) {
``` 



[File:ERC721Harness.sol#L8](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/certora/harnesses/ERC721Harness.sol#L8) 
```solidity
7:    constructor(string memory name, string memory symbol) ERC721(name, symbol) {}
``` 



[File:ERC721Harness.sol#L8](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/certora/harnesses/ERC721Harness.sol#L8) 
```solidity
7:    constructor(string memory name, string memory symbol) ERC721(name, symbol) {}
``` 



[File:GovernorTimelockCompound.sol#L37](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/governance/extensions/GovernorTimelockCompound.sol#L37) 
```solidity
36:    constructor(ICompoundTimelock timelockAddress) {
``` 



[File:ERC4626Mock.sol#L7](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/ERC4626Mock.sol#L7) 
```solidity
6:    constructor(address underlying) ERC20("ERC4626Mock", "E4626M") ERC4626(IERC20(underlying)) {}
``` 



[File:IncentivizedERC20.sol#L26](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/protocol/tokenization/IncentivizedERC20.sol#L26) 
```solidity
25:  constructor(
26:    string memory name,
27:    string memory symbol,
28:    uint8 decimals
29:  ) public {
``` 



[File:IncentivizedERC20.sol#L26](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/protocol/tokenization/IncentivizedERC20.sol#L26) 
```solidity
25:  constructor(
26:    string memory name,
27:    string memory symbol,
28:    uint8 decimals
29:  ) public {
``` 



[File:IncentivizedERC20.sol#L26](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/protocol/tokenization/IncentivizedERC20.sol#L26) 
```solidity
25:  constructor(
26:    string memory name,
27:    string memory symbol,
28:    uint8 decimals
29:  ) public {
``` 



[File:CrossChainEnabledPolygonChild.sol#L32](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/crosschain/polygon/CrossChainEnabledPolygonChild.sol#L32) 
```solidity
31:    constructor(address fxChild) {
``` 



[File:ArraysMock.sol#L12](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/ArraysMock.sol#L12) 
```solidity
11:    constructor(uint256[] memory array) {
``` 



[File:ArraysMock.sol#L30](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/ArraysMock.sol#L30) 
```solidity
29:    constructor(address[] memory array) {
``` 



[File:ArraysMock.sol#L44](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/ArraysMock.sol#L44) 
```solidity
43:    constructor(bytes32[] memory array) {
``` 



[File:ERC4626Mock.sol#L7](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/ERC4626Mock.sol#L7) 
```solidity
6:    constructor(address underlying) ERC20("ERC4626Mock", "E4626M") ERC4626(IERC20(underlying)) {}
``` 



[File:MyTokenWrapped.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/docs/governance/MyTokenWrapped.sol#L10) 
```solidity
9:    constructor(
10:        IERC20 wrappedToken
11:    ) ERC20("MyTokenWrapped", "MTK") ERC20Permit("MyTokenWrapped") ERC20Wrapper(wrappedToken) {}
``` 



[File:ERC20WrapperHarness.sol#L8](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/certora/harnesses/ERC20WrapperHarness.sol#L8) 
```solidity
7:    constructor(IERC20 _underlying, string memory _name, string memory _symbol) ERC20(_name, _symbol) ERC20Wrapper(_underlying) {}
``` 



[File:ERC20WrapperHarness.sol#L8](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/certora/harnesses/ERC20WrapperHarness.sol#L8) 
```solidity
7:    constructor(IERC20 _underlying, string memory _name, string memory _symbol) ERC20(_name, _symbol) ERC20Wrapper(_underlying) {}
``` 



[File:ERC20WrapperHarness.sol#L8](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/certora/harnesses/ERC20WrapperHarness.sol#L8) 
```solidity
7:    constructor(IERC20 _underlying, string memory _name, string memory _symbol) ERC20(_name, _symbol) ERC20Wrapper(_underlying) {}
``` 



[File:ERC4626.sol#L34](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/lib/solmate/src/mixins/ERC4626.sol#L34) 
```solidity
33:    constructor(
34:        ERC20 _asset,
35:        string memory _name,
36:        string memory _symbol
37:    ) ERC20(_name, _symbol, _asset.decimals()) {
``` 



[File:ERC4626.sol#L34](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/lib/solmate/src/mixins/ERC4626.sol#L34) 
```solidity
33:    constructor(
34:        ERC20 _asset,
35:        string memory _name,
36:        string memory _symbol
37:    ) ERC20(_name, _symbol, _asset.decimals()) {
``` 



[File:ERC4626.sol#L34](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/lib/solmate/src/mixins/ERC4626.sol#L34) 
```solidity
33:    constructor(
34:        ERC20 _asset,
35:        string memory _name,
36:        string memory _symbol
37:    ) ERC20(_name, _symbol, _asset.decimals()) {
``` 



[File:GovernorPreventLateQuorum.sol#L37](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/governance/extensions/GovernorPreventLateQuorum.sol#L37) 
```solidity
36:    constructor(uint64 initialVoteExtension) {
``` 



[File:Owned.sol#L29](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/lib/solmate/src/auth/Owned.sol#L29) 
```solidity
28:    constructor(address _owner) {
``` 



[File:ERC4626.sol#L59](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/token/ERC20/extensions/ERC4626.sol#L59) 
```solidity
58:    constructor(IERC20 asset_) {
``` 



[File:ERC4626.sol#L59](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/token/ERC20/extensions/ERC4626.sol#L59) 
```solidity
58:    constructor(IERC20 asset_) {
``` 



[File:ERC4626.sol#L59](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/token/ERC20/extensions/ERC4626.sol#L59) 
```solidity
58:    constructor(IERC20 asset_) {
``` 



[File:GovernorVotesComp.sol#L17](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/governance/extensions/GovernorVotesComp.sol#L17) 
```solidity
16:    constructor(ERC20VotesComp token_) {
``` 



[File:ERC3156FlashBorrowerMock.sol#L25](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/ERC3156FlashBorrowerMock.sol#L25) 
```solidity
24:    constructor(bool enableReturn, bool enableApprove) {
``` 



[File:ERC3156FlashBorrowerMock.sol#L25](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/ERC3156FlashBorrowerMock.sol#L25) 
```solidity
24:    constructor(bool enableReturn, bool enableApprove) {
``` 



[File:MyGovernor1.sol#L17](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/wizard/MyGovernor1.sol#L17) 
```solidity
16:    constructor(
17:        IVotes _token,
18:        TimelockController _timelock
19:    ) Governor("MyGovernor") GovernorVotes(_token) GovernorVotesQuorumFraction(4) GovernorTimelockControl(_timelock) {}
``` 



[File:MyGovernor1.sol#L17](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/wizard/MyGovernor1.sol#L17) 
```solidity
16:    constructor(
17:        IVotes _token,
18:        TimelockController _timelock
19:    ) Governor("MyGovernor") GovernorVotes(_token) GovernorVotesQuorumFraction(4) GovernorTimelockControl(_timelock) {}
``` 



[File:Governor.sol#L84](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/governance/Governor.sol#L84) 
```solidity
83:    constructor(string memory name_) EIP712(name_, version()) {
``` 



[File:AdminUpgradeabilityProxy.sol#L21](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/dependencies/openzeppelin/upgradeability/AdminUpgradeabilityProxy.sol#L21) 
```solidity
20:  constructor(
21:    address _logic,
22:    address _admin,
23:    bytes memory _data
24:  ) public payable UpgradeabilityProxy(_logic, _data) {
``` 



[File:AdminUpgradeabilityProxy.sol#L21](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/dependencies/openzeppelin/upgradeability/AdminUpgradeabilityProxy.sol#L21) 
```solidity
20:  constructor(
21:    address _logic,
22:    address _admin,
23:    bytes memory _data
24:  ) public payable UpgradeabilityProxy(_logic, _data) {
``` 



[File:AdminUpgradeabilityProxy.sol#L21](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/dependencies/openzeppelin/upgradeability/AdminUpgradeabilityProxy.sol#L21) 
```solidity
20:  constructor(
21:    address _logic,
22:    address _admin,
23:    bytes memory _data
24:  ) public payable UpgradeabilityProxy(_logic, _data) {
``` 



[File:CrossChainEnabledAMB.sol#L32](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/crosschain/amb/CrossChainEnabledAMB.sol#L32) 
```solidity
31:    constructor(address bridge) {
``` 



[File:CrossChainEnabledPolygonChild.sol#L32](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/crosschain/polygon/CrossChainEnabledPolygonChild.sol#L32) 
```solidity
31:    constructor(address fxChild) {
``` 



[File:TransactionValidator.sol#L54](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/src/core/TransactionValidator.sol#L54) 
```solidity
53:    constructor(address _addressProvider) AddressProviderService(_addressProvider) {}
``` 



[File:ERC721ConsecutiveMock.sol#L14](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/token/ERC721ConsecutiveMock.sol#L14) 
```solidity
13:    constructor(
14:        string memory name,
15:        string memory symbol,
16:        address[] memory delegates,
17:        address[] memory receivers,
18:        uint96[] memory amounts
19:    ) ERC721(name, symbol) EIP712(name, "1") {
``` 



[File:ERC721ConsecutiveMock.sol#L14](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/token/ERC721ConsecutiveMock.sol#L14) 
```solidity
13:    constructor(
14:        string memory name,
15:        string memory symbol,
16:        address[] memory delegates,
17:        address[] memory receivers,
18:        uint96[] memory amounts
19:    ) ERC721(name, symbol) EIP712(name, "1") {
``` 



[File:ERC721ConsecutiveMock.sol#L14](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/token/ERC721ConsecutiveMock.sol#L14) 
```solidity
13:    constructor(
14:        string memory name,
15:        string memory symbol,
16:        address[] memory delegates,
17:        address[] memory receivers,
18:        uint96[] memory amounts
19:    ) ERC721(name, symbol) EIP712(name, "1") {
``` 



[File:ERC721ConsecutiveMock.sol#L14](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/token/ERC721ConsecutiveMock.sol#L14) 
```solidity
13:    constructor(
14:        string memory name,
15:        string memory symbol,
16:        address[] memory delegates,
17:        address[] memory receivers,
18:        uint96[] memory amounts
19:    ) ERC721(name, symbol) EIP712(name, "1") {
``` 



[File:ERC721ConsecutiveMock.sol#L14](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/token/ERC721ConsecutiveMock.sol#L14) 
```solidity
13:    constructor(
14:        string memory name,
15:        string memory symbol,
16:        address[] memory delegates,
17:        address[] memory receivers,
18:        uint96[] memory amounts
19:    ) ERC721(name, symbol) EIP712(name, "1") {
``` 



[File:ERC721ConsecutiveMock.sol#L58](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/token/ERC721ConsecutiveMock.sol#L58) 
```solidity
57:    constructor(string memory name, string memory symbol) ERC721(name, symbol) {
``` 



[File:ERC721ConsecutiveMock.sol#L58](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/token/ERC721ConsecutiveMock.sol#L58) 
```solidity
57:    constructor(string memory name, string memory symbol) ERC721(name, symbol) {
``` 



[File:ApprovalToZero.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/ext/woke/weird/ApprovalToZero.sol#L10) 
```solidity
9:    constructor(uint _totalSupply) ERC20(_totalSupply) public {}
``` 



[File:FlashLoanReceiverBase.sol#L18](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/flashloan/base/FlashLoanReceiverBase.sol#L18) 
```solidity
17:  constructor(ILendingPoolAddressesProvider provider) public {
``` 



[File:ERC2771Context.sol#L16](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/metatx/ERC2771Context.sol#L16) 
```solidity
15:    constructor(address trustedForwarder) {
``` 



[File:TransferFee.sol#L13](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/ext/woke/weird/TransferFee.sol#L13) 
```solidity
12:    constructor(uint _totalSupply, uint _fee) ERC20(_totalSupply) public {
``` 



[File:TransferFee.sol#L13](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/ext/woke/weird/TransferFee.sol#L13) 
```solidity
12:    constructor(uint _totalSupply, uint _fee) ERC20(_totalSupply) public {
``` 



[File:TimelockControllerHarness.sol#L6](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/certora/harnesses/TimelockControllerHarness.sol#L6) 
```solidity
5:    constructor(
6:        uint256 minDelay,
7:        address[] memory proposers,
8:        address[] memory executors,
9:        address admin
10:    ) TimelockController(minDelay, proposers, executors, admin) {}
``` 



[File:TimelockControllerHarness.sol#L6](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/certora/harnesses/TimelockControllerHarness.sol#L6) 
```solidity
5:    constructor(
6:        uint256 minDelay,
7:        address[] memory proposers,
8:        address[] memory executors,
9:        address admin
10:    ) TimelockController(minDelay, proposers, executors, admin) {}
``` 



[File:TimelockControllerHarness.sol#L6](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/certora/harnesses/TimelockControllerHarness.sol#L6) 
```solidity
5:    constructor(
6:        uint256 minDelay,
7:        address[] memory proposers,
8:        address[] memory executors,
9:        address admin
10:    ) TimelockController(minDelay, proposers, executors, admin) {}
``` 



[File:TimelockControllerHarness.sol#L6](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/certora/harnesses/TimelockControllerHarness.sol#L6) 
```solidity
5:    constructor(
6:        uint256 minDelay,
7:        address[] memory proposers,
8:        address[] memory executors,
9:        address admin
10:    ) TimelockController(minDelay, proposers, executors, admin) {}
``` 



[File:ERC4626.sol#L34](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solmate/src/mixins/ERC4626.sol#L34) 
```solidity
33:    constructor(
34:        ERC20 _asset,
35:        string memory _name,
36:        string memory _symbol
37:    ) ERC20(_name, _symbol, _asset.decimals()) {
``` 



[File:ERC4626.sol#L34](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solmate/src/mixins/ERC4626.sol#L34) 
```solidity
33:    constructor(
34:        ERC20 _asset,
35:        string memory _name,
36:        string memory _symbol
37:    ) ERC20(_name, _symbol, _asset.decimals()) {
``` 



[File:ERC4626.sol#L34](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solmate/src/mixins/ERC4626.sol#L34) 
```solidity
33:    constructor(
34:        ERC20 _asset,
35:        string memory _name,
36:        string memory _symbol
37:    ) ERC20(_name, _symbol, _asset.decimals()) {
``` 



[File:receivers.sol#L27](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/crosschain/receivers.sol#L27) 
```solidity
26:    constructor(address bridge) CrossChainEnabledAMB(bridge) {}
``` 



[File:receivers.sol#L35](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/crosschain/receivers.sol#L35) 
```solidity
34:    constructor(address bridge) CrossChainEnabledArbitrumL1(bridge) {}
``` 



[File:receivers.sol#L45](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/crosschain/receivers.sol#L45) 
```solidity
44:    constructor(address bridge) CrossChainEnabledOptimism(bridge) {}
``` 



[File:receivers.sol#L53](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/crosschain/receivers.sol#L53) 
```solidity
52:    constructor(address bridge) CrossChainEnabledPolygonChild(bridge) {}
``` 



[File:MyGovernor2.sol#L19](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/wizard/MyGovernor2.sol#L19) 
```solidity
18:    constructor(
19:        IVotes _token,
20:        TimelockController _timelock
21:    ) Governor("MyGovernor") GovernorVotes(_token) GovernorVotesQuorumFraction(4) GovernorTimelockControl(_timelock) {}
``` 



[File:MyGovernor2.sol#L19](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/wizard/MyGovernor2.sol#L19) 
```solidity
18:    constructor(
19:        IVotes _token,
20:        TimelockController _timelock
21:    ) Governor("MyGovernor") GovernorVotes(_token) GovernorVotesQuorumFraction(4) GovernorTimelockControl(_timelock) {}
``` 



[File:ERC4626.sol#L59](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/token/ERC20/extensions/ERC4626.sol#L59) 
```solidity
58:    constructor(IERC20 asset_) {
``` 



[File:ERC4626.sol#L59](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/token/ERC20/extensions/ERC4626.sol#L59) 
```solidity
58:    constructor(IERC20 asset_) {
``` 



[File:ERC4626.sol#L59](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/token/ERC20/extensions/ERC4626.sol#L59) 
```solidity
58:    constructor(IERC20 asset_) {
``` 



[File:ERC4626OffsetMock.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/token/ERC4626OffsetMock.sol#L10) 
```solidity
9:    constructor(uint8 offset_) {
``` 



[File:PaymentSplitter.sol#L51](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/finance/PaymentSplitter.sol#L51) 
```solidity
50:    constructor(address[] memory payees, uint256[] memory shares_) payable {
``` 



[File:PaymentSplitter.sol#L51](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/finance/PaymentSplitter.sol#L51) 
```solidity
50:    constructor(address[] memory payees, uint256[] memory shares_) payable {
``` 



[File:MyGovernor2.sol#L19](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/wizard/MyGovernor2.sol#L19) 
```solidity
18:    constructor(
19:        IVotes _token,
20:        TimelockController _timelock
21:    ) Governor("MyGovernor") GovernorVotes(_token) GovernorVotesQuorumFraction(4) GovernorTimelockControl(_timelock) {}
``` 



[File:MyGovernor2.sol#L19](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/wizard/MyGovernor2.sol#L19) 
```solidity
18:    constructor(
19:        IVotes _token,
20:        TimelockController _timelock
21:    ) Governor("MyGovernor") GovernorVotes(_token) GovernorVotesQuorumFraction(4) GovernorTimelockControl(_timelock) {}
``` 



[File:GovernorVotes.sol#L17](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/governance/extensions/GovernorVotes.sol#L17) 
```solidity
16:    constructor(IVotes tokenAddress) {
``` 



[File:BlockList.sol#L19](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/ext/woke/weird/BlockList.sol#L19) 
```solidity
18:    constructor(uint _totalSupply) ERC20(_totalSupply) public {
``` 



[File:ReturnsFalse.sol#L28](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/ext/woke/weird/ReturnsFalse.sol#L28) 
```solidity
27:    constructor(uint _totalSupply) public {
``` 



[File:StableAndVariableTokensHelper.sol#L16](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/deployments/StableAndVariableTokensHelper.sol#L16) 
```solidity
15:  constructor(address payable _pool, address _addressesProvider) public {
``` 



[File:StableAndVariableTokensHelper.sol#L16](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/deployments/StableAndVariableTokensHelper.sol#L16) 
```solidity
15:  constructor(address payable _pool, address _addressesProvider) public {
``` 



[File:MintableDelegationERC20.sol#L13](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/mocks/tokens/MintableDelegationERC20.sol#L13) 
```solidity
12:  constructor(
13:    string memory name,
14:    string memory symbol,
15:    uint8 decimals
16:  ) public ERC20(name, symbol) {
``` 



[File:MintableDelegationERC20.sol#L13](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/mocks/tokens/MintableDelegationERC20.sol#L13) 
```solidity
12:  constructor(
13:    string memory name,
14:    string memory symbol,
15:    uint8 decimals
16:  ) public ERC20(name, symbol) {
``` 



[File:MintableDelegationERC20.sol#L13](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/mocks/tokens/MintableDelegationERC20.sol#L13) 
```solidity
12:  constructor(
13:    string memory name,
14:    string memory symbol,
15:    uint8 decimals
16:  ) public ERC20(name, symbol) {
``` 



[File:ERC1155ReceiverMock.sol#L17](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/token/ERC1155ReceiverMock.sol#L17) 
```solidity
16:    constructor(bytes4 recRetval, bool recReverts, bytes4 batRetval, bool batReverts) {
``` 



[File:ERC1155ReceiverMock.sol#L17](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/token/ERC1155ReceiverMock.sol#L17) 
```solidity
16:    constructor(bytes4 recRetval, bool recReverts, bytes4 batRetval, bool batReverts) {
``` 



[File:ERC1155ReceiverMock.sol#L17](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/token/ERC1155ReceiverMock.sol#L17) 
```solidity
16:    constructor(bytes4 recRetval, bool recReverts, bytes4 batRetval, bool batReverts) {
``` 



[File:ERC1155ReceiverMock.sol#L17](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/token/ERC1155ReceiverMock.sol#L17) 
```solidity
16:    constructor(bytes4 recRetval, bool recReverts, bytes4 batRetval, bool batReverts) {
``` 



[File:GovernorSettings.sol#L25](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/governance/extensions/GovernorSettings.sol#L25) 
```solidity
24:    constructor(uint256 initialVotingDelay, uint256 initialVotingPeriod, uint256 initialProposalThreshold) {
``` 



[File:GovernorSettings.sol#L25](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/governance/extensions/GovernorSettings.sol#L25) 
```solidity
24:    constructor(uint256 initialVotingDelay, uint256 initialVotingPeriod, uint256 initialProposalThreshold) {
``` 



[File:GovernorSettings.sol#L25](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/governance/extensions/GovernorSettings.sol#L25) 
```solidity
24:    constructor(uint256 initialVotingDelay, uint256 initialVotingPeriod, uint256 initialProposalThreshold) {
``` 



[File:WalletRegistry.sol#L29](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/src/core/registries/WalletRegistry.sol#L29) 
```solidity
28:    constructor(address _addressProvider) AddressProviderService(_addressProvider) {}
``` 



[File:PaymentSplitter.sol#L51](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/finance/PaymentSplitter.sol#L51) 
```solidity
50:    constructor(address[] memory payees, uint256[] memory shares_) payable {
``` 



[File:PaymentSplitter.sol#L51](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/finance/PaymentSplitter.sol#L51) 
```solidity
50:    constructor(address[] memory payees, uint256[] memory shares_) payable {
``` 



[File:UpgradeabilityProxy.sol#L20](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/dependencies/openzeppelin/upgradeability/UpgradeabilityProxy.sol#L20) 
```solidity
19:  constructor(address _logic, bytes memory _data) public payable {
``` 



[File:UpgradeabilityProxy.sol#L20](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/dependencies/openzeppelin/upgradeability/UpgradeabilityProxy.sol#L20) 
```solidity
19:  constructor(address _logic, bytes memory _data) public payable {
``` 



[File:UpgradeabilityProxy.sol#L20](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/dependencies/openzeppelin/upgradeability/UpgradeabilityProxy.sol#L20) 
```solidity
19:  constructor(address _logic, bytes memory _data) public payable {
``` 



[File:BaseUniswapAdapter.sol#L40](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/adapters/BaseUniswapAdapter.sol#L40) 
```solidity
39:  constructor(
40:    ILendingPoolAddressesProvider addressesProvider,
41:    IUniswapV2Router02 uniswapRouter,
42:    address wethAddress
43:  ) public FlashLoanReceiverBase(addressesProvider) {
``` 



[File:BaseUniswapAdapter.sol#L40](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/adapters/BaseUniswapAdapter.sol#L40) 
```solidity
39:  constructor(
40:    ILendingPoolAddressesProvider addressesProvider,
41:    IUniswapV2Router02 uniswapRouter,
42:    address wethAddress
43:  ) public FlashLoanReceiverBase(addressesProvider) {
``` 



[File:BaseUniswapAdapter.sol#L40](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/adapters/BaseUniswapAdapter.sol#L40) 
```solidity
39:  constructor(
40:    ILendingPoolAddressesProvider addressesProvider,
41:    IUniswapV2Router02 uniswapRouter,
42:    address wethAddress
43:  ) public FlashLoanReceiverBase(addressesProvider) {
``` 



[File:TransferFee.sol#L13](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/ext/woke/weird/TransferFee.sol#L13) 
```solidity
12:    constructor(uint _totalSupply, uint _fee) ERC20(_totalSupply) public {
``` 



[File:TransferFee.sol#L13](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/ext/woke/weird/TransferFee.sol#L13) 
```solidity
12:    constructor(uint _totalSupply, uint _fee) ERC20(_totalSupply) public {
``` 



[File:Governor.sol#L84](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/governance/Governor.sol#L84) 
```solidity
83:    constructor(string memory name_) EIP712(name_, version()) {
``` 



[File:ERC1271WalletMock.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/ERC1271WalletMock.sol#L10) 
```solidity
9:    constructor(address originalOwner) {
``` 



[File:GovernorVotesQuorumFraction.sol#L33](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/governance/extensions/GovernorVotesQuorumFraction.sol#L33) 
```solidity
32:    constructor(uint256 quorumNumeratorValue) {
``` 



[File:ERC20.sol#L57](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/dependencies/openzeppelin/contracts/ERC20.sol#L57) 
```solidity
56:  constructor(string memory name, string memory symbol) public {
``` 



[File:ERC20.sol#L57](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/dependencies/openzeppelin/contracts/ERC20.sol#L57) 
```solidity
56:  constructor(string memory name, string memory symbol) public {
``` 



[File:MyGovernor3.sol#L17](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/wizard/MyGovernor3.sol#L17) 
```solidity
16:    constructor(
17:        IVotes _token,
18:        TimelockController _timelock
19:    ) Governor("MyGovernor") GovernorVotes(_token) GovernorVotesQuorumFraction(4) GovernorTimelockControl(_timelock) {}
``` 



[File:MyGovernor3.sol#L17](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/wizard/MyGovernor3.sol#L17) 
```solidity
16:    constructor(
17:        IVotes _token,
18:        TimelockController _timelock
19:    ) Governor("MyGovernor") GovernorVotes(_token) GovernorVotesQuorumFraction(4) GovernorTimelockControl(_timelock) {}
``` 



[File:ArraysMock.sol#L12](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/ArraysMock.sol#L12) 
```solidity
11:    constructor(uint256[] memory array) {
``` 



[File:ArraysMock.sol#L30](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/ArraysMock.sol#L30) 
```solidity
29:    constructor(address[] memory array) {
``` 



[File:ArraysMock.sol#L44](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/ArraysMock.sol#L44) 
```solidity
43:    constructor(bytes32[] memory array) {
``` 



[File:UpgradeableBeacon.sol#L28](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/proxy/beacon/UpgradeableBeacon.sol#L28) 
```solidity
27:    constructor(address implementation_) {
``` 



[File:Owned.sol#L29](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solmate/src/auth/Owned.sol#L29) 
```solidity
28:    constructor(address _owner) {
``` 



[File:ERC20Mock.sol#L56](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/ext/woke/ERC20Mock.sol#L56) 
```solidity
55:    constructor(string memory name_, string memory symbol_, uint8 decimals_) {
``` 



[File:ERC20Mock.sol#L56](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/ext/woke/ERC20Mock.sol#L56) 
```solidity
55:    constructor(string memory name_, string memory symbol_, uint8 decimals_) {
``` 



[File:ERC20Mock.sol#L56](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solady/ext/woke/ERC20Mock.sol#L56) 
```solidity
55:    constructor(string memory name_, string memory symbol_, uint8 decimals_) {
``` 



[File:UpgradeableBeacon.sol#L28](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/proxy/beacon/UpgradeableBeacon.sol#L28) 
```solidity
27:    constructor(address implementation_) {
``` 



[File:Uint96.sol#L32](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/solady/ext/woke/weird/Uint96.sol#L32) 
```solidity
31:    constructor(uint96 _supply) public {
``` 



[File:TimelockControllerHarness.sol#L6](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/certora/harnesses/TimelockControllerHarness.sol#L6) 
```solidity
5:    constructor(
6:        uint256 minDelay,
7:        address[] memory proposers,
8:        address[] memory executors,
9:        address admin
10:    ) TimelockController(minDelay, proposers, executors, admin) {}
``` 



[File:TimelockControllerHarness.sol#L6](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/certora/harnesses/TimelockControllerHarness.sol#L6) 
```solidity
5:    constructor(
6:        uint256 minDelay,
7:        address[] memory proposers,
8:        address[] memory executors,
9:        address admin
10:    ) TimelockController(minDelay, proposers, executors, admin) {}
``` 



[File:TimelockControllerHarness.sol#L6](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/certora/harnesses/TimelockControllerHarness.sol#L6) 
```solidity
5:    constructor(
6:        uint256 minDelay,
7:        address[] memory proposers,
8:        address[] memory executors,
9:        address admin
10:    ) TimelockController(minDelay, proposers, executors, admin) {}
``` 



[File:TimelockControllerHarness.sol#L6](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/certora/harnesses/TimelockControllerHarness.sol#L6) 
```solidity
5:    constructor(
6:        uint256 minDelay,
7:        address[] memory proposers,
8:        address[] memory executors,
9:        address admin
10:    ) TimelockController(minDelay, proposers, executors, admin) {}
``` 



[File:ERC721.sol#L57](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solmate/src/tokens/ERC721.sol#L57) 
```solidity
56:    constructor(string memory _name, string memory _symbol) {
``` 



[File:ERC721.sol#L57](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/solmate/src/tokens/ERC721.sol#L57) 
```solidity
56:    constructor(string memory _name, string memory _symbol) {
``` 



[File:EIP712.sol#L31](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solady/src/utils/EIP712.sol#L31) 
```solidity
30:    constructor() {
``` 



[File:EIP712.sol#L31](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solady/src/utils/EIP712.sol#L31) 
```solidity
30:    constructor() {
``` 



[File:ERC20PresetMinterPauser.sol#L38](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/token/ERC20/presets/ERC20PresetMinterPauser.sol#L38) 
```solidity
37:    constructor(string memory name, string memory symbol) ERC20(name, symbol) {
``` 



[File:ERC20PresetMinterPauser.sol#L38](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/token/ERC20/presets/ERC20PresetMinterPauser.sol#L38) 
```solidity
37:    constructor(string memory name, string memory symbol) ERC20(name, symbol) {
``` 



[File:ERC721ConsecutiveMock.sol#L14](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/token/ERC721ConsecutiveMock.sol#L14) 
```solidity
13:    constructor(
14:        string memory name,
15:        string memory symbol,
16:        address[] memory delegates,
17:        address[] memory receivers,
18:        uint96[] memory amounts
19:    ) ERC721(name, symbol) EIP712(name, "1") {
``` 



[File:ERC721ConsecutiveMock.sol#L14](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/token/ERC721ConsecutiveMock.sol#L14) 
```solidity
13:    constructor(
14:        string memory name,
15:        string memory symbol,
16:        address[] memory delegates,
17:        address[] memory receivers,
18:        uint96[] memory amounts
19:    ) ERC721(name, symbol) EIP712(name, "1") {
``` 



[File:ERC721ConsecutiveMock.sol#L14](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/token/ERC721ConsecutiveMock.sol#L14) 
```solidity
13:    constructor(
14:        string memory name,
15:        string memory symbol,
16:        address[] memory delegates,
17:        address[] memory receivers,
18:        uint96[] memory amounts
19:    ) ERC721(name, symbol) EIP712(name, "1") {
``` 



[File:ERC721ConsecutiveMock.sol#L14](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/token/ERC721ConsecutiveMock.sol#L14) 
```solidity
13:    constructor(
14:        string memory name,
15:        string memory symbol,
16:        address[] memory delegates,
17:        address[] memory receivers,
18:        uint96[] memory amounts
19:    ) ERC721(name, symbol) EIP712(name, "1") {
``` 



[File:ERC721ConsecutiveMock.sol#L14](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/token/ERC721ConsecutiveMock.sol#L14) 
```solidity
13:    constructor(
14:        string memory name,
15:        string memory symbol,
16:        address[] memory delegates,
17:        address[] memory receivers,
18:        uint96[] memory amounts
19:    ) ERC721(name, symbol) EIP712(name, "1") {
``` 



[File:ERC721ConsecutiveMock.sol#L58](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/token/ERC721ConsecutiveMock.sol#L58) 
```solidity
57:    constructor(string memory name, string memory symbol) ERC721(name, symbol) {
``` 



[File:ERC721ConsecutiveMock.sol#L58](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/token/ERC721ConsecutiveMock.sol#L58) 
```solidity
57:    constructor(string memory name, string memory symbol) ERC721(name, symbol) {
``` 



[File:ERC20WrapperHarness.sol#L8](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/certora/harnesses/ERC20WrapperHarness.sol#L8) 
```solidity
7:    constructor(IERC20 _underlying, string memory _name, string memory _symbol) ERC20(_name, _symbol) ERC20Wrapper(_underlying) {}
``` 



[File:ERC20WrapperHarness.sol#L8](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/certora/harnesses/ERC20WrapperHarness.sol#L8) 
```solidity
7:    constructor(IERC20 _underlying, string memory _name, string memory _symbol) ERC20(_name, _symbol) ERC20Wrapper(_underlying) {}
``` 



[File:ERC20WrapperHarness.sol#L8](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/certora/harnesses/ERC20WrapperHarness.sol#L8) 
```solidity
7:    constructor(IERC20 _underlying, string memory _name, string memory _symbol) ERC20(_name, _symbol) ERC20Wrapper(_underlying) {}
``` 



[File:GovernorSettings.sol#L25](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/governance/extensions/GovernorSettings.sol#L25) 
```solidity
24:    constructor(uint256 initialVotingDelay, uint256 initialVotingPeriod, uint256 initialProposalThreshold) {
``` 



[File:GovernorSettings.sol#L25](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/governance/extensions/GovernorSettings.sol#L25) 
```solidity
24:    constructor(uint256 initialVotingDelay, uint256 initialVotingPeriod, uint256 initialProposalThreshold) {
``` 



[File:GovernorSettings.sol#L25](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/governance/extensions/GovernorSettings.sol#L25) 
```solidity
24:    constructor(uint256 initialVotingDelay, uint256 initialVotingPeriod, uint256 initialProposalThreshold) {
``` 



[File:PolicyRegistry.sol#L24](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/src/core/registries/PolicyRegistry.sol#L24) 
```solidity
23:    constructor(address _addressProvider) AddressProviderService(_addressProvider) {}
``` 



[File:RolesAuthority.sol#L24](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solady/lib/solmate/src/auth/authorities/RolesAuthority.sol#L24) 
```solidity
23:    constructor(address _owner, Authority _authority) Auth(_owner, _authority) {}
``` 



[File:RolesAuthority.sol#L24](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/solady/lib/solmate/src/auth/authorities/RolesAuthority.sol#L24) 
```solidity
23:    constructor(address _owner, Authority _authority) Auth(_owner, _authority) {}
``` 



[File:ERC20Capped.sol#L18](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/token/ERC20/extensions/ERC20Capped.sol#L18) 
```solidity
17:    constructor(uint256 cap_) {
``` 



[File:AaveOracle.sol#L37](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/misc/AaveOracle.sol#L37) 
```solidity
36:  constructor(
37:    address[] memory assets,
38:    address[] memory sources,
39:    address fallbackOracle,
40:    address baseCurrency,
41:    uint256 baseCurrencyUnit
42:  ) public {
``` 



[File:AaveOracle.sol#L37](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/misc/AaveOracle.sol#L37) 
```solidity
36:  constructor(
37:    address[] memory assets,
38:    address[] memory sources,
39:    address fallbackOracle,
40:    address baseCurrency,
41:    uint256 baseCurrencyUnit
42:  ) public {
``` 



[File:AaveOracle.sol#L37](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/misc/AaveOracle.sol#L37) 
```solidity
36:  constructor(
37:    address[] memory assets,
38:    address[] memory sources,
39:    address fallbackOracle,
40:    address baseCurrency,
41:    uint256 baseCurrencyUnit
42:  ) public {
``` 



[File:AaveOracle.sol#L37](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/misc/AaveOracle.sol#L37) 
```solidity
36:  constructor(
37:    address[] memory assets,
38:    address[] memory sources,
39:    address fallbackOracle,
40:    address baseCurrency,
41:    uint256 baseCurrencyUnit
42:  ) public {
``` 



[File:AaveOracle.sol#L37](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/misc/AaveOracle.sol#L37) 
```solidity
36:  constructor(
37:    address[] memory assets,
38:    address[] memory sources,
39:    address fallbackOracle,
40:    address baseCurrency,
41:    uint256 baseCurrencyUnit
42:  ) public {
``` 



[File:AccessControlDefaultAdminRulesHarness.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/certora/harnesses/AccessControlDefaultAdminRulesHarness.sol#L10) 
```solidity
9:    constructor(
10:        uint48 initialDelay,
11:        address initialDefaultAdmin,
12:        uint48 delayIncreaseWait
13:    ) AccessControlDefaultAdminRules(initialDelay, initialDefaultAdmin) {
``` 



[File:AccessControlDefaultAdminRulesHarness.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/certora/harnesses/AccessControlDefaultAdminRulesHarness.sol#L10) 
```solidity
9:    constructor(
10:        uint48 initialDelay,
11:        address initialDefaultAdmin,
12:        uint48 delayIncreaseWait
13:    ) AccessControlDefaultAdminRules(initialDelay, initialDefaultAdmin) {
``` 



[File:AccessControlDefaultAdminRulesHarness.sol#L10](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/certora/harnesses/AccessControlDefaultAdminRulesHarness.sol#L10) 
```solidity
9:    constructor(
10:        uint48 initialDelay,
11:        address initialDefaultAdmin,
12:        uint48 delayIncreaseWait
13:    ) AccessControlDefaultAdminRules(initialDelay, initialDefaultAdmin) {
``` 



[File:WildcatMarketControllerFactory.sol#L72](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/src/WildcatMarketControllerFactory.sol#L72) 
```solidity
71:  constructor(
72:    address _archController,
73:    address _sentinel,
74:    MarketParameterConstraints memory constraints
75:  ) {
``` 



[File:WildcatMarketControllerFactory.sol#L72](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/src/WildcatMarketControllerFactory.sol#L72) 
```solidity
71:  constructor(
72:    address _archController,
73:    address _sentinel,
74:    MarketParameterConstraints memory constraints
75:  ) {
``` 



[File:WildcatMarketControllerFactory.sol#L72](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/src/WildcatMarketControllerFactory.sol#L72) 
```solidity
71:  constructor(
72:    address _archController,
73:    address _sentinel,
74:    MarketParameterConstraints memory constraints
75:  ) {
``` 



[File:GovernorPreventLateQuorumMock.sol#L18](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/governance/GovernorPreventLateQuorumMock.sol#L18) 
```solidity
17:    constructor(uint256 quorum_) {
``` 



 --- 


