pub fn report_section_content() -> String {
    String::from(
        r##" 
### Ensure non zero data when initializing storage variables in the constructor.

Ex.

Incorrect
```js
contract A {
    address owner;
    constructor(address _owner) {
        owner = _owner;
    }
}
```

Correct
```js
contract A {
    address owner;
    constructor(address _owner) {
        require(_owner != address(0), "Owner cannot be the zero address");
        owner = _owner;
    }
}
```    "##,
    )
}
