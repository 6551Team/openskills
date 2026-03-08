---
name: opentrade-dex-swap
description: "This skill should be used when the user asks to 'swap tokens', 'trade OKB for USDC', 'buy tokens', 'sell tokens', 'exchange crypto', 'convert tokens', 'swap SOL for USDC', 'get a swap quote', 'execute a trade', 'find the best swap route', 'cheapest way to swap', 'optimal swap', 'compare swap rates', or mentions swapping, trading, buying, selling, or exchanging tokens on XLayer, Solana, Ethereum, Base, BSC, Arbitrum, Polygon, or any of 20+ supported chains. Aggregates liquidity from 500+ DEX sources for optimal routing and price. Supports slippage control, price impact protection, and cross-DEX route optimization. Do NOT use for general programming questions about swap code, or for analytical questions about historical swap volume."
license: Apache-2.0
metadata:
  author: 6551
  version: "1.0.0"
  homepage: "https://6551.io"
---

# OpenTrade DEX Aggregator Skill

5 API endpoints for multi-chain swap aggregation — quote, approve, and execute via HTTP.

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

- For token search → use `opentrade-token`
- For market prices → use `opentrade-market`
- For transaction broadcasting → use `opentrade-transaction`
- For wallet balances / portfolio → use `opentrade-wallet`

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
- If response contains `"router": "okx"` and `"version": "v1"`, use: `https://ai.6551.io/open/trader/okx/v1/swap/quote`
- If response is empty, use: `https://ai.6551.io/open/trader/okx/v1/swap/quote`

### EVM Swap (quote → approve → swap)

```bash
# 1. Quote — sell 100 USDC for OKB on XLayer
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/swap/quote?chainIndex=196&fromTokenAddress=0x74b7f16337b8972027f6196a17a631ac6de26d22&toTokenAddress=0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee&amount=100000000&swapMode=exactIn"
# → Expected: X.XX OKB, gas fee, price impact

# 2. Approve — ERC-20 tokens need approval before swap (skip for native OKB)
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/swap/approve?chainIndex=196&tokenContractAddress=0x74b7f16337b8972027f6196a17a631ac6de26d22&approveAmount=100000000"
# → Returns approval calldata: sign and broadcast via opentrade-transaction

# 3. Swap
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/swap/swap?chainIndex=196&fromTokenAddress=0x74b7f16337b8972027f6196a17a631ac6de26d22&toTokenAddress=0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee&amount=100000000&slippagePercent=1&userWalletAddress=0xYourWallet&swapMode=exactIn"
# → Returns tx data: sign and broadcast via opentrade-transaction
```

### Solana Swap

```bash
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/swap/swap?chainIndex=501&fromTokenAddress=11111111111111111111111111111111&toTokenAddress=DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263&amount=1000000000&slippagePercent=1&userWalletAddress=YourSolanaWallet&swapMode=exactIn"
# → Returns tx data: sign and broadcast via opentrade-transaction
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

## Native Token Addresses

> **CRITICAL**: Each chain has a specific native token address. Using the wrong address will cause swap transactions to fail.

| Chain | Native Token Address |
|---|---|
| EVM (Ethereum, BSC, Polygon, Arbitrum, Base, XLayer, etc.) | `0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee` |
| Solana | `11111111111111111111111111111111` |
| Sui | `0x2::sui::SUI` |
| Tron | `T9yD14Nj9j7xAB4dbGeiX9h8unkKHxuWwb` |
| Ton | `EQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAM9c` |

> **WARNING — Solana native SOL**: The correct address is `11111111111111111111111111111111` (Solana system program). Do **NOT** use `So11111111111111111111111111111111111111112` (wSOL SPL token) — it is a different token and will cause swap failures.

## API Index

| # | API Name | API Endpoint | Description |
|---|---|---|---|
| 1 | Quote | GET `/trader/{router}/{version}/swap/quote` | Get swap quote (read-only price estimate) |
| 2 | Swap | GET `/trader/{router}/{version}/swap/swap` | Get swap transaction data |
| 3 | Approve | GET `/trader/{router}/{version}/swap/approve` | Get ERC-20 approval transaction |
| 4 | Chains | GET `/trader/{router}/{version}/swap/chains` | Get supported chains |
| 5 | Liquidity | GET `/trader/{router}/{version}/swap/liquidity` | Get available liquidity sources |

## Cross-Skill Workflows

### Workflow A: Research Token Before Buying

> User: "Tell me about BONK, show me the chart, then buy if it looks good"

```
1. opentrade-token     → search BONK on Solana
2. opentrade-market    → get price info (market cap, liquidity, 24h volume)
3. opentrade-market    → get K-line chart for visual trend
       ↓ user decides to buy
