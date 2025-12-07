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

- **09_atm_simulator** (CAPSTONE PROJECT - MODULE 1)

  - Developed a persistent CLI-based financial transaction simulation.
  - **Core Concepts:** State persistence, variable scoping, and input validation pipelines.
  - Implemented business logic for deposits, withdrawals, and insufficient fund checks.
  - Utilized compound assignment operators and infinite loops for continuous runtime.

- **10_dungeon_battle** (LOGIC CHALLENGE)

  - Developed a turn-based combat simulation (RPG style).
  - **Key Features:**
    - **Multi-State Management:** Tracking player and enemy health simultaneously.
    - **RNG Implementation:** Randomized damage, healing, and escape mechanics.
    - **Game Loop:** Implemented a sequential turn system (Player Turn -> Enemy Turn).
    - **Logic Gates:** Handling death conditions, health caps, and successful escapes properly.

- **11_mars_survival** (STRATEGY SIMULATION)

  - Developed a resource management survival game engine.
  - **Key Features:**
    - **Resource Logic:** Balancing Energy, Oxygen, and Water variables dynamically.
    - **Control Flow:** Utilized `match` expression for cleaner and safer menu handling.
    - **Error Correction:** Implemented logic to prevent state progression (day skip) on invalid inputs.
    - **RNG Events:** Random disaster generation (Storms, Radiation) affecting game state.

- **12_warehouse_manager** (LOGISTICS SIMULATION)

  - Developed a warehouse inventory management system.
  - **Key Features:**
    - **Capacity Management:** Logic gates to prevent overstocking (`stock <= capacity`).
    - **Financial Logic:** Profit calculation using type casting (`i32` to `f64`) and margin application.
    - **State Persistence:** Tracking stock levels and cash flow across business cycles.
    - **Constants:** Utilized `const` for defining immutable constraints like Warehouse Capacity.

- **13_crypto_trader** (ASSET MANAGEMENT SIMULATION)

  - Developed a financial trading bot simulation with real-time volatility.
  - **Key Features:**
    - **Portfolio Logic:** Tracking two different asset types (Fiat/Crypto) simultaneously.
    - **Market Volatility:** Simulating price fluctuations using `rand` range generation.
    - **Wealth Calculation:** Dynamic calculation of total net worth based on current market price.
    - **Win Condition:** Implemented a profit target threshold (`>=`) logic.

- **14_structs_and_enums**

  - Introduction to **Data Modeling**: Defining custom types using `struct` (Player, Car).
  - Understanding **Enumerations**: Restricting options using `enum` (Direction, Color).
  - **Pattern Matching:** Using `match` to handle Enum variants logically.
  - **Debug Printing:** Learned `#[derive(Debug)]` and `{:?}` syntax for displaying custom types.
  - **Multi-Binary:** Created a secondary simulation (`garage.rs`) to reinforce concepts.    

## Goals

- Master memory safety concepts (Ownership, Borrowing).
- Build CLI tools and system utilities.
- Explore Web3 and blockchain development with Rust.
- Develop high-performance Backend APIs.
- **Long-term Vision:** Full Stack Development + Rust + Web3 integration.

## Tools

- **Language:** Rust
- **Package Manager:** Cargo
- **IDE:** VS Code
- **VCS:** Git & GitHub

---

Created by **Emre Esim**
