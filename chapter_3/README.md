# Chapter 3: Common Programming Concepts

Based on [Chapter 3 of The Rust Programming Language](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html).

## 1. Tempreature Converter

Convert a temperature from Fahrenheit to Celsius and back. Read the temperature from
user input, convert it with `(f - 32) * 5 / 9` and `c * 9 / 5 + 32`, and print both
results. Use functions and handle bad input with `match`.

```bash
cd temp_converter
cargo run
```