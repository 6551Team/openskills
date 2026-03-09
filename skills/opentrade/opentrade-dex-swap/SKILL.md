---
name: opentrade-dex-swap
description: "This skill should be used when the user asks to 'swap tokens', 'trade OKB for USDC', 'buy tokens', 'sell tokens', 'exchange crypto', 'convert tokens', 'swap SOL for USDC', 'get a swap quote', 'execute a trade', 'find the best swap route', 'cheapest way to swap', 'optimal swap', 'compare swap rates', or mentions swapping, trading, buying, selling, or exchanging tokens on XLayer, Solana, Ethereum, Base, BSC, Arbitrum, Polygon, or any of 20+ supported chains. Aggregates liquidity from 500+ DEX sources for optimal routing and price. Supports slippage control, price impact protection, and cross-DEX route optimization. Do NOT use for general programming questions about swap code, or for analytical questions about historical swap volume."
license: Apache-2.0
metadata:
  author: 6551
  version: "1.0.0"
  homepage: "https://6551.io"
---

# OpenTrade DEX Aggregator CLI

5 commands for multi-chain swap aggregation — quote, approve, and execute.

## Pre-flight Checks

Every time before running any `opentrade` command, always follow these steps in order. Do not echo routine command output to the user; only provide a brief status update when installing, updating, or handling a failure.

1. **Confirm installed**: Run `which opentrade`. If not found, install it:
   ```bash
   curl -sSL https://raw.githubusercontent.com/6551/openskills/main/install.sh | sh
   ```
   If the install script fails, ask the user to install manually following the instructions at: https://github.com/6551/openskills

2. **Check for updates**: Read `~/.opentrade/last_check` and compare it with the current timestamp:
   ```bash
   cached_ts=$(cat ~/.opentrade/last_check 2>/dev/null || true)
   now=$(date +%s)
   ```
   - If `cached_ts` is non-empty and `(now - cached_ts) < 43200` (12 hours), skip the update and proceed.
   - Otherwise (file missing or older than 12 hours), run the installer to check for updates:
     ```bash
     curl -sSL https://raw.githubusercontent.com/6551/openskills/main/install.sh | sh
     ```
     If a newer version is installed, tell the user and suggest updating their opentrade skills from https://github.com/6551/openskills to get the latest features.
3. If any `opentrade` command fails with an unexpected error during this
   session, try reinstalling before giving up:
   ```bash
   curl -sSL https://raw.githubusercontent.com/6551/openskills/main/install.sh | sh
   ```
4. **Authentication**: Get your API token at https://6551.io/mcp

   Set environment variable:
   ```bash
   export OPEN_TOKEN="your_token_here"
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

   Priority: Environment variable > Config file

## Skill Routing

- For token search → use `opentrade-token`
- For market prices → use `opentrade-market`
- For transaction broadcasting → use `opentrade-transaction`
- For wallet balances / portfolio → use `opentrade-wallet`

## Quickstart

### Router Discovery

**IMPORTANT**: If the user has not specified a trading router, you MUST first discover available routers:

```bash
opentrade trade routers
```

**Response format:**
```json
{
  "data": [
    {
      "name": "okx test",
      "router": "okx",
      "version": "v1",
      "quota_reward": 0,
      "is_active": true,
      "created_at": "2026-03-03 21:24:34",
      "updated_at": "2026-03-03 21:24:34"
    }
  ],
  "success": true
}
```

**Usage:**
- Extract `router` and `version` from the response
- Use these values in subsequent commands with `--router` and `--version` flags
- **Default fallback**: If the API returns no data or is empty, use `--router okx --version v1`

**Example:**
- If response contains `"router": "okx"` and `"version": "v1"`, use: `opentrade swap quote --router okx --version v1 ...`
- If response is empty, use: `opentrade swap quote --router okx --version v1 ...`

### EVM Swap (quote → approve → swap)

```bash
# 1. Quote — sell 100 USDC for OKB on XLayer
opentrade swap quote \
  --router okx \
  --version v1 \
  --from 0x74b7f16337b8972027f6196a17a631ac6de26d22 \
  --to 0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee \
  --amount 100000000 \
  --chain xlayer
# → Expected: X.XX OKB, gas fee, price impact

# 2. Approve — ERC-20 tokens need approval before swap (skip for native OKB)
opentrade swap approve \
  --router okx \
  --version v1 \
  --token 0x74b7f16337b8972027f6196a17a631ac6de26d22 \
  --amount 100000000 \
  --chain xlayer
