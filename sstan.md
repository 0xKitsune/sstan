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
 | [[L-0]](#[L-0]) | <Strong>Use a locked pragma version instead of a floating pragma version</Strong> - Instances: 29 |
 | [[H-1]](#[H-1]) | <Strong>Uninitialized storage variables</Strong> - Instances: 11 |
# <h3>Optimizations</h3> 

 | Classification | Title | 
 |-------|---------| 
 | [[G-0]](#[G-0]) | <Strong>Cache array length during for loop definition.</Strong> - Instances: 4 |
 | [[G-1]](#[G-1]) | <Strong>Cache Storage Variables in Memory</Strong> - Instances: 7 |
 | [[G-2]](#[G-2]) | <Strong>Mark storage variables as `constant` if they never change.</Strong> - Instances: 4 |
 | [[G-3]](#[G-3]) | <Strong>Event is not properly indexed.</Strong> - Instances: 5 |
 | [[G-4]](#[G-4]) | <Strong>Mark storage variables as `immutable` if they never change after contract initialization.</Strong> - Instances: 2 |
 | [[G-5]](#[G-5]) | <Strong>`unchecked{++i}` instead of `i++` (or use assembly when applicable)</Strong> - Instances: 4 |
 | [[G-6]](#[G-6]) | <Strong>Use `calldata` instead of `memory` for function arguments that do not get mutated.</Strong> - Instances: 8 |
 | [[G-7]](#[G-7]) | <Strong>Optimal Comparison</Strong> - Instances: 2 |
 | [[G-8]](#[G-8]) | <Strong>Tightly pack storage variables</Strong> - Instances: 2 |
 | [[G-9]](#[G-9]) | <Strong>Mark functions as payable (with discretion)</Strong> - Instances: 19 |
 | [[G-10]](#[G-10]) | <Strong>Consider marking constants as private</Strong> - Instances: 7 |
 | [[G-11]](#[G-11]) | <Strong>Right shift or Left shift instead of dividing or multiplying by powers of two</Strong> - Instances: 1 |
 | [[G-12]](#[G-12]) | <Strong>Use assembly to hash instead of Solidity</Strong> - Instances: 2 |
 | [[G-13]](#[G-13]) | <Strong>Use assembly for math (add, sub, mul, div)</Strong> - Instances: 5 |
 | [[G-14]](#[G-14]) | <Strong>Use assembly to write storage values</Strong> - Instances: 8 |
 | [[G-15]](#[G-15]) | <Strong>Use custom errors instead of string error messages</Strong> - Instances: 2 |
# <h3>Quality Assurance</h3> 

 | Classification | Title | 
 |-------|---------| 
 | [[NC-0]](#[NC-0]) | <Strong>Constructor should be listed before any other function</Strong> - Instances: 1 |
 | [[NC-1]](#[NC-1]) | <Strong>Private variables should contain a leading underscore</Strong> - Instances: 13 |
 | [[NC-2]](#[NC-2]) | <Strong>Constructor should initialize all variables</Strong> - Instances: 7 |
 | [[NC-3]](#[NC-3]) | <Strong>Consider importing specific identifiers instead of the whole file</Strong> - Instances: 2 |
 | [[NC-4]](#[NC-4]) | <Strong>Constants & Immutables should be named with screaming snake case</Strong> - Instances: 9 |
 | [[NC-5]](#[NC-5]) | <Strong>Remove any unused functions</Strong> - Instances: 15 |
 | [[NC-6]](#[NC-6]) | <Strong>Remove any unused returns</Strong> - Instances: 7 |

 <details open> 
 <summary> 
 <h3>Vulnerabilities - Instances: 2 </h3> 
 </summary> 
  


 <details open> 
 <summary> 
 <a name=[L-0]></a> [L-0] 
 <h3> Use a locked pragma version instead of a floating pragma version - Instances: 29 </h3> 
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

 

```solidity
File:IWorldIDGroups.sol 
1:pragma solidity ^0.8.21;
``` 



```solidity
File:ITreeVerifier.sol 
1:pragma solidity ^0.8.21;
``` 



```solidity
File:WorldIDRouterImplMock.sol 
1:pragma solidity ^0.8.21;
``` 



```solidity
File:SimpleStateBridge.sol 
1:pragma solidity ^0.8.21;
``` 



```solidity
File:VerifierLookupTable.sol 
1:pragma solidity ^0.8.21;
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
1:pragma solidity ^0.8.21;
``` 



```solidity
File:IWorldID.sol 
1:pragma solidity ^0.8.21;
``` 



```solidity
File:WorldIDImpl.sol 
1:pragma solidity ^0.8.21;
``` 



```solidity
File:SequencerVerifier.sol 
1:pragma solidity ^0.8.21;
``` 



```solidity
File:CheckInitialized.sol 
1:pragma solidity ^0.8.21;
``` 



```solidity
File:WorldIDIdentityManagerImplMock.sol 
1:pragma solidity ^0.8.21;
``` 



```solidity
File:WorldIDProxy.sol 
1:pragma solidity ^0.8.21;
``` 



```solidity
File:IBaseWorldID.sol 
1:pragma solidity ^0.8.21;
``` 



```solidity
File:WorldIDRouterImplV1.sol 
1:pragma solidity ^0.8.21;
``` 



```solidity
File:IBridge.sol 
1:pragma solidity ^0.8.21;
``` 



```solidity
File:SemaphoreTreeDepthValidator.sol 
1:pragma solidity ^0.8.21;
``` 



```solidity
File:TypeConverter.sol 
1:pragma solidity ^0.8.21;
``` 



```solidity
File:SimpleSemaphoreVerifier.sol 
1:pragma solidity ^0.8.21;
``` 



```solidity
File:VerifierLookupTableTest.sol 
1:pragma solidity ^0.8.21;
``` 



```solidity
File:WorldIDRouterTest.sol 
1:pragma solidity ^0.8.21;
``` 



```solidity
File:DeletionTreeVerifier.sol 
21:pragma solidity ^0.8.21;
``` 



```solidity
File:WorldIDRouter.sol 
1:pragma solidity ^0.8.21;
``` 



```solidity
File:WorldIDTest.sol 
1:pragma solidity ^0.8.21;
``` 



```solidity
File:UnimplementedTreeVerifier.sol 
1:pragma solidity ^0.8.21;
``` 



```solidity
File:WorldIDIdentityManagerImplV2.sol 
0:pragma solidity ^0.8.21;
``` 



```solidity
File:WorldIDIdentityManager.sol 
1:pragma solidity ^0.8.21;
``` 



```solidity
File:SimpleVerifier.sol 
1:pragma solidity ^0.8.21;
``` 



```solidity
File:InsertionTreeVerifier.sol 
21:pragma solidity ^0.8.21;
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
1:pragma solidity ^0.8.21;
``` 

 
 </details>

 <details open> 
 <summary> 
 <a name=[H-1]></a> [H-1] 
 <h3> Uninitialized storage variables - Instances: 11 </h3> 
 </summary>
 
> A storage variable that is declared but not initialized will have a default value of zero (or the equivalent, such as an empty array for array types or zero-address for address types). Failing to initialize a storage variable can pose risks if the contract logic assumes that the variable has been explicitly set to a particular value. 

```solidity
File:CheckInitialized.sol 
37:    uint256[49] private __gap;
``` 



```solidity
File:VerifierLookupTableTest.sol 
17:    ITreeVerifier internal nullVerifier = ITreeVerifier(address(0x0));
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
93:    bool internal _isStateBridgeEnabled;
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
90:    IBridge internal _stateBridge;
``` 



```solidity
File:WorldIDRouterTest.sol 
26:    IWorldID internal nullManager = IWorldID(nullAddress);
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
77:    bytes packedDeletionIndices = abi.encodePacked(
78:        uint32(0), uint32(2), uint32(4), uint32(6), uint32(8), uint32(10), uint32(12), uint32(14)
79:    );
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
88:    ITreeVerifier unimplementedVerifier = new UnimplementedTreeVerifier();
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
80:    uint32 deletionBatchSize = 8;
``` 



```solidity
File:WorldIDTest.sol 
16:    Vm internal hevm = Vm(HEVM_ADDRESS);
``` 



```solidity
File:WorldIDTest.sol 
18:    address internal thisAddress = address(this);
``` 



```solidity
File:WorldIDTest.sol 
17:    address internal nullAddress = address(0x0);
``` 

 
 </details> 
 </details>

 <details open> 
 <summary> 
 <h3>Optimizations - Instances: 16 </h3> 
 </summary> 
  


 <details open> 
 <summary> 
 <a name=[G-0]></a> [G-0] 
 <h3> Cache array length during for loop definition. - Instances: 4 </h3> 
 </summary>
 
 
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
 

```solidity
File:WorldIDIdentityManagerTest.sol 
263:        for (uint256 i = 0; i < batchSizes.length; ++i) {
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
300:        for (uint256 i = 0; i < arr.length; ++i) {
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
320:        for (uint256 i = 0; i < idents.length; ++i) {
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
324:        for (uint256 i = 0; i < idents.length; ++i) {
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
371:        for (uint256 i = 0; i < idents.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
35:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
48:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
61:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
74:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
87:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
100:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
113:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
126:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
139:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
152:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
165:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
178:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
191:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
204:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
217:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
230:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
243:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
256:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
269:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
282:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
295:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
308:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
321:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
334:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
347:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
360:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
373:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
386:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
399:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
412:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
425:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
438:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
451:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
464:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
477:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
490:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
503:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
516:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
529:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
542:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:DeletionTreeVerifier.sol 
292:        for (uint256 i = 0; i < input.length; i++) {
``` 



```solidity
File:InsertionTreeVerifier.sol 
248:        for (uint256 i = 0; i < input.length; i++) {
``` 

 
 </details>

 <details open> 
 <summary> 
 <a name=[G-1]></a> [G-1] 
 <h3> Cache Storage Variables in Memory - Instances: 7 </h3> 
 </summary>
 
  
 Cache Array Length - Gas Report - Savings: ~0 
 <details>  
 <summary>  
  </summary> 
  
 </details> 
 

```solidity
File:VerifierLookupTable.sol 
127:        verifier_lut[batchSize] = verifier;
``` 



```solidity
File:WorldIDRouterTest.sol 
42:        makeNewRouter(thisWorldID);
``` 



```solidity
File:WorldIDRouterTest.sol 
60:        routerImplAddress = address(routerImpl);
``` 



```solidity
File:WorldIDRouterTest.sol 
68:        router = new Router(routerImplAddress, initCallData);
``` 



```solidity
File:WorldIDRouterTest.sol 
69:        routerAddress = address(router);
``` 



```solidity
File:WorldIDRouterTest.sol 
76:        routerImplAddress = address(routerImpl);
``` 



```solidity
File:WorldIDRouterTest.sol 
77:        router = new Router(routerImplAddress, new bytes(0x0));
``` 



```solidity
File:WorldIDRouterTest.sol 
78:        routerAddress = address(router);
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
353:            revert NotLatestRoot(preRoot, _latestRoot);
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
384:            _latestRoot = postRoot;
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
446:            revert NotLatestRoot(preRoot, _latestRoot);
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
631:            return RootInfo(_latestRoot, 0, true);
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
701:        batchInsertionVerifiers = newTable;
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
737:        identityUpdateVerifiers = newTable;
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
770:        semaphoreVerifier = newVerifier;
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
805:        rootHistoryExpiry = newExpiryTime;
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
840:        _identityOperator = newIdentityOperator;
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
104:        identityCommitments[0] = 0x1;
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
105:        identityCommitments[1] = 0x2;
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
106:        identityCommitments[2] = 0x3;
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
138:        defaultInsertVerifiers.addVerifier(initialBatchSize, treeVerifier);
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
138:        defaultInsertVerifiers.addVerifier(initialBatchSize, treeVerifier);
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
138:        defaultInsertVerifiers.addVerifier(initialBatchSize, treeVerifier);
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
142:            defaultInsertVerifiers,
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
174:        managerImplV1Address = address(managerImplV1);
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
187:        identityManager = new IdentityManager(managerImplV1Address, initCallData);
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
188:        identityManagerAddress = address(identityManager);
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
192:        managerImplAddress = address(managerImpl);
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
196:            UUPSUpgradeable.upgradeToAndCall, (address(managerImplAddress), initCallV2)
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
200:        assertCallSucceedsOn(identityManagerAddress, upgradeCall, new bytes(0x0));
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
280:        managerImplAddress = address(managerImpl);
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
281:        identityManager = new IdentityManager(managerImplAddress, new bytes(0x0));
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
282:        identityManagerAddress = address(identityManager);
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
289:        slotCounter = (slotCounter + 1) % (identityCommitments.length - 1);
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
289:        slotCounter = (slotCounter + 1) % (identityCommitments.length - 1);
``` 



```solidity
File:VerifierLookupTableTest.sol 
48:        defaultVerifierAddress = address(defaultVerifier);
``` 



```solidity
File:VerifierLookupTableTest.sol 
51:        lookupTableAddress = address(lookupTable);
``` 



```solidity
File:VerifierLookupTableTest.sol 
52:        lookupTable.addVerifier(initialBatchSize, defaultVerifier);
``` 



```solidity
File:VerifierLookupTableTest.sol 
52:        lookupTable.addVerifier(initialBatchSize, defaultVerifier);
``` 



```solidity
File:WorldIDRouterImplV1.sol 
176:        return routingTable[groupNumber];
``` 



```solidity
File:WorldIDRouterImplV1.sol 
276:        routingTable[groupId] = newTarget;
``` 



```solidity
File:WorldIDIdentityManagerImplV2.sol 
183:        batchDeletionVerifiers = newTable;
``` 

 
 </details>

 <details open> 
 <summary> 
 <a name=[G-2]></a> [G-2] 
 <h3> Mark storage variables as `constant` if they never change. - Instances: 4 </h3> 
 </summary>
 
 
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
 

```solidity
File:WorldIDTest.sol 
18:    address internal thisAddress = address(this);
``` 



```solidity
File:WorldIDTest.sol 
17:    address internal nullAddress = address(0x0);
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
77:    bytes packedDeletionIndices = abi.encodePacked(
78:        uint32(0), uint32(2), uint32(4), uint32(6), uint32(8), uint32(10), uint32(12), uint32(14)
79:    );
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
80:    uint32 deletionBatchSize = 8;
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
92:    uint256 initialBatchSize = 30;
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
39:    uint8 internal treeDepth = 16;
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
38:    uint256 internal initialRoot = 0x0;
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
63:    uint256 identityCommitmentsSize = 3;
``` 



```solidity
File:VerifierLookupTableTest.sol 
18:    uint256 internal defaultBatchSize = 30;
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
93:    bool internal _isStateBridgeEnabled;
``` 

 
 </details>

 <details open> 
 <summary> 
 <a name=[G-3]></a> [G-3] 
 <h3> Event is not properly indexed. - Instances: 5 </h3> 
 </summary>
 
 
> When possible, always include a minimum of 3 indexed event topics to save gas 
 
#### Gas Report - Savings: ~0 
 <details>  
 <summary>  
  </summary> 
  
 </details> 
 

```solidity
File:SimpleVerifier.sol 
11:    event VerifiedProof(uint256 batchSize);
``` 



```solidity
File:WorldIDRouterImplV1.sol 
95:    event GroupIdentityManagerRouterImplInitialized(IWorldID initialGroupIdentityManager);
``` 



```solidity
File:WorldIDRouterTest.sol 
32:    event GroupIdentityManagerRouterImplInitialized(IWorldID initialGroupIdentityManager);
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
243:    event WorldIDIdentityManagerImplInitialized(uint8 _treeDepth, uint256 initialRoot);
``` 



```solidity
File:SimpleStateBridge.sol 
12:    event SetRootHistoryExpiry(uint256 expiryTime);
``` 

 
 </details>

 <details open> 
 <summary> 
 <a name=[G-4]></a> [G-4] 
 <h3> Mark storage variables as `immutable` if they never change after contract initialization. - Instances: 2 </h3> 
 </summary>
 
 
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
 

```solidity
File:SimpleVerifier.sol 
9:    uint256 batchSize;
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
62:    uint256[] identityCommitments;
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
64:    uint256[8] insertionProof;
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
81:    uint256[8] deletionProof;
``` 

 
 </details>

 <details open> 
 <summary> 
 <a name=[G-5]></a> [G-5] 
 <h3> `unchecked{++i}` instead of `i++` (or use assembly when applicable) - Instances: 4 </h3> 
 </summary>
 
 
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
 

```solidity
File:WorldIDIdentityManagerTest.sol 
263:        for (uint256 i = 0; i < batchSizes.length; ++i) {
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
324:        for (uint256 i = 0; i < idents.length; ++i) {
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
371:        for (uint256 i = 0; i < idents.length; ++i) {
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
300:        for (uint256 i = 0; i < arr.length; ++i) {
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
320:        for (uint256 i = 0; i < idents.length; ++i) {
``` 



```solidity
File:DeletionTreeVerifier.sol 
292:        for (uint256 i = 0; i < input.length; i++) {
``` 



```solidity
File:DeletionTreeVerifier.sol 
152:        for (uint256 i = 0; i < 4; i++) {
``` 



```solidity
File:TypeConverter.sol 
87:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
113:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
425:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
347:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
451:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
152:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
165:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
217:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
100:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
61:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
191:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
139:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
282:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
438:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
464:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
503:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
529:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
399:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
126:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
178:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
8:        for (uint256 i = 0; i < 20; i++) {
``` 



```solidity
File:TypeConverter.sol 
295:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
477:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
321:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
243:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
35:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
360:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
308:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
516:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
542:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
204:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
386:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
269:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
230:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
334:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
412:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
256:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
48:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
373:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
74:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:TypeConverter.sol 
490:        for (uint256 i = 0; i < input.length; ++i) {
``` 



```solidity
File:InsertionTreeVerifier.sol 
117:        for (uint256 i = 0; i < 4; i++) {
``` 



```solidity
File:InsertionTreeVerifier.sol 
248:        for (uint256 i = 0; i < input.length; i++) {
``` 

 
 </details>

 <details open> 
 <summary> 
 <a name=[G-6]></a> [G-6] 
 <h3> Use `calldata` instead of `memory` for function arguments that do not get mutated. - Instances: 8 </h3> 
 </summary>
 
 
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
 

```solidity
File:TypeConverter.sol 
33:    function makeDynArray(uint8[1] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
46:    function makeDynArray(uint16[1] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
59:    function makeDynArray(uint32[1] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
72:    function makeDynArray(uint256[1] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
85:    function makeDynArray(uint8[2] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
98:    function makeDynArray(uint16[2] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
111:    function makeDynArray(uint32[2] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
124:    function makeDynArray(uint256[2] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
137:    function makeDynArray(uint8[3] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
150:    function makeDynArray(uint16[3] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
163:    function makeDynArray(uint32[3] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
176:    function makeDynArray(uint256[3] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
189:    function makeDynArray(uint8[4] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
202:    function makeDynArray(uint16[4] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
215:    function makeDynArray(uint32[4] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
228:    function makeDynArray(uint256[4] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
241:    function makeDynArray(uint8[5] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
254:    function makeDynArray(uint16[5] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
267:    function makeDynArray(uint32[5] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
280:    function makeDynArray(uint256[5] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
293:    function makeDynArray(uint8[6] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
306:    function makeDynArray(uint16[6] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
319:    function makeDynArray(uint32[6] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
332:    function makeDynArray(uint256[6] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
345:    function makeDynArray(uint8[7] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
358:    function makeDynArray(uint16[7] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
371:    function makeDynArray(uint32[7] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
384:    function makeDynArray(uint256[7] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
397:    function makeDynArray(uint8[8] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
410:    function makeDynArray(uint16[8] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
423:    function makeDynArray(uint32[8] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
436:    function makeDynArray(uint256[8] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
449:    function makeDynArray(uint8[9] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
462:    function makeDynArray(uint16[9] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
475:    function makeDynArray(uint32[9] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
488:    function makeDynArray(uint256[9] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
501:    function makeDynArray(uint8[10] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
514:    function makeDynArray(uint16[10] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
527:    function makeDynArray(uint32[10] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
540:    function makeDynArray(uint256[10] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:UnimplementedTreeVerifier.sol 
27:        uint256[2] memory a,
``` 



```solidity
File:UnimplementedTreeVerifier.sol 
30:        uint256[1] memory input
31:    ) external pure returns (bool) {
``` 



```solidity
File:UnimplementedTreeVerifier.sol 
28:        uint256[2][2] memory b,
``` 



```solidity
File:UnimplementedTreeVerifier.sol 
29:        uint256[2] memory c,
``` 



```solidity
File:SequencerVerifier.sol 
15:        uint256[2][2] memory b,
``` 



```solidity
File:SequencerVerifier.sol 
17:        uint256[1] memory input
18:    ) external pure override returns (bool) {
``` 



```solidity
File:SequencerVerifier.sol 
14:        uint256[2] memory a,
``` 



```solidity
File:SequencerVerifier.sol 
16:        uint256[2] memory c,
``` 



```solidity
File:SimpleVerifier.sol 
21:        uint256[1] memory input
22:    ) external override returns (bool result) {
``` 



```solidity
File:SimpleVerifier.sol 
19:        uint256[2][2] memory b,
``` 



```solidity
File:SimpleVerifier.sol 
20:        uint256[2] memory c,
``` 



```solidity
File:SimpleVerifier.sol 
18:        uint256[2] memory a,
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
245:    function makeVerifierLookupTables(uint256[] memory batchSizes)
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
298:    function cloneArray(uint256[] memory arr) public pure returns (uint256[] memory out) {
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
315:    function prepareInsertIdentitiesTestCase(uint128[] memory idents, uint128[8] memory prf)
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
315:    function prepareInsertIdentitiesTestCase(uint128[] memory idents, uint128[8] memory prf)
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
338:    function prepareDeleteIdentitiesTestCase(uint128[8] memory prf)
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
357:    function prepareUpdateIdentitiesTestCase(uint128[] memory idents, uint128[8] memory prf)
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
357:    function prepareUpdateIdentitiesTestCase(uint128[] memory idents, uint128[8] memory prf)
``` 



```solidity
File:WorldIDTest.sol 
28:    function assertCallSucceedsOn(address target, bytes memory callData) public {
``` 



```solidity
File:WorldIDTest.sol 
40:        bytes memory callData,
``` 



```solidity
File:WorldIDTest.sol 
41:        bytes memory expectedReturnData
42:    ) public {
``` 



```solidity
File:WorldIDTest.sol 
52:    function assertCallFailsOn(address target, bytes memory callData) public {
``` 



```solidity
File:WorldIDTest.sol 
64:        bytes memory callData,
``` 



```solidity
File:WorldIDTest.sol 
65:        bytes memory expectedReturnData
66:    ) public {
``` 



```solidity
File:WorldIDTest.sol 
78:    function encodeStringRevert(string memory reason) public pure returns (bytes memory data) {
``` 



```solidity
File:InsertionTreeVerifier.sol 
45:    function negate(G1Point memory p) internal pure returns (G1Point memory) {
``` 



```solidity
File:InsertionTreeVerifier.sol 
57:    function plus(G1Point memory p1, G1Point memory p2) internal view returns (G1Point memory r) {
``` 



```solidity
File:InsertionTreeVerifier.sol 
57:    function plus(G1Point memory p1, G1Point memory p2) internal view returns (G1Point memory r) {
``` 



```solidity
File:InsertionTreeVerifier.sol 
81:    function scalar_mul(G1Point memory p, uint256 s) internal view returns (G1Point memory r) {
``` 



```solidity
File:InsertionTreeVerifier.sol 
103:        G1Point memory a1,
``` 



```solidity
File:InsertionTreeVerifier.sol 
105:        G1Point memory b1,
``` 



```solidity
File:InsertionTreeVerifier.sol 
106:        G2Point memory b2,
``` 



```solidity
File:InsertionTreeVerifier.sol 
110:        G2Point memory d2
111:    ) internal view returns (bool) {
``` 



```solidity
File:InsertionTreeVerifier.sol 
109:        G1Point memory d1,
``` 



```solidity
File:InsertionTreeVerifier.sol 
104:        G2Point memory a2,
``` 



```solidity
File:InsertionTreeVerifier.sol 
107:        G1Point memory c1,
``` 



```solidity
File:InsertionTreeVerifier.sol 
108:        G2Point memory c2,
``` 



```solidity
File:InsertionTreeVerifier.sol 
222:        uint256[1] memory input
223:    ) public view returns (bool r) {
``` 



```solidity
File:InsertionTreeVerifier.sol 
220:        uint256[2][2] memory b,
``` 



```solidity
File:InsertionTreeVerifier.sol 
219:        uint256[2] memory a,
``` 



```solidity
File:InsertionTreeVerifier.sol 
221:        uint256[2] memory c,
``` 



```solidity
File:DeletionTreeVerifier.sol 
45:    function negate(G1Point memory p) internal pure returns (G1Point memory) {
``` 



```solidity
File:DeletionTreeVerifier.sol 
57:    function plus(G1Point memory p1, G1Point memory p2) internal view returns (G1Point memory r) {
``` 



```solidity
File:DeletionTreeVerifier.sol 
57:    function plus(G1Point memory p1, G1Point memory p2) internal view returns (G1Point memory r) {
``` 



```solidity
File:DeletionTreeVerifier.sol 
80:    function plus_raw(uint256[4] memory input, G1Point memory r) internal view {
``` 



```solidity
File:DeletionTreeVerifier.sol 
80:    function plus_raw(uint256[4] memory input, G1Point memory r) internal view {
``` 



```solidity
File:DeletionTreeVerifier.sol 
99:    function scalar_mul(G1Point memory p, uint256 s) internal view returns (G1Point memory r) {
``` 



```solidity
File:DeletionTreeVerifier.sol 
119:    function scalar_mul_raw(uint256[3] memory input, G1Point memory r) internal view {
``` 



```solidity
File:DeletionTreeVerifier.sol 
119:    function scalar_mul_raw(uint256[3] memory input, G1Point memory r) internal view {
``` 



```solidity
File:DeletionTreeVerifier.sol 
143:        G2Point memory c2,
``` 



```solidity
File:DeletionTreeVerifier.sol 
138:        G1Point memory a1,
``` 



```solidity
File:DeletionTreeVerifier.sol 
141:        G2Point memory b2,
``` 



```solidity
File:DeletionTreeVerifier.sol 
144:        G1Point memory d1,
``` 



```solidity
File:DeletionTreeVerifier.sol 
140:        G1Point memory b1,
``` 



```solidity
File:DeletionTreeVerifier.sol 
142:        G1Point memory c1,
``` 



```solidity
File:DeletionTreeVerifier.sol 
145:        G2Point memory d2
146:    ) internal view returns (bool) {
``` 



```solidity
File:DeletionTreeVerifier.sol 
139:        G2Point memory a2,
``` 



```solidity
File:DeletionTreeVerifier.sol 
245:        uint256[3] memory mul_input,
``` 



```solidity
File:DeletionTreeVerifier.sol 
246:        Pairing.G1Point memory p,
``` 



```solidity
File:DeletionTreeVerifier.sol 
248:        Pairing.G1Point memory q
249:    ) internal view {
``` 



```solidity
File:DeletionTreeVerifier.sol 
270:        uint256[2] memory c,
``` 



```solidity
File:DeletionTreeVerifier.sol 
268:        uint256[2] memory a,
``` 



```solidity
File:DeletionTreeVerifier.sol 
269:        uint256[2][2] memory b,
``` 

 
 </details>

 <details open> 
 <summary> 
 <a name=[G-7]></a> [G-7] 
 <h3> Optimal Comparison - Instances: 2 </h3> 
 </summary>
 
 
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
 

```solidity
File:WorldIDRouterImplV1.sol 
166:        if (groupNumber >= groupCount()) {
``` 



```solidity
File:WorldIDRouterImplV1.sol 
271:        if (groupId >= groupCount()) {
``` 



```solidity
File:SemaphoreTreeDepthValidator.sol 
14:        return treeDepth >= minDepth && treeDepth <= maxDepth;
``` 



```solidity
File:SemaphoreTreeDepthValidator.sol 
14:        return treeDepth >= minDepth && treeDepth <= maxDepth;
``` 

 
 </details>

 <details open> 
 <summary> 
 <a name=[G-8]></a> [G-8] 
 <h3> Tightly pack storage variables - Instances: 2 </h3> 
 </summary>
 
 
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
 

```solidity
File:WorldIDIdentityManagerImplV1.sol 
59:    address internal _identityOperator;
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
31:    IdentityManager internal identityManager;
``` 

 
 </details>

 <details open> 
 <summary> 
 <a name=[G-9]></a> [G-9] 
 <h3> Mark functions as payable (with discretion) - Instances: 19 </h3> 
 </summary>
 
 
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
 

```solidity
File:WorldIDIdentityManagerImplV1.sol 
144:    function NO_SUCH_ROOT() public pure returns (RootInfo memory rootInfo) {
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
275:    function initialize(
276:        uint8 _treeDepth,
277:        uint256 initialRoot,
278:        VerifierLookupTable _batchInsertionVerifiers,
279:        VerifierLookupTable _batchUpdateVerifiers,
280:        ISemaphoreVerifier _semaphoreVerifier
281:    ) public reinitializer(1) {
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
345:    function registerIdentities(
346:        uint256[8] calldata insertionProof,
347:        uint256 preRoot,
348:        uint32 startIndex,
349:        uint256[] calldata identityCommitments,
350:        uint256 postRoot
351:    ) public virtual onlyProxy onlyInitialized onlyIdentityOperator {
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
437:    function updateIdentities(
438:        uint256[8] calldata updateProof,
439:        uint256 preRoot,
440:        uint32[] calldata leafIndices,
441:        uint256[] calldata oldIdentities,
442:        uint256[] calldata newIdentities,
443:        uint256 postRoot
444:    ) public virtual onlyProxy onlyInitialized onlyIdentityOperator {
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
555:    function calculateIdentityRegistrationInputHash(
556:        uint32 startIndex,
557:        uint256 preRoot,
558:        uint256 postRoot,
559:        uint256[] calldata identityCommitments
560:    ) public view virtual onlyProxy onlyInitialized returns (bytes32 hash) {
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
595:    function calculateIdentityUpdateInputHash(
596:        uint256 preRoot,
597:        uint256 postRoot,
598:        uint32[] calldata leafIndices,
599:        uint256[] calldata oldIdentities,
600:        uint256[] calldata newIdentities
601:    ) public view virtual onlyProxy onlyInitialized returns (bytes32 hash) {
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
611:    function latestRoot() public view virtual onlyProxy onlyInitialized returns (uint256) {
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
622:    function queryRoot(uint256 root)
623:        public
624:        view
625:        virtual
626:        onlyProxy
627:        onlyInitialized
628:        returns (RootInfo memory)
629:    {
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
652:    function requireValidRoot(uint256 root) public view virtual onlyProxy onlyInitialized {
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
676:    function getRegisterIdentitiesVerifierLookupTableAddress()
677:        public
678:        view
679:        virtual
680:        onlyProxy
681:        onlyInitialized
682:        returns (address)
683:    {
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
693:    function setRegisterIdentitiesVerifierLookupTable(VerifierLookupTable newTable)
694:        public
695:        virtual
696:        onlyProxy
697:        onlyInitialized
698:        onlyOwner
699:    {
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
712:    function getIdentityUpdateVerifierLookupTableAddress()
713:        public
714:        view
715:        virtual
716:        onlyProxy
717:        onlyInitialized
718:        returns (address)
719:    {
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
729:    function setIdentityUpdateVerifierLookupTable(VerifierLookupTable newTable)
730:        public
731:        virtual
732:        onlyProxy
733:        onlyInitialized
734:        onlyOwner
735:    {
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
746:    function getSemaphoreVerifierAddress()
747:        public
748:        view
749:        virtual
750:        onlyProxy
751:        onlyInitialized
752:        returns (address)
753:    {
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
762:    function setSemaphoreVerifier(ISemaphoreVerifier newVerifier)
763:        public
764:        virtual
765:        onlyProxy
766:        onlyInitialized
767:        onlyOwner
768:    {
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
779:    function getRootHistoryExpiry()
780:        public
781:        view
782:        virtual
783:        onlyProxy
784:        onlyInitialized
785:        returns (uint256)
786:    {
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
794:    function setRootHistoryExpiry(uint256 newExpiryTime)
795:        public
796:        virtual
797:        onlyProxy
798:        onlyInitialized
799:        onlyOwner
800:    {
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
813:    function getTreeDepth() public view virtual onlyProxy onlyInitialized returns (uint8) {
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
821:    function identityOperator() public view virtual onlyProxy onlyInitialized returns (address) {
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
831:    function setIdentityOperator(address newIdentityOperator)
832:        public
833:        virtual
834:        onlyProxy
835:        onlyInitialized
836:        onlyOwner
837:        returns (address)
838:    {
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
860:    function verifyProof(
861:        uint256 root,
862:        uint256 signalHash,
863:        uint256 nullifierHash,
864:        uint256 externalNullifierHash,
865:        uint256[8] calldata proof
866:    ) public view virtual onlyProxy onlyInitialized {
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
135:    function setUp() public {
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
165:    function makeNewIdentityManager(
166:        uint8 actualTreeDepth,
167:        uint256 actualPreRoot,
168:        VerifierLookupTable insertVerifiers,
169:        VerifierLookupTable deletionVerifiers,
170:        VerifierLookupTable updateVerifiers,
171:        ISemaphoreVerifier actualSemaphoreVerifier
172:    ) public {
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
212:    function makeNewIdentityManager(uint256 actualPreRoot, uint256[] calldata batchSizes) public {
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
245:    function makeVerifierLookupTables(uint256[] memory batchSizes)
246:        public
247:        returns (
248:            VerifierLookupTable insertVerifiers,
249:            VerifierLookupTable deletionVerifiers,
250:            VerifierLookupTable updateVerifiers
251:        )
252:    {
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
278:    function makeUninitIdentityManager() public {
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
287:    function rotateSlot() public returns (uint256) {
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
298:    function cloneArray(uint256[] memory arr) public pure returns (uint256[] memory out) {
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
315:    function prepareInsertIdentitiesTestCase(uint128[] memory idents, uint128[8] memory prf)
316:        public
317:        pure
318:        returns (uint256[] memory preparedIdents, uint256[8] memory actualProof)
319:    {
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
338:    function prepareDeleteIdentitiesTestCase(uint128[8] memory prf)
339:        public
340:        pure
341:        returns (uint256[8] memory actualProof)
342:    {
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
357:    function prepareUpdateIdentitiesTestCase(uint128[] memory idents, uint128[8] memory prf)
358:        public
359:        pure
360:        returns (
361:            uint32[] memory leafIndices,
362:            uint256[] memory oldIdents,
363:            uint256[] memory newIdents,
364:            uint256[8] memory actualProof
365:        )
366:    {
``` 



```solidity
File:SimpleSemaphoreVerifier.sol 
11:    function verifyProof(
12:        uint256 merkleTreeRoot,
13:        uint256 nullifierHash,
14:        uint256 signal,
15:        uint256 externalNullifier,
16:        uint256[8] calldata proof,
17:        uint256 merkleTreeDepth
18:    ) external pure {
``` 



```solidity
File:DeletionTreeVerifier.sol 
267:    function verifyProof(
268:        uint256[2] memory a,
269:        uint256[2][2] memory b,
270:        uint256[2] memory c,
271:        uint256[1] calldata input
272:    ) public view returns (bool r) {
``` 



```solidity
File:WorldIDRouterImplMock.sol 
15:    function initialize(uint32 data) public virtual reinitializer(2) {
``` 



```solidity
File:WorldIDRouterImplMock.sol 
20:    function someMoreData() public view returns (uint32) {
``` 



```solidity
File:WorldIDRouterTest.sol 
40:    function setUp() public {
``` 



```solidity
File:WorldIDRouterTest.sol 
58:    function makeNewRouter(IWorldID initialGroupAddress) public {
``` 



```solidity
File:WorldIDRouterTest.sol 
74:    function makeUninitRouter() public {
``` 



```solidity
File:SimpleVerifier.sol 
17:    function verifyProof(
18:        uint256[2] memory a,
19:        uint256[2][2] memory b,
20:        uint256[2] memory c,
21:        uint256[1] memory input
22:    ) external override returns (bool result) {
``` 



```solidity
File:SimpleVerifier.sol 
35:    function isValidInput(uint256 a) public pure returns (bool) {
``` 



```solidity
File:SimpleVerifier.sol 
39:    function calculateInputHash(
40:        uint32 startIndex,
41:        uint256 preRoot,
42:        uint256 postRoot,
43:        uint256[] calldata identityCommitments
44:    ) public pure returns (bytes32 hash) {
``` 



```solidity
File:WorldIDImpl.sol 
85:    function renounceOwnership() public view override onlyOwner {
``` 



```solidity
File:SequencerVerifier.sol 
13:    function verifyProof(
14:        uint256[2] memory a,
15:        uint256[2][2] memory b,
16:        uint256[2] memory c,
17:        uint256[1] memory input
18:    ) external pure override returns (bool) {
``` 



```solidity
File:SimpleStateBridge.sol 
14:    function sendRootMultichain(uint256 root) external virtual override {
``` 



```solidity
File:SimpleStateBridge.sol 
18:    function setRootHistoryExpiry(uint256 expiryTime) external virtual override {
``` 



```solidity
File:SimpleStateBridge.sol 
26:    function verifyProof(uint256 root, uint256, uint256, uint256, uint256[8] calldata proof)
27:        external
28:    {
``` 



```solidity
File:WorldIDIdentityManagerImplMock.sol 
15:    function initialize(uint32 data) public virtual reinitializer(3) {
``` 



```solidity
File:WorldIDIdentityManagerImplMock.sol 
20:    function someMoreData() public view returns (uint32) {
``` 



```solidity
File:WorldIDTest.sol 
28:    function assertCallSucceedsOn(address target, bytes memory callData) public {
``` 



```solidity
File:WorldIDTest.sol 
38:    function assertCallSucceedsOn(
39:        address target,
40:        bytes memory callData,
41:        bytes memory expectedReturnData
42:    ) public {
``` 



```solidity
File:WorldIDTest.sol 
52:    function assertCallFailsOn(address target, bytes memory callData) public {
``` 



```solidity
File:WorldIDTest.sol 
62:    function assertCallFailsOn(
63:        address target,
64:        bytes memory callData,
65:        bytes memory expectedReturnData
66:    ) public {
``` 



```solidity
File:WorldIDTest.sol 
78:    function encodeStringRevert(string memory reason) public pure returns (bytes memory data) {
``` 



```solidity
File:InsertionTreeVerifier.sol 
218:    function verifyProof(
219:        uint256[2] memory a,
220:        uint256[2][2] memory b,
221:        uint256[2] memory c,
222:        uint256[1] memory input
223:    ) public view returns (bool r) {
``` 



```solidity
File:TypeConverter.sol 
4:    function toString(address x) public pure returns (string memory) {
``` 



```solidity
File:TypeConverter.sol 
18:    function char(bytes1 b) public pure returns (bytes1 c) {
``` 



```solidity
File:TypeConverter.sol 
33:    function makeDynArray(uint8[1] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
46:    function makeDynArray(uint16[1] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
59:    function makeDynArray(uint32[1] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
72:    function makeDynArray(uint256[1] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
85:    function makeDynArray(uint8[2] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
98:    function makeDynArray(uint16[2] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
111:    function makeDynArray(uint32[2] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
124:    function makeDynArray(uint256[2] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
137:    function makeDynArray(uint8[3] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
150:    function makeDynArray(uint16[3] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
163:    function makeDynArray(uint32[3] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
176:    function makeDynArray(uint256[3] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
189:    function makeDynArray(uint8[4] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
202:    function makeDynArray(uint16[4] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
215:    function makeDynArray(uint32[4] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
228:    function makeDynArray(uint256[4] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
241:    function makeDynArray(uint8[5] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
254:    function makeDynArray(uint16[5] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
267:    function makeDynArray(uint32[5] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
280:    function makeDynArray(uint256[5] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
293:    function makeDynArray(uint8[6] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
306:    function makeDynArray(uint16[6] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
319:    function makeDynArray(uint32[6] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
332:    function makeDynArray(uint256[6] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
345:    function makeDynArray(uint8[7] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
358:    function makeDynArray(uint16[7] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
371:    function makeDynArray(uint32[7] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
384:    function makeDynArray(uint256[7] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
397:    function makeDynArray(uint8[8] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
410:    function makeDynArray(uint16[8] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
423:    function makeDynArray(uint32[8] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
436:    function makeDynArray(uint256[8] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
449:    function makeDynArray(uint8[9] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
462:    function makeDynArray(uint16[9] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
475:    function makeDynArray(uint32[9] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
488:    function makeDynArray(uint256[9] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
501:    function makeDynArray(uint8[10] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
514:    function makeDynArray(uint16[10] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
527:    function makeDynArray(uint32[10] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:TypeConverter.sol 
540:    function makeDynArray(uint256[10] memory input) public pure returns (uint256[] memory array) {
``` 



```solidity
File:VerifierLookupTable.sol 
87:    function getVerifierFor(uint256 batchSize) public view returns (ITreeVerifier verifier) {
``` 



```solidity
File:VerifierLookupTable.sol 
102:    function addVerifier(uint256 batchSize, ITreeVerifier verifier) public onlyOwner {
``` 



```solidity
File:VerifierLookupTable.sol 
121:    function updateVerifier(uint256 batchSize, ITreeVerifier verifier)
122:        public
123:        onlyOwner
124:        returns (ITreeVerifier oldVerifier)
125:    {
``` 



```solidity
File:VerifierLookupTable.sol 
138:    function disableVerifier(uint256 batchSize)
139:        public
140:        onlyOwner
141:        returns (ITreeVerifier oldVerifier)
142:    {
``` 



```solidity
File:VerifierLookupTable.sol 
172:    function renounceOwnership() public view override onlyOwner {
``` 



```solidity
File:UnimplementedTreeVerifier.sol 
26:    function verifyProof(
27:        uint256[2] memory a,
28:        uint256[2][2] memory b,
29:        uint256[2] memory c,
30:        uint256[1] memory input
31:    ) external pure returns (bool) {
``` 



```solidity
File:VerifierLookupTableTest.sol 
29:    function setUp() public {
``` 



```solidity
File:VerifierLookupTableTest.sol 
46:    function makeNewLUT(uint256 initialBatchSize) public {
``` 



```solidity
File:WorldIDRouterImplV1.sol 
122:    function initialize(IWorldID initialGroupIdentityManager) public reinitializer(1) {
``` 



```solidity
File:WorldIDRouterImplV1.sol 
157:    function routeFor(uint256 groupNumber)
158:        public
159:        view
160:        virtual
161:        onlyProxy
162:        onlyInitialized
163:        returns (IWorldID)
164:    {
``` 



```solidity
File:WorldIDRouterImplV1.sol 
195:    function addGroup(IWorldID groupIdentityManager)
196:        public
197:        virtual
198:        onlyProxy
199:        onlyInitialized
200:        onlyOwner
201:    {
``` 



```solidity
File:WorldIDRouterImplV1.sol 
220:    function updateGroup(uint256 groupId, IWorldID newTargetAddress)
221:        public
222:        virtual
223:        onlyProxy
224:        onlyInitialized
225:        onlyOwner
226:        returns (IWorldID oldTarget)
227:    {
``` 



```solidity
File:WorldIDRouterImplV1.sol 
239:    function disableGroup(uint256 groupId)
240:        public
241:        virtual
242:        onlyProxy
243:        onlyInitialized
244:        onlyOwner
245:        returns (IWorldID oldTarget)
246:    {
``` 



```solidity
File:WorldIDRouterImplV1.sol 
286:    function groupCount() public view virtual onlyProxy onlyInitialized returns (uint256 count) {
``` 



```solidity
File:WorldIDRouterImplV1.sol 
341:    function verifyProof(
342:        uint256 root,
343:        uint256 groupId,
344:        uint256 signalHash,
345:        uint256 nullifierHash,
346:        uint256 externalNullifierHash,
347:        uint256[8] calldata proof
348:    ) external virtual onlyProxy onlyInitialized {
``` 



```solidity
File:WorldIDIdentityManagerImplV2.sol 
62:    function initializeV2(VerifierLookupTable _batchUpdateVerifiers) public reinitializer(2) {
``` 



```solidity
File:WorldIDIdentityManagerImplV2.sol 
99:    function deleteIdentities(
100:        uint256[8] calldata deletionProof,
101:        uint32 batchSize,
102:        bytes calldata packedDeletionIndices,
103:        uint256 preRoot,
104:        uint256 postRoot
105:    ) public virtual onlyProxy onlyInitialized onlyIdentityOperator {
``` 



```solidity
File:WorldIDIdentityManagerImplV2.sol 
158:    function getDeleteIdentitiesVerifierLookupTableAddress()
159:        public
160:        view
161:        virtual
162:        onlyProxy
163:        onlyInitialized
164:        returns (address)
165:    {
``` 



```solidity
File:WorldIDIdentityManagerImplV2.sol 
175:    function setDeleteIdentitiesVerifierLookupTable(VerifierLookupTable newTable)
176:        public
177:        virtual
178:        onlyProxy
179:        onlyInitialized
180:        onlyOwner
181:    {
``` 



```solidity
File:WorldIDIdentityManagerImplV2.sol 
210:    function calculateIdentityDeletionInputHash(
211:        bytes calldata packedDeletionIndices,
212:        uint256 preRoot,
213:        uint256 postRoot,
214:        uint32 batchSize
215:    ) public view virtual onlyProxy onlyInitialized returns (bytes32 hash) {
``` 

 
 </details>

 <details open> 
 <summary> 
 <a name=[G-10]></a> [G-10] 
 <h3> Consider marking constants as private - Instances: 7 </h3> 
 </summary>
 
 
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
 

```solidity
File:VerifierLookupTable.sol 
18:    address internal constant nullAddress = address(0x0);
``` 



```solidity
File:VerifierLookupTable.sol 
21:    ITreeVerifier internal constant nullVerifier = ITreeVerifier(nullAddress);
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
54:    bytes32 internal constant insertionInputHash =
55:        0x7d7f77c56064e1f8577de14bba99eff85599ab0e76d0caeadd1ad61674b8a9c3;
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
56:    uint32 internal constant startIndex = 0;
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
57:    uint256 internal constant insertionPreRoot =
58:        0x18f43331537ee2af2e3d758d50f72106467c6eea50371dd528d57eb2b856d238;
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
59:    uint256 internal constant insertionPostRoot =
60:        0x5c1e52b41a571293b30efacd2afdb7173b20cfaf1f646c4ac9f96eb75848270;
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
71:    bytes32 internal constant deletionInputHash =
72:        0x227590f99431e20f2f95fdfb1b7dfb648c04242c950c31263ba165647c96501a;
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
73:    uint256 internal constant deletionPreRoot =
74:        0x18cb13df3e79b9f847a1494d0a2e6f3cc0041d9cae7e5ccb8cd1852ecdc4af58;
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
75:    uint256 internal constant deletionPostRoot =
76:        0x82fcf94594d7363636338e2c29242cc77e3d04f36c8ad64d294d2ab4d251708;
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
84:    uint256 internal constant SNARK_SCALAR_FIELD =
85:        21888242871839275222246405745257275088548364400416034343698204186575808495617;
``` 



```solidity
File:WorldIDRouterImplV1.sol 
48:    IWorldID internal constant NULL_ROUTER = IWorldID(address(0x0));
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
75:    uint256 internal constant SNARK_SCALAR_FIELD =
76:        21888242871839275222246405745257275088548364400416034343698204186575808495617;
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
77:    uint256 internal constant SNARK_SCALAR_FIELD_MIN_ONE =
78:        21888242871839275222246405745257275088548364400416034343698204186575808495616;
``` 



```solidity
File:DeletionTreeVerifier.sol 
28:    uint256 constant PRIME_Q =
29:        21888242871839275222246405745257275088696311157297823662689037894645226208583;
``` 



```solidity
File:DeletionTreeVerifier.sol 
186:    uint256 constant SNARK_SCALAR_FIELD =
187:        21888242871839275222246405745257275088548364400416034343698204186575808495617;
``` 



```solidity
File:DeletionTreeVerifier.sol 
188:    uint256 constant PRIME_Q =
189:        21888242871839275222246405745257275088696311157297823662689037894645226208583;
``` 



```solidity
File:InsertionTreeVerifier.sol 
28:    uint256 constant PRIME_Q =
29:        21888242871839275222246405745257275088696311157297823662689037894645226208583;
``` 



```solidity
File:InsertionTreeVerifier.sol 
150:    uint256 constant SNARK_SCALAR_FIELD =
151:        21888242871839275222246405745257275088548364400416034343698204186575808495617;
``` 



```solidity
File:InsertionTreeVerifier.sol 
152:    uint256 constant PRIME_Q =
153:        21888242871839275222246405745257275088696311157297823662689037894645226208583;
``` 



```solidity
File:SequencerVerifier.sol 
10:    uint256 internal constant SNARK_SCALAR_FIELD =
11:        21888242871839275222246405745257275088548364400416034343698204186575808495617;
``` 

 
 </details>

 <details open> 
 <summary> 
 <a name=[G-11]></a> [G-11] 
 <h3> Right shift or Left shift instead of dividing or multiplying by powers of two - Instances: 1 </h3> 
 </summary>
 
 
> Right shift or left shift when possible to save gas.        
         
 
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
 

```solidity
File:TypeConverter.sol 
9:            bytes1 b = bytes1(uint8(uint256(uint160(x)) / (2 ** (8 * (19 - i)))));
``` 



```solidity
File:TypeConverter.sol 
10:            bytes1 hi = bytes1(uint8(b) / 16);
``` 



```solidity
File:TypeConverter.sol 
11:            bytes1 lo = bytes1(uint8(b) - 16 * uint8(hi));
``` 



```solidity
File:TypeConverter.sol 
12:            s[2 * i + 2] = char(hi);
``` 



```solidity
File:TypeConverter.sol 
13:            s[2 * i + 3] = char(lo);
``` 

 
 </details>

 <details open> 
 <summary> 
 <a name=[G-12]></a> [G-12] 
 <h3> Use assembly to hash instead of Solidity - Instances: 2 </h3> 
 </summary>
 
 
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
 

```solidity
File:SimpleVerifier.sol 
48:        hash = keccak256(bytesToHash);
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
564:        hash = keccak256(bytesToHash);
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
605:        hash = keccak256(bytesToHash);
``` 

 
 </details>

 <details open> 
 <summary> 
 <a name=[G-13]></a> [G-13] 
 <h3> Use assembly for math (add, sub, mul, div) - Instances: 5 </h3> 
 </summary>
 
 
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
 

```solidity
File:InsertionTreeVerifier.sol 
50:            return G1Point(p.X, PRIME_Q - (p.Y % PRIME_Q));
``` 



```solidity
File:InsertionTreeVerifier.sol 
118:            uint256 j = i * 6;
``` 



```solidity
File:InsertionTreeVerifier.sol 
119:            input[j + 0] = p1[i].X;
``` 



```solidity
File:InsertionTreeVerifier.sol 
120:            input[j + 1] = p1[i].Y;
``` 



```solidity
File:InsertionTreeVerifier.sol 
121:            input[j + 2] = p2[i].X[0];
``` 



```solidity
File:InsertionTreeVerifier.sol 
122:            input[j + 3] = p2[i].X[1];
``` 



```solidity
File:InsertionTreeVerifier.sol 
123:            input[j + 4] = p2[i].Y[0];
``` 



```solidity
File:InsertionTreeVerifier.sol 
124:            input[j + 5] = p2[i].Y[1];
``` 



```solidity
File:InsertionTreeVerifier.sol 
250:            vk_x = Pairing.plus(vk_x, Pairing.scalar_mul(vk.IC[i + 1], input[i]));
``` 



```solidity
File:DeletionTreeVerifier.sol 
50:            return G1Point(p.X, PRIME_Q - (p.Y % PRIME_Q));
``` 



```solidity
File:DeletionTreeVerifier.sol 
153:            uint256 j = i * 6;
``` 



```solidity
File:DeletionTreeVerifier.sol 
154:            input[j + 0] = p1[i].X;
``` 



```solidity
File:DeletionTreeVerifier.sol 
155:            input[j + 1] = p1[i].Y;
``` 



```solidity
File:DeletionTreeVerifier.sol 
156:            input[j + 2] = p2[i].X[0];
``` 



```solidity
File:DeletionTreeVerifier.sol 
157:            input[j + 3] = p2[i].X[1];
``` 



```solidity
File:DeletionTreeVerifier.sol 
158:            input[j + 4] = p2[i].Y[0];
``` 



```solidity
File:DeletionTreeVerifier.sol 
159:            input[j + 5] = p2[i].Y[1];
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
289:        slotCounter = (slotCounter + 1) % (identityCommitments.length - 1);
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
289:        slotCounter = (slotCounter + 1) % (identityCommitments.length - 1);
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
376:                newIdents[i] = idents[i] - 1;
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
378:                newIdents[i] = idents[i] + 1;
``` 



```solidity
File:TypeConverter.sol 
9:            bytes1 b = bytes1(uint8(uint256(uint160(x)) / (2 ** (8 * (19 - i)))));
``` 



```solidity
File:TypeConverter.sol 
9:            bytes1 b = bytes1(uint8(uint256(uint160(x)) / (2 ** (8 * (19 - i)))));
``` 



```solidity
File:TypeConverter.sol 
9:            bytes1 b = bytes1(uint8(uint256(uint160(x)) / (2 ** (8 * (19 - i)))));
``` 



```solidity
File:TypeConverter.sol 
10:            bytes1 hi = bytes1(uint8(b) / 16);
``` 



```solidity
File:TypeConverter.sol 
11:            bytes1 lo = bytes1(uint8(b) - 16 * uint8(hi));
``` 



```solidity
File:TypeConverter.sol 
11:            bytes1 lo = bytes1(uint8(b) - 16 * uint8(hi));
``` 



```solidity
File:TypeConverter.sol 
12:            s[2 * i + 2] = char(hi);
``` 



```solidity
File:TypeConverter.sol 
12:            s[2 * i + 2] = char(hi);
``` 



```solidity
File:TypeConverter.sol 
13:            s[2 * i + 3] = char(lo);
``` 



```solidity
File:TypeConverter.sol 
13:            s[2 * i + 3] = char(lo);
``` 



```solidity
File:TypeConverter.sol 
19:        if (uint8(b) < 10) return bytes1(uint8(b) + 0x30);
``` 



```solidity
File:TypeConverter.sol 
20:        else return bytes1(uint8(b) + 0x57);
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
639:            bool isValid = !(block.timestamp - rootTimestamp > rootHistoryExpiry);
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
667:        if (block.timestamp - rootTimestamp > rootHistoryExpiry) {
``` 

 
 </details>

 <details open> 
 <summary> 
 <a name=[G-14]></a> [G-14] 
 <h3> Use assembly to write storage values - Instances: 8 </h3> 
 </summary>
 
 
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
 

```solidity
File:WorldIDRouterImplMock.sol 
16:        _someMoreData = data;
``` 



```solidity
File:VerifierLookupTableTest.sol 
48:        defaultVerifierAddress = address(defaultVerifier);
``` 



```solidity
File:VerifierLookupTableTest.sol 
51:        lookupTableAddress = address(lookupTable);
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
174:        managerImplV1Address = address(managerImplV1);
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
188:        identityManagerAddress = address(identityManager);
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
192:        managerImplAddress = address(managerImpl);
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
280:        managerImplAddress = address(managerImpl);
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
282:        identityManagerAddress = address(identityManager);
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
289:        slotCounter = (slotCounter + 1) % (identityCommitments.length - 1);
``` 



```solidity
File:CheckInitialized.sol 
20:        _initialized = true;
``` 



```solidity
File:WorldIDIdentityManagerImplMock.sol 
16:        _someMoreData = data;
``` 



```solidity
File:SimpleVerifier.sol 
14:        batchSize = _batchSize;
``` 



```solidity
File:WorldIDRouterTest.sol 
60:        routerImplAddress = address(routerImpl);
``` 



```solidity
File:WorldIDRouterTest.sol 
69:        routerAddress = address(router);
``` 



```solidity
File:WorldIDRouterTest.sol 
76:        routerImplAddress = address(routerImpl);
``` 



```solidity
File:WorldIDRouterTest.sol 
78:        routerAddress = address(router);
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
290:        treeDepth = _treeDepth;
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
291:        rootHistoryExpiry = 1 hours;
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
292:        _latestRoot = initialRoot;
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
296:        _identityOperator = owner();
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
384:            _latestRoot = postRoot;
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
520:            _latestRoot = postRoot;
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
805:        rootHistoryExpiry = newExpiryTime;
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
840:        _identityOperator = newIdentityOperator;
``` 

 
 </details>

 <details open> 
 <summary> 
 <a name=[G-15]></a> [G-15] 
 <h3> Use custom errors instead of string error messages - Instances: 2 </h3> 
 </summary>
 
 
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
 

```solidity
File:DeletionTreeVerifier.sol 
73:        require(success, "pairing-add-failed");
``` 



```solidity
File:DeletionTreeVerifier.sol 
91:        require(success, "pairing-add-failed");
``` 



```solidity
File:DeletionTreeVerifier.sol 
112:        require(success, "pairing-mul-failed");
``` 



```solidity
File:DeletionTreeVerifier.sol 
129:        require(success, "pairing-mul-failed");
``` 



```solidity
File:DeletionTreeVerifier.sol 
174:        require(success, "pairing-opcode-failed");
``` 



```solidity
File:DeletionTreeVerifier.sol 
279:        require(proof.A.X < PRIME_Q, "verifier-aX-gte-prime-q");
``` 



```solidity
File:DeletionTreeVerifier.sol 
280:        require(proof.A.Y < PRIME_Q, "verifier-aY-gte-prime-q");
``` 



```solidity
File:DeletionTreeVerifier.sol 
282:        require(proof.B.X[0] < PRIME_Q, "verifier-bX0-gte-prime-q");
``` 



```solidity
File:DeletionTreeVerifier.sol 
283:        require(proof.B.Y[0] < PRIME_Q, "verifier-bY0-gte-prime-q");
``` 



```solidity
File:DeletionTreeVerifier.sol 
285:        require(proof.B.X[1] < PRIME_Q, "verifier-bX1-gte-prime-q");
``` 



```solidity
File:DeletionTreeVerifier.sol 
286:        require(proof.B.Y[1] < PRIME_Q, "verifier-bY1-gte-prime-q");
``` 



```solidity
File:DeletionTreeVerifier.sol 
288:        require(proof.C.X < PRIME_Q, "verifier-cX-gte-prime-q");
``` 



```solidity
File:DeletionTreeVerifier.sol 
289:        require(proof.C.Y < PRIME_Q, "verifier-cY-gte-prime-q");
``` 



```solidity
File:DeletionTreeVerifier.sol 
293:            require(input[i] < SNARK_SCALAR_FIELD, "verifier-gte-snark-scalar-field");
``` 



```solidity
File:InsertionTreeVerifier.sol 
73:        require(success, "pairing-add-failed");
``` 



```solidity
File:InsertionTreeVerifier.sol 
94:        require(success, "pairing-mul-failed");
``` 



```solidity
File:InsertionTreeVerifier.sol 
139:        require(success, "pairing-opcode-failed");
``` 



```solidity
File:InsertionTreeVerifier.sol 
235:        require(proof.A.X < PRIME_Q, "verifier-aX-gte-prime-q");
``` 



```solidity
File:InsertionTreeVerifier.sol 
236:        require(proof.A.Y < PRIME_Q, "verifier-aY-gte-prime-q");
``` 



```solidity
File:InsertionTreeVerifier.sol 
238:        require(proof.B.X[0] < PRIME_Q, "verifier-bX0-gte-prime-q");
``` 



```solidity
File:InsertionTreeVerifier.sol 
239:        require(proof.B.Y[0] < PRIME_Q, "verifier-bY0-gte-prime-q");
``` 



```solidity
File:InsertionTreeVerifier.sol 
241:        require(proof.B.X[1] < PRIME_Q, "verifier-bX1-gte-prime-q");
``` 



```solidity
File:InsertionTreeVerifier.sol 
242:        require(proof.B.Y[1] < PRIME_Q, "verifier-bY1-gte-prime-q");
``` 



```solidity
File:InsertionTreeVerifier.sol 
244:        require(proof.C.X < PRIME_Q, "verifier-cX-gte-prime-q");
``` 



```solidity
File:InsertionTreeVerifier.sol 
245:        require(proof.C.Y < PRIME_Q, "verifier-cY-gte-prime-q");
``` 



```solidity
File:InsertionTreeVerifier.sol 
249:            require(input[i] < SNARK_SCALAR_FIELD, "verifier-gte-snark-scalar-field");
``` 

 
 </details> 
 </details>

 <details open> 
 <summary> 
 <h3>Quality Assurance - Instances: 7 </h3> 
 </summary> 
  


 <details open> 
 <summary> 
 <a name=[NC-0]></a> [NC-0] 
 <h3> Constructor should be listed before any other function - Instances: 1 </h3> 
 </summary>
 Description of the qa pattern goes here 

```solidity
File:WorldIDIdentityManagerImplV1.sol 
250:    constructor() {
``` 

 
 </details>

 <details open> 
 <summary> 
 <a name=[NC-1]></a> [NC-1] 
 <h3> Private variables should contain a leading underscore - Instances: 13 </h3> 
 </summary>
 Description of the qa pattern goes here 

```solidity
File:WorldIDTest.sol 
17:    address internal nullAddress = address(0x0);
``` 



```solidity
File:WorldIDTest.sol 
18:    address internal thisAddress = address(this);
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
96:    uint8 internal treeDepth;
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
71:    uint256 internal rootHistoryExpiry;
``` 



```solidity
File:WorldIDRouterTest.sol 
23:    address internal routerAddress;
``` 



```solidity
File:WorldIDRouterTest.sol 
24:    address internal routerImplAddress;
``` 



```solidity
File:VerifierLookupTableTest.sol 
18:    uint256 internal defaultBatchSize = 30;
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
38:    uint256 internal initialRoot = 0x0;
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
45:    address internal managerImplV1Address;
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
47:    uint256 internal slotCounter = 0;
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
41:    address internal identityManagerAddress;
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
39:    uint8 internal treeDepth = 16;
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
43:    address internal managerImplAddress;
``` 

 
 </details>

 <details open> 
 <summary> 
 <a name=[NC-2]></a> [NC-2] 
 <h3> Constructor should initialize all variables - Instances: 7 </h3> 
 </summary>
 Description of the qa pattern goes here 

```solidity
File:WorldIDIdentityManager.sol 
29:    constructor(address _logic, bytes memory _data) payable WorldIDProxy(_logic, _data) {
``` 



```solidity
File:WorldIDIdentityManager.sol 
29:    constructor(address _logic, bytes memory _data) payable WorldIDProxy(_logic, _data) {
``` 



```solidity
File:WorldIDProxy.sol 
23:    constructor(address _logic, bytes memory _data) payable ERC1967Proxy(_logic, _data) {
``` 



```solidity
File:WorldIDProxy.sol 
23:    constructor(address _logic, bytes memory _data) payable ERC1967Proxy(_logic, _data) {
``` 



```solidity
File:WorldIDRouter.sol 
27:    constructor(address _logic, bytes memory _data) payable WorldIDProxy(_logic, _data) {
``` 



```solidity
File:WorldIDRouter.sol 
27:    constructor(address _logic, bytes memory _data) payable WorldIDProxy(_logic, _data) {
``` 



```solidity
File:SimpleVerifier.sol 
13:    constructor(uint256 _batchSize) {
``` 

 
 </details>

 <details open> 
 <summary> 
 <a name=[NC-3]></a> [NC-3] 
 <h3> Consider importing specific identifiers instead of the whole file - Instances: 2 </h3> 
 </summary>
 This will minimize compiled code size and help with readability 

```solidity
File:WorldIDIdentityManagerImplV2.sol 
2:import "./WorldIDIdentityManagerImplV1.sol";
``` 



```solidity
File:WorldIDTest.sol 
6:import "forge-std/console.sol";
``` 

 
 </details>

 <details open> 
 <summary> 
 <a name=[NC-4]></a> [NC-4] 
 <h3> Constants & Immutables should be named with screaming snake case - Instances: 9 </h3> 
 </summary>
 Consider renaming to follow convention 

```solidity
File:WorldIDIdentityManagerTest.sol 
54:    bytes32 internal constant insertionInputHash =
55:        0x7d7f77c56064e1f8577de14bba99eff85599ab0e76d0caeadd1ad61674b8a9c3;
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
56:    uint32 internal constant startIndex = 0;
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
57:    uint256 internal constant insertionPreRoot =
58:        0x18f43331537ee2af2e3d758d50f72106467c6eea50371dd528d57eb2b856d238;
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
59:    uint256 internal constant insertionPostRoot =
60:        0x5c1e52b41a571293b30efacd2afdb7173b20cfaf1f646c4ac9f96eb75848270;
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
71:    bytes32 internal constant deletionInputHash =
72:        0x227590f99431e20f2f95fdfb1b7dfb648c04242c950c31263ba165647c96501a;
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
73:    uint256 internal constant deletionPreRoot =
74:        0x18cb13df3e79b9f847a1494d0a2e6f3cc0041d9cae7e5ccb8cd1852ecdc4af58;
``` 



```solidity
File:WorldIDIdentityManagerTest.sol 
75:    uint256 internal constant deletionPostRoot =
76:        0x82fcf94594d7363636338e2c29242cc77e3d04f36c8ad64d294d2ab4d251708;
``` 



```solidity
File:VerifierLookupTable.sol 
18:    address internal constant nullAddress = address(0x0);
``` 



```solidity
File:VerifierLookupTable.sol 
21:    ITreeVerifier internal constant nullVerifier = ITreeVerifier(nullAddress);
``` 

 
 </details>

 <details open> 
 <summary> 
 <a name=[NC-5]></a> [NC-5] 
 <h3> Remove any unused functions - Instances: 15 </h3> 
 </summary>
  

```solidity
File:WorldIDRouterImplV1.sol 
312:    function nextGroupId()
313:        internal
314:        view
315:        virtual
316:        onlyProxy
317:        onlyInitialized
318:        returns (uint256 groupId)
319:    {
``` 



```solidity
File:SemaphoreTreeDepthValidator.sol 
11:    function validate(uint8 treeDepth) internal pure returns (bool supportedDepth) {
``` 



```solidity
File:CheckInitialized.sol 
19:    function __setInitialized() internal onlyInitializing {
``` 



```solidity
File:InsertionTreeVerifier.sol 
57:    function plus(G1Point memory p1, G1Point memory p2) internal view returns (G1Point memory r) {
``` 



```solidity
File:InsertionTreeVerifier.sol 
45:    function negate(G1Point memory p) internal pure returns (G1Point memory) {
``` 



```solidity
File:InsertionTreeVerifier.sol 
81:    function scalar_mul(G1Point memory p, uint256 s) internal view returns (G1Point memory r) {
``` 



```solidity
File:InsertionTreeVerifier.sol 
102:    function pairing(
103:        G1Point memory a1,
104:        G2Point memory a2,
105:        G1Point memory b1,
106:        G2Point memory b2,
107:        G1Point memory c1,
108:        G2Point memory c2,
109:        G1Point memory d1,
110:        G2Point memory d2
111:    ) internal view returns (bool) {
``` 



```solidity
File:WorldIDImpl.sol 
49:    function __WorldIDImpl_init() internal virtual onlyInitializing {
``` 



```solidity
File:WorldIDImpl.sol 
70:    function _authorizeUpgrade(address newImplementation)
71:        internal
72:        virtual
73:        override
74:        onlyProxy
75:        onlyOwner
76:    {
``` 



```solidity
File:DeletionTreeVerifier.sol 
119:    function scalar_mul_raw(uint256[3] memory input, G1Point memory r) internal view {
``` 



```solidity
File:DeletionTreeVerifier.sol 
45:    function negate(G1Point memory p) internal pure returns (G1Point memory) {
``` 



```solidity
File:DeletionTreeVerifier.sol 
99:    function scalar_mul(G1Point memory p, uint256 s) internal view returns (G1Point memory r) {
``` 



```solidity
File:DeletionTreeVerifier.sol 
80:    function plus_raw(uint256[4] memory input, G1Point memory r) internal view {
``` 



```solidity
File:DeletionTreeVerifier.sol 
137:    function pairing(
138:        G1Point memory a1,
139:        G2Point memory a2,
140:        G1Point memory b1,
141:        G2Point memory b2,
142:        G1Point memory c1,
143:        G2Point memory c2,
144:        G1Point memory d1,
145:        G2Point memory d2
146:    ) internal view returns (bool) {
``` 



```solidity
File:DeletionTreeVerifier.sol 
57:    function plus(G1Point memory p1, G1Point memory p2) internal view returns (G1Point memory r) {
``` 

 
 </details>

 <details open> 
 <summary> 
 <a name=[NC-6]></a> [NC-6] 
 <h3> Remove any unused returns - Instances: 7 </h3> 
 </summary>
  

```solidity
File:InsertionTreeVerifier.sol 
218:    function verifyProof(
219:        uint256[2] memory a,
220:        uint256[2][2] memory b,
221:        uint256[2] memory c,
222:        uint256[1] memory input
223:    ) public view returns (bool r) {
``` 



```solidity
File:WorldIDTest.sol 
78:    function encodeStringRevert(string memory reason) public pure returns (bytes memory data) {
``` 



```solidity
File:DeletionTreeVerifier.sol 
267:    function verifyProof(
268:        uint256[2] memory a,
269:        uint256[2][2] memory b,
270:        uint256[2] memory c,
271:        uint256[1] calldata input
272:    ) public view returns (bool r) {
``` 



```solidity
File:WorldIDRouterImplV1.sol 
286:    function groupCount() public view virtual onlyProxy onlyInitialized returns (uint256 count) {
``` 



```solidity
File:WorldIDRouterImplV1.sol 
312:    function nextGroupId()
313:        internal
314:        view
315:        virtual
316:        onlyProxy
317:        onlyInitialized
318:        returns (uint256 groupId)
319:    {
``` 



```solidity
File:WorldIDIdentityManagerImplV1.sol 
144:    function NO_SUCH_ROOT() public pure returns (RootInfo memory rootInfo) {
``` 



```solidity
File:WorldIDIdentityManagerImplV2.sol 
210:    function calculateIdentityDeletionInputHash(
211:        bytes calldata packedDeletionIndices,
212:        uint256 preRoot,
213:        uint256 postRoot,
214:        uint32 batchSize
215:    ) public view virtual onlyProxy onlyInitialized returns (bytes32 hash) {
``` 

 
 </details> 
 </details>
