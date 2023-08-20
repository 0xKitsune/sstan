pub fn report_section_content() -> String {
    String::from(
        r##"
Incorrect assembly shift math
Ex. x << 5.
#### Incorrect
```js
assembly {
    shl(x, 5)
}
```
#### Correct
```js
assembly {
    shl(5,x)
}
```


"##,
    )
}
