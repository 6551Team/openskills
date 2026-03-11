# OpenTrade CLI

Command-line interface for OpenTrade API - multi-chain DEX aggregator, token search, market data, and portfolio management.

## Installation

### Quick Install (macOS/Linux)

```bash
curl -sSL https://raw.githubusercontent.com/yourusername/openskills/main/skills/opentrade/cli/install.sh | sh
```

### Manual Install

```bash
# Clone the repository
git clone https://github.com/yourusername/openskills.git
cd openskills/skills/opentrade/cli

# Build and install
cargo build --release
sudo cp target/release/opentrade /usr/local/bin/

# Verify installation
opentrade --version
```

## Prerequisites

Get your API token at: https://6551.io/mcp

Set environment variable:
```bash
export OPEN_TOKEN="your-token-here"
```

Or create a config file:
- macOS/Linux: `~/.config/openskills/credentials.json`
- Windows: `%APPDATA%\openskills\credentials.json`

Config file format:
```json
{
  "token": "your-token-here"
}
```

## Usage

### Global Options

```bash
opentrade [OPTIONS] <COMMAND>

Options:
  --trader <TRADER>      Trading router (default: okx)
  --api <VERSION>        API version (default: v1)
  --base-url <URL>       Backend service URL (default: https://ai.6551.io)
  --chain <CHAIN>        Chain name (ethereum, solana, xlayer, etc.)
  -o, --output <FORMAT>  Output format: json, table (default: json)
  -V, --version          Show version information
  -h, --help             Print help
```

### DEX Swap Commands

```bash
# Get swap quote
opentrade swap quote \
  --from 0x74b7f16337b8972027f6196a17a631ac6de26d22 \
  --to 0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee \
  --amount 100000000 \
  --chain xlayer

# Get swap transaction
opentrade swap swap \
  --from 0x74b7f16337b8972027f6196a17a631ac6de26d22 \
  --to 0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee \
  --amount 100000000 \
  --chain xlayer \
  --wallet 0xYourWallet \
  --slippage 1

# Get approval transaction
opentrade swap approve \
  --token 0x74b7f16337b8972027f6196a17a631ac6de26d22 \
  --amount 100000000 \
  --chain xlayer

# List supported chains
opentrade swap chains

# List liquidity sources
opentrade swap liquidity --chain xlayer
```

### Token Commands

```bash
# Search tokens
opentrade token search BONK --chains solana

# Get token holders
opentrade token holders <address> --chain solana

# Get trending tokens
opentrade token trending --chains 1,501 --sort-by 5 --time-frame 4
```

### Market Commands

```bash
# Get token price
opentrade market price <address> --chain ethereum

# Get K-line chart data
opentrade market candles <address> --chain ethereum --period 1h

# Get market overview
opentrade market overview --chain ethereum
```

### Portfolio Commands

```bash
# Get wallet balance
opentrade portfolio balance <address> --chain ethereum

# Get portfolio value
opentrade portfolio value <address>

# Get transaction history
opentrade portfolio history <address> --chain ethereum
```

### Gateway Commands

```bash
# Broadcast transaction
opentrade gateway broadcast --signed-tx <tx> --address <addr> --chain ethereum

# Query transaction status
opentrade gateway query --tx-hash <hash> --chain ethereum
```

## Chain Names

| Chain | Name | chainIndex |
|---|---|---|
| XLayer | `xlayer` | `196` |
| Solana | `solana` | `501` |
| Ethereum | `ethereum` | `1` |
| Base | `base` | `8453` |
| BSC | `bsc` | `56` |
| Arbitrum | `arbitrum` | `42161` |
| Polygon | `polygon` | `137` |
| Optimism | `optimism` | `10` |
| Avalanche | `avalanche` | `43114` |

## Examples

### Full Swap Workflow

```bash
# 1. Search for token
opentrade token search USDC --chains xlayer

# 2. Get quote
opentrade swap quote \
  --from 0x74b7f16337b8972027f6196a17a631ac6de26d22 \
  --to 0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee \
  --amount 100000000 \
  --chain xlayer

# 3. Get approval (if needed for ERC-20)
opentrade swap approve \
  --token 0x74b7f16337b8972027f6196a17a631ac6de26d22 \
  --amount 100000000 \
  --chain xlayer

# 4. Broadcast approval (user signs first)
opentrade gateway broadcast \
  --signed-tx <signed_approval_tx> \
  --address <your_address> \
  --chain xlayer

# 5. Get swap transaction
opentrade swap swap \
  --from 0x74b7f16337b8972027f6196a17a631ac6de26d22 \
  --to 0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee \
  --amount 100000000 \
  --chain xlayer \
  --wallet <your_address> \
  --slippage 1

# 6. Broadcast swap (user signs first)
opentrade gateway broadcast \
  --signed-tx <signed_swap_tx> \
  --address <your_address> \
  --chain xlayer
```

## Development

```bash
# Build
cargo build

# Run tests
cargo test

# Build release
cargo build --release

# Run with debug output
RUST_LOG=debug opentrade swap quote --from ... --to ... --amount ... --chain xlayer
```

## License

Apache-2.0