4. opentrade-dex-swap  → get quote
5. opentrade-dex-swap  → get swap transaction
6. opentrade-transaction → broadcast transaction
```

### Workflow B: Quick Swap

> User: "Swap 100 USDC for ETH on Ethereum"

```
1. opentrade-dex-swap  → get quote
2. opentrade-dex-swap  → get approval (if needed)
3. opentrade-dex-swap  → get swap transaction
4. opentrade-transaction → broadcast
5. opentrade-transaction → track status
```

## Operation Flow

### Step 1: Identify Intent

- Get swap quote → use Quote API
- Execute swap → use Swap API
- Approve ERC-20 → use Approve API
- Check supported chains → use Chains API
- View liquidity sources → use Liquidity API

### Step 2: Collect Parameters

- Missing chain → recommend XLayer (low gas, fast) as default
- Missing token address → use `opentrade-token` to search
- Missing amount → ask user
- Missing wallet address → ask user (required for swap execution)

### Step 3: Call and Display

- Call API endpoint with parameters
- Display results with appropriate precision
- Show USD value alongside token amounts

### Step 4: Suggest Next Steps

After displaying results, suggest relevant follow-up actions:

| Just called | Suggest |
|---|---|
| Quote | 1. Execute swap → Swap API 2. Check token details → `opentrade-token` 3. View price chart → `opentrade-market` |
| Swap | 1. Broadcast transaction → `opentrade-transaction` 2. Track status → `opentrade-transaction` |
| Approve | 1. Execute swap → Swap API |
| Chains | 1. Get quote on a chain → Quote API |
| Liquidity | 1. Get quote → Quote API |

Present conversationally, e.g.: "Swap complete! Would you like to broadcast the transaction?" — never expose skill names or endpoint paths to the user.

## API Reference

### 1. Quote

Get swap quote (read-only price estimate).

```bash
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/swap/quote?chainIndex=<chain>&fromTokenAddress=<from>&toTokenAddress=<to>&amount=<amount>&swapMode=<mode>"
```

**Parameters:**
| Param | Required | Default | Description |
|---|---|---|---|
| `chainIndex` | Yes | - | Chain ID (e.g., 1 for Ethereum, 501 for Solana) |
| `fromTokenAddress` | Yes | - | Source token contract address |
| `toTokenAddress` | Yes | - | Destination token contract address |
| `amount` | Yes | - | Amount in minimal units (wei/lamports) |
| `swapMode` | No | `exactIn` | `exactIn` or `exactOut` |

### 2. Swap

Get swap transaction data (quote → sign → broadcast).

```bash
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/swap/swap?chainIndex=<chain>&fromTokenAddress=<from>&toTokenAddress=<to>&amount=<amount>&slippagePercent=<slippage>&userWalletAddress=<wallet>&swapMode=<mode>"
```

**Parameters:**
| Param | Required | Default | Description |
|---|---|---|---|
| `chainIndex` | Yes | - | Chain ID |
| `fromTokenAddress` | Yes | - | Source token contract address |
| `toTokenAddress` | Yes | - | Destination token contract address |
| `amount` | Yes | - | Amount in minimal units |
| `slippagePercent` | No | `1` | Slippage tolerance (e.g., "1" for 1%) |
| `userWalletAddress` | Yes | - | User wallet address |
| `swapMode` | No | `exactIn` | `exactIn` or `exactOut` |

### 3. Approve

Get ERC-20 approval transaction data.

```bash
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/swap/approve?chainIndex=<chain>&tokenContractAddress=<token>&approveAmount=<amount>"
```

**Parameters:**
| Param | Required | Default | Description |
|---|---|---|---|
| `chainIndex` | Yes | - | Chain ID |
| `tokenContractAddress` | Yes | - | Token contract address to approve |
| `approveAmount` | Yes | - | Approval amount in minimal units |

### 4. Chains

Get supported chains for DEX aggregator.

```bash
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/swap/chains"
```

No parameters required.

### 5. Liquidity

Get available liquidity sources on a chain.

```bash
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/swap/liquidity?chainIndex=<chain>"
```

**Parameters:**
| Param | Required | Default | Description |
|---|---|---|---|
| `chainIndex` | Yes | - | Chain ID |

## Input / Output Examples

**User says:** "Swap 100 USDC for ETH on Ethereum"

```bash
# 1. Get quote
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/swap/quote?chainIndex=1&fromTokenAddress=0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48&toTokenAddress=0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2&amount=100000000&swapMode=exactIn"

# 2. Get approval
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/swap/approve?chainIndex=1&tokenContractAddress=0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48&approveAmount=100000000"

# 3. Get swap transaction
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/swap/swap?chainIndex=1&fromTokenAddress=0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48&toTokenAddress=0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2&amount=100000000&slippagePercent=1&userWalletAddress=0x...&swapMode=exactIn"
```

## Edge Cases

- **Invalid token address**: returns error — prompt user to verify or use `opentrade-token` to search
- **Unsupported chain**: returns error — try a different chain
- **Insufficient liquidity**: quote may show high price impact — warn user
- **High slippage (>5%)**: warn user, suggest splitting the trade or adjusting slippage
- **Large price impact (>10%)**: strongly warn, suggest reducing amount
- **Honeypot token**: `isHoneyPot = true` — block trade and warn user
- **Tax token**: `taxRate` non-zero — display to user (e.g. 5% buy tax)
- **Insufficient balance**: check balance first, show current balance, suggest adjusting amount
- **exactOut not supported**: only Ethereum/Base/BSC/Arbitrum — prompt user to use `exactIn`
- **Solana native SOL address**: Must use `11111111111111111111111111111111` (system program), NOT `So11111111111111111111111111111111111111112` (wSOL)
- **Network error**: retry once, then prompt user to try again later
- **Region restriction (error code 50125 or 80001)**: do NOT show the raw error code to the user. Instead, display a friendly message: `⚠️ Service is not available in your region. Please switch to a supported region and try again.`

## Amount Display Rules

- Always display in UI units (`1.5 ETH`), never base units
- Show USD value alongside (`1.5 ETH ≈ $4,500`)
- Amounts are in minimal units for API calls (wei for EVM, lamports for Solana)

## Global Notes

- Amounts must be in **minimal units** (wei/lamports)
- `exactOut` only on Ethereum(`1`)/Base(`8453`)/BSC(`56`)/Arbitrum(`42161`)
- Check `isHoneyPot` and `taxRate` — surface safety info to users
- EVM contract addresses must be **all lowercase**
- Use `chainIndex` parameter (not `chainId`)
- All output is JSON format
- Get your API token at https://6551.io/mcp
- Each request consumes 1 quota unit
- Base URL: `https://ai.6551.io/open`
- Authentication: Bearer token in Authorization header
