---
name: opentrade-token
description: "This skill should be used when the user asks to 'find a token', 'search for a token', 'look up PEPE', 'what's trending', 'top tokens', 'trending tokens on Solana', 'token rankings', 'who holds this token', 'holder distribution', 'is this token safe', 'token market cap', 'token liquidity', 'research a token', 'tell me about this token', 'token info', or mentions searching for tokens by name or address, discovering trending tokens, viewing token rankings, checking holder distribution, or analyzing token market cap and liquidity. Covers token search, metadata, market cap, liquidity, volume, trending token rankings, and holder analysis across XLayer, Solana, Ethereum, Base, BSC, Arbitrum, Polygon, and 20+ other chains. Do NOT use when the user says only a single generic word like 'tokens' or 'crypto' without specifying a token name, action, or question. For simple current price checks, price charts, candlestick data, or trade history, use okx-market instead."
license: Apache-2.0
metadata:
  author: 6551
  version: "1.0.0"
  homepage: "https://6551.io"
---

# OpenTrade DEX Token Info Skill

5 API endpoints for token search, metadata, detailed pricing, rankings, and holder distribution via HTTP.

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

- For real-time prices / K-lines / trade history → use `opentrade-market`
- For swap execution → use `opentrade-dex-swap`
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
- If response contains `"router": "okx"` and `"version": "v1"`, use: `https://ai.6551.io/open/trader/okx/v1/token/search`
- If response is empty, use: `https://ai.6551.io/open/trader/okx/v1/token/search`

```bash
# Search token
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/token/search?chains=1,501&search=xETH"

# Get token basic info
curl -s -X POST -H "Authorization: Bearer $OPEN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '[{"chainIndex":"196","tokenContractAddress":"0xe7b000003a45145decf8a28fc755ad5ec5ea025a"}]' \
  "https://ai.6551.io/open/trader/{router}/{version}/token/basic-info"

# Get detailed price info
curl -s -X POST -H "Authorization: Bearer $OPEN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '[{"chainIndex":"196","tokenContractAddress":"0xe7b000003a45145decf8a28fc755ad5ec5ea025a"}]' \
  "https://ai.6551.io/open/trader/{router}/{version}/token/price-info"

# What's trending on Solana by volume?
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/token/toplist?chains=501&sortBy=5&timeFrame=4"

# Check holder distribution
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/token/holder?chainIndex=196&tokenContractAddress=0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"
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

## API Index

| # | API Name | API Endpoint | Description |
|---|---|---|---|
| 1 | Search | GET `/trader/{router}/{version}/token/search` | Search for tokens by name, symbol, or address |
| 2 | Info | POST `/trader/{router}/{version}/token/info` | Get token basic info (name, symbol, decimals, logo) |
| 3 | PriceInfo | POST `/trader/{router}/{version}/token/price-info` | Get detailed price info (price, market cap, liquidity, volume, 24h change) |
| 4 | Trending | GET `/trader/{router}/{version}/token/trending` | Get trending / top tokens |
| 5 | Holders | GET `/trader/{router}/{version}/token/holders` | Get token holder distribution (top 20) |

## Boundary: token vs market skill

| Need | Use this skill (`opentrade-token`) | Use `opentrade-market` instead |
|---|---|---|
| Search token by name/symbol | Search command | - |
| Token metadata (decimals, logo) | Info command | - |
| Price + market cap + liquidity + multi-timeframe change | PriceInfo command | - |
| Token ranking (trending) | Trending command | - |
| Holder distribution | Holders command | - |
| Raw real-time price (single value) | - | Price command |
| K-line / candlestick chart | - | Kline command |
| Trade history (buy/sell log) | - | Trades command |
| Index price (multi-source aggregate) | - | Index command |

**Rule of thumb**: `opentrade-token` = token discovery & enriched analytics. `opentrade-market` = raw price feeds & charts.

## Cross-Skill Workflows

This skill is the typical **entry point** — users often start by searching/discovering tokens, then proceed to swap.

### Workflow A: Search → Research → Buy

> User: "Find BONK token, analyze it, then buy some"

```
1. opentrade-token     → search BONK on Solana
2. opentrade-token     → get price-info (market cap, liquidity, volume24H)
3. opentrade-token     → get holders (top 20 distribution)
4. opentrade-market    → get K-line chart (hourly price chart)
       ↓ user decides to buy
5. opentrade-dex-swap  → get quote
6. opentrade-dex-swap  → get swap transaction
7. opentrade-transaction → broadcast
```

**Data handoff**:
- `tokenContractAddress` from step 1 → reused in all subsequent steps
- `chainIndex` from step 1 → reused in all subsequent steps
- `decimals` from step 1 or Info command → needed for minimal unit conversion in swap

### Workflow B: Discover Trending → Investigate → Trade

> User: "What's trending on Solana?"

```
1. opentrade-token     → get trending tokens (top by 24h volume)
       ↓ user picks a token
2. opentrade-token     → get price-info (detailed analytics)
3. opentrade-token     → get holders (check if whale-dominated)
4. opentrade-market    → get K-line (visual trend)
       ↓ user decides to trade
5. opentrade-dex-swap  → execute swap
```

### Workflow C: Token Verification Before Swap

Before swapping an unknown token, always verify:

```
1. opentrade-token     → search token by name
2. Check communityRecognized:
   - true → proceed with normal caution
   - false → warn user about risk
3. opentrade-token     → get price-info, check liquidity:
   - liquidity < $10K → warn about high slippage risk
   - liquidity < $1K → strongly discourage trade
