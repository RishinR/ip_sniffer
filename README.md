# Multi-threaded IP Sniffer

This is a multi-threaded command-line IP sniffer written in Rust. It scans an IP address for open ports using multiple threads to improve efficiency.

## Features
- Scans all 65,535 ports of a given IP address
- Supports multi-threading to speed up the scanning process
- Provides a simple CLI interface with customizable thread count

## Usage

### Basic Usage
To sniff an IP address with the default number of threads (4):
```sh
cargo run -- <IP_ADDRESS>
```
Example:
```sh
cargo run -- 192.168.1.1
```

### Customizing the Number of Threads
You can specify the number of threads using the `-j` flag:
```sh
cargo run -- -j <THREAD_COUNT> <IP_ADDRESS>
```
Example:
```sh
cargo run -- -j 10 192.168.1.1
```

### Help Message
To display the help message:
```sh
cargo run -- -h
```
OR
```sh
cargo run -- -help
```

## Installation
1. Ensure you have Rust installed. If not, install it via [rustup](https://rustup.rs/):
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. Clone this repository:
   ```sh
   git clone https://github.com/RishinR/ip_sniffer.git
   cd ip_sniffer
   ```
3. Build the project:
   ```sh
   cargo build --release
   ```

## How It Works
- The program takes an IP address as input.
- It creates multiple threads, each scanning different parts of the port range.
- If a port is open, it prints the port number.

## Example Output
```sh
cargo run -- 192.168.1.1
.
.
22 is open
80 is open
443 is open
```

## License
This project is open-source and licensed under the MIT License.

## Contributions
Contributions are welcome! Feel free to submit a pull request or open an issue.

