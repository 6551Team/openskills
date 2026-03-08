---
name: opentrade-wallet
description: "This skill should be used when the user asks to 'check my wallet balance', 'show my token holdings', 'how much OKB do I have', 'what tokens do I have', 'check my portfolio value', 'view my assets', 'how much is my portfolio worth', 'what's in my wallet', 'show transaction history', 'recent transactions', or mentions checking wallet balance, total assets, token holdings, portfolio value, remaining funds, transaction history, or multi-chain balance lookup. Supports XLayer, Solana, Ethereum, Base, BSC, Arbitrum, Polygon, and 20+ other chains. Do NOT use for general programming questions about balance variables or API documentation. Do NOT use when the user is asking how to build or integrate a balance feature into code."
license: Apache-2.0
metadata:
  author: 6551
  version: "1.0.0"
  homepage: "https://6551.io"
---

# OpenTrade Wallet Portfolio Skill

4 API endpoints for wallet balance queries, portfolio value, and transaction history via HTTP.

## Prerequisites

Get your API token at: https://6551.io/mcp

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

- For token prices / K-lines → use `opentrade-market`
- For token search / metadata → use `opentrade-token`
- For swap execution → use `opentrade-dex-swap`
- For transaction broadcasting → use `opentrade-transaction`

## Quickstart

### Router Discovery

**IMPORTANT**: If the user has not specified a trading router, you MUST first discover available routers:

```bash
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/routers"
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
- Use these values to construct API URLs: `https://ai.6551.io/open/trader/{router}/{version}/...`
- **Default fallback**: If the API returns no data or is empty, use `router=okx` and `version=v1`

**Example:**
- If response contains `"router": "okx"` and `"version": "v1"`, use: `https://ai.6551.io/open/trader/okx/v1/wallet/balance`
- If response is empty, use: `https://ai.6551.io/open/trader/okx/v1/wallet/balance`

```bash
# Get all token balances for a wallet
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/balance/all-balances?address=0xYourWallet&chainIndex=1"

# Get total portfolio value
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/balance/total-value?address=0xYourWallet&chainIndex=1"

# Batch query specific token balances
curl -s -X POST -H "Authorization: Bearer $OPEN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"address":"0xYourWallet","chainIndex":"1","tokenAddresses":["0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48","0xdAC17F958D2ee523a2206206994597C13D831ec7"]}' \
  "https://ai.6551.io/open/trader/{router}/{version}/balance/token-balances"

# Get transaction history
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io# NOT AVAILABLE: /open/trader/{router}/{version}/balance/transactions?address=0xYourWallet&chainIndex=1&limit=20&offset=0"
```

## Chain Name Support

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

**Address format note**: EVM addresses (`0x...`) work across Ethereum/BSC/Polygon/Arbitrum/Base etc. Solana addresses (Base58) have different format. Do NOT mix formats across chain types.

## API Index

| # | API Name | API Endpoint | Description |
|---|---|---|---|
| 1 | AllBalances | GET `/trader/{router}/{version}/balance/all-balances` | Get all token balances for a wallet |
| 2 | TotalValue | GET `/trader/{router}/{version}/balance/total-value` | Get total portfolio value in USD |
| 3 | Balances | POST `/trader/{router}/{version}/balance/token-balances` | Batch query specific token balances |
| 4 | Transactions | GET `/trader/{router}/{version}/balance/history` | Get address transaction history |

## Cross-Skill Workflows

This skill is often used **before swap** (to verify sufficient balance) or **as portfolio entry point**.

### Workflow A: Pre-Swap Balance Check

> User: "Swap 1 SOL for BONK"

```
1. opentrade-token     → search BONK (get tokenContractAddress)
       ↓ tokenContractAddress
2. opentrade-wallet    → get all balances (verify SOL balance >= 1)
       ↓ balance field (UI units) → convert to minimal units for swap
3. opentrade-dex-swap  → get quote (from SOL to BONK)
4. opentrade-dex-swap  → execute swap
5. opentrade-transaction → broadcast
6. opentrade-wallet    → verify new balance
```

