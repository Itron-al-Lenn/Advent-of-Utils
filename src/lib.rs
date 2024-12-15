/*!
This crate aims to help you on your [Advent of Code] journey. Not by solving the problems for you but by helping you with everything else.

[Advent of Code]: https://adventofcode.com/

What does this mean exactly? This program:
- Fetches your personal input and caches them locally
- Runs your solution
- Does a basic benchmark on how fast they are
- Displays your results

# Setup
That the binary is able to access your implementation you have to do the following:

Your crate has to be set up as a dynamic library. For that add the following to your `Crate.toml`:
```toml
[lib]
crate-type = ["cdylib"]
```

Your
*/
#![warn(missing_docs)]
mod options;
mod solution;

pub use aou_macro::add_days;
pub use options::AocOption;
pub use solution::Solution;
