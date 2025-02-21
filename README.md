# Fault-Injection-Env-for-Redundant-Application
Project for the System Programming Course, regarding a fault injection environment for redundant application/

# Fault Injection Environment for Redundant Applications

## Table of Contents
- [Introduction](#introduction)
- [Project Overview](#project-overview)
- [Code Transformation for Fault Tolerance](#code-transformation-for-fault-tolerance)
- [Fault Injection Environment](#fault-injection-environment)
- [Usage](#usage)
- [Project Structure](#project-structure)
- [Results and Analysis](#results-and-analysis)
- [Future Improvements](#future-improvements)
- [Authors](#authors)

## Introduction
Fault tolerance is a critical aspect of safety-critical applications. Traditional reliability metrics such as MTBF (Mean Time Between Failures) require long observation periods, making it difficult to assess system dependability efficiently. This project introduces a software-based fault injection environment to evaluate the robustness of redundant applications using the Rust programming language.

## Project Overview
The project aims to:
1. Implement a **fault-tolerant application** by systematically modifying Rust source code to introduce redundancy.
2. Develop a **fault injection environment** to test the effectiveness of these transformations by simulating faults.

This approach is based on the **single bit-flip fault model**, commonly used in software fault injection research.

## Code Transformation for Fault Tolerance
To introduce redundancy into the application, the following transformations are applied:

- **Rule #1:** Each variable `x` is duplicated into `cp1` and `cp2`.
- **Rule #2:** All write operations are performed on both copies.
- **Rule #3:** Before any read operation, the consistency between `cp1` and `cp2` is checked. If a mismatch is found, an error is raised.

This logic is encapsulated in a custom Rust generic type `Hardened<T>`, which includes various traits to support arithmetic operations, comparisons, and error handling.

## Fault Injection Environment
The fault injection system follows a **Pipes and Filters** architecture, consisting of:
1. **Fault List Manager (FLM):** Generates a fault list containing variable names, injection times, and flipped bits.
2. **Injector (FIM):** Introduces faults in the application during execution.
3. **Analyzer:** Collects execution data, detects errors, and categorizes them into silent and detected faults.

The system supports concurrent execution using **multi-threading** with Rust’s `mpsc` (multiple producer, single consumer) channels for inter-component communication.

## Usage
### Prerequisites
- Rust compiler
- Cargo package manager

### Running the Application
```sh
# Clone the repository
git clone <repository-url>
cd <project-directory>

# Build and run the fault-tolerant application
cargo run --release

# Execute the fault injection environment
cargo run --bin fault_injector

# Analyze results
cat results/report.json
```

## Project Structure
```
├── src/
│   ├── hardened.rs         # Implementation of Hardened<T>
│   ├── fault_list.rs       # Fault List Manager
│   ├── injector.rs         # Fault Injector
│   ├── analyzer.rs         # Result Analyzer
│   ├── main.rs             # Entry point
├── data/
│   ├── dataset.txt         # Predefined datasets
│   ├── input.txt           # User-defined inputs
├── results/
│   ├── report.json         # Experiment results
├── Cargo.toml              # Rust dependencies and project metadata
└── README.md               # Project documentation
```

## Results and Analysis
- The fault injection experiments were conducted on three algorithms:
  - Selection Sort
  - Bubble Sort
  - Matrix Multiplication
- The analysis categorized faults as:
  - **Silent Faults:** Undetected faults that did not affect output.
  - **Detected Faults:** Errors identified by the system.
  - **Fatal Faults:** Errors leading to incorrect results.

### Sample Results
```
Total Faults Injected: 2000
Detected Faults: 1490 (74.5%)
Silent Faults: 510 (25.5%)
Fatal Faults: 251 (12.55%)
```

## Future Improvements
- **Optimization of fault list generation** to reduce redundant fault injections.
- **Expansion of fault models** beyond single bit-flip faults.
- **Integration with machine learning** for predictive fault analysis.

## Authors
- **Carlo Migliaccio**
- **Federico Pretini**
- **Alessandro Scavone**
- **Mattia Viglino**

Project developed at **Politecnico di Torino** for the **Programmazione di Sistema** course (2024/25).