**Data handoff**:
- `tokenContractAddress` from token search → feeds into swap `fromTokenAddress` / `toTokenAddress`
- `balance` from wallet is **UI units**; swap needs **minimal units** → multiply by `10^decimal`
- If balance < required amount → inform user, do NOT proceed to swap

### Workflow B: Portfolio Overview + Analysis

> User: "Show me my portfolio"

```
1. opentrade-wallet    → get total value (USD value across all chains)
2. opentrade-wallet    → get all balances (list all tokens)
       ↓ top holdings by USD value
3. opentrade-token     → get price-info (enrich with 24h change, market cap)
4. opentrade-market    → get K-line (price charts for tokens of interest)
```

**Data handoff**:
- `address` from user → used in all wallet queries
- `tokenAddresses` from step 2 → used in step 3 for enriched analytics

### Workflow C: Check Specific Token Balance

> User: "How much USDC do I have?"

```
1. opentrade-token     → search USDC (get token address)
2. opentrade-wallet    → get token-balances (query specific token)
3. opentrade-market    → get price (current USD value)
```

### Workflow D: Sell Underperforming Tokens

```
1. opentrade-wallet    → get all balances (list all holdings)
       ↓ tokenContractAddress + chainIndex for each
2. opentrade-token     → get price-info (get priceChange24H per token)
3. Filter by negative change → user confirms which to sell
4. opentrade-dex-swap  → get quote → execute swap → sell
5. opentrade-transaction → broadcast
```

**Key conversion**: `balance` (UI units) × `10^decimal` = `amount` (minimal units) for swap.

## Operation Flow

### Step 1: Identify Intent

- Check all token balances → AllBalances command
- Get portfolio total value → TotalValue command
- Check specific token balances → Balances command
- View transaction history → Transactions command

### Step 2: Collect Parameters

- Missing wallet address → ask user for their wallet address
- Missing chain → ask which chain, or check all major chains (Ethereum, BSC, Polygon)
- For specific tokens → use `opentrade-token` Search to get token addresses
- For transaction history → ask how many transactions to show (default 20)

### Step 3: Call and Display

- Call API endpoint with parameters
- Display results with appropriate formatting
- Show token amounts in UI units with symbols
- Display USD values alongside token amounts
- Sort by USD value descending for portfolio view

### Step 4: Suggest Next Steps

| Just called | Suggest |
|---|---|
| AllBalances | 1. Get total value → TotalValue 2. View price charts → `opentrade-market` 3. Execute swap → `opentrade-dex-swap` |
| TotalValue | 1. View detailed balances → AllBalances 2. View transaction history → Transactions |
| Balances | 1. Get current prices → `opentrade-market` 2. Execute swap → `opentrade-dex-swap` |
| Transactions | 1. Check current balances → AllBalances 2. View token details → `opentrade-token` |

## API Reference

### 1. AllBalances

Get all token balances for a wallet address (including native token).

```bash
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/balance/all-balances?address=<address>&chainIndex=<chain>"
```

**Parameters:**
| Param | Required | Description |
|---|---|---|
| `address` | Yes | Wallet address |
| `chainIndex` | Yes | Chain ID |

**Response fields:**
- `tokens[]`: Array of token balances
  - `tokenAddress`: Token contract address (empty string `""` for native token)
  - `symbol`: Token symbol
  - `balance`: Token balance in minimal units
  - `decimals`: Token decimals
  - `price`: Current price in USD
  - `value`: USD value of holdings

### 2. TotalValue

Get total portfolio value in USD for a wallet.

```bash
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/balance/total-value?address=<address>&chainIndex=<chain>"
```

**Parameters:**
| Param | Required | Description |
|---|---|---|
| `address` | Yes | Wallet address |
| `chainIndex` | Yes | Chain ID |

**Response fields:**
- `totalValue`: Total portfolio value in USD
- `currency`: Currency (USD)

### 3. Balances

Batch query specific token balances.

