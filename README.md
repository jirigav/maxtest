# ReBoolTest

This repository contains an implementation of an algorithm for randomness testing presented by Chatterjee et al. [1]


## Requirements
You need to have [Rust](https://www.rust-lang.org/tools/install) and Python library [SciPy](https://scipy.org/install/) (version 1.7.0 or newer) installed.

## Run the tool

Compile using `cargo build --release`

Use `./target/release/rebooltest <BLOCK_SIZE> <DATA>` 

The tool divides the provided data into halves, on the first half, it finds a distinguisher producing maximum Z-score and then evaluates the distinguisher on the second half. 

## References
[1] [https://link.springer.com/chapter/10.1007/978-3-031-22912-1_21](https://link.springer.com/chapter/10.1007/978-3-031-22912-1_21)