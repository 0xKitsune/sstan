// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.11;

import "./libraries/Math.sol";
import "./interfaces/IUnderlying.sol";
import "./interfaces/IVotingEscrow.sol";
import {IVoter} from "./interfaces/IVoter.sol";
import "./interfaces/IVotingDist.sol";

// codifies the minting rules as per ve(3,3), abstracted from the token to support any token that allows minting

contract Minter {

    uint internal constant week = 86400 * 7; // allows minting once per week (reset every Thursday 00:00 UTC)
    uint internal decay = 9900; // weekly decay of 1%
    uint internal constant target_base = 10000; // 2% per week target emission
    uint internal constant tail_emission = 3; // 0.03% per week tail emission
    uint internal constant tail_base = 1000; // 0.2% per week target emission
    underlying public immutable _token;
    IVoter public immutable _voter;
    IVotingEscrow public immutable _ve;
    IVotingDist public immutable _ve_dist;
    uint public weekly;
    uint public boost;
    uint public active_period;
    uint public last_epoch;
    uint internal constant lock = 86400 * 7 * 52 * 4;

    address internal initializer;
    address internal admin;

    event Mint(address indexed sender, uint weekly, uint circulating_supply, uint circulating_emission);

    modifier onlyAdmin() {
        require(msg.sender == admin);
        _;
    }

    constructor(
        address __voter, // the voting & distribution system
        address  __ve, // the ve(3,3) system that will be locked into
        address __ve_dist, // the distribution system that ensures users aren't diluted
        address _admin,
        uint _weekly
    ) {
        initializer = msg.sender;
        require(
            __voter != address(0) &&
            __ve != address(0) &&
            __ve_dist != address(0) &&
            _admin != address(0),
            "Minter: zero address provided in constructor"
        );
        admin = _admin;
        _token = underlying(IVotingEscrow(__ve).token());
        _voter = IVoter(__voter);
        _ve = IVotingEscrow(__ve);
        _ve_dist = IVotingDist(__ve_dist);
        active_period = (block.timestamp + (2*week)) / week * week;
        last_epoch = block.timestamp;
        boost = 0;
        // 1.7% of the LM target (36.9% of total supply)
        weekly = _weekly;
    }

    function initialize(
        // address[] memory claimants,
        // uint[] memory amounts,
        // uint max // sum amounts / max = % ownership of top protocols, so if initial 20m is distributed, and target is 25% protocol ownership, then max - 4 x 20m = 80m
    ) external {
        require(initializer == msg.sender);
        // _token.mint(address(this), max);
        // _token.approve(address(_ve), type(uint).max);
        // for (uint i = 0; i < claimants.length; i++) {
        //     _ve.create_lock_for(amounts[i], lock, claimants[i]);
        // }
        initializer = address(0);
        active_period = block.timestamp / week * week; // 
    }

    /// @param _decay: new value of emissions
    /// @param _boost: new value of boost
    function setEmissions(uint _decay, uint _boost) public onlyAdmin {
        require(block.timestamp >= last_epoch + 26 weeks, "must wait next period");
        decay = _decay;
        boost = _boost;
        last_epoch = block.timestamp + 26 weeks;
    }

    // calculate circulating supply as total token supply - locked supply
    function circulating_supply() public view returns (uint) {
        return _token.totalSupply() - _ve.totalSupply();
    }

    // emission calculation is 1% of available supply to mint adjusted by circulating / total supply
    function _calculate_emission() private view returns (uint) {
        return weekly * decay * circulating_supply() / target_base / _token.totalSupply();
    }

    // emission calculation is 1% of available supply to mint adjusted by circulating / total supply
    function calculate_emission() public view returns (uint) {
        uint _emission = _calculate_emission();
        return _emission + boost;
    }

    // weekly emission takes the max of calculated (aka target) emission versus circulating tail end emission
    function weekly_emission() public view returns (uint) {
        return Math.max(calculate_emission(), circulating_emission());
    }

    // calculates tail end (infinity) decay as 0.3% of total supply
    function circulating_emission() public view returns (uint) {
        return circulating_supply() * tail_emission / tail_base;
    }

    // calculate inflation and adjust ve balances accordingly
    function calculate_growth(uint _minted) public view returns (uint) {
        return _ve.totalSupply() * _minted / _token.totalSupply();
    }

    // update period can only be called once per cycle (1 week)
    function update_period() external returns (uint) {
        uint _period = active_period;
        if (block.timestamp >= _period + week && initializer == address(0)) { // only trigger if new week
            _period = block.timestamp / week * week;
            active_period = _period;
            weekly = weekly_emission();

            uint _growth = calculate_growth(weekly);
            uint _required = _growth + weekly;
            uint _balanceOf = _token.balanceOf(address(this));
            if (_balanceOf < _required) {
                _token.mint(address(this), _required-_balanceOf);
            }

            require(_token.transfer(address(_ve_dist), _growth), "growth transfer failed");
            
            _ve_dist.checkpoint_token(); // checkpoint token balance that was just minted in ve_dist
            _ve_dist.checkpoint_total_supply(); // checkpoint supply

            _token.approve(address(_voter), weekly);
            _voter.notifyRewardAmount(weekly);

            emit Mint(msg.sender, weekly, circulating_supply(), circulating_emission());
        }
        return _period;
    }
}
