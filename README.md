# Rust Learning Journey

This repository documents my progress in learning the Rust programming language. It contains various projects, exercises, and experiments ranging from basic syntax to advanced concepts.

## Structure

Each project is organized into its own directory for clarity.

- **01_hello_world**
  - Basic setup and first Rust program using Cargo.
- **02_basic_types**
  - Introduction to scalar types: Integers (`i32`), Floats (`f64`), Booleans, and Characters.
  - Understanding variable mutability with `mut`.
- **03_math_operations**

  - Arithmetic operations and mathematical logic.
  - Understanding **Integer Division** vs Float Division.
  - **Type Casting:** Converting types using the `as` keyword.

- **04_control_flow**

  - Decision making with `if`, `else if`, and `else`.
  - Using `if` as an expression in `let` statements (Rust specific feature).
  - Managing **Multiple Binaries** in `src/bin` (Scenarios: Exam Pass, Savings Goal, Game Store).

- **05_loops**

  - Mastering iteration logic: `loop`, `while`, and `for`.
  - Using range syntax (`..` and `..=`).
  - Real-world scenarios: Server Retry, Battery Life simulation, and NFT Minting logic.

- **06_user_input**

  - Handling Standard Input (`std::io`).
  - The Data Transformation Pipeline: `read_line` -> `trim()` -> `parse()`.
  - Converting Strings to Integers (`i32`) and Floats (`f64`).
  - **Real World Scenario:** Logistics Weight Checker logic with price calculation.

- **07_guessing_game**

  - Implemented a complete CLI game loop.
  - Used external crate `rand` for random number generation.
  - Combined `loop`, user input, type parsing, and conditional logic (`if/else`).
  - **Error Handling:** Replaced `expect` with `match` to handle invalid inputs gracefully (Crash-proof).
  - **Status:** Functional & Playable.

- **08_unit_converter**
  - Built a CLI tool for temperature conversion (Celsius <-> Fahrenheit).
  - Implemented mathematical formulas using `f64` precision.
  - Used `loop` for continuous operation and specific `break` points.
  - **Logic:** Menu selection handling and input parsing with safety checks (`match`).

## Goals

- Master memory safety concepts (Ownership, Borrowing).
- Build CLI tools and system utilities.
- Explore Web3 and blockchain development with Rust.
- **Long-term Vision:** Full Stack Development + Rust + Web3 integration.

## Tools

- **Language:** Rust
- **Package Manager:** Cargo
- **IDE:** VS Code
- **VCS:** Git & GitHub

---

Created by **Emre Esim**
