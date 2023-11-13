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
 | [[NC-0]](#[NC-0]) | Contract names should be in PascalCase | 43 |

## Vulnerabilities - Total: 0 




## Optimizations - Total: 0 




## Quality Assurance - Total: 43 

<a name=[NC-0]></a>
### [NC-0] Contract names should be in PascalCase - Instances: 43 

 
> Ensure that contract definitions are declared using PascalCase 

 --- 

[File:console.sol#L4](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/forge-std/src/console.sol#L4) 
```solidity
``` 



[File:StdJson.sol#L29](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/forge-std/src/StdJson.sol#L29) 
```solidity
``` 



[File:StdMath.sol#L4](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/lib/forge-std/src/StdMath.sol#L4) 
```solidity
``` 



[File:StdStorage.sol#L16](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/forge-std/src/StdStorage.sol#L16) 
```solidity
15:library stdStorageSafe {
16:    event SlotFound(address who, bytes4 fsig, bytes32 keysHash, uint256 slot);
17:    event WARNING_UninitedSlot(address who, uint256 slot);
18:
19:    Vm private constant vm = Vm(address(uint160(uint256(keccak256("hevm cheat code")))));
20:
21:    function sigs(string memory sigStr) internal pure returns (bytes4) {
22:        return bytes4(keccak256(bytes(sigStr)));
23:    }
24:
25:    /// @notice find an arbitrary storage slot given a function sig, input data, address of the contract and a value to check against
26:    // slot complexity:
27:    //  if flat, will be bytes32(uint256(uint));
28:    //  if map, will be keccak256(abi.encode(key, uint(slot)));
29:    //  if deep map, will be keccak256(abi.encode(key1, keccak256(abi.encode(key0, uint(slot)))));
30:    //  if map struct, will be bytes32(uint256(keccak256(abi.encode(key1, keccak256(abi.encode(key0, uint(slot)))))) + structFieldDepth);
31:    function find(StdStorage storage self) internal returns (uint256) {
32:        address who = self._target;
33:        bytes4 fsig = self._sig;
34:        uint256 field_depth = self._depth;
35:        bytes32[] memory ins = self._keys;
36:
37:        // calldata to test against
38:        if (self.finds[who][fsig][keccak256(abi.encodePacked(ins, field_depth))]) {
39:            return self.slots[who][fsig][keccak256(abi.encodePacked(ins, field_depth))];
40:        }
41:        bytes memory cald = abi.encodePacked(fsig, flatten(ins));
42:        vm.record();
43:        bytes32 fdat;
44:        {
45:            (, bytes memory rdat) = who.staticcall(cald);
46:            fdat = bytesToBytes32(rdat, 32 * field_depth);
47:        }
48:
49:        (bytes32[] memory reads,) = vm.accesses(address(who));
50:        if (reads.length == 1) {
51:            bytes32 curr = vm.load(who, reads[0]);
52:            if (curr == bytes32(0)) {
53:                emit WARNING_UninitedSlot(who, uint256(reads[0]));
54:            }
55:            if (fdat != curr) {
56:                require(
57:                    false,
58:                    "stdStorage find(StdStorage): Packed slot. This would cause dangerous overwriting and currently isn't supported."
59:                );
60:            }
61:            emit SlotFound(who, fsig, keccak256(abi.encodePacked(ins, field_depth)), uint256(reads[0]));
62:            self.slots[who][fsig][keccak256(abi.encodePacked(ins, field_depth))] = uint256(reads[0]);
63:            self.finds[who][fsig][keccak256(abi.encodePacked(ins, field_depth))] = true;
64:        } else if (reads.length > 1) {
65:            for (uint256 i = 0; i < reads.length; i++) {
66:                bytes32 prev = vm.load(who, reads[i]);
67:                if (prev == bytes32(0)) {
68:                    emit WARNING_UninitedSlot(who, uint256(reads[i]));
69:                }
70:                // store
71:                vm.store(who, reads[i], bytes32(hex"1337"));
72:                bool success;
73:                bytes memory rdat;
74:                {
75:                    (success, rdat) = who.staticcall(cald);
76:                    fdat = bytesToBytes32(rdat, 32 * field_depth);
77:                }
78:
79:                if (success && fdat == bytes32(hex"1337")) {
80:                    // we found which of the slots is the actual one
81:                    emit SlotFound(who, fsig, keccak256(abi.encodePacked(ins, field_depth)), uint256(reads[i]));
82:                    self.slots[who][fsig][keccak256(abi.encodePacked(ins, field_depth))] = uint256(reads[i]);
83:                    self.finds[who][fsig][keccak256(abi.encodePacked(ins, field_depth))] = true;
84:                    vm.store(who, reads[i], prev);
85:                    break;
86:                }
87:                vm.store(who, reads[i], prev);
88:            }
89:        } else {
90:            revert("stdStorage find(StdStorage): No storage use detected for target.");
91:        }
92:
93:        require(
94:            self.finds[who][fsig][keccak256(abi.encodePacked(ins, field_depth))],
95:            "stdStorage find(StdStorage): Slot(s) not found."
96:        );
97:
98:        delete self._target;
99:        delete self._sig;
100:        delete self._keys;
101:        delete self._depth;
102:
103:        return self.slots[who][fsig][keccak256(abi.encodePacked(ins, field_depth))];
104:    }
105:
106:    function target(StdStorage storage self, address _target) internal returns (StdStorage storage) {
107:        self._target = _target;
108:        return self;
109:    }
110:
111:    function sig(StdStorage storage self, bytes4 _sig) internal returns (StdStorage storage) {
112:        self._sig = _sig;
113:        return self;
114:    }
115:
116:    function sig(StdStorage storage self, string memory _sig) internal returns (StdStorage storage) {
117:        self._sig = sigs(_sig);
118:        return self;
119:    }
120:
121:    function with_key(StdStorage storage self, address who) internal returns (StdStorage storage) {
122:        self._keys.push(bytes32(uint256(uint160(who))));
123:        return self;
124:    }
125:
126:    function with_key(StdStorage storage self, uint256 amt) internal returns (StdStorage storage) {
127:        self._keys.push(bytes32(amt));
128:        return self;
129:    }
130:
131:    function with_key(StdStorage storage self, bytes32 key) internal returns (StdStorage storage) {
132:        self._keys.push(key);
133:        return self;
134:    }
135:
136:    function depth(StdStorage storage self, uint256 _depth) internal returns (StdStorage storage) {
137:        self._depth = _depth;
138:        return self;
139:    }
140:
141:    function read(StdStorage storage self) private returns (bytes memory) {
142:        address t = self._target;
143:        uint256 s = find(self);
144:        return abi.encode(vm.load(t, bytes32(s)));
145:    }
146:
147:    function read_bytes32(StdStorage storage self) internal returns (bytes32) {
148:        return abi.decode(read(self), (bytes32));
149:    }
150:
151:    function read_bool(StdStorage storage self) internal returns (bool) {
152:        int256 v = read_int(self);
153:        if (v == 0) return false;
154:        if (v == 1) return true;
155:        revert("stdStorage read_bool(StdStorage): Cannot decode. Make sure you are reading a bool.");
156:    }
157:
158:    function read_address(StdStorage storage self) internal returns (address) {
159:        return abi.decode(read(self), (address));
160:    }
161:
162:    function read_uint(StdStorage storage self) internal returns (uint256) {
163:        return abi.decode(read(self), (uint256));
164:    }
165:
166:    function read_int(StdStorage storage self) internal returns (int256) {
167:        return abi.decode(read(self), (int256));
168:    }
169:
170:    function bytesToBytes32(bytes memory b, uint256 offset) private pure returns (bytes32) {
171:        bytes32 out;
172:
173:        uint256 max = b.length > 32 ? 32 : b.length;
174:        for (uint256 i = 0; i < max; i++) {
175:            out |= bytes32(b[offset + i] & 0xFF) >> (i * 8);
176:        }
177:        return out;
178:    }
179:
180:    function flatten(bytes32[] memory b) private pure returns (bytes memory) {
181:        bytes memory result = new bytes(b.length * 32);
182:        for (uint256 i = 0; i < b.length; i++) {
183:            bytes32 k = b[i];
184:            /// @solidity memory-safe-assembly
185:            assembly {
186:                mstore(add(result, add(32, mul(32, i))), k)
187:            }
188:        }
189:
190:        return result;
191:    }
192:}
193:
``` 



[File:StdStorage.sol#L195](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/forge-std/src/StdStorage.sol#L195) 
```solidity
``` 



[File:StdMath.sol#L4](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/forge-std/src/StdMath.sol#L4) 
```solidity
``` 



[File:StdError.sol#L5](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/forge-std/src/StdError.sol#L5) 
```solidity
``` 



[File:StdMath.sol#L4](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/lib/forge-std/src/StdMath.sol#L4) 
```solidity
``` 



[File:console.sol#L4](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/lib/forge-std/src/console.sol#L4) 
```solidity
``` 



[File:StdError.sol#L5](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/forge-std/src/StdError.sol#L5) 
```solidity
``` 



[File:console2.sol#L9](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/forge-std/src/console2.sol#L9) 
```solidity
``` 



[File:safeconsole.sol#L6](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/forge-std/src/safeconsole.sol#L6) 
```solidity
``` 



[File:console2.sol#L9](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/lib/forge-std/src/console2.sol#L9) 
```solidity
``` 



[File:StdStorage.sol#L16](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/lib/forge-std/src/StdStorage.sol#L16) 
```solidity
15:library stdStorageSafe {
16:    event SlotFound(address who, bytes4 fsig, bytes32 keysHash, uint256 slot);
17:    event WARNING_UninitedSlot(address who, uint256 slot);
18:
19:    Vm private constant vm = Vm(address(uint160(uint256(keccak256("hevm cheat code")))));
20:
21:    function sigs(string memory sigStr) internal pure returns (bytes4) {
22:        return bytes4(keccak256(bytes(sigStr)));
23:    }
24:
25:    /// @notice find an arbitrary storage slot given a function sig, input data, address of the contract and a value to check against
26:    // slot complexity:
27:    //  if flat, will be bytes32(uint256(uint));
28:    //  if map, will be keccak256(abi.encode(key, uint(slot)));
29:    //  if deep map, will be keccak256(abi.encode(key1, keccak256(abi.encode(key0, uint(slot)))));
30:    //  if map struct, will be bytes32(uint256(keccak256(abi.encode(key1, keccak256(abi.encode(key0, uint(slot)))))) + structFieldDepth);
31:    function find(StdStorage storage self) internal returns (uint256) {
32:        address who = self._target;
33:        bytes4 fsig = self._sig;
34:        uint256 field_depth = self._depth;
35:        bytes32[] memory ins = self._keys;
36:
37:        // calldata to test against
38:        if (self.finds[who][fsig][keccak256(abi.encodePacked(ins, field_depth))]) {
39:            return self.slots[who][fsig][keccak256(abi.encodePacked(ins, field_depth))];
40:        }
41:        bytes memory cald = abi.encodePacked(fsig, flatten(ins));
42:        vm.record();
43:        bytes32 fdat;
44:        {
45:            (, bytes memory rdat) = who.staticcall(cald);
46:            fdat = bytesToBytes32(rdat, 32 * field_depth);
47:        }
48:
49:        (bytes32[] memory reads,) = vm.accesses(address(who));
50:        if (reads.length == 1) {
51:            bytes32 curr = vm.load(who, reads[0]);
52:            if (curr == bytes32(0)) {
53:                emit WARNING_UninitedSlot(who, uint256(reads[0]));
54:            }
55:            if (fdat != curr) {
56:                require(
57:                    false,
58:                    "stdStorage find(StdStorage): Packed slot. This would cause dangerous overwriting and currently isn't supported."
59:                );
60:            }
61:            emit SlotFound(who, fsig, keccak256(abi.encodePacked(ins, field_depth)), uint256(reads[0]));
62:            self.slots[who][fsig][keccak256(abi.encodePacked(ins, field_depth))] = uint256(reads[0]);
63:            self.finds[who][fsig][keccak256(abi.encodePacked(ins, field_depth))] = true;
64:        } else if (reads.length > 1) {
65:            for (uint256 i = 0; i < reads.length; i++) {
66:                bytes32 prev = vm.load(who, reads[i]);
67:                if (prev == bytes32(0)) {
68:                    emit WARNING_UninitedSlot(who, uint256(reads[i]));
69:                }
70:                // store
71:                vm.store(who, reads[i], bytes32(hex"1337"));
72:                bool success;
73:                bytes memory rdat;
74:                {
75:                    (success, rdat) = who.staticcall(cald);
76:                    fdat = bytesToBytes32(rdat, 32 * field_depth);
77:                }
78:
79:                if (success && fdat == bytes32(hex"1337")) {
80:                    // we found which of the slots is the actual one
81:                    emit SlotFound(who, fsig, keccak256(abi.encodePacked(ins, field_depth)), uint256(reads[i]));
82:                    self.slots[who][fsig][keccak256(abi.encodePacked(ins, field_depth))] = uint256(reads[i]);
83:                    self.finds[who][fsig][keccak256(abi.encodePacked(ins, field_depth))] = true;
84:                    vm.store(who, reads[i], prev);
85:                    break;
86:                }
87:                vm.store(who, reads[i], prev);
88:            }
89:        } else {
90:            revert("stdStorage find(StdStorage): No storage use detected for target.");
91:        }
92:
93:        require(
94:            self.finds[who][fsig][keccak256(abi.encodePacked(ins, field_depth))],
95:            "stdStorage find(StdStorage): Slot(s) not found."
96:        );
97:
98:        delete self._target;
99:        delete self._sig;
100:        delete self._keys;
101:        delete self._depth;
102:
103:        return self.slots[who][fsig][keccak256(abi.encodePacked(ins, field_depth))];
104:    }
105:
106:    function target(StdStorage storage self, address _target) internal returns (StdStorage storage) {
107:        self._target = _target;
108:        return self;
109:    }
110:
111:    function sig(StdStorage storage self, bytes4 _sig) internal returns (StdStorage storage) {
112:        self._sig = _sig;
113:        return self;
114:    }
115:
116:    function sig(StdStorage storage self, string memory _sig) internal returns (StdStorage storage) {
117:        self._sig = sigs(_sig);
118:        return self;
119:    }
120:
121:    function with_key(StdStorage storage self, address who) internal returns (StdStorage storage) {
122:        self._keys.push(bytes32(uint256(uint160(who))));
123:        return self;
124:    }
125:
126:    function with_key(StdStorage storage self, uint256 amt) internal returns (StdStorage storage) {
127:        self._keys.push(bytes32(amt));
128:        return self;
129:    }
130:
131:    function with_key(StdStorage storage self, bytes32 key) internal returns (StdStorage storage) {
132:        self._keys.push(key);
133:        return self;
134:    }
135:
136:    function depth(StdStorage storage self, uint256 _depth) internal returns (StdStorage storage) {
137:        self._depth = _depth;
138:        return self;
139:    }
140:
141:    function read(StdStorage storage self) private returns (bytes memory) {
142:        address t = self._target;
143:        uint256 s = find(self);
144:        return abi.encode(vm.load(t, bytes32(s)));
145:    }
146:
147:    function read_bytes32(StdStorage storage self) internal returns (bytes32) {
148:        return abi.decode(read(self), (bytes32));
149:    }
150:
151:    function read_bool(StdStorage storage self) internal returns (bool) {
152:        int256 v = read_int(self);
153:        if (v == 0) return false;
154:        if (v == 1) return true;
155:        revert("stdStorage read_bool(StdStorage): Cannot decode. Make sure you are reading a bool.");
156:    }
157:
158:    function read_address(StdStorage storage self) internal returns (address) {
159:        return abi.decode(read(self), (address));
160:    }
161:
162:    function read_uint(StdStorage storage self) internal returns (uint256) {
163:        return abi.decode(read(self), (uint256));
164:    }
165:
166:    function read_int(StdStorage storage self) internal returns (int256) {
167:        return abi.decode(read(self), (int256));
168:    }
169:
170:    function bytesToBytes32(bytes memory b, uint256 offset) private pure returns (bytes32) {
171:        bytes32 out;
172:
173:        uint256 max = b.length > 32 ? 32 : b.length;
174:        for (uint256 i = 0; i < max; i++) {
175:            out |= bytes32(b[offset + i] & 0xFF) >> (i * 8);
176:        }
177:        return out;
178:    }
179:
180:    function flatten(bytes32[] memory b) private pure returns (bytes memory) {
181:        bytes memory result = new bytes(b.length * 32);
182:        for (uint256 i = 0; i < b.length; i++) {
183:            bytes32 k = b[i];
184:            /// @solidity memory-safe-assembly
185:            assembly {
186:                mstore(add(result, add(32, mul(32, i))), k)
187:            }
188:        }
189:
190:        return result;
191:    }
192:}
193:
``` 



[File:StdStorage.sol#L195](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/lib/forge-std/src/StdStorage.sol#L195) 
```solidity
``` 



[File:StdJson.sol#L29](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/lib/forge-std/src/StdJson.sol#L29) 
```solidity
``` 



[File:console2.sol#L9](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/forge-std/src/console2.sol#L9) 
```solidity
``` 



[File:StdMath.sol#L4](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/lib/forge-std/src/StdMath.sol#L4) 
```solidity
``` 



[File:StdJson.sol#L29](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/lib/forge-std/src/StdJson.sol#L29) 
```solidity
``` 



[File:StdStorage.sol#L16](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/lib/forge-std/src/StdStorage.sol#L16) 
```solidity
15:library stdStorageSafe {
16:    event SlotFound(address who, bytes4 fsig, bytes32 keysHash, uint256 slot);
17:    event WARNING_UninitedSlot(address who, uint256 slot);
18:
19:    Vm private constant vm = Vm(address(uint160(uint256(keccak256("hevm cheat code")))));
20:
21:    function sigs(string memory sigStr) internal pure returns (bytes4) {
22:        return bytes4(keccak256(bytes(sigStr)));
23:    }
24:
25:    /// @notice find an arbitrary storage slot given a function sig, input data, address of the contract and a value to check against
26:    // slot complexity:
27:    //  if flat, will be bytes32(uint256(uint));
28:    //  if map, will be keccak256(abi.encode(key, uint(slot)));
29:    //  if deep map, will be keccak256(abi.encode(key1, keccak256(abi.encode(key0, uint(slot)))));
30:    //  if map struct, will be bytes32(uint256(keccak256(abi.encode(key1, keccak256(abi.encode(key0, uint(slot)))))) + structFieldDepth);
31:    function find(StdStorage storage self) internal returns (uint256) {
32:        address who = self._target;
33:        bytes4 fsig = self._sig;
34:        uint256 field_depth = self._depth;
35:        bytes32[] memory ins = self._keys;
36:
37:        // calldata to test against
38:        if (self.finds[who][fsig][keccak256(abi.encodePacked(ins, field_depth))]) {
39:            return self.slots[who][fsig][keccak256(abi.encodePacked(ins, field_depth))];
40:        }
41:        bytes memory cald = abi.encodePacked(fsig, flatten(ins));
42:        vm.record();
43:        bytes32 fdat;
44:        {
45:            (, bytes memory rdat) = who.staticcall(cald);
46:            fdat = bytesToBytes32(rdat, 32 * field_depth);
47:        }
48:
49:        (bytes32[] memory reads,) = vm.accesses(address(who));
50:        if (reads.length == 1) {
51:            bytes32 curr = vm.load(who, reads[0]);
52:            if (curr == bytes32(0)) {
53:                emit WARNING_UninitedSlot(who, uint256(reads[0]));
54:            }
55:            if (fdat != curr) {
56:                require(
57:                    false,
58:                    "stdStorage find(StdStorage): Packed slot. This would cause dangerous overwriting and currently isn't supported."
59:                );
60:            }
61:            emit SlotFound(who, fsig, keccak256(abi.encodePacked(ins, field_depth)), uint256(reads[0]));
62:            self.slots[who][fsig][keccak256(abi.encodePacked(ins, field_depth))] = uint256(reads[0]);
63:            self.finds[who][fsig][keccak256(abi.encodePacked(ins, field_depth))] = true;
64:        } else if (reads.length > 1) {
65:            for (uint256 i = 0; i < reads.length; i++) {
66:                bytes32 prev = vm.load(who, reads[i]);
67:                if (prev == bytes32(0)) {
68:                    emit WARNING_UninitedSlot(who, uint256(reads[i]));
69:                }
70:                // store
71:                vm.store(who, reads[i], bytes32(hex"1337"));
72:                bool success;
73:                bytes memory rdat;
74:                {
75:                    (success, rdat) = who.staticcall(cald);
76:                    fdat = bytesToBytes32(rdat, 32 * field_depth);
77:                }
78:
79:                if (success && fdat == bytes32(hex"1337")) {
80:                    // we found which of the slots is the actual one
81:                    emit SlotFound(who, fsig, keccak256(abi.encodePacked(ins, field_depth)), uint256(reads[i]));
82:                    self.slots[who][fsig][keccak256(abi.encodePacked(ins, field_depth))] = uint256(reads[i]);
83:                    self.finds[who][fsig][keccak256(abi.encodePacked(ins, field_depth))] = true;
84:                    vm.store(who, reads[i], prev);
85:                    break;
86:                }
87:                vm.store(who, reads[i], prev);
88:            }
89:        } else {
90:            revert("stdStorage find(StdStorage): No storage use detected for target.");
91:        }
92:
93:        require(
94:            self.finds[who][fsig][keccak256(abi.encodePacked(ins, field_depth))],
95:            "stdStorage find(StdStorage): Slot(s) not found."
96:        );
97:
98:        delete self._target;
99:        delete self._sig;
100:        delete self._keys;
101:        delete self._depth;
102:
103:        return self.slots[who][fsig][keccak256(abi.encodePacked(ins, field_depth))];
104:    }
105:
106:    function target(StdStorage storage self, address _target) internal returns (StdStorage storage) {
107:        self._target = _target;
108:        return self;
109:    }
110:
111:    function sig(StdStorage storage self, bytes4 _sig) internal returns (StdStorage storage) {
112:        self._sig = _sig;
113:        return self;
114:    }
115:
116:    function sig(StdStorage storage self, string memory _sig) internal returns (StdStorage storage) {
117:        self._sig = sigs(_sig);
118:        return self;
119:    }
120:
121:    function with_key(StdStorage storage self, address who) internal returns (StdStorage storage) {
122:        self._keys.push(bytes32(uint256(uint160(who))));
123:        return self;
124:    }
125:
126:    function with_key(StdStorage storage self, uint256 amt) internal returns (StdStorage storage) {
127:        self._keys.push(bytes32(amt));
128:        return self;
129:    }
130:
131:    function with_key(StdStorage storage self, bytes32 key) internal returns (StdStorage storage) {
132:        self._keys.push(key);
133:        return self;
134:    }
135:
136:    function depth(StdStorage storage self, uint256 _depth) internal returns (StdStorage storage) {
137:        self._depth = _depth;
138:        return self;
139:    }
140:
141:    function read(StdStorage storage self) private returns (bytes memory) {
142:        address t = self._target;
143:        uint256 s = find(self);
144:        return abi.encode(vm.load(t, bytes32(s)));
145:    }
146:
147:    function read_bytes32(StdStorage storage self) internal returns (bytes32) {
148:        return abi.decode(read(self), (bytes32));
149:    }
150:
151:    function read_bool(StdStorage storage self) internal returns (bool) {
152:        int256 v = read_int(self);
153:        if (v == 0) return false;
154:        if (v == 1) return true;
155:        revert("stdStorage read_bool(StdStorage): Cannot decode. Make sure you are reading a bool.");
156:    }
157:
158:    function read_address(StdStorage storage self) internal returns (address) {
159:        return abi.decode(read(self), (address));
160:    }
161:
162:    function read_uint(StdStorage storage self) internal returns (uint256) {
163:        return abi.decode(read(self), (uint256));
164:    }
165:
166:    function read_int(StdStorage storage self) internal returns (int256) {
167:        return abi.decode(read(self), (int256));
168:    }
169:
170:    function bytesToBytes32(bytes memory b, uint256 offset) private pure returns (bytes32) {
171:        bytes32 out;
172:
173:        uint256 max = b.length > 32 ? 32 : b.length;
174:        for (uint256 i = 0; i < max; i++) {
175:            out |= bytes32(b[offset + i] & 0xFF) >> (i * 8);
176:        }
177:        return out;
178:    }
179:
180:    function flatten(bytes32[] memory b) private pure returns (bytes memory) {
181:        bytes memory result = new bytes(b.length * 32);
182:        for (uint256 i = 0; i < b.length; i++) {
183:            bytes32 k = b[i];
184:            /// @solidity memory-safe-assembly
185:            assembly {
186:                mstore(add(result, add(32, mul(32, i))), k)
187:            }
188:        }
189:
190:        return result;
191:    }
192:}
193:
``` 



[File:StdStorage.sol#L195](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/lib/forge-std/src/StdStorage.sol#L195) 
```solidity
``` 



[File:StdJson.sol#L29](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/lib/forge-std/src/StdJson.sol#L29) 
```solidity
``` 



[File:StdJson.sol#L29](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/forge-std/src/StdJson.sol#L29) 
```solidity
``` 



[File:StdStorage.sol#L16](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/lib/forge-std/src/StdStorage.sol#L16) 
```solidity
15:library stdStorageSafe {
16:    event SlotFound(address who, bytes4 fsig, bytes32 keysHash, uint256 slot);
17:    event WARNING_UninitedSlot(address who, uint256 slot);
18:
19:    Vm private constant vm = Vm(address(uint160(uint256(keccak256("hevm cheat code")))));
20:
21:    function sigs(string memory sigStr) internal pure returns (bytes4) {
22:        return bytes4(keccak256(bytes(sigStr)));
23:    }
24:
25:    /// @notice find an arbitrary storage slot given a function sig, input data, address of the contract and a value to check against
26:    // slot complexity:
27:    //  if flat, will be bytes32(uint256(uint));
28:    //  if map, will be keccak256(abi.encode(key, uint(slot)));
29:    //  if deep map, will be keccak256(abi.encode(key1, keccak256(abi.encode(key0, uint(slot)))));
30:    //  if map struct, will be bytes32(uint256(keccak256(abi.encode(key1, keccak256(abi.encode(key0, uint(slot)))))) + structFieldDepth);
31:    function find(StdStorage storage self) internal returns (uint256) {
32:        address who = self._target;
33:        bytes4 fsig = self._sig;
34:        uint256 field_depth = self._depth;
35:        bytes32[] memory ins = self._keys;
36:
37:        // calldata to test against
38:        if (self.finds[who][fsig][keccak256(abi.encodePacked(ins, field_depth))]) {
39:            return self.slots[who][fsig][keccak256(abi.encodePacked(ins, field_depth))];
40:        }
41:        bytes memory cald = abi.encodePacked(fsig, flatten(ins));
42:        vm.record();
43:        bytes32 fdat;
44:        {
45:            (, bytes memory rdat) = who.staticcall(cald);
46:            fdat = bytesToBytes32(rdat, 32 * field_depth);
47:        }
48:
49:        (bytes32[] memory reads,) = vm.accesses(address(who));
50:        if (reads.length == 1) {
51:            bytes32 curr = vm.load(who, reads[0]);
52:            if (curr == bytes32(0)) {
53:                emit WARNING_UninitedSlot(who, uint256(reads[0]));
54:            }
55:            if (fdat != curr) {
56:                require(
57:                    false,
58:                    "stdStorage find(StdStorage): Packed slot. This would cause dangerous overwriting and currently isn't supported."
59:                );
60:            }
61:            emit SlotFound(who, fsig, keccak256(abi.encodePacked(ins, field_depth)), uint256(reads[0]));
62:            self.slots[who][fsig][keccak256(abi.encodePacked(ins, field_depth))] = uint256(reads[0]);
63:            self.finds[who][fsig][keccak256(abi.encodePacked(ins, field_depth))] = true;
64:        } else if (reads.length > 1) {
65:            for (uint256 i = 0; i < reads.length; i++) {
66:                bytes32 prev = vm.load(who, reads[i]);
67:                if (prev == bytes32(0)) {
68:                    emit WARNING_UninitedSlot(who, uint256(reads[i]));
69:                }
70:                // store
71:                vm.store(who, reads[i], bytes32(hex"1337"));
72:                bool success;
73:                bytes memory rdat;
74:                {
75:                    (success, rdat) = who.staticcall(cald);
76:                    fdat = bytesToBytes32(rdat, 32 * field_depth);
77:                }
78:
79:                if (success && fdat == bytes32(hex"1337")) {
80:                    // we found which of the slots is the actual one
81:                    emit SlotFound(who, fsig, keccak256(abi.encodePacked(ins, field_depth)), uint256(reads[i]));
82:                    self.slots[who][fsig][keccak256(abi.encodePacked(ins, field_depth))] = uint256(reads[i]);
83:                    self.finds[who][fsig][keccak256(abi.encodePacked(ins, field_depth))] = true;
84:                    vm.store(who, reads[i], prev);
85:                    break;
86:                }
87:                vm.store(who, reads[i], prev);
88:            }
89:        } else {
90:            revert("stdStorage find(StdStorage): No storage use detected for target.");
91:        }
92:
93:        require(
94:            self.finds[who][fsig][keccak256(abi.encodePacked(ins, field_depth))],
95:            "stdStorage find(StdStorage): Slot(s) not found."
96:        );
97:
98:        delete self._target;
99:        delete self._sig;
100:        delete self._keys;
101:        delete self._depth;
102:
103:        return self.slots[who][fsig][keccak256(abi.encodePacked(ins, field_depth))];
104:    }
105:
106:    function target(StdStorage storage self, address _target) internal returns (StdStorage storage) {
107:        self._target = _target;
108:        return self;
109:    }
110:
111:    function sig(StdStorage storage self, bytes4 _sig) internal returns (StdStorage storage) {
112:        self._sig = _sig;
113:        return self;
114:    }
115:
116:    function sig(StdStorage storage self, string memory _sig) internal returns (StdStorage storage) {
117:        self._sig = sigs(_sig);
118:        return self;
119:    }
120:
121:    function with_key(StdStorage storage self, address who) internal returns (StdStorage storage) {
122:        self._keys.push(bytes32(uint256(uint160(who))));
123:        return self;
124:    }
125:
126:    function with_key(StdStorage storage self, uint256 amt) internal returns (StdStorage storage) {
127:        self._keys.push(bytes32(amt));
128:        return self;
129:    }
130:
131:    function with_key(StdStorage storage self, bytes32 key) internal returns (StdStorage storage) {
132:        self._keys.push(key);
133:        return self;
134:    }
135:
136:    function depth(StdStorage storage self, uint256 _depth) internal returns (StdStorage storage) {
137:        self._depth = _depth;
138:        return self;
139:    }
140:
141:    function read(StdStorage storage self) private returns (bytes memory) {
142:        address t = self._target;
143:        uint256 s = find(self);
144:        return abi.encode(vm.load(t, bytes32(s)));
145:    }
146:
147:    function read_bytes32(StdStorage storage self) internal returns (bytes32) {
148:        return abi.decode(read(self), (bytes32));
149:    }
150:
151:    function read_bool(StdStorage storage self) internal returns (bool) {
152:        int256 v = read_int(self);
153:        if (v == 0) return false;
154:        if (v == 1) return true;
155:        revert("stdStorage read_bool(StdStorage): Cannot decode. Make sure you are reading a bool.");
156:    }
157:
158:    function read_address(StdStorage storage self) internal returns (address) {
159:        return abi.decode(read(self), (address));
160:    }
161:
162:    function read_uint(StdStorage storage self) internal returns (uint256) {
163:        return abi.decode(read(self), (uint256));
164:    }
165:
166:    function read_int(StdStorage storage self) internal returns (int256) {
167:        return abi.decode(read(self), (int256));
168:    }
169:
170:    function bytesToBytes32(bytes memory b, uint256 offset) private pure returns (bytes32) {
171:        bytes32 out;
172:
173:        uint256 max = b.length > 32 ? 32 : b.length;
174:        for (uint256 i = 0; i < max; i++) {
175:            out |= bytes32(b[offset + i] & 0xFF) >> (i * 8);
176:        }
177:        return out;
178:    }
179:
180:    function flatten(bytes32[] memory b) private pure returns (bytes memory) {
181:        bytes memory result = new bytes(b.length * 32);
182:        for (uint256 i = 0; i < b.length; i++) {
183:            bytes32 k = b[i];
184:            /// @solidity memory-safe-assembly
185:            assembly {
186:                mstore(add(result, add(32, mul(32, i))), k)
187:            }
188:        }
189:
190:        return result;
191:    }
192:}
193:
``` 



[File:StdStorage.sol#L195](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/lib/forge-std/src/StdStorage.sol#L195) 
```solidity
``` 



[File:StdError.sol#L5](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/lib/forge-std/src/StdError.sol#L5) 
```solidity
``` 



[File:StdMath.sol#L4](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/forge-std/src/StdMath.sol#L4) 
```solidity
``` 



[File:console2.sol#L9](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/forge-std/src/console2.sol#L9) 
```solidity
``` 



[File:StdJson.sol#L29](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/forge-std/src/StdJson.sol#L29) 
```solidity
``` 



[File:StdError.sol#L5](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/lib/forge-std/src/StdError.sol#L5) 
```solidity
``` 



[File:StdError.sol#L5](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/lib/forge-std/src/StdError.sol#L5) 
```solidity
``` 



[File:console2.sol#L9](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/lib/forge-std/src/console2.sol#L9) 
```solidity
``` 



[File:console2.sol#L9](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/openzeppelin-contracts/lib/forge-std/src/console2.sol#L9) 
```solidity
``` 



[File:console.sol#L4](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/forge-std/src/console.sol#L4) 
```solidity
``` 



[File:StdStorage.sol#L16](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/forge-std/src/StdStorage.sol#L16) 
```solidity
15:library stdStorageSafe {
16:    event SlotFound(address who, bytes4 fsig, bytes32 keysHash, uint256 slot);
17:    event WARNING_UninitedSlot(address who, uint256 slot);
18:
19:    Vm private constant vm = Vm(address(uint160(uint256(keccak256("hevm cheat code")))));
20:
21:    function sigs(string memory sigStr) internal pure returns (bytes4) {
22:        return bytes4(keccak256(bytes(sigStr)));
23:    }
24:
25:    /// @notice find an arbitrary storage slot given a function sig, input data, address of the contract and a value to check against
26:    // slot complexity:
27:    //  if flat, will be bytes32(uint256(uint));
28:    //  if map, will be keccak256(abi.encode(key, uint(slot)));
29:    //  if deep map, will be keccak256(abi.encode(key1, keccak256(abi.encode(key0, uint(slot)))));
30:    //  if map struct, will be bytes32(uint256(keccak256(abi.encode(key1, keccak256(abi.encode(key0, uint(slot)))))) + structFieldDepth);
31:    function find(StdStorage storage self) internal returns (uint256) {
32:        address who = self._target;
33:        bytes4 fsig = self._sig;
34:        uint256 field_depth = self._depth;
35:        bytes32[] memory ins = self._keys;
36:
37:        // calldata to test against
38:        if (self.finds[who][fsig][keccak256(abi.encodePacked(ins, field_depth))]) {
39:            return self.slots[who][fsig][keccak256(abi.encodePacked(ins, field_depth))];
40:        }
41:        bytes memory cald = abi.encodePacked(fsig, flatten(ins));
42:        vm.record();
43:        bytes32 fdat;
44:        {
45:            (, bytes memory rdat) = who.staticcall(cald);
46:            fdat = bytesToBytes32(rdat, 32 * field_depth);
47:        }
48:
49:        (bytes32[] memory reads,) = vm.accesses(address(who));
50:        if (reads.length == 1) {
51:            bytes32 curr = vm.load(who, reads[0]);
52:            if (curr == bytes32(0)) {
53:                emit WARNING_UninitedSlot(who, uint256(reads[0]));
54:            }
55:            if (fdat != curr) {
56:                require(
57:                    false,
58:                    "stdStorage find(StdStorage): Packed slot. This would cause dangerous overwriting and currently isn't supported."
59:                );
60:            }
61:            emit SlotFound(who, fsig, keccak256(abi.encodePacked(ins, field_depth)), uint256(reads[0]));
62:            self.slots[who][fsig][keccak256(abi.encodePacked(ins, field_depth))] = uint256(reads[0]);
63:            self.finds[who][fsig][keccak256(abi.encodePacked(ins, field_depth))] = true;
64:        } else if (reads.length > 1) {
65:            for (uint256 i = 0; i < reads.length; i++) {
66:                bytes32 prev = vm.load(who, reads[i]);
67:                if (prev == bytes32(0)) {
68:                    emit WARNING_UninitedSlot(who, uint256(reads[i]));
69:                }
70:                // store
71:                vm.store(who, reads[i], bytes32(hex"1337"));
72:                bool success;
73:                bytes memory rdat;
74:                {
75:                    (success, rdat) = who.staticcall(cald);
76:                    fdat = bytesToBytes32(rdat, 32 * field_depth);
77:                }
78:
79:                if (success && fdat == bytes32(hex"1337")) {
80:                    // we found which of the slots is the actual one
81:                    emit SlotFound(who, fsig, keccak256(abi.encodePacked(ins, field_depth)), uint256(reads[i]));
82:                    self.slots[who][fsig][keccak256(abi.encodePacked(ins, field_depth))] = uint256(reads[i]);
83:                    self.finds[who][fsig][keccak256(abi.encodePacked(ins, field_depth))] = true;
84:                    vm.store(who, reads[i], prev);
85:                    break;
86:                }
87:                vm.store(who, reads[i], prev);
88:            }
89:        } else {
90:            revert("stdStorage find(StdStorage): No storage use detected for target.");
91:        }
92:
93:        require(
94:            self.finds[who][fsig][keccak256(abi.encodePacked(ins, field_depth))],
95:            "stdStorage find(StdStorage): Slot(s) not found."
96:        );
97:
98:        delete self._target;
99:        delete self._sig;
100:        delete self._keys;
101:        delete self._depth;
102:
103:        return self.slots[who][fsig][keccak256(abi.encodePacked(ins, field_depth))];
104:    }
105:
106:    function target(StdStorage storage self, address _target) internal returns (StdStorage storage) {
107:        self._target = _target;
108:        return self;
109:    }
110:
111:    function sig(StdStorage storage self, bytes4 _sig) internal returns (StdStorage storage) {
112:        self._sig = _sig;
113:        return self;
114:    }
115:
116:    function sig(StdStorage storage self, string memory _sig) internal returns (StdStorage storage) {
117:        self._sig = sigs(_sig);
118:        return self;
119:    }
120:
121:    function with_key(StdStorage storage self, address who) internal returns (StdStorage storage) {
122:        self._keys.push(bytes32(uint256(uint160(who))));
123:        return self;
124:    }
125:
126:    function with_key(StdStorage storage self, uint256 amt) internal returns (StdStorage storage) {
127:        self._keys.push(bytes32(amt));
128:        return self;
129:    }
130:
131:    function with_key(StdStorage storage self, bytes32 key) internal returns (StdStorage storage) {
132:        self._keys.push(key);
133:        return self;
134:    }
135:
136:    function depth(StdStorage storage self, uint256 _depth) internal returns (StdStorage storage) {
137:        self._depth = _depth;
138:        return self;
139:    }
140:
141:    function read(StdStorage storage self) private returns (bytes memory) {
142:        address t = self._target;
143:        uint256 s = find(self);
144:        return abi.encode(vm.load(t, bytes32(s)));
145:    }
146:
147:    function read_bytes32(StdStorage storage self) internal returns (bytes32) {
148:        return abi.decode(read(self), (bytes32));
149:    }
150:
151:    function read_bool(StdStorage storage self) internal returns (bool) {
152:        int256 v = read_int(self);
153:        if (v == 0) return false;
154:        if (v == 1) return true;
155:        revert("stdStorage read_bool(StdStorage): Cannot decode. Make sure you are reading a bool.");
156:    }
157:
158:    function read_address(StdStorage storage self) internal returns (address) {
159:        return abi.decode(read(self), (address));
160:    }
161:
162:    function read_uint(StdStorage storage self) internal returns (uint256) {
163:        return abi.decode(read(self), (uint256));
164:    }
165:
166:    function read_int(StdStorage storage self) internal returns (int256) {
167:        return abi.decode(read(self), (int256));
168:    }
169:
170:    function bytesToBytes32(bytes memory b, uint256 offset) private pure returns (bytes32) {
171:        bytes32 out;
172:
173:        uint256 max = b.length > 32 ? 32 : b.length;
174:        for (uint256 i = 0; i < max; i++) {
175:            out |= bytes32(b[offset + i] & 0xFF) >> (i * 8);
176:        }
177:        return out;
178:    }
179:
180:    function flatten(bytes32[] memory b) private pure returns (bytes memory) {
181:        bytes memory result = new bytes(b.length * 32);
182:        for (uint256 i = 0; i < b.length; i++) {
183:            bytes32 k = b[i];
184:            /// @solidity memory-safe-assembly
185:            assembly {
186:                mstore(add(result, add(32, mul(32, i))), k)
187:            }
188:        }
189:
190:        return result;
191:    }
192:}
193:
``` 



[File:StdStorage.sol#L195](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/forge-std/src/StdStorage.sol#L195) 
```solidity
``` 



[File:console.sol#L4](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/sol-utils/lib/forge-std/src/console.sol#L4) 
```solidity
``` 



[File:StdError.sol#L5](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/forge-std/src/StdError.sol#L5) 
```solidity
``` 



[File:StdMath.sol#L4](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/forge-std/src/StdMath.sol#L4) 
```solidity
``` 



[File:console.sol#L4](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/openzeppelin-contracts/lib/forge-std/src/console.sol#L4) 
```solidity
``` 



[File:StdStorage.sol#L16](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/forge-std/src/StdStorage.sol#L16) 
```solidity
15:library stdStorageSafe {
16:    event SlotFound(address who, bytes4 fsig, bytes32 keysHash, uint256 slot);
17:    event WARNING_UninitedSlot(address who, uint256 slot);
18:
19:    Vm private constant vm = Vm(address(uint160(uint256(keccak256("hevm cheat code")))));
20:
21:    function sigs(string memory sigStr) internal pure returns (bytes4) {
22:        return bytes4(keccak256(bytes(sigStr)));
23:    }
24:
25:    /// @notice find an arbitrary storage slot given a function sig, input data, address of the contract and a value to check against
26:    // slot complexity:
27:    //  if flat, will be bytes32(uint256(uint));
28:    //  if map, will be keccak256(abi.encode(key, uint(slot)));
29:    //  if deep map, will be keccak256(abi.encode(key1, keccak256(abi.encode(key0, uint(slot)))));
30:    //  if map struct, will be bytes32(uint256(keccak256(abi.encode(key1, keccak256(abi.encode(key0, uint(slot)))))) + structFieldDepth);
31:    function find(StdStorage storage self) internal returns (uint256) {
32:        address who = self._target;
33:        bytes4 fsig = self._sig;
34:        uint256 field_depth = self._depth;
35:        bytes32[] memory ins = self._keys;
36:
37:        // calldata to test against
38:        if (self.finds[who][fsig][keccak256(abi.encodePacked(ins, field_depth))]) {
39:            return self.slots[who][fsig][keccak256(abi.encodePacked(ins, field_depth))];
40:        }
41:        bytes memory cald = abi.encodePacked(fsig, flatten(ins));
42:        vm.record();
43:        bytes32 fdat;
44:        {
45:            (, bytes memory rdat) = who.staticcall(cald);
46:            fdat = bytesToBytes32(rdat, 32 * field_depth);
47:        }
48:
49:        (bytes32[] memory reads,) = vm.accesses(address(who));
50:        if (reads.length == 1) {
51:            bytes32 curr = vm.load(who, reads[0]);
52:            if (curr == bytes32(0)) {
53:                emit WARNING_UninitedSlot(who, uint256(reads[0]));
54:            }
55:            if (fdat != curr) {
56:                require(
57:                    false,
58:                    "stdStorage find(StdStorage): Packed slot. This would cause dangerous overwriting and currently isn't supported."
59:                );
60:            }
61:            emit SlotFound(who, fsig, keccak256(abi.encodePacked(ins, field_depth)), uint256(reads[0]));
62:            self.slots[who][fsig][keccak256(abi.encodePacked(ins, field_depth))] = uint256(reads[0]);
63:            self.finds[who][fsig][keccak256(abi.encodePacked(ins, field_depth))] = true;
64:        } else if (reads.length > 1) {
65:            for (uint256 i = 0; i < reads.length; i++) {
66:                bytes32 prev = vm.load(who, reads[i]);
67:                if (prev == bytes32(0)) {
68:                    emit WARNING_UninitedSlot(who, uint256(reads[i]));
69:                }
70:                // store
71:                vm.store(who, reads[i], bytes32(hex"1337"));
72:                bool success;
73:                bytes memory rdat;
74:                {
75:                    (success, rdat) = who.staticcall(cald);
76:                    fdat = bytesToBytes32(rdat, 32 * field_depth);
77:                }
78:
79:                if (success && fdat == bytes32(hex"1337")) {
80:                    // we found which of the slots is the actual one
81:                    emit SlotFound(who, fsig, keccak256(abi.encodePacked(ins, field_depth)), uint256(reads[i]));
82:                    self.slots[who][fsig][keccak256(abi.encodePacked(ins, field_depth))] = uint256(reads[i]);
83:                    self.finds[who][fsig][keccak256(abi.encodePacked(ins, field_depth))] = true;
84:                    vm.store(who, reads[i], prev);
85:                    break;
86:                }
87:                vm.store(who, reads[i], prev);
88:            }
89:        } else {
90:            revert("stdStorage find(StdStorage): No storage use detected for target.");
91:        }
92:
93:        require(
94:            self.finds[who][fsig][keccak256(abi.encodePacked(ins, field_depth))],
95:            "stdStorage find(StdStorage): Slot(s) not found."
96:        );
97:
98:        delete self._target;
99:        delete self._sig;
100:        delete self._keys;
101:        delete self._depth;
102:
103:        return self.slots[who][fsig][keccak256(abi.encodePacked(ins, field_depth))];
104:    }
105:
106:    function target(StdStorage storage self, address _target) internal returns (StdStorage storage) {
107:        self._target = _target;
108:        return self;
109:    }
110:
111:    function sig(StdStorage storage self, bytes4 _sig) internal returns (StdStorage storage) {
112:        self._sig = _sig;
113:        return self;
114:    }
115:
116:    function sig(StdStorage storage self, string memory _sig) internal returns (StdStorage storage) {
117:        self._sig = sigs(_sig);
118:        return self;
119:    }
120:
121:    function with_key(StdStorage storage self, address who) internal returns (StdStorage storage) {
122:        self._keys.push(bytes32(uint256(uint160(who))));
123:        return self;
124:    }
125:
126:    function with_key(StdStorage storage self, uint256 amt) internal returns (StdStorage storage) {
127:        self._keys.push(bytes32(amt));
128:        return self;
129:    }
130:
131:    function with_key(StdStorage storage self, bytes32 key) internal returns (StdStorage storage) {
132:        self._keys.push(key);
133:        return self;
134:    }
135:
136:    function depth(StdStorage storage self, uint256 _depth) internal returns (StdStorage storage) {
137:        self._depth = _depth;
138:        return self;
139:    }
140:
141:    function read(StdStorage storage self) private returns (bytes memory) {
142:        address t = self._target;
143:        uint256 s = find(self);
144:        return abi.encode(vm.load(t, bytes32(s)));
145:    }
146:
147:    function read_bytes32(StdStorage storage self) internal returns (bytes32) {
148:        return abi.decode(read(self), (bytes32));
149:    }
150:
151:    function read_bool(StdStorage storage self) internal returns (bool) {
152:        int256 v = read_int(self);
153:        if (v == 0) return false;
154:        if (v == 1) return true;
155:        revert("stdStorage read_bool(StdStorage): Cannot decode. Make sure you are reading a bool.");
156:    }
157:
158:    function read_address(StdStorage storage self) internal returns (address) {
159:        return abi.decode(read(self), (address));
160:    }
161:
162:    function read_uint(StdStorage storage self) internal returns (uint256) {
163:        return abi.decode(read(self), (uint256));
164:    }
165:
166:    function read_int(StdStorage storage self) internal returns (int256) {
167:        return abi.decode(read(self), (int256));
168:    }
169:
170:    function bytesToBytes32(bytes memory b, uint256 offset) private pure returns (bytes32) {
171:        bytes32 out;
172:
173:        uint256 max = b.length > 32 ? 32 : b.length;
174:        for (uint256 i = 0; i < max; i++) {
175:            out |= bytes32(b[offset + i] & 0xFF) >> (i * 8);
176:        }
177:        return out;
178:    }
179:
180:    function flatten(bytes32[] memory b) private pure returns (bytes memory) {
181:        bytes memory result = new bytes(b.length * 32);
182:        for (uint256 i = 0; i < b.length; i++) {
183:            bytes32 k = b[i];
184:            /// @solidity memory-safe-assembly
185:            assembly {
186:                mstore(add(result, add(32, mul(32, i))), k)
187:            }
188:        }
189:
190:        return result;
191:    }
192:}
193:
``` 



[File:StdStorage.sol#L195](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-brahma/contracts/lib/forge-std/src/StdStorage.sol#L195) 
```solidity
``` 



[File:console.sol#L4](https://github.com/0xKitsune/sstan/blob/main/bin/scope/2023-10-wildcat/lib/openzeppelin-contracts/lib/forge-std/src/console.sol#L4) 
```solidity
``` 



 --- 