# → Returns approval calldata: sign and broadcast via opentrade-transaction

# 3. Swap
opentrade swap swap \
  --router okx \
  --version v1 \
  --from 0x74b7f16337b8972027f6196a17a631ac6de26d22 \
  --to 0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee \
  --amount 100000000 \
  --chain xlayer \
  --wallet 0xYourWallet \
  --slippage 1
# → Returns tx data → user signs → broadcast via opentrade-transaction
```

### Solana Swap (quote → swap)

```bash
# 1. Quote — sell 1 SOL for USDC on Solana
opentrade swap quote \
  --router okx \
  --version v1 \
  --from 11111111111111111111111111111111 \
  --to EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v \
  --amount 1000000000 \
  --chain solana
# → Expected: X.XX USDC, gas fee, price impact

# 2. Swap (no approval needed on Solana)
opentrade swap swap \
  --router okx \
  --version v1 \
  --from 11111111111111111111111111111111 \
  --to EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v \
  --amount 1000000000 \
  --chain solana \
  --wallet YourSolanaWallet \
  --slippage 1
# → Returns tx data → user signs → broadcast via opentrade-transaction
```

## Command Index

```bash
opentrade trade routers                   # Discover available routers
opentrade swap quote                      # Get swap quote
opentrade swap approve                    # Get ERC-20 approval calldata
opentrade swap swap                       # Get swap transaction data
opentrade swap liquidity                  # List DEX sources for a chain
```

## Commands

### 1. Router Discovery

Discover available trading routers before executing swaps.

```bash
opentrade trade routers
```

**Response:**
```json
{
  "data": [
    {
      "name": "okx test",
      "router": "okx",
      "version": "v1",
      "quota_reward": 0,
      "is_active": true,
      "created_at": "2026-03-03 21:24:34",
      "updated_at": "2026-03-03 21:24:34"
    }
  ],
  "success": true
}
```

**Usage:**
- Always call this first if user hasn't specified a router
- Use `router` and `version` values in subsequent commands
- Default to `okx` / `v1` if response is empty

---

### 2. Get Swap Quote

Get a quote for swapping tokens.

```bash
opentrade swap quote \
  --router <router> \
  --version <version> \
  --from <token_address> \
  --to <token_address> \
  --amount <amount_in_minimal_units> \
  --chain <chain_name> \
  [--slippage <percent>] \
  [--mode <exactIn|exactOut>]
```

**Parameters:**
- `--router`: Router name (from routers command, default: `okx`)
- `--version`: Router version (from routers command, default: `v1`)
- `--from`: Source token address (use `0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee` for native tokens on EVM, `11111111111111111111111111111111` for SOL)
- `--to`: Destination token address
- `--amount`: Amount in minimal units (wei for EVM, lamports for Solana)
- `--chain`: Chain name (e.g., `ethereum`, `solana`, `xlayer`, `base`, `bsc`, `arbitrum`, `polygon`)
- `--slippage`: Slippage tolerance in percent (default: `1`, range: `0.01-50`)
- `--mode`: Swap mode — `exactIn` (default) or `exactOut` (only on Ethereum/Base/BSC/Arbitrum)

**Example:**
```bash
# Quote: Sell 100 USDC for ETH on Ethereum
opentrade swap quote \
  --router okx \
  --version v1 \
  --from 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48 \
  --to 0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee \
  --amount 100000000 \
  --chain ethereum \
  --slippage 1
```

**Response:**
```json
{
  "data": {
    "fromToken": {
      "address": "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48",
      "symbol": "USDC",
      "decimals": 6
    },
    "toToken": {
      "address": "0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee",
      "symbol": "ETH",
      "decimals": 18
    },
    "fromAmount": "100000000",
    "toAmount": "32145678901234567",
    "estimatedGas": "150000",
    "gasFeeInUsd": "5.23",
    "priceImpact": "0.15",
    "minReceiveAmount": "31824321890123456",
    "routes": [
      {
        "dex": "Uniswap V3",
        "percentage": 100
      }
    ]
  },
  "success": true
}
```

**Key Fields:**
- `toAmount`: Expected output amount in minimal units
- `estimatedGas`: Gas estimate
- `gasFeeInUsd`: Gas cost in USD
- `priceImpact`: Price impact percentage
- `minReceiveAmount`: Minimum amount after slippage
- `routes`: DEX routing breakdown

---

### 3. Get Approval Data

Get ERC-20 token approval transaction data (EVM chains only).

```bash
opentrade swap approve \
  --router <router> \
  --version <version> \
  --token <token_address> \
  --amount <amount_in_minimal_units> \
  --chain <chain_name>
