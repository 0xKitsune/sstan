
 <details open> 
 <summary> 
 <Strong>`array[index] += amount` is cheaper than `array[index] = array[index] + amount` (or related variants)</Strong> Instances(1) 
 </summary> 
 
 <details> 
 <summary> 
 When updating a value in an array with arithmetic, using `array[index] += amount` is cheaper than `array[index] = array[index] + amount`. This is because you avoid an additonal `mload` when the array is stored in memory, and an `sload` when the array is stored in storage. This can be applied for any arithmetic operation including `+=`, `-=`,`/=`,`*=`,`^=`,`&=`, `%=`, `<<=`,`>>=`, and `>>>=`. This optimization can be particularly significant if the pattern occurs during a loop. 
 Gas Savings: ~38  </summary> 
 <Strong> Assign Update Array Value - Gas Report </Strong> 
 
        ```js

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
                uint256[] memory data = new uint256[](10);
        
                uint256 newAmount = 100000;
                c0.assignMemory(data, newAmount);
                c1.assignAddMemory(data, newAmount);
        
                c2.assignStorage(newAmount);
                c3.assignAddStorage(newAmount);
            }
        }
        
        contract Contract0 {
            function assignMemory(uint256[] memory amounts, uint256 newAmount) public {
                amounts[0] = amounts[0] + newAmount;
            }
        }
        
        contract Contract1 {
            function assignAddMemory(uint256[] memory amounts, uint256 newAmount)
                public
            {
                amounts[0] += newAmount;
            }
        }
        
        contract Contract2 {
            uint256[] amounts;
        
            constructor() {
                amounts = new uint256[](10);
            }
        
            function assignStorage(uint256 newAmount) public {
                amounts[0] = amounts[0] + newAmount;
            }
        }
        
        contract Contract3 {
            uint256[] amounts;
        
            constructor() {
                amounts = new uint256[](10);
            }
        
            function assignAddStorage(uint256 newAmount) public {
                amounts[0] += newAmount;
                newAmount++;
            }
        }
        ```
        
        ### Gas Report
        
        ```js
        ╭───────────────────────────────────────────┬─────────────────┬──────┬────────┬──────┬─────────╮
        │ src/test/GasTest.t.sol:Contract0 contract ┆                 ┆      ┆        ┆      ┆         │
        ╞═══════════════════════════════════════════╪═════════════════╪══════╪════════╪══════╪═════════╡
        │ Deployment Cost                           ┆ Deployment Size ┆      ┆        ┆      ┆         │
        ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
        │ 179420                                    ┆ 928             ┆      ┆        ┆      ┆         │
        ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
        │ Function Name                             ┆ min             ┆ avg  ┆ median ┆ max  ┆ # calls │
        ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
        │ assignMemory                              ┆ 3611            ┆ 3611 ┆ 3611   ┆ 3611 ┆ 1       │
        ╰───────────────────────────────────────────┴─────────────────┴──────┴────────┴──────┴─────────╯
        ╭───────────────────────────────────────────┬─────────────────┬──────┬────────┬──────┬─────────╮
        │ src/test/GasTest.t.sol:Contract1 contract ┆                 ┆      ┆        ┆      ┆         │
        ╞═══════════════════════════════════════════╪═════════════════╪══════╪════════╪══════╪═════════╡
        │ Deployment Cost                           ┆ Deployment Size ┆      ┆        ┆      ┆         │
        ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
        │ 174820                                    ┆ 905             ┆      ┆        ┆      ┆         │
        ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
        │ Function Name                             ┆ min             ┆ avg  ┆ median ┆ max  ┆ # calls │
        ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
        │ assignAddMemory                           ┆ 3573            ┆ 3573 ┆ 3573   ┆ 3573 ┆ 1       │
        ╰───────────────────────────────────────────┴─────────────────┴──────┴────────┴──────┴─────────╯
        ╭───────────────────────────────────────────┬─────────────────┬───────┬────────┬───────┬─────────╮
        │ src/test/GasTest.t.sol:Contract2 contract ┆                 ┆       ┆        ┆       ┆         │
        ╞═══════════════════════════════════════════╪═════════════════╪═══════╪════════╪═══════╪═════════╡
        │ Deployment Cost                           ┆ Deployment Size ┆       ┆        ┆       ┆         │
        ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
        │ 144011                                    ┆ 779             ┆       ┆        ┆       ┆         │
        ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
        │ Function Name                             ┆ min             ┆ avg   ┆ median ┆ max   ┆ # calls │
        ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
        │ assignStorage                             ┆ 25052           ┆ 25052 ┆ 25052  ┆ 25052 ┆ 1       │
        ╰───────────────────────────────────────────┴─────────────────┴───────┴────────┴───────┴─────────╯
        ╭───────────────────────────────────────────┬─────────────────┬───────┬────────┬───────┬─────────╮
        │ src/test/GasTest.t.sol:Contract3 contract ┆                 ┆       ┆        ┆       ┆         │
        ╞═══════════════════════════════════════════╪═════════════════╪═══════╪════════╪═══════╪═════════╡
        │ Deployment Cost                           ┆ Deployment Size ┆       ┆        ┆       ┆         │
        ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
        │ 156623                                    ┆ 842             ┆       ┆        ┆       ┆         │
        ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
        │ Function Name                             ┆ min             ┆ avg   ┆ median ┆ max   ┆ # calls │
        ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
        │ assignAddStorage                          ┆ 25024           ┆ 25024 ┆ 25024  ┆ 25024 ┆ 1       │
        ╰───────────────────────────────────────────┴─────────────────┴───────┴────────┴───────┴─────────╯
        
        ```
    
 
 </details> 
 

 <span style="color: green;">File: </span> assign_update_array_value.sol 11-11 
 ```solidity 
 vals[0] = vals[0] + 1 
 ``` 
 </details>

 <details open> 
 <summary> 
 <Strong>Use assembly when getting a contract's balance of ETH</Strong> Instances(1) 
 </summary> 
 
 <details> 
 <summary> 
 
    You can use `selfbalance()` instead of `address(this).balance` when getting your contract's balance of ETH to save gas. Additionally, you can use `balance(address)` instead of `address.balance()` when getting an external contract's balance of ETH.
     
 Gas Savings: ~15  </summary> 
 <Strong> Address Balance Optimization - Gas Report </Strong> 
 
