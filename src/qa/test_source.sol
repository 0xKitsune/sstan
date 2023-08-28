
    contract Contract {

        address immutable IS_FINE;
        address constant is_bad = address(1);
        address immutable Is_Bad;
        address constant ALSO_IS_FINE = address(1);
        constructor(address _isFine, address _isBad) {
            IS_FINE = _isFine;
            Is_Bad = _isBad;
        }
        
    }
    
    contract Contract1 {
        address public owner;
        function test() public {
            owner = address(0);
        }
        constructor() {
            owner = address(1);
        }
    }
  
    contract Contract2 {
        address public owner;
        receive() external payable {}
        constructor() {
            owner = address(1);
        }
    }
   
    contract Contract3 {
        address public owner;
        modifier onlyOwner {
            require(
            msg.sender == owner,
            "Only owner can call this function."
            );
            _;
        }
        constructor() {
            owner = address(0);
        }
    }
    
    contract Contract4 {
        address public owner;
        function test() public {
            owner = address(0);
        }
    }
    
    contract Contract0 {
        address public owner;
        
        constructor(address _owner) {
            owner = _owner;
        }
    }
    contract Contract0 {
        address public owner;
        
        constructor(address _owner) {
            require(_owner != address(0), "Owner cannot be zero address");
            owner = _owner;
        }
    }
    
    import "filename.sol";
    contract Contract0 {
        
        function msgSender() internal view returns(address) {
            return msg.sender;
        }

        function _msgSender() internal view returns(address) {
            return msg.sender;
        }

        function _msgData() private view returns(bytes calldata) {
            return msg.data;
        }

        function msgData() private view returns(bytes calldata) {
            return msg.data;
        }
    }
    
    interface IContract {}

    interface Contract0 {
        function foo() external returns (uint256 x);
    }
    
    
    contract Contract0 {
        address public addr1;
        address public _addr2;
        address private _addr3;
        address private addr4;
        address internal _addr5;
        address internal addr6;
    }
    
    contract Contract0 {
        address public owner;
        uint x = 1e7;
        constructor(address _owner) {
            owner = _owner;
        }
    }
  
    contract Contract0 {
        address public owner;
        uint x = 10000000;
        constructor(address _owner) {
            require(_owner != address(0), "Owner cannot be zero address");
            owner = _owner;
        }
    }
    
    contract Contract {

        address IS_NOT_FINE;
        address isFine;
        address alsoIsFine;
        address ALSO_IS_BAD;
        constructor() {

        }
        
    }
    
    contract Contract0 {
        
        function isUnused() internal {

        }

        function isUsed() internal {
            
        }

        function useIsUsed() public {
            isUsed();
        }
        
    }
    
    contract Contract0 {
        address public owner;
        
        function foo() public returns (uint256 x) {
            uint256 y = 0;
            return y;
        }
    }
  
    contract Contract0 {
        address public owner;
        
        function foo() public returns (uint256 x) {
            x = 0;
        }
    }
    
    contract Contract {

        address immutable IS_FINE;
        address constant is_bad = address(1);
        address immutable Is_Bad;
        address constant ALSO_IS_FINE = address(1);
        constructor(address _isFine, address _isBad) {
            IS_FINE = _isFine;
            Is_Bad = _isBad;
        }
        
    }
    
    contract Contract1 {
        address public owner;
        function test() public {
            owner = address(0);
        }
        constructor() {
            owner = address(1);
        }
    }
  
    contract Contract2 {
        address public owner;
        receive() external payable {}
        constructor() {
            owner = address(1);
        }
    }
   
    contract Contract3 {
        address public owner;
        modifier onlyOwner {
            require(
            msg.sender == owner,
            "Only owner can call this function."
            );
            _;
        }
        constructor() {
            owner = address(0);
        }
    }
    
    contract Contract4 {
        address public owner;
        function test() public {
            owner = address(0);
        }
    }
    
    contract Contract0 {
        address public owner;
        
        constructor(address _owner) {
            owner = _owner;
        }
    }
    contract Contract0 {
        address public owner;
        
        constructor(address _owner) {
            require(_owner != address(0), "Owner cannot be zero address");
            owner = _owner;
        }
    }
    
    
    contract Contract0 {
        address public addr1;
        address public _addr2;
        address private _addr3;
        address private addr4;
        address internal _addr5;
        address internal addr6;
    }
    
    import "filename.sol";
    contract Contract0 {
        
        function msgSender() internal view returns(address) {
            return msg.sender;
        }

        function _msgSender() internal view returns(address) {
            return msg.sender;
        }

        function _msgData() private view returns(bytes calldata) {
            return msg.data;
        }

        function msgData() private view returns(bytes calldata) {
            return msg.data;
        }
    }
    
    interface IContract {}

    interface Contract0 {
        function foo() external returns (uint256 x);
    }
    
    contract Contract0 {
        address public owner;
        uint x = 1e7;
        constructor(address _owner) {
            owner = _owner;
        }
    }
  
    contract Contract0 {
        address public owner;
        uint x = 10000000;
        constructor(address _owner) {
            require(_owner != address(0), "Owner cannot be zero address");
            owner = _owner;
        }
    }
    
    contract Contract0 {
        
        function isUnused() internal {

        }

        function isUsed() internal {
            
        }

        function useIsUsed() public {
            isUsed();
        }
        
    }
    
    contract Contract {

        address IS_NOT_FINE;
        address isFine;
        address alsoIsFine;
        address ALSO_IS_BAD;
        constructor() {

        }
        
    }
    
    contract Contract0 {
        address public owner;
        
        function foo() public returns (uint256 x) {
            uint256 y = 0;
            return y;
        }
    }
  
    contract Contract0 {
        address public owner;
        
        function foo() public returns (uint256 x) {
            x = 0;
        }
    }
    