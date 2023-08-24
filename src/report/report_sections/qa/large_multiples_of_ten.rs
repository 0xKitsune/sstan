pub fn report_section_content() -> String {
    String::from(
        r##" 
### Large multiples of ten should be denoted in scientific notation.
Ex:

Bad
```js
uint256 x = 100000;
```

Good
```js
uint256 x = 1e5;
```
    "##,
    )
}