```js
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


```js
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
 

 <span style="color: green;">File: </span> address_balance.sol 5-5 
 ```solidity 
 address(this).balance 
 ```

 <span style="color: green;">File: </span> address_balance.sol 10-10 
 ```solidity 
 address(addr).balance 
 ``` 
 </details>

 <details open> 
 <summary> 
 <Strong>Use assembly to check for address(0)</Strong> Instances(1) 
 </summary> 
 
 <details> 
 <summary> 
  
 Gas Savings: ~6  </summary> 
 <Strong> Address Zero Optimization - Gas Report </Strong> 
 
    ```js


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
    
    ```js
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
 

 <span style="color: green;">File: </span> address_zero.sol 6-6 
 ```solidity 
 _addr == address(0) 
 ```

 <span style="color: green;">File: </span> address_zero.sol 10-10 
 ```solidity 
 _addr != address(0) 
 ```

 <span style="color: green;">File: </span> address_zero.sol 14-14 
 ```solidity 
 address(0) == _addr 
 ```

 <span style="color: green;">File: </span> address_zero.sol 18-18 
 ```solidity 
 address(0) != _addr 
 ``` 
 </details>

 <details open> 
 <summary> 
 <Strong>`array[index] += amount` is cheaper than `array[index] = array[index] + amount` (or related variants)</Strong> Instances(1) 
 </summary> 
 
 <details> 
 <summary> 
 When updating a value in an array with arithmetic, using `array[index] += amount` is cheaper than `array[index] = array[index] + amount`. This is because you avoid an additonal `mload` when the array is stored in memory, and an `sload` when the array is stored in storage. This can be applied for any arithmetic operation including `+=`, `-=`,`/=`,`*=`,`^=`,`&=`, `%=`, `<<=`,`>>=`, and `>>>=`. This optimization can be particularly significant if the pattern occurs during a loop. 
 Gas Savings: ~38  </summary> 
 <Strong> Assign Update Array Value - Gas Report </Strong> 
 
        ```js

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
                uint256[] memory data = new uint256[](10);
        
                uint256 newAmount = 100000;
                c0.assignMemory(data, newAmount);
                c1.assignAddMemory(data, newAmount);
        
                c2.assignStorage(newAmount);
                c3.assignAddStorage(newAmount);
            }
        }
        
        contract Contract0 {
            function assignMemory(uint256[] memory amounts, uint256 newAmount) public {
                amounts[0] = amounts[0] + newAmount;
            }
        }
        
        contract Contract1 {
            function assignAddMemory(uint256[] memory amounts, uint256 newAmount)
                public
            {
                amounts[0] += newAmount;
            }
        }
        
        contract Contract2 {
            uint256[] amounts;
        
            constructor() {
                amounts = new uint256[](10);
            }
        
            function assignStorage(uint256 newAmount) public {
                amounts[0] = amounts[0] + newAmount;
            }
        }
        
        contract Contract3 {
            uint256[] amounts;
        
            constructor() {
                amounts = new uint256[](10);
            }
        
            function assignAddStorage(uint256 newAmount) public {
                amounts[0] += newAmount;
                newAmount++;
            }
        }
        ```
        
        ### Gas Report
        
        ```js
        ╭───────────────────────────────────────────┬─────────────────┬──────┬────────┬──────┬─────────╮
        │ src/test/GasTest.t.sol:Contract0 contract ┆                 ┆      ┆        ┆      ┆         │
        ╞═══════════════════════════════════════════╪═════════════════╪══════╪════════╪══════╪═════════╡
        │ Deployment Cost                           ┆ Deployment Size ┆      ┆        ┆      ┆         │
        ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
        │ 179420                                    ┆ 928             ┆      ┆        ┆      ┆         │
        ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
        │ Function Name                             ┆ min             ┆ avg  ┆ median ┆ max  ┆ # calls │
        ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
        │ assignMemory                              ┆ 3611            ┆ 3611 ┆ 3611   ┆ 3611 ┆ 1       │
        ╰───────────────────────────────────────────┴─────────────────┴──────┴────────┴──────┴─────────╯
        ╭───────────────────────────────────────────┬─────────────────┬──────┬────────┬──────┬─────────╮
        │ src/test/GasTest.t.sol:Contract1 contract ┆                 ┆      ┆        ┆      ┆         │
        ╞═══════════════════════════════════════════╪═════════════════╪══════╪════════╪══════╪═════════╡
        │ Deployment Cost                           ┆ Deployment Size ┆      ┆        ┆      ┆         │
        ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
        │ 174820                                    ┆ 905             ┆      ┆        ┆      ┆         │
        ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
        │ Function Name                             ┆ min             ┆ avg  ┆ median ┆ max  ┆ # calls │
        ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
        │ assignAddMemory                           ┆ 3573            ┆ 3573 ┆ 3573   ┆ 3573 ┆ 1       │
        ╰───────────────────────────────────────────┴─────────────────┴──────┴────────┴──────┴─────────╯
        ╭───────────────────────────────────────────┬─────────────────┬───────┬────────┬───────┬─────────╮
        │ src/test/GasTest.t.sol:Contract2 contract ┆                 ┆       ┆        ┆       ┆         │
        ╞═══════════════════════════════════════════╪═════════════════╪═══════╪════════╪═══════╪═════════╡
        │ Deployment Cost                           ┆ Deployment Size ┆       ┆        ┆       ┆         │
        ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
        │ 144011                                    ┆ 779             ┆       ┆        ┆       ┆         │
        ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
        │ Function Name                             ┆ min             ┆ avg   ┆ median ┆ max   ┆ # calls │
        ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
        │ assignStorage                             ┆ 25052           ┆ 25052 ┆ 25052  ┆ 25052 ┆ 1       │
        ╰───────────────────────────────────────────┴─────────────────┴───────┴────────┴───────┴─────────╯
        ╭───────────────────────────────────────────┬─────────────────┬───────┬────────┬───────┬─────────╮
        │ src/test/GasTest.t.sol:Contract3 contract ┆                 ┆       ┆        ┆       ┆         │
        ╞═══════════════════════════════════════════╪═════════════════╪═══════╪════════╪═══════╪═════════╡
        │ Deployment Cost                           ┆ Deployment Size ┆       ┆        ┆       ┆         │
        ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
        │ 156623                                    ┆ 842             ┆       ┆        ┆       ┆         │
        ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
        │ Function Name                             ┆ min             ┆ avg   ┆ median ┆ max   ┆ # calls │
        ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌┤
        │ assignAddStorage                          ┆ 25024           ┆ 25024 ┆ 25024  ┆ 25024 ┆ 1       │
        ╰───────────────────────────────────────────┴─────────────────┴───────┴────────┴───────┴─────────╯
        
        ```
    
 
 </details> 
 

 <span style="color: green;">File: </span> bool_equals_bool.sol 7-7 
 ```solidity 
 check == true 
 ```

 <span style="color: green;">File: </span> bool_equals_bool.sol 14-14 
 ```solidity 
 check == false 
 ```

 <span style="color: green;">File: </span> bool_equals_bool.sol 20-20 
 ```solidity 
 false == check 
 ```

 <span style="color: green;">File: </span> bool_equals_bool.sol 26-26 
 ```solidity 
 true == check 
 ```

 <span style="color: green;">File: </span> bool_equals_bool.sol 32-32 
 ```solidity 
 check != true 
 ```

 <span style="color: green;">File: </span> bool_equals_bool.sol 39-39 
 ```solidity 
 check != false 
 ```

 <span style="color: green;">File: </span> bool_equals_bool.sol 45-45 
 ```solidity 
 false != check 
 ```

 <span style="color: green;">File: </span> bool_equals_bool.sol 51-51 
 ```solidity 
 true != check 
 ``` 
 </details>
