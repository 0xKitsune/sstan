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
 | [[NC-0]](#[NC-0]) | Large contracts with many external functions should inherit an interface | 6 |

## Vulnerabilities - Total: 0 




## Optimizations - Total: 0 




## Quality Assurance - Total: 6 

<a name=[NC-0]></a>
### [NC-0] Large contracts with many external functions should inherit an interface - Instances: 6 

 
> Consider inheriting the interface to ensure the interface matches the contract spec
 

 --- 

[File:DoubleEndedQueueHarness.sol#L7](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/certora/harnesses/DoubleEndedQueueHarness.sol#L7) 
```solidity
6:contract DoubleEndedQueueHarness {
``` 



[File:WildcatArchController.sol#L8](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/src/WildcatArchController.sol#L8) 
```solidity
7:contract WildcatArchController is Ownable {
``` 



[File:DoubleEndedQueueHarness.sol#L7](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/certora/harnesses/DoubleEndedQueueHarness.sol#L7) 
```solidity
6:contract DoubleEndedQueueHarness {
``` 



[File:SwapPair.sol#L13](https://github.com/0xKitsune/sstan/blob/main/bin/scope/3xcalibur/scope/SwapPair.sol#L13) 
```solidity
12:contract SwapPair {
``` 



[File:DoubleEndedQueueHarness.sol#L7](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/certora/harnesses/DoubleEndedQueueHarness.sol#L7) 
```solidity
6:contract DoubleEndedQueueHarness {
``` 



[File:Voter.sol#L15](https://github.com/0xKitsune/sstan/blob/main/bin/scope/3xcalibur/scope/Voter.sol#L15) 
```solidity
14:contract Voter {
``` 



 --- 


