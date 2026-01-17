# Level 1 – Fundamentals & Logic Building

This level covers my first steps with Rust.

The focus here is not writing perfect code, but understanding how Rust thinks.  
Most projects are small, but each one introduces a new concept or reinforces a previous one through practice.

I intentionally wrote everything myself instead of copying solutions.  
Mistakes are part of this level, and that is by design.

---

## Projects

**01_hello_world** Basic setup and my first Rust program using Cargo.

**02_basic_types** Introduction to scalar types such as integers, floats, booleans, and characters.  
Also explored variable mutability using `mut`.

**03_math_operations** Worked with arithmetic operations and basic mathematical logic.  
Learned the difference between integer division and float division.  
Practiced type casting using the `as` keyword.

**04_control_flow** Used `if`, `else if`, and `else` for decision making.  
Learned that `if` can be used as an expression in Rust.  
Also experimented with multiple binaries inside `src/bin` using small real-life scenarios.

**05_loops** Practiced iteration using `loop`, `while`, and `for`.  
Learned range syntax (`..` and `..=`).  
Applied loops to simple real-world inspired simulations.

**06_user_input** Learned how to handle standard input using `std::io`.  
Built a full input transformation pipeline: reading, trimming, and parsing.  
Converted user input into integers and floats safely.

**07_guessing_game** Built a complete CLI guessing game.  
Used the `rand` crate for random number generation.  
Combined loops, input parsing, and conditionals into a single flow.  
Replaced `expect` with `match` to prevent crashes on invalid input.

**08_unit_converter** Created a CLI temperature conversion tool (Celsius ↔ Fahrenheit).  
Used `f64` for precise calculations.  
Implemented menu logic with loops and safe input handling.

**09_atm_simulator (Capstone – Module 1)** Developed a CLI-based ATM simulation.  
Focused on state persistence, deposits, withdrawals, and balance checks.  
Used infinite loops to keep the program running continuously.

**10_dungeon_battle (Logic Challenge)** Built a turn-based combat simulation.  
Managed multiple states such as player health and enemy health.  
Used randomness for damage, healing, and escape mechanics.  
Handled win, loss, and escape conditions correctly.

**11_mars_survival (Strategy Simulation)** Created a resource management survival simulation.  
Tracked energy, oxygen, and water dynamically.  
Used `match` for clean menu handling.  
Added random disaster events that affect the game state.

**12_warehouse_manager (Logistics Simulation)** Developed an inventory and warehouse management system.  
Prevented overstocking using logical constraints.  
Calculated profits using type casting and margins.  
Used constants for fixed limits like warehouse capacity.

**13_crypto_trader (Asset Management Simulation)** Built a crypto trading simulation with market volatility.  
Tracked both fiat and crypto assets.  
Calculated total net worth dynamically.  
Implemented a profit target as a win condition.

**14_structs_and_enums** Introduced custom data modeling using structs.  
Learned how enums restrict possible values.  
Used `match` for pattern matching.  
Practiced debug printing with `#[derive(Debug)]`.

**15_vectors_intro** Learned the basics of dynamic collections with `Vec<T>`.  
Understood the difference between vectors and fixed-size arrays.  
Practiced pushing, popping, and indexing elements.

**16_vectors_deep_dive** Iterated over vectors using references to preserve ownership.  
Modified elements using mutable references.  
Replaced unsafe indexing with `.get()` and handled `Option<T>`.  
Safely handled empty vector cases using `match`.

**17_personal_library** Combined structs and vectors to build a small library system.  
Implemented basic CRUD operations.  
Built an interactive CLI loop.  
Added bounds checking to prevent crashes.

**18_order_manager** Used enums to manage fixed order states.  
Embedded enums inside structs.  
Updated order status dynamically by ID.  
Refactored repeated input logic into a helper function.

**19_rpg_inventory** Built an RPG-style inventory system.  
Combined enums, structs, and vectors into a single model.  
Implemented full item lifecycle management.  
Reused helper functions for cleaner and more modular code.

---

## Notes

Code in this level may be revisited or refactored later.  
The goal here is learning through experimentation, not perfection.