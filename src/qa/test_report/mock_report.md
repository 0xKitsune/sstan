
<details open>
                                <summary><a name=N-3>[N-3]</a> Constructor should initialize all variables Instances(1)</summary>
                                <p>Description of the qa pattern goes here</p>
                                File: test.sol 5-5: <br> ```solidity <br>constructor(address _owner) {owner = _owner} <br>``` <br></details>

<details open>
                                <summary><a name=N-7>[N-7]</a> Consider using scientific notation for large multiples of 10 Instances(1)</summary>
                                <p>For example 100000 can be written as 1e5</p>
                                File: test.sol 8-8: <br> ```solidity <br>uint256 x = 10000000; <br>``` <br></details>
<details open>
                                <summary><a name=N-2>[N-2]</a> Private variables should contain a leading underscore Instances(3)</summary>
                                <p>Description of the qa pattern goes here</p>
                                File: test.sol 5-5: <br> ```solidity <br>address public _addr2; <br>``` <br>File: test.sol 6-7: <br> ```solidity <br>address private addr4; <br>``` <br>File: test.sol 9-9: <br> ```solidity <br>address internal addr6; <br>``` <br></details>
<details open>
                                <summary><a name=N-4>[N-4]</a> Consider importing specific identifiers instead of the whole file Instances(1)</summary>
                                <p>This will minimize compiled code size and help with readability</p>
                                File: test.sol 2-4: <br> ```solidity <br>import "filename.sol"; <br>``` <br></details>



<details open>
                                <summary><a name=N-5>[N-5]</a> Interface names should start with an I Instances(1)</summary>
                                <p>Consider renaming for consistency</p>
                                File: test.sol 0-0: <br> ```solidity <br>interface Contract0 {} <br>``` <br></details>
<details open>
                                <summary><a name=N-1>[N-1]</a> Constructor should be listed before any other function Instances(1)</summary>
                                <p>Description of the qa pattern goes here</p>
                                File: test.sol 7-8: <br> ```solidity <br>constructor() {owner = address(1)} <br>``` <br></details>


<details open>
                                <summary><a name=N-9>[N-9]</a> Storage variables should be named with camel case Instances(2)</summary>
                                <p>Consider renaming to follow convention</p>
                                File: test.sol 3-3: <br> ```solidity <br>address IS_NOT_FINE; <br>``` <br>File: test.sol 5-6: <br> ```solidity <br>address ALSO_IS_BAD; <br>``` <br></details>

<details open>
                                <summary><a name=N-6>[N-6]</a> Constants & Immutables should be named with screaming snake case Instances(2)</summary>
                                <p>Consider renaming to follow convention</p>
                                File: test.sol 5-5: <br> ```solidity <br>address constant is_bad = address(1); <br>``` <br>File: test.sol 5-6: <br> ```solidity <br>address immutable Is_Bad; <br>``` <br></details>
