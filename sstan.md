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
 | [[G-0]](#[G-0]) | Avoid Reading From Storage in a for loop | 15 |
## Quality Assurance 

 | Classification | Title | Instances | 
 |:-------:|:---------|:-------:| 

## Vulnerabilities - Total: 0 




## Optimizations - Total: 15 

<a name=[G-0]></a>
### [G-0] Avoid Reading From Storage in a for loop - Instances: 15 

 
  
  - Savings: ~0 
 <details>  
 <summary>  
  </summary> 
  
 </details> 
 

 --- 

[File:GovernorTimelockCompound.sol#L98](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/governance/extensions/GovernorTimelockCompound.sol#L98) 
```solidity
97:        for (uint256 i = 0; i < targets.length; ++i) {
98:            require(
99:                !_timelock.queuedTransactions(keccak256(abi.encode(targets[i], values[i], "", calldatas[i], eta))),
100:                "GovernorTimelockCompound: identical proposal action already queued"
101:            );
102:            _timelock.queueTransaction(targets[i], values[i], "", calldatas[i], eta);
103:        }
104:
``` 



[File:GovernorTimelockCompound.sol#L98](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/governance/extensions/GovernorTimelockCompound.sol#L98) 
```solidity
97:        for (uint256 i = 0; i < targets.length; ++i) {
98:            require(
99:                !_timelock.queuedTransactions(keccak256(abi.encode(targets[i], values[i], "", calldatas[i], eta))),
100:                "GovernorTimelockCompound: identical proposal action already queued"
101:            );
102:            _timelock.queueTransaction(targets[i], values[i], "", calldatas[i], eta);
103:        }
104:
``` 



[File:GovernorTimelockCompound.sol#L124](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/governance/extensions/GovernorTimelockCompound.sol#L124) 
```solidity
123:        for (uint256 i = 0; i < targets.length; ++i) {
124:            _timelock.executeTransaction(targets[i], values[i], "", calldatas[i], eta);
125:        }
126:    }
``` 



[File:GovernorTimelockCompound.sol#L146](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/governance/extensions/GovernorTimelockCompound.sol#L146) 
```solidity
145:            for (uint256 i = 0; i < targets.length; ++i) {
146:                _timelock.cancelTransaction(targets[i], values[i], "", calldatas[i], eta);
147:            }
148:        }
``` 



[File:MockFlashLoanReceiver.sol#L61](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/mocks/flashloan/MockFlashLoanReceiver.sol#L61) 
```solidity
60:    for (uint256 i = 0; i < assets.length; i++) {
61:      //mint to this contract the specific amount
62:      MintableERC20 token = MintableERC20(assets[i]);
63:
64:      //check the contract has the specified balance
65:      require(
66:        amounts[i] <= IERC20(assets[i]).balanceOf(address(this)),
67:        'Invalid balance for the contract'
68:      );
69:
70:      uint256 amountToReturn =
71:        (_amountToApprove != 0) ? _amountToApprove : amounts[i].add(premiums[i]);
72:      //execution does not fail - mint tokens and return them to the _destination
73:
74:      token.mint(premiums[i]);
75:
76:      IERC20(assets[i]).approve(address(LENDING_POOL), amountToReturn);
77:    }
78:
``` 



[File:MockFlashLoanReceiver.sol#L61](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/mocks/flashloan/MockFlashLoanReceiver.sol#L61) 
```solidity
60:    for (uint256 i = 0; i < assets.length; i++) {
61:      //mint to this contract the specific amount
62:      MintableERC20 token = MintableERC20(assets[i]);
63:
64:      //check the contract has the specified balance
65:      require(
66:        amounts[i] <= IERC20(assets[i]).balanceOf(address(this)),
67:        'Invalid balance for the contract'
68:      );
69:
70:      uint256 amountToReturn =
71:        (_amountToApprove != 0) ? _amountToApprove : amounts[i].add(premiums[i]);
72:      //execution does not fail - mint tokens and return them to the _destination
73:
74:      token.mint(premiums[i]);
75:
76:      IERC20(assets[i]).approve(address(LENDING_POOL), amountToReturn);
77:    }
78:
``` 



[File:Governor.sol#L388](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/governance/Governor.sol#L388) 
```solidity
387:            for (uint256 i = 0; i < targets.length; ++i) {
388:                if (targets[i] == address(this)) {
389:                    _governanceCall.pushBack(keccak256(calldatas[i]));
390:                }
391:            }
392:        }
``` 



[File:CallReceiverMock.sol#L50](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/mocks/CallReceiverMock.sol#L50) 
```solidity
49:        for (uint256 i = 0; ; ++i) {
50:            _array.push(i);
51:        }
52:    }
``` 



[File:CallReceiverMock.sol#L50](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/mocks/CallReceiverMock.sol#L50) 
```solidity
49:        for (uint256 i = 0; ; ++i) {
50:            _array.push(i);
51:        }
52:    }
``` 



[File:ATokensAndRatesHelper.sol#L49](https://github.com/0xKitsune/sstan/blob/main/bin/scope/protocol-v2/contracts/deployments/ATokensAndRatesHelper.sol#L49) 
```solidity
48:    for (uint256 i = 0; i < inputParams.length; i++) {
49:      emit deployedContracts(
50:        address(new AToken()),
51:        address(
52:          new DefaultReserveInterestRateStrategy(
53:            LendingPoolAddressesProvider(addressesProvider),
54:            inputParams[i].rates[0],
55:            inputParams[i].rates[1],
56:            inputParams[i].rates[2],
57:            inputParams[i].rates[3],
58:            inputParams[i].rates[4],
59:            inputParams[i].rates[5]
60:          )
61:        )
62:      );
63:    }
64:  }
``` 



[File:Governor.sol#L388](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/governance/Governor.sol#L388) 
```solidity
387:            for (uint256 i = 0; i < targets.length; ++i) {
388:                if (targets[i] == address(this)) {
389:                    _governanceCall.pushBack(keccak256(calldatas[i]));
390:                }
391:            }
392:        }
``` 



[File:GovernorTimelockCompound.sol#L98](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/governance/extensions/GovernorTimelockCompound.sol#L98) 
```solidity
97:        for (uint256 i = 0; i < targets.length; ++i) {
98:            require(
99:                !_timelock.queuedTransactions(keccak256(abi.encode(targets[i], values[i], "", calldatas[i], eta))),
100:                "GovernorTimelockCompound: identical proposal action already queued"
101:            );
102:            _timelock.queueTransaction(targets[i], values[i], "", calldatas[i], eta);
103:        }
104:
``` 



[File:GovernorTimelockCompound.sol#L98](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/governance/extensions/GovernorTimelockCompound.sol#L98) 
```solidity
97:        for (uint256 i = 0; i < targets.length; ++i) {
98:            require(
99:                !_timelock.queuedTransactions(keccak256(abi.encode(targets[i], values[i], "", calldatas[i], eta))),
100:                "GovernorTimelockCompound: identical proposal action already queued"
101:            );
102:            _timelock.queueTransaction(targets[i], values[i], "", calldatas[i], eta);
103:        }
104:
``` 



[File:GovernorTimelockCompound.sol#L124](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/governance/extensions/GovernorTimelockCompound.sol#L124) 
```solidity
123:        for (uint256 i = 0; i < targets.length; ++i) {
124:            _timelock.executeTransaction(targets[i], values[i], "", calldatas[i], eta);
125:        }
126:    }
``` 



[File:GovernorTimelockCompound.sol#L146](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/contracts/governance/extensions/GovernorTimelockCompound.sol#L146) 
```solidity
145:            for (uint256 i = 0; i < targets.length; ++i) {
146:                _timelock.cancelTransaction(targets[i], values[i], "", calldatas[i], eta);
147:            }
148:        }
``` 



[File:WildcatMarketController.sol#L133](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/src/WildcatMarketController.sol#L133) 
```solidity
132:    for (uint256 i = 0; i < count; i++) {
133:      arr[i] = _authorizedLenders.at(start + i);
134:    }
135:  }
``` 



[File:WildcatMarketController.sol#L154](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/src/WildcatMarketController.sol#L154) 
```solidity
153:    for (uint256 i = 0; i < lenders.length; i++) {
154:      address lender = lenders[i];
155:      if (_authorizedLenders.add(lender)) {
156:        emit LenderAuthorized(lender);
157:      }
158:    }
159:  }
``` 



[File:WildcatMarketController.sol#L170](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/src/WildcatMarketController.sol#L170) 
```solidity
169:    for (uint256 i = 0; i < lenders.length; i++) {
170:      address lender = lenders[i];
171:      if (_authorizedLenders.remove(lender)) {
172:        emit LenderDeauthorized(lender);
173:      }
174:    }
175:  }
``` 



[File:WildcatMarketController.sol#L183](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/src/WildcatMarketController.sol#L183) 
```solidity
182:    for (uint256 i; i < markets.length; i++) {
183:      address market = markets[i];
184:      if (!_controlledMarkets.contains(market)) {
185:        revert NotControlledMarket();
186:      }
187:      WildcatMarket(market).updateAccountAuthorization(lender, _authorizedLenders.contains(lender));
188:    }
189:  }
``` 



[File:WildcatMarketController.sol#L183](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/src/WildcatMarketController.sol#L183) 
```solidity
182:    for (uint256 i; i < markets.length; i++) {
183:      address market = markets[i];
184:      if (!_controlledMarkets.contains(market)) {
185:        revert NotControlledMarket();
186:      }
187:      WildcatMarket(market).updateAccountAuthorization(lender, _authorizedLenders.contains(lender));
188:    }
189:  }
``` 



[File:WildcatMarketController.sol#L212](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/src/WildcatMarketController.sol#L212) 
```solidity
211:    for (uint256 i = 0; i < count; i++) {
212:      arr[i] = _controlledMarkets.at(start + i);
213:    }
214:  }
``` 



[File:WildcatArchController.sol#L93](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/src/WildcatArchController.sol#L93) 
```solidity
92:    for (uint256 i = 0; i < count; i++) {
93:      arr[i] = _borrowers.at(start + i);
94:    }
95:  }
``` 



[File:WildcatArchController.sol#L136](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/src/WildcatArchController.sol#L136) 
```solidity
135:    for (uint256 i = 0; i < count; i++) {
136:      arr[i] = _controllerFactories.at(start + i);
137:    }
138:  }
``` 



[File:WildcatArchController.sol#L179](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/src/WildcatArchController.sol#L179) 
```solidity
178:    for (uint256 i = 0; i < count; i++) {
179:      arr[i] = _controllers.at(start + i);
180:    }
181:  }
``` 



[File:WildcatArchController.sol#L222](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/src/WildcatArchController.sol#L222) 
```solidity
221:    for (uint256 i = 0; i < count; i++) {
222:      arr[i] = _markets.at(start + i);
223:    }
224:  }
``` 



[File:CallReceiverMock.sol#L50](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/contracts/mocks/CallReceiverMock.sol#L50) 
```solidity
49:        for (uint256 i = 0; ; ++i) {
50:            _array.push(i);
51:        }
52:    }
``` 



[File:WildcatMarketControllerFactory.sol#L146](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/src/WildcatMarketControllerFactory.sol#L146) 
```solidity
145:    for (uint256 i = 0; i < count; i++) {
146:      arr[i] = _deployedControllers.at(start + i);
147:    }
148:  }
``` 



[File:ExpectedBalances.sol#L305](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/test/helpers/ExpectedBalances.sol#L305) 
```solidity
304:    for (uint256 j; j < accountsLength; j++) {
305:      address account = accounts[j];
306:      uint256 expectedBalance = accountsMap.get(account);
307:      uint256 actualBalance = account.balance;
308:      if (expectedBalance != actualBalance) {
309:        revert(
310:          BalanceErrorMessages.nativeUnexpectedBalance(account, expectedBalance, actualBalance)
311:        );
312:      }
313:    }
314:  }
``` 



[File:ExpectedBalances.sol#L320](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/test/helpers/ExpectedBalances.sol#L320) 
```solidity
319:    for (uint256 i; i < accounts.length; i++) {
320:      address account = accounts[i];
321:      accountBalances[i] = NativeAccountDump(account, accountsMap.get(account));
322:    }
323:  }
``` 



[File:ExpectedBalances.sol#L379](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/test/helpers/ExpectedBalances.sol#L379) 
```solidity
378:    for (uint256 i; i < length; i++) {
379:      address token = tokens.at(i);
380:      EnumerableMap.AddressToUintMap storage accountsMap = tokenAccounts[token];
381:      address[] memory accounts = accountsMap.keys();
382:      uint256 accountsLength = accounts.length;
383:      for (uint256 j; j < accountsLength; j++) {
384:        address account = accounts[j];
385:        uint256 expectedBalance = accountsMap.get(account);
386:        uint256 actualBalance = IERC20(token).balanceOf(account);
387:        if (expectedBalance != actualBalance) {
388:          revert(
389:            BalanceErrorMessages.erc20UnexpectedBalance(
390:              token,
391:              account,
392:              expectedBalance,
393:              actualBalance
394:            )
395:          );
396:        }
397:      }
398:    }
399:  }
``` 



[File:ExpectedBalances.sol#L405](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/test/helpers/ExpectedBalances.sol#L405) 
```solidity
404:    for (uint256 i; i < length; i++) {
405:      address token = tokens.at(i);
406:      EnumerableMap.AddressToUintMap storage accountsMap = tokenAccounts[token];
407:      address[] memory accounts = accountsMap.keys();
408:      ERC20TokenDump memory tokenDump = ERC20TokenDump({
409:        token: token,
410:        accounts: accounts,
411:        balances: new uint256[](accounts.length)
412:      });
413:      tokenDumps[i] = tokenDump;
414:      for (uint256 j; j < accounts.length; j++) {
415:        address account = accounts[j];
416:        tokenDump.balances[j] = accountsMap.get(account);
417:      }
418:    }
419:  }
``` 



[File:GovernorTimelockCompound.sol#L98](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/governance/extensions/GovernorTimelockCompound.sol#L98) 
```solidity
97:        for (uint256 i = 0; i < targets.length; ++i) {
98:            require(
99:                !_timelock.queuedTransactions(keccak256(abi.encode(targets[i], values[i], "", calldatas[i], eta))),
100:                "GovernorTimelockCompound: identical proposal action already queued"
101:            );
102:            _timelock.queueTransaction(targets[i], values[i], "", calldatas[i], eta);
103:        }
104:
``` 



[File:GovernorTimelockCompound.sol#L98](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/governance/extensions/GovernorTimelockCompound.sol#L98) 
```solidity
97:        for (uint256 i = 0; i < targets.length; ++i) {
98:            require(
99:                !_timelock.queuedTransactions(keccak256(abi.encode(targets[i], values[i], "", calldatas[i], eta))),
100:                "GovernorTimelockCompound: identical proposal action already queued"
101:            );
102:            _timelock.queueTransaction(targets[i], values[i], "", calldatas[i], eta);
103:        }
104:
``` 



[File:GovernorTimelockCompound.sol#L124](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/governance/extensions/GovernorTimelockCompound.sol#L124) 
```solidity
123:        for (uint256 i = 0; i < targets.length; ++i) {
124:            _timelock.executeTransaction(targets[i], values[i], "", calldatas[i], eta);
125:        }
126:    }
``` 



[File:GovernorTimelockCompound.sol#L146](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/governance/extensions/GovernorTimelockCompound.sol#L146) 
```solidity
145:            for (uint256 i = 0; i < targets.length; ++i) {
146:                _timelock.cancelTransaction(targets[i], values[i], "", calldatas[i], eta);
147:            }
148:        }
``` 



[File:Governor.sol#L388](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/contracts/governance/Governor.sol#L388) 
```solidity
387:            for (uint256 i = 0; i < targets.length; ++i) {
388:                if (targets[i] == address(this)) {
389:                    _governanceCall.pushBack(keccak256(calldatas[i]));
390:                }
391:            }
392:        }
``` 



 --- 



## Quality Assurance - Total: 0 



