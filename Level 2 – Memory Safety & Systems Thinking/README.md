# Level 2: Memory Safety & Systems Thinking

This level focuses on Rust's memory management system.

## Project List

### 1. Ownership Basics (01_ownership_intro)
- Concept: Understanding Ownership rules.
- Key Takeaway: The difference between Moving ownership and Borrowing via references.

### 2. Mutability Logic (02_mutability)
- Concept: Variable states.
- Key Takeaway: Using 'let mut' for changes and Shadowing to transform data types.

### 3. The Text Cutter (03_mutable_references)
- Concept: Modifying data through references.
- Key Takeaway: Using Mutable References (&mut) to modify data in-place without cloning.

### 4. Slices Logic (04_slices)
- Concept: Accessing partial data without copying.
- Key Takeaway: Using String Slices (&str) and range syntax to view specific parts of memory.

### 5. The Package Builder (05_structs)
- Concept: Grouping related data and adding behavior.
- Key Takeaway: Using `struct` for data layout and `impl` blocks to define methods and constructors.

### 6. RPG Class System (06_rpg_class)
- Concept: Combining Structs and Enums.
- Key Takeaway: Using `match` to handle different behaviors based on the Enum variant stored inside a Struct.

### 7. Bank Account Logic (07_bank_account)
- Concept: State management and Logic.
- Key Takeaway: Using `&mut self` to modify data and `if/else` inside methods to prevent invalid operations (like negative balance).

### 8. The Log Collector (08_log_collector)
- Concept: Managing lists of data (Vectors).
- Key Takeaway: Using `Vec::push` to store multiple items and `for` loops to iterate through them.