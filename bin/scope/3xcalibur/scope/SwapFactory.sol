// SPDX-License-Identifier: MIT
pragma solidity ^0.8.11;

import "./SwapPair.sol";

contract SwapFactory {

    bool public isPaused;
    address public pauser;
    address public pendingPauser;
    address public admin;

    mapping(address => mapping(address => mapping(bool => address))) public getPair;
    address[] public allPairs;
    mapping(address => bool) public isPair; // simplified check if its a pair, given that `stable` flag might not be available in peripherals
    mapping(bool => uint) public fee;
    address public immutable feeCollector;

    address internal _temp0;
    address internal _temp1;
    bool internal _temp2;
    uint internal _temp3;

    event PairCreated(address indexed token0, address indexed token1, bool stable, address pair, uint);
    
    modifier onlyAdmin() {
        require(msg.sender == admin, "Voter: only admin");
        _;
    }

    constructor(address _feeCollector) {
        require(
            _feeCollector != address(0),
            "SwapFactory: zero address provided in constructor"
        );
        pauser = msg.sender;
        fee[true] = 369; // 0.0369% for stable swaps (hundredth of a basis point / 369/1000000)
        fee[false] = 2700; // 0.27% for vaiable swaps (hundredth of a basis point / 2700/1000000)
        feeCollector = _feeCollector;
        admin = msg.sender;
    }

    function allPairsLength() external view returns (uint) {
        return allPairs.length;
    }

    function setPauser(address _pauser) external {
        require(msg.sender == pauser);
        pendingPauser = _pauser;
    }

    function acceptPauser() external {
        require(msg.sender == pendingPauser);
        pauser = pendingPauser;
    }

    function setPause(bool _state) external {
        require(msg.sender == pauser);
        isPaused = _state;
    }

    function setFeeTier(bool _stable, uint _fee) external {
        require(msg.sender == admin, "SwapFactory: only admin");
        fee[_stable] = _fee;
    }

    function setAdmin(address _admin) external {
        require(msg.sender == admin && _admin != address(0), "SwapFactory; wrong input parameters");
        admin = _admin;
    }

    function pairCodeHash() external pure returns (bytes32) {
        return keccak256(type(SwapPair).creationCode);
    }

    function getInitializable() external view returns (address, address, bool, uint) {
        return (_temp0, _temp1, _temp2, _temp3);
    }

    function createPair(address tokenA, address tokenB, bool stable) external returns (address pair) {
        require(tokenA != tokenB, 'IA'); // BaseV1: IDENTICAL_ADDRESSES
        (address token0, address token1) = tokenA < tokenB ? (tokenA, tokenB) : (tokenB, tokenA);
        require(token0 != address(0), 'ZA'); // BaseV1: ZERO_ADDRESS
        require(getPair[token0][token1][stable] == address(0), 'PE'); // BaseV1: PAIR_EXISTS - single check is sufficient
        bytes32 salt = keccak256(abi.encodePacked(token0, token1, stable)); // notice salt includes stable as well, 3 parameters
        (_temp0, _temp1, _temp2, _temp3) = (token0, token1, stable, fee[stable]);
        pair = address(new SwapPair{salt:salt}());
        getPair[token0][token1][stable] = pair;
        getPair[token1][token0][stable] = pair; // populate mapping in the reverse direction
        allPairs.push(pair);
        isPair[pair] = true;
        emit PairCreated(token0, token1, stable, pair, allPairs.length);
    }
}
