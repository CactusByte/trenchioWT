# Solana Wallet Tracker

A real-time monitoring tool that tracks Solana wallet transactions and interactions with designated programs using WebSocket connections.

## Features

- Real-time transaction monitoring via WebSocket connection
- Track multiple wallet addresses simultaneously
- Program-specific transaction filtering
- Configurable RPC endpoint support
- Asynchronous processing using Tokio

## Prerequisites

- Rust 1.70 or higher
- Cargo package manager
- Access to a Solana RPC WebSocket endpoint (e.g., Helius, QuickNode, or Solana's public RPC)

## Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/solana-wallet-tracker
cd solana-wallet-tracker
```

2. Build the project:
```bash
cargo build --release
```

## Configuration

Create a `.env` file in the project root with the following variables:
```env
RPC_WS_URL=wss://your-rpc-endpoint.com
PROGRAM_ID=your_program_id_here
WALLET_ADDRESSES=wallet1,wallet2,wallet3
```

## Usage

Run the tracker:
```bash
cargo run --release
```

The program will:
1. Connect to the configured WebSocket endpoint
2. Monitor transactions involving the specified program
3. Print notifications when tracked wallets interact with the program

## Example Output

```
[INFO] Connected to WebSocket endpoint
[INFO] Transaction detected: 5KtPn1LGuxhFqnmqX6hs4BJxZfPVqQEzNnbwnZPS6kpQ
[INFO] Wallet HxFLKUAmAMLz1jtT3hbvCMELwH5H9tpM2QugP8sKyfhc interacted with program
```

## Error Handling

The application includes robust error handling for:
- WebSocket connection issues
- Invalid wallet addresses
- RPC endpoint failures
- Transaction parsing errors

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Disclaimer

This tool is for educational purposes. Always ensure you're following Solana RPC providers' terms of service and rate limits.