```

**Parameters:**
- `--router`: Router name (from routers command, default: `okx`)
- `--version`: Router version (from routers command, default: `v1`)
- `--token`: Token contract address to approve
- `--amount`: Amount to approve in minimal units (use max uint256 for unlimited)
- `--chain`: Chain name (e.g., `ethereum`, `xlayer`, `base`, `bsc`, `arbitrum`, `polygon`)

**Example:**
```bash
# Approve 100 USDC on Ethereum
opentrade swap approve \
  --router okx \
  --version v1 \
  --token 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48 \
  --amount 100000000 \
  --chain ethereum
```

**Response:**
```json
{
  "data": {
    "to": "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48",
    "data": "0x095ea7b3000000000000000000000000...",
    "value": "0",
    "gasLimit": "50000"
  },
  "success": true
}
```

**Next Steps:**
1. Sign the transaction with user's wallet
2. Broadcast via `opentrade-transaction`
3. Wait for confirmation
4. Proceed with swap

**Note:** Native tokens (ETH, OKB, SOL) do not require approval.

---

### 4. Get Swap Transaction

Get the swap transaction data to execute the trade.

```bash
opentrade swap swap \
  --router <router> \
  --version <version> \
  --from <token_address> \
  --to <token_address> \
  --amount <amount_in_minimal_units> \
  --chain <chain_name> \
  --wallet <user_wallet_address> \
  [--slippage <percent>] \
  [--mode <exactIn|exactOut>]
```

**Parameters:**
- `--router`: Router name (from routers command, default: `okx`)
- `--version`: Router version (from routers command, default: `v1`)
- `--from`: Source token address
- `--to`: Destination token address
- `--amount`: Amount in minimal units
- `--chain`: Chain name
- `--wallet`: User's wallet address
- `--slippage`: Slippage tolerance in percent (default: `1`)
- `--mode`: Swap mode — `exactIn` (default) or `exactOut`

**Example:**
```bash
# Swap 100 USDC for ETH on Ethereum
opentrade swap swap \
  --router okx \
  --version v1 \
  --from 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48 \
  --to 0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee \
  --amount 100000000 \
  --chain ethereum \
  --wallet 0xYourWalletAddress \
  --slippage 1
```

**Response (EVM):**
```json
{
  "data": {
    "to": "0x1111111254eeb25477b68fb85ed929f73a960582",
    "data": "0x12aa3caf000000000000000000000000...",
    "value": "0",
    "gasLimit": "200000"
  },
  "success": true
}
```

**Response (Solana):**
```json
{
  "data": {
    "transaction": "base64_encoded_transaction_data",
    "signers": ["wallet_address"]
  },
  "success": true
}
```

**Next Steps:**
1. Sign the transaction with user's wallet
2. Broadcast via `opentrade-transaction`
3. Track transaction status

---

### 5. Get Liquidity Sources

List available DEX sources for a specific chain.

```bash
opentrade swap liquidity \
  --router <router> \
  --version <version> \
  --chain <chain_name>
```

**Parameters:**
- `--router`: Router name (from routers command, default: `okx`)
- `--version`: Router version (from routers command, default: `v1`)
- `--chain`: Chain name (e.g., `ethereum`, `solana`, `xlayer`, `base`)

**Example:**
```bash
# Get DEX sources on XLayer
opentrade swap liquidity \
  --router okx \
  --version v1 \
  --chain xlayer
```

**Response:**
```json
{
  "data": {
    "dexList": [
      {
        "name": "XLayer DEX",
        "protocol": "uniswap-v2"
      },
      {
        "name": "CurveNG",
        "protocol": "curve"
      },
      {
        "name": "Merchant Moe",
        "protocol": "trader-joe"
      }
    ]
  },
  "success": true
}
```

---

## Supported Chains

| Chain | Name | Native Token Address (EVM) |
|-------|------|----------------------------|
| Ethereum | `ethereum` | `0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee` |
| XLayer | `xlayer` | `0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee` |
| Base | `base` | `0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee` |
| BSC | `bsc` | `0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee` |
| Arbitrum | `arbitrum` | `0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee` |
| Polygon | `polygon` | `0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee` |
| Optimism | `optimism` | `0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee` |
| Avalanche | `avalanche` | `0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee` |
| Solana | `solana` | `11111111111111111111111111111111` |

**Note:** For Solana native SOL, use `11111111111111111111111111111111` (system program), NOT `So11111111111111111111111111111111111111112` (wSOL).

---

## Workflow Examples

### Complete EVM Swap Flow

**User says:** "Swap 100 USDC for ETH on Ethereum"

```bash
# Step 1: Discover router (if not already known)
opentrade trade routers
# → Use router=okx, version=v1

