# Port Sniffer

## Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) must be installed on your system.

## How to Run

1. Clone the repository:

2. Open a terminal in the project directory and run:

   - To view usage/help:
     ```bash
     cargo run -- -h
     ```

   - To run the program with custom options:
     ```bash
     cargo run -- -j <Number of Threads> <Your IP Address>
     ```

   Replace `<Number of Threads>` with how many threads you want to use, and `<Your IP Address>` with the target IP.