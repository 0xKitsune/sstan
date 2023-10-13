// SPDX-License-Identifier: Unlicense
pragma solidity 0.8.11;
pragma experimental ABIEncoderV2;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract Multiswap {

    address public immutable router;

    constructor(address _router) {
        require(
            _router != address(0),
            "Multiswap: zero address provided in constructor"
        );
        router = _router;
    }

    error ErrorSwapping(uint index);

    event Multiswapped(address indexed _token, uint _amountIn, uint[] amountsOut);

    /// @notice Swaps an asset to up to 5 other assets according to predetermined weights
    /// @param _token           The asset to swap (address(0) if ETH)
    /// @param _amount          The amount to swap (0 if _token is ETH)
    /// @param _weights         The respective weights to be attributed to each assets (in basis points, 10000 = 100%)
    /// @param _swapData        An array of data to be passed to each swap
    /// @return amountsOut An array with the respective amounts of assets received
    function multiswap(
        address _token,
        uint _amount,
        bytes[] memory _swapData,
        uint[] calldata _weights
    ) external payable returns (uint[] memory) {
        uint length = _swapData.length;
        // Checks
        require(length > 1 && length <= 5 && _weights.length == length, "length");
        require(_assertWeights(_weights), "wrong weights");
        // Effects
        uint[] memory amountsOut = new uint[](length);
        bool eth;
        if (eth = (_token == address(0))) {
            // Caller wants to multiswap some ETH
            require(msg.value > 0, "no ETH sent");
            for (uint i = 0; i < length; ++i) { 
                uint amount_ = msg.value * _weights[i] / 10000;
                (bool success, bytes memory data) = router.call{value: amount_}(_swapData[i]);
                if (!success) revert ErrorSwapping(i);
                uint[] memory out = abi.decode(data, (uint[]));
                amountsOut[i] = out[out.length - 1];
            }
        } else {
            // Caller wants to multiswap a token
            require(_amount > 0, "no tokens sent");
            (bool transferFromSuccess) = IERC20(_token).transferFrom(msg.sender, address(this), _amount);
            require(transferFromSuccess, "transferFrom failed");
            IERC20(_token).approve(router, _amount);
            for (uint i = 0; i < length; ++i) {
                (bool callSuccess, bytes memory data) = router.call(_swapData[i]);
                if (!callSuccess) revert ErrorSwapping(i);
                uint[] memory out = abi.decode(data, (uint[]));
                amountsOut[i] = out[out.length - 1];
            }
        }
        emit Multiswapped(_token, eth ? msg.value : _amount, amountsOut);

        return amountsOut;
    }

    // ***** INTERNAL *****
    function _assertWeights(uint[] calldata _weights) internal pure returns (bool) {
        uint totalWeight = 10000; // Basis points
        uint weightSum = 0;
        uint length = _weights.length;
        for (uint i = 0; i < length; ++i) {
            weightSum += _weights[i];
        }
        return weightSum == totalWeight;
    }
}
