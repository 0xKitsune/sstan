contract Contract0 {
        address public owner;
        
        constructor(address _owner) {
            require(_owner != address(0), "Owner cannot be zero address");
            owner = _owner;
        }
    }

    
  contract Contract0 {
        address public owner;
        
        constructor(address _owner) {
            owner = _owner;
        }
    }