# Step 2: Get quote
opentrade swap quote \
  --router okx \
  --version v1 \
  --from 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48 \
  --to 0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee \
  --amount 100000000 \
  --chain ethereum \
  --slippage 1
# → Display: "You'll receive ~0.032 ETH, gas fee: $5.23, price impact: 0.15%"

# Step 3: Check if approval needed (skip for native tokens)
opentrade swap approve \
  --router okx \
  --version v1 \
  --token 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48 \
  --amount 100000000 \
  --chain ethereum
# → Returns approval tx → user signs → broadcast via opentrade-transaction

# Step 4: Execute swap
opentrade swap swap \
  --router okx \
  --version v1 \
  --from 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48 \
  --to 0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee \
  --amount 100000000 \
  --chain ethereum \
  --wallet 0xYourWallet \
  --slippage 1
# → Returns tx data → user signs → broadcast via opentrade-transaction
```

### Complete Solana Swap Flow

**User says:** "Swap 1 SOL for USDC on Solana"

```bash
# Step 1: Discover router
opentrade trade routers
# → Use router=okx, version=v1

# Step 2: Get quote
opentrade swap quote \
  --router okx \
  --version v1 \
  --from 11111111111111111111111111111111 \
  --to EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v \
  --amount 1000000000 \
  --chain solana \
  --slippage 1
# → Display: "You'll receive ~180 USDC, gas fee: $0.0001, price impact: 0.05%"

# Step 3: Execute swap (no approval needed on Solana)
opentrade swap swap \
  --router okx \
  --version v1 \
  --from 11111111111111111111111111111111 \
  --to EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v \
  --amount 1000000000 \
  --chain solana \
  --wallet YourSolanaWallet \
  --slippage 1
# → Returns tx data → user signs → broadcast via opentrade-transaction
```

### Check Available DEXes

**User says:** "What DEXes are available on XLayer?"

```bash
opentrade swap liquidity \
  --router okx \
  --version v1 \
  --chain xlayer
# → Display: CurveNG, XLayer DEX, ... (DEX sources on XLayer)
```

---

## Edge Cases

- **High slippage (>5%)**: warn user, suggest splitting the trade or adjusting slippage
- **Large price impact (>10%)**: strongly warn, suggest reducing amount
- **Honeypot token**: `isHoneyPot = true` — block trade and warn user
- **Tax token**: `taxRate` non-zero — display to user (e.g. 5% buy tax)
- **Insufficient balance**: check balance first, show current balance, suggest adjusting amount
- **exactOut not supported**: only Ethereum/Base/BSC/Arbitrum — prompt user to use `exactIn`
- **Solana native SOL address**: Must use `11111111111111111111111111111111` (system program), NOT `So11111111111111111111111111111111111111112` (wSOL)
- **Network error**: retry once, then prompt user to try again later
- **Region restriction (error code 50125 or 80001)**: do NOT show the raw error code to the user. Instead, display a friendly message: `⚠️ Service is not available in your region. Please switch to a supported region and try again.`

---

## Amount Display Rules

- Input/output amounts in UI units (`1.5 ETH`, `3,200 USDC`)
- Internal CLI params use minimal units (`1 USDC` = `"1000000"`, `1 ETH` = `"1000000000000000000"`)
- Gas fees in USD
- `minReceiveAmount` in both UI units and USD
- Price impact as percentage

---

## Global Notes

- Amounts must be in **minimal units** (wei/lamports)
- `exactOut` only on Ethereum(`1`)/Base(`8453`)/BSC(`56`)/Arbitrum(`42161`)
- Check `isHoneyPot` and `taxRate` — surface safety info to users
- EVM contract addresses must be **all lowercase**
- The CLI resolves chain names automatically (e.g., `ethereum` → `1`, `solana` → `501`)
- The CLI handles authentication internally via environment variables or config file
- Get your API token at https://6551.io/mcp
- Each request consumes 1 quota unit
- Base URL: `https://ai.6551.io/open`
- Authentication: Bearer token in Authorization header
