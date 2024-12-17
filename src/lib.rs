/*!
# Advent-of-Utils

## Introduction
`Advent of Utils` helps you solving your [Advent of Code](https://adventofcode.com/) challenges. Not by implementing the solutions for you, but by handling the boilerplate work so you can focus on solving the puzzles.

## Table of Contents
- [Features](#features)
- [Setup and Usage](#setup-and-usage)
- [Implementation Guide](#implementation-guide)
- [CLI Reference](#cli-reference)
- [Disclaimer](#disclaimer)

## Features
- 🚀 Automatic input fetching with local caching
- 📊 Built-in performance benchmarking
- 🔧 Simple macro-based setup
- 💾 Session token management

## Setup

### Prerequisites
1. An Advent of Code account and session token
2. Rust toolchain installed

### Installation
Add this to your `Cargo.toml`:
```toml
[dependencies]
advent-of-utils = "0.1.0"  # Replace with actual version

[lib]
crate-type = ["cdylib"]
```

### Environment Setup

Set your Advent of Code session token:

```bash
# Linux/MacOS
export AOC_SESSION=your_session_token

# Windows (PowerShell)
$env:AOC_SESSION="your_session_token"
```

You will find your session token in the cookies on the [Advent of Code](https://adventofcode.com/) page when you are logged in.

## CLI Reference

### Available Commands

```bash
# Run a specific day's solution
aou run <YEAR> <DAY>

# Run all implemented solutions
aou run <YEAR>

# Show help
aou --help
```
*/
#![warn(missing_docs)]
mod options;
mod solution;

pub use aou_macro::add_days;
pub use options::AocOption;
pub use solution::Solution;