4. opentrade-dex-swap  → get quote, check isHoneyPot and taxRate
```

## Operation Flow

### Step 1: Identify Intent

- Search for token → Search API
- Get token metadata → Info API
- Get detailed price/market data → PriceInfo API
- Find trending tokens → Trending API
- Check holder distribution → Holders API

### Step 2: Collect Parameters

- Missing chain → ask user which chain, or default to Ethereum + Solana for search
- Missing token address → use Search API first
- For trending: ask sort preference (volume, price change, market cap)

### Step 3: Call and Display

- Call API endpoint with parameters
- Display results with appropriate formatting
- Highlight important metrics (liquidity, holder concentration)

### Step 4: Suggest Next Steps

| Just called | Suggest |
|---|---|
| Search | 1. Get detailed info → PriceInfo 2. Check holders → Holders 3. View chart → `opentrade-market` |
| Info | 1. Get price details → PriceInfo 2. Check holders → Holders |
| PriceInfo | 1. View chart → `opentrade-market` 2. Check holders → Holders 3. Execute swap → `opentrade-dex-swap` |
| Trending | 1. Research a token → PriceInfo 2. Check holders → Holders |
| Holders | 1. View price chart → `opentrade-market` 2. Execute swap → `opentrade-dex-swap` |

## API Reference

### 1. Search

Search for tokens by name, symbol, or address.

```bash
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/token/search?chains=<chains>&search=<query>"
```

**Parameters:**
| Param | Required | Default | Description |
|---|---|---|---|
| `chains` | Yes | - | Comma-separated chain IDs (e.g., "1,501") |
| `search` | Yes | - | Search keyword (name, symbol, or contract address) |

**Response fields:**
- `tokenContractAddress`: Token contract address
- `symbol`: Token symbol
- `tokenName`: Token name
- `chainIndex`: Chain ID
- `decimals`: Token decimals
- `logoUrl`: Token logo URL
- `communityRecognized`: Whether token is verified

### 2. Info

Get token basic info (name, symbol, decimals, logo).

```bash
curl -s -X POST -H "Authorization: Bearer $OPEN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '[{"chainIndex":"<chain>","tokenContractAddress":"<address>"}]' \
  "https://ai.6551.io/open/trader/{router}/{version}/token/basic-info"
```

**Request body:** Array of objects
| Field | Required | Description |
|---|---|---|
| `chainIndex` | Yes | Chain ID |
| `tokenContractAddress` | Yes | Token contract address |

**Response fields:**
- `symbol`: Token symbol
- `tokenName`: Token name
- `decimals`: Token decimals
- `logoUrl`: Token logo URL
- `totalSupply`: Total token supply

### 3. PriceInfo

Get detailed price info (price, market cap, liquidity, volume, 24h change).

```bash
curl -s -X POST -H "Authorization: Bearer $OPEN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '[{"chainIndex":"<chain>","tokenContractAddress":"<address>"}]' \
  "https://ai.6551.io/open/trader/{router}/{version}/token/price-info"
```

**Request body:** Array of objects
| Field | Required | Description |
|---|---|---|
| `chainIndex` | Yes | Chain ID |
| `tokenContractAddress` | Yes | Token contract address |

**Response fields:**
- `price`: Current price in USD
- `marketCap`: Market capitalization
- `liquidity`: Total liquidity
- `volume24h`: 24-hour trading volume
- `priceChange24h`: 24-hour price change percentage
- `priceChange1h`: 1-hour price change percentage
- `priceChange5m`: 5-minute price change percentage

### 4. Trending

Get trending / top tokens.

```bash
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/token/toplist?chains=<chains>&sortBy=<sort>&timeFrame=<time>"
```

**Parameters:**
| Param | Required | Default | Description |
|---|---|---|---|
| `chains` | Yes | - | Comma-separated chain IDs |
| `sortBy` | No | `5` | Sort by: 2=price change, 5=volume, 6=market cap |
| `timeFrame` | No | `4` | Time frame: 1=5min, 2=1h, 3=4h, 4=24h |

**Response fields:**
- `tokenContractAddress`: Token address
- `symbol`: Token symbol
- `tokenName`: Token name
- `price`: Current price
- `priceChange`: Price change percentage
- `volume`: Trading volume
- `marketCap`: Market capitalization
- `liquidity`: Total liquidity

### 5. Holders

Get token holder distribution (top 20).

```bash
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/token/holder?chainIndex=<chain>&tokenContractAddress=<address>"
```

**Parameters:**
| Param | Required | Description |
|---|---|---|
| `chainIndex` | Yes | Chain ID |
| `tokenContractAddress` | Yes | Token contract address |

**Response fields:**
- `holderCount`: Total number of holders
- `top10HolderPercent`: Percentage held by top 10 holders
- `holders[]`: Array of top 20 holders
  - `address`: Holder address
  - `balance`: Token balance
  - `percentage`: Percentage of total supply

## Input / Output Examples

**User says:** "Find BONK token on Solana"

```bash
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/token/search?chains=501&search=BONK"
# → Display: BONK token found, address: DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263
```

**User says:** "What's trending on Ethereum by volume?"

```bash
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/token/toplist?chains=1&sortBy=5&timeFrame=4"
# → Display: Top 10 tokens by 24h volume
```

## Edge Cases

- **Token not found**: returns empty results — suggest user check spelling or try different chain
- **Multiple matches**: display all matches, let user choose
- **Low liquidity token**: warn user about high slippage risk
- **Unverified token**: warn user about potential scam risk
- **High holder concentration**: warn about whale manipulation risk

## Amount Display Rules

- Always display in UI units with appropriate decimals
- Show USD value alongside token amounts
- Use scientific notation for very small/large numbers

## Global Notes

- EVM contract addresses must be **all lowercase**
- Use `chainIndex` parameter (not `chainId`)
- All output is JSON format
- Get your API token at https://6551.io/mcp
- Each request consumes 1 quota unit
- For batch queries, use POST with JSON array body
