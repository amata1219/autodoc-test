# README.md

# Fibonacci Generator

This repository contains code in the Rust programming language for a Fibonacci number generator.

# File Structure

This repository only contains one Rust file:

- `main.rs`: This file has functions to generate the Fibonacci number.

# Code Overview

## `main.rs`
This rust source file contains two main functions:

1. `fn fibonacci(n: u32) -> u64`:
   - This function accepts a single parameter, `n` (an unsigned 32-bit integer), and returns an unsigned 64-bit integer.
   - The purpose of this function is to generate Fibonacci sequence and then return the "nth" number in the Fibonacci series, where `n` is an argument.
   - It first checks if `n` is 0 or 1 and returns the same value if it is.
   - It then initializes two mutable variables `a` and `b` to 0 and 1 respectively.
   - Later, it runs a for loop from 2 to "nth" number and keeps resetting `a` and `b` with new values in sequence.
   - Finally, returns the value at `b` as the "nth" value in the Fibonacci sequence.

2. `fn main()`:
   - The main function is the entry point for the program.
   - It first sets `n` equal to 20.
   - It then calls the `fibonacci` function, passing `n` as the argument, and prints the result in a formatted string.

# How to Run

- Make sure you have Rust installed on your machine. If you don't, please follow the guide [here](https://www.rust-lang.org/tools/install) to install rust.
- To run the program, navigate to the directory where `main.rs` is located.
- Run the `rustc main.rs` command to compile the code.
- Use `./main` to execute the compiled code.

# Outputs

The program will output the 20th number in the Fibonacci sequence.

# Support 

For any issues or improvements, please open an issue on GitHub.

# Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.