```bash
curl -s -X POST -H "Authorization: Bearer $OPEN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"address":"<address>","chainIndex":"<chain>","tokenAddresses":["<token1>","<token2>"]}' \
  "https://ai.6551.io/open/trader/{router}/{version}/balance/token-balances"
```

**Request body:**
| Field | Required | Description |
|---|---|---|
| `address` | Yes | Wallet address |
| `chainIndex` | Yes | Chain ID |
| `tokenAddresses` | Yes | Array of token contract addresses |

**Response fields:**
- `balances[]`: Array of token balances
  - `tokenAddress`: Token contract address
  - `balance`: Token balance in minimal units
  - `symbol`: Token symbol
  - `decimals`: Token decimals

### 4. Transactions

Get transaction history for a wallet address.

```bash
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io# NOT AVAILABLE: /open/trader/{router}/{version}/portfolio/transactions?address=<address>&chainIndex=<chain>&limit=<n>&offset=<offset>"
```

**Parameters:**
| Param | Required | Default | Description |
|---|---|---|---|
| `address` | Yes | - | Wallet address |
| `chainIndex` | Yes | - | Chain ID |
| `limit` | No | `20` | Number of transactions to return (max 100) |
| `offset` | No | `0` | Pagination offset |

**Response fields:**
- `transactions[]`: Array of transactions
  - `txHash`: Transaction hash
  - `blockNumber`: Block number
  - `timestamp`: Transaction timestamp
  - `from`: Sender address
  - `to`: Recipient address
  - `value`: Transfer value in minimal units
  - `tokenAddress`: Token contract address (empty for native token)
  - `tokenSymbol`: Token symbol
  - `type`: Transaction type (transfer, swap, etc.)
- `total`: Total number of transactions

## Input / Output Examples

**User says:** "Check my wallet balance on Ethereum"

```bash
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/portfolio/all-balances?address=0x...&chainIndex=1"
# → Display: ETH: 1.5 ($4,500), USDC: 1000 ($1,000), Total: $5,500
```

**User says:** "How much is my portfolio worth?"

```bash
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/portfolio/total-value?address=0x...&chainIndex=1"
# → Display: Total portfolio value: $10,500
```

**User says:** "Show my recent transactions"

```bash
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io# NOT AVAILABLE: /open/trader/{router}/{version}/portfolio/transactions?address=0x...&chainIndex=1&limit=10"
# → Display: Last 10 transactions with timestamps, amounts, and token symbols
```

## Edge Cases

- **Invalid address**: returns error — verify address format
- **Address with no balance**: returns empty array or zero value
- **Unsupported chain**: returns error — check supported chains
- **Rate limiting**: too many requests — implement exponential backoff
- **Stale prices**: prices may be slightly delayed — refresh if needed
- **Large portfolio**: pagination may be needed for transaction history

## Amount Display Rules

- Token amounts in UI units (`1.5 ETH`), never base units (`1500000000000000000`)
- USD values with 2 decimal places (`$1,234.56`)
- Large amounts in shorthand (`$1.2M`, `$5.3K`)
- Sort by USD value descending in portfolio view
- Show percentage of total portfolio for each token
- Highlight tokens with significant value (>1% of portfolio)

## Multi-Chain Portfolio

To check portfolio across multiple chains:

```bash
# Check Ethereum
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/portfolio/total-value?address=0x...&chainIndex=1"

# Check BSC
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/portfolio/total-value?address=0x...&chainIndex=56"

# Check Polygon
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/portfolio/total-value?address=0x...&chainIndex=137"

# Sum up for total cross-chain portfolio value
```

## Global Notes

- EVM contract addresses must be **all lowercase**
- Use `chainIndex` parameter (not `chainId`)
- Native token (ETH, BNB, etc.) is represented with empty string `""` as token address
- Balances are returned in minimal units (wei for EVM, lamports for Solana)
- Values are calculated in USD based on current market prices
- Transaction history includes both native token and ERC-20 transfers
- All output is JSON format
- Get your API token at https://6551.io/mcp
- Each request consumes 1 quota unit
