# Rust IPC Channel Example

A Rust project demonstrating bidirectional inter-process communication (IPC) using the `ipc-channel` library. This project shows how to establish communication between a parent and child process using IPC channels.

## Overview

This project consists of two main components:
- **Parent Process** (`parent.rs`): The main process that spawns a child process and establishes IPC communication
- **Child Process** (`child.rs`): The spawned process that communicates back to the parent

The communication is bidirectional, allowing both processes to send and receive messages from each other.

## Features

- **Bidirectional Communication**: Both parent and child processes can send and receive messages
- **IPC Channel**: Uses the `ipc-channel` library for efficient inter-process communication
- **Process Management**: Parent process spawns and manages the child process lifecycle
- **Error Handling**: Includes proper error handling for IPC operations

## Dependencies

- `ipc-channel = "0.20"` - Provides IPC communication capabilities

## Project Structure

```
rust_ipc/
├── Cargo.toml          # Project configuration and dependencies
├── src/
│   ├── parent.rs       # Parent process implementation
│   └── child.rs        # Child process implementation
└── README.md           # This file
```

## How It Works

### 1. Parent Process Flow
1. Creates an IPC one-shot server to establish communication
2. Spawns a child process using `cargo run --bin child`
3. Passes the server name to the child as a command-line argument
4. Accepts the connection from the child to establish bidirectional channels
5. Sends a series of messages to the child and waits for responses
6. Sends a "quit" message to terminate the child process
7. Waits for the child process to complete and prints its output

### 2. Child Process Flow
1. Parses the server name from command-line arguments
2. Creates two IPC channels for bidirectional communication
3. Connects to the parent's one-shot server
4. Sends the channel handles to the parent
5. Enters a message loop to receive and respond to parent messages
6. Exits when it receives the "quit" message

### 3. Communication Protocol
- **Parent → Child**: Sends string messages
- **Child → Parent**: Responds with acknowledgment messages in the format "Child received: {original_message}"
- **Termination**: Parent sends "quit" to signal child termination

## Usage

### Running the Project

To run the parent process (which will automatically spawn the child):

```bash
cargo run --bin parent
```

### Running Individual Binaries

You can also run the binaries individually:

```bash
# Run parent process
cargo run --bin parent

# Run child process (requires channel name argument)
cargo run --bin child channel_name:your_channel_name
```

## Example Output

```
Parent process started
One Shot Server name - Parent: /tmp/rust_ipc_12345
Child process ID: 67890
Child process started
One Shot Server name: /tmp/rust_ipc_12345
Received from child: Child received: Hello from parent
Received from child: Child received: How are you?
Received from child: Child received: Goodbye!
Child process exiting.
Child process finished.
--------------------- stdout: Child process started
One Shot Server name: /tmp/rust_ipc_12345
Received from parent: Hello from parent
Received from parent: How are you?
Received from parent: Goodbye!
Received from parent: quit
Child process exiting.
Child process finished.
-----------------------------------------
```

## Code Structure

### Parent Process (`parent.rs`)
- Creates an `IpcOneShotServer` for initial handshake
- Spawns child process with server name as argument
- Establishes bidirectional communication channels
- Manages message exchange and process lifecycle

### Child Process (`child.rs`)
- Parses command-line arguments for server name
- Creates IPC channels for communication
- Connects to parent's one-shot server
- Implements message handling loop with proper termination

## Key Concepts

### IPC Channel Types Used
- **IpcOneShotServer**: Used for initial connection establishment
- **IpcSender**: For sending messages between processes
- **IpcReceiver**: For receiving messages from other processes
- **channel()**: Creates a pair of sender/receiver for bidirectional communication

### Error Handling
The project includes error handling for:
- IPC connection failures
- Message sending/receiving errors
- Process spawning failures
- Command-line argument parsing

## Build and Test

```bash
# Build the project
cargo build

# Run tests (if any)
cargo test

# Run the parent process
cargo run --bin parent
```

## Use Cases

This boilerplate code can be extended for various IPC scenarios:
- Microservices communication
- Parent-child process coordination
- Distributed task processing
- Plugin architectures
- Process isolation with communication needs

## License

This project is provided as-is for educational and demonstration purposes.
