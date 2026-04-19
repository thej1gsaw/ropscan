# ropscan

A Rust-based linear disassembly scanner that identifies and formats potential ROP (Return-Oriented Programming) gadgets ending in `ret`.

## Overview

`ropscan` processes 64-bit ELF binaries to locate sequences of instructions that terminate with a return. Unlike brute-force scanners, this tool uses **Linear Disassembly**, meaning it follows the intended logical flow of the program as defined by the compiler.

## Features

- **Linear Scanning**: Uses the `Capstone` engine (Intel syntax) to disassemble the `.text` section.
- **Fixed-Window Extraction**: When a `ret` instruction is encountered, the tool automatically captures the 5 preceding instructions.
- **Clean Formatting**: 
  - Removes the raw hex opcode bytes.
  - Collapses the large whitespace padding between mnemonics and operands.
  - Joins instructions with `; ` for a clean, single-line representation.
- **Address Mapping**: Prefixes every gadget with the memory address of the starting instruction.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- `capstone` crate and its system dependencies

## Usage

Run the scanner by providing the path to your target binary:

```bash
cargo run -- /path/to/binary
