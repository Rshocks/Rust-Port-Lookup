# Port Lookup

## Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) must be installed on your system.

## How to Run

1. Clone the repository:

2. Open a terminal in the project directory and run:

   - To view usage/help:
     ```bash
     cargo run -- -h
     ```

   - Replace `<Your IP Address>` with the target IP:
     ```bash
     cargo run -- -a `<IP Addresss>`
     ```

   - Replace `<Start Port>` with the port you want to start to lookup from:
     ```bash
     cargo run -- -s `<Start Port>`
     ```

   - Replace `<End Port>` with the port you want to end the port search from:
     ```bash
     cargo run -- -e `<End Port>`
     ```

   - To combine all of the above provide a target `<IP Address>` and the port range `<Start Port>` - `<End Port>`:
     ```bash
     cargo run -- -a `<IP Addresss>` -s `<Start Port>` -e `<End Port>`
     ```
