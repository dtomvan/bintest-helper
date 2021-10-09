[![Crates.io](https://img.shields.io/crates/v/bintest_helper.svg)](https://crates.io/crates/bintest_helper)

## Bintest Helper
Use this in combination with [assert_cmd](https://github.com/assert-rs/assert_cmd).
Example:
```rust
fn main() {
    println!("Hello, World!");
}

#[cfg(test)]
mod tests {
    #[bintest_helper::bintest_helper]
    #[test]
    fn main_test() {
        use assert_cmd::Command;

        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        cmd.assert().stdout("Hello, World!");
    }
}
```
See also: the `examples` directory.

Current version: 0.1.0

Always use this at your own risk.

## License
License: MIT

Copyright 2021 Tom van Dijk

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
