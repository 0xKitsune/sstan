// SPDX-License-Identifier: MIT
pragma solidity 0.8.11;

import '@openzeppelin/contracts/token/ERC20/IERC20.sol';
import "./interfaces/IVotingEscrow.sol";
import "../core/interfaces/ISwapPair.sol";
import "./interfaces/IGaugeFactory.sol";
import "./interfaces/IBribeFactory.sol";
import "./interfaces/IGauge.sol";
import "./interfaces/IBribe.sol";
import "../core/interfaces/ISwapFactory.sol";
import "./interfaces/IMinter.sol";
import {IVoter} from "./interfaces/IVoter.sol";

contract Voter {

    address public immutable _ve;
    address public immutable factory;
    address public convenience;
    address internal immutable base;
    address public gaugeFactory;
    address public immutable bribeFactory;
    uint internal constant DURATION = 7 days; // rewards are released over 7 days
    address public minter;

    uint public totalWeight; // total voting weight

    address[] public allGauges; // all gauges viable for incentives
    mapping(address => address) public gauges; // pair => maturity => gauge
    mapping(address => address) public poolForGauge; // gauge => pool
    mapping(address => address) public bribes; // gauge => bribe
    mapping(address => uint256) public weights; // gauge => weight
    mapping(uint => mapping(address => uint256)) public votes; // nft => gauge => votes
    mapping(uint => address[]) public gaugeVote; // nft => gauge
    mapping(uint => uint) public usedWeights;  // nft => total voting weight of user
    mapping(uint => uint) public lastVote; // nft id => timestamp of last vote 
    mapping(address => bool) public isGauge;
    mapping(address => bool) public isLive; // gauge => status (live or not)

    mapping(address => bool) public isWhitelisted;
    mapping(address => mapping(address => bool)) public isReward;
    mapping(address => mapping(address => bool)) public isBribe;


    mapping(address => uint) public claimable;
    uint internal index;
    mapping(address => uint) internal supplyIndex;
    address public admin;

    event GaugeCreated(address indexed gauge, address creator, address indexed bribe, address indexed pair);
    event Voted(address indexed voter, uint tokenId, uint256 weight);
    event Abstained(uint tokenId, uint256 weight);
    event Deposit(address indexed lp, address indexed gauge, uint tokenId, uint amount);
    event Withdraw(address indexed lp, address indexed gauge, uint tokenId, uint amount);
    event NotifyReward(address indexed sender, address indexed reward, uint amount);
    event DistributeReward(address indexed sender, address indexed gauge, uint amount);
    event Attach(address indexed owner, address indexed gauge, uint tokenId);
    event Detach(address indexed owner, address indexed gauge, uint tokenId);
    event Whitelisted(address indexed whitelister, address indexed token);
    event GaugeKilled(address indexed gauge);
    event GaugeRevived(address indexed gauge);

    modifier onlyAdmin() {
        require(msg.sender == admin, "Voter: only admin");
        _;
    }

    constructor(address __ve, address _factory, address  _gauges, address _bribes) {
        require(
            __ve != address(0) &&
            _factory != address(0) &&
            _gauges != address(0) &&
            _bribes != address(0),
            "Voter: zero address provided in constructor"
        );
        _ve = __ve;
        factory = _factory;
        base = IVotingEscrow(__ve).token();
        gaugeFactory = _gauges;
        bribeFactory = _bribes;
        minter = msg.sender;
        admin = msg.sender;
    }

    // simple re-entrancy check
    uint internal _unlocked = 1;
    modifier lock() {
        require(_unlocked == 1);
        _unlocked = 2;
        _;
        _unlocked = 1;
    }

    //TODO: decide which tokens to whitelist after deployment
    // @param _tokens list of ERC20 tokens to whitelist
    // @param _minter address of the minter contract
    function initialize(/*address[] memory _tokens*/address _minter) external {
        require(msg.sender == minter);
        // for (uint i = 0; i < _tokens.length; i++) {
        //     _whitelist(_tokens[i]);
        // }
        minter = _minter;
    }

    // TODO: change denominator
    //@return the fee that needs to be paid or held in a lock to be allowed to whitelist tokens
    function listing_fee() public view returns (uint) {
        return (IERC20(base).totalSupply() - IERC20(_ve).totalSupply()) / 10000;
    }

    function setAdmin(address _admin) external onlyAdmin {
        require(_admin != address(0), "zero address");
        admin = _admin;
    }

    function setReward(address _gauge, address _token, bool _status) external onlyAdmin {
        isReward[_gauge][_token] = _status;
    }

    function setBribe(address _bribe, address _token, bool _status) external onlyAdmin {
        isBribe[_bribe][_token] = _status;
    }

    function killGauge(address _gauge) external onlyAdmin {
        require(isLive[_gauge], "gauge is not live");
        isLive[_gauge] = false;
        claimable[_gauge] = 0;
        emit GaugeKilled(_gauge);
    }

    function reviveGauge(address _gauge) external onlyAdmin {
        require(!isLive[_gauge], "gauge is live");
        isLive[_gauge] = true;
        emit GaugeRevived(_gauge);
    }

    // @param _tokenId the ID of the veNFT to reset votes for
    function reset(uint _tokenId) external { // OR msg.sender == votingescrow when withdrawing
        require(_activePeriod() > lastVote[_tokenId], "voted recently");
        require(IVotingEscrow(_ve).isApprovedOrOwner(msg.sender, _tokenId));
        lastVote[_tokenId] = block.timestamp;
        _reset(_tokenId);
        IVotingEscrow(_ve).abstain(_tokenId);
    }

    function _reset(uint _tokenId) internal {
        address[] storage _gaugeVote = gaugeVote[_tokenId];
        uint _gaugeVoteCnt = _gaugeVote.length;
        uint256 _totalWeight = 0;
        for (uint i = 0; i < _gaugeVoteCnt; i++) {
            address _gauge = _gaugeVote[i];
            uint256 _votes = votes[_tokenId][_gauge];
                if (_votes != 0) {
                        _updateFor(_gauge);
                        weights[_gauge] -= _votes;
                        votes[_tokenId][_gauge] -= _votes;
                        if (_votes > 0) {
                            IBribe(bribes[_gauge])._withdraw(uint256(_votes), _tokenId);
                            _totalWeight += _votes;
                        } else {
                            _totalWeight -= _votes;
                        }
                    emit Abstained(_tokenId, _votes);
                }
            }
        totalWeight -= uint256(_totalWeight);
        usedWeights[_tokenId] = 0;
        delete gaugeVote[_tokenId];
    }

    // @param _tokenId the ID of the NFT to poke
    // @notice To be called on voters abusing their voting power
    // @notice _weights are the same as the last ID's vote !
    function poke(uint _tokenId) external {
        address[] memory _gaugeVote = gaugeVote[_tokenId];
        uint _gaugeCnt = _gaugeVote.length;
        uint256[] memory _weights = new uint256[](_gaugeCnt);

        for (uint i = 0; i < _gaugeCnt; i++) {
            _weights[i] = votes[_tokenId][_gaugeVote[i]];
        }

        _vote(_tokenId, _gaugeVote, _weights);
    }

    function _vote(uint _tokenId, address[] memory _gaugeVote, uint256[] memory _weights) internal {
        _reset(_tokenId);
        uint _gaugeCnt = _gaugeVote.length;
        uint256 _weight = IVotingEscrow(_ve).balanceOfNFT(_tokenId);
        uint256 _totalVoteWeight = 0;
        uint256 _totalWeight = 0;
        uint256 _usedWeight = 0;

        for (uint i = 0; i < _gaugeCnt; i++) {
            _totalVoteWeight += _weights[i];
        }

        for (uint i = 0; i < _gaugeCnt; i++) {
            address _gauge = _gaugeVote[i];
            if (isGauge[_gauge]) {
                uint256 _gaugeWeight = _weights[i] * _weight / _totalVoteWeight;
                require(votes[_tokenId][_gauge] == 0);
                require(_gaugeWeight != 0);
                _updateFor(_gauge);

                gaugeVote[_tokenId].push(_gauge);

                weights[_gauge] += _gaugeWeight;
                votes[_tokenId][_gauge] += _gaugeWeight;
                IBribe(bribes[_gauge])._deposit(_gaugeWeight, _tokenId);
                _usedWeight += _gaugeWeight;
                _totalWeight += _gaugeWeight;
                emit Voted(msg.sender, _tokenId, _gaugeWeight);
            }
        }
        if (_usedWeight > 0) IVotingEscrow(_ve).voting(_tokenId);
        totalWeight += _totalWeight;
        usedWeights[_tokenId] = _usedWeight;
    }

    // @param _tokenId The id of the veNFT to vote with
    // @param _gaugeVote The list of gauges to vote for
    // @param _weights The list of weights to vote for each gauge
    // @notice the sum of weights is the total weight of the veNFT at max
    function vote(uint tokenId, address[] calldata _gaugeVote, uint256[] calldata _weights) external {
        uint _lockEnd = IVotingEscrow(_ve).locked__end(tokenId);
        require(_activePeriod() > lastVote[tokenId], "already voted");
        require(_nextPeriod() < _lockEnd, "lock expires soon");
        require(IVotingEscrow(_ve).isApprovedOrOwner(msg.sender, tokenId));
        require(_gaugeVote.length == _weights.length);
        lastVote[tokenId] = block.timestamp;
        _vote(tokenId, _gaugeVote, _weights);
    }

    function whitelist(address _token) public onlyAdmin {
        _whitelist(_token);
    }

    function _whitelist(address _token) internal {
        require(!isWhitelisted[_token]);
        isWhitelisted[_token] = true;
        emit Whitelisted(msg.sender, _token);
    }

    // @param _pair the pair to create a gauge for
    // @return the address of the gauge 
    function createSwapGauge(address _pair) external returns (address gauge) {
        require(ISwapFactory(factory).isPair(_pair), "!pair");
        require(gauges[_pair] == address(0x0), "exists");
        (address _tokenA, address _tokenB) = ISwapPair(_pair).tokens();
        require(isWhitelisted[_tokenA] && isWhitelisted[_tokenB], "!whitelisted");
        address _bribe = IBribeFactory(bribeFactory).createBribe();
        address _gauge = IGaugeFactory(gaugeFactory).createGauge(_pair, _bribe, _ve);
        IERC20(base).approve(_gauge, type(uint).max);
        bribes[_gauge] = _bribe;
        gauges[_pair] = _gauge;
        poolForGauge[_gauge] = _pair;
        isGauge[_gauge] = true;
        isLive[_gauge] = true;
        isReward[_gauge][_tokenA] = true;
        isReward[_gauge][_tokenB] = true;
        isReward[_gauge][base] = true;
        isBribe[_bribe][_tokenA] = true;
        isBribe[_bribe][_tokenB] = true;
        _updateFor(_gauge);
        allGauges.push(_gauge);
        emit GaugeCreated(_gauge, msg.sender, _bribe, _pair);
        return _gauge;
    }

    function attachTokenToGauge(uint tokenId, address account) external {
        require(isGauge[msg.sender]);
        if (tokenId > 0) IVotingEscrow(_ve).attach(tokenId);
        emit Attach(account, msg.sender, tokenId);
    }

    function emitDeposit(uint tokenId, address account, uint amount) external {
        require(isGauge[msg.sender]);
        emit Deposit(account, msg.sender, tokenId, amount);
    }

    function detachTokenFromGauge(uint tokenId, address account) external {
        require(isGauge[msg.sender]);
        if (tokenId > 0) IVotingEscrow(_ve).detach(tokenId);
        emit Detach(account, msg.sender, tokenId);
    }

    function emitWithdraw(uint tokenId, address account, uint amount) external {
        require(isGauge[msg.sender]);
        emit Withdraw(account, msg.sender, tokenId, amount);
    }

    function length() external view returns (uint) {
        return allGauges.length;
    }

    // @notice called by Minter contract to distribute weekly rewards
    // @param _amount the amount of tokens distributed
    function notifyRewardAmount(uint _amount) external {
        _safeTransferFrom(base, msg.sender, address(this), _amount); // transfer the distro in
        uint256 _ratio = _amount * 1e18 / totalWeight; // 1e18 adjustment is removed during claim
        if (_ratio > 0) {
            index += _ratio;
        }
        emit NotifyReward(msg.sender, base, _amount);
    }

    function updateFor(address[] memory _gauges) external {
        for (uint i = 0; i < _gauges.length; i++) {
            _updateFor(_gauges[i]);
        }
    }

    function updateForRange(uint start, uint end) public {
        for (uint i = start; i < end; i++) {
            _updateFor(allGauges[i]);
        }
    }

    function updateAll() external {
        updateForRange(0, allGauges.length);
    }

    // @notice update a gauge eligibility for rewards to the current index
    // @param _gauge the gauge to update
    function updateGauge(address _gauge) external {
        _updateFor(_gauge);
    }

    function _updateFor(address _gauge) internal {
        uint256 _supplied = weights[_gauge];
        if (_supplied > 0) {
            uint _supplyIndex = supplyIndex[_gauge];
            uint _index = index; // get global index0 for accumulated distro
            supplyIndex[_gauge] = _index; // update _gauge current position to global position
            uint _delta = _index - _supplyIndex; // see if there is any difference that need to be accrued
            if (_delta > 0) {
                uint _share = uint(_supplied) * _delta / 1e18; // add accrued difference for each supplied token
                if (isLive[_gauge]) {
                    claimable[_gauge] += _share;
                }
            }
        } else {
            supplyIndex[_gauge] = index; // new users are set to the default global state
        }
    }

    // @notice allow a gauge depositor to claim earned rewards if any
    // @param _gauges list of gauges contracts to claim rewards on
    // @param _tokens list of  tokens to claim
    function claimRewards(address[] memory _gauges, address[][] memory _tokens) external {
        for (uint i = 0; i < _gauges.length; i++) {
            IGauge(_gauges[i]).getReward(msg.sender, _tokens[i]);
        }
    }

    // @notice allow a voter to claim earned bribes if any
    // @param _bribes list of bribes contracts to claims bribes on
    // @param _tokens list of the tokens to claim
    // @param _tokenId the ID of veNFT to claim bribes for
    function claimBribes(address[] memory _bribes, address[][] memory _tokens, uint _tokenId) external {
        require(IVotingEscrow(_ve).isApprovedOrOwner(msg.sender, _tokenId));
        for (uint i = 0; i < _bribes.length; i++) {
            IBribe(_bribes[i]).getRewardForOwner(_tokenId, _tokens[i]);
        }
    }

    // @notice allow voter to claim earned fees
    // @param _fees list of bribes contracts to claim fees on
    // @param _tokens list of the tokens to claim
    // @param _tokenId the ID of veNFT to claim fees for
    function claimFees(address[] memory _fees, address[][] memory _tokens, uint _tokenId) external {
        require(IVotingEscrow(_ve).isApprovedOrOwner(msg.sender, _tokenId));
        for (uint i = 0; i < _fees.length; i++) {
            IBribe(_fees[i]).getRewardForOwner(_tokenId, _tokens[i]);
        }
    }

    // @notice distribute earned fees to the bribe contract for a given gauge
    // @param _gauges the gauges to distribute fees for
    function distributeFees(address[] memory _gauges) external {
        for (uint i = 0; i < _gauges.length; i++) {
            IGauge(_gauges[i]).claimFees();
        }
    }

    // @notice distribute fair share of rewards to a gauge
    // @param _gauge the gauge to distribute rewards to
    function distribute(address _gauge) public lock {
        IMinter(minter).update_period();
        _updateFor(_gauge);
        uint _claimable = claimable[_gauge];
        if (_claimable > IGauge(_gauge).left(base) && _claimable / DURATION > 0) {
            claimable[_gauge] = 0;
            IGauge(_gauge).notifyRewardAmount(base, _claimable);
            emit DistributeReward(msg.sender, _gauge, _claimable);
        }
    }

    function distro() external {
        distribute(0, allGauges.length);
    }

    function distribute() external {
        distribute(0, allGauges.length);
    }

    function distribute(uint start, uint finish) public {
        for (uint x = start; x < finish; x++) {
            distribute(allGauges[x]);
        }
    }

    function distribute(address[] memory _gauges) external {
        for (uint x = 0; x < _gauges.length; x++) {
            distribute(_gauges[x]);
        }
    }

    // @notice current active vote period
    // @return the UNIX timestamp of the beginning of the current vote period
    function _activePeriod() internal view returns (uint activePeriod) {
        activePeriod = block.timestamp / DURATION * DURATION;
    }

    // @notice next vote period
    // @return the UNIX timestamp of the beginning of the next vote period
    function _nextPeriod() internal view returns(uint nextPeriod) {
        nextPeriod = (block.timestamp + DURATION) / DURATION * DURATION;
    }


    function _safeTransferFrom(address token, address from, address to, uint256 value) internal {
        require(token.code.length > 0);
        (bool success, bytes memory data) =
        token.call(abi.encodeWithSelector(IERC20.transferFrom.selector, from, to, value));
        require(success && (data.length == 0 || abi.decode(data, (bool))));
    }
}
