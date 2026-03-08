---
name: opentrade-market
description: "This skill should be used when the user asks 'what's the price of OKB', 'check token price', 'how much is OKB', 'show me the price chart', 'get candlestick data', 'show K-line chart', 'view trade history', 'recent trades for SOL', 'price trend', 'index price', 'what are smart money wallets buying', 'show me whale signals', 'KOL token signals', 'what tokens are smart money buying', 'show me the signal list', 'which chains support signals', 'show me new meme tokens', 'list meme pump tokens', 'which chains support meme pump', or mentions checking a token's current price, viewing price charts, candlestick data, trade history, historical price trends, smart money / whale / KOL on-chain trading signals, signal-supported chains, or meme pump token discovery. Covers real-time on-chain prices, K-line/candlestick charts, trade logs, index prices, smart money signals, and meme pump token scanning across XLayer, Solana, Ethereum, Base, BSC, Arbitrum, Polygon, and 20+ other chains. For token search, market cap, liquidity analysis, trending tokens, or holder distribution, use okx-token instead."
license: Apache-2.0
metadata:
  author: 6551
  version: "1.0.0"
  homepage: "https://6551.io"
---

# OpenTrade DEX Market Data Skill

9 API endpoints for on-chain prices, trades, candlesticks, index prices, smart money signals, and meme pump token scanning via HTTP.

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

- For token search / metadata / rankings / holder analysis → use `opentrade-token`
- For swap execution → use `opentrade-dex-swap`
- For transaction broadcasting → use `opentrade-transaction`
- For wallet balances / portfolio → use `opentrade-wallet`
- Signal data (smart money / whale / KOL buy signals, signal-supported chains) → use `opentrade-market`
- Meme pump token scanning (token lists, supported chains/protocols) → use `opentrade-market`

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
- If response contains `"router": "okx"` and `"version": "v1"`, use: `https://ai.6551.io/open/trader/okx/v1/market/price`
- If response is empty, use: `https://ai.6551.io/open/trader/okx/v1/market/price`

```bash
# Get real-time price of OKB on XLayer
curl -s -X POST -H "Authorization: Bearer $OPEN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '[{"chainIndex":"196","tokenContractAddress":"0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"}]' \
  "https://ai.6551.io/open/trader/{router}/{version}/market/price"

# Get hourly candles
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/market/kline?chainIndex=196&tokenContractAddress=0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee&bar=1H&limit=24"

# Solana SOL candles (use wSOL SPL token address for candles/trades)
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/market/kline?chainIndex=501&tokenContractAddress=So11111111111111111111111111111111111111112&bar=1H&limit=24"

# Get batch prices for multiple tokens
curl -s -X POST -H "Authorization: Bearer $OPEN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '[{"chainIndex":"196","tokenContractAddress":"0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"},{"chainIndex":"501","tokenContractAddress":"So11111111111111111111111111111111111111112"}]' \
  "https://ai.6551.io/open/trader/{router}/{version}/market/price"

# Get smart money signals on Solana
curl -s -X POST -H "Authorization: Bearer $OPEN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"chainIndex":"501","walletType":"1,2,3","minAmountUsd":"1000"}' \
  "https://ai.6551.io/open/trader/{router}/{version}/market/signal-list"

# Get supported chains for meme pump
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/market/memepump-chains"

# List new meme pump tokens on Solana
curl -s -X POST -H "Authorization: Bearer $OPEN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"chainIndex":"501","sortBy":"createdTimestamp","limit":"30"}' \
  "https://ai.6551.io/open/trader/{router}/{version}/market/memepump-tokens"
```

## Chain Name Support


| Chain     | Name        | chainIndex |
| --------- | ----------- | ---------- |
| XLayer    | `xlayer`    | `196`      |
| Solana    | `solana`    | `501`      |
| Ethereum  | `ethereum`  | `1`        |
| Base      | `base`      | `8453`     |
| BSC       | `bsc`       | `56`       |
| Arbitrum  | `arbitrum`  | `42161`    |
| Polygon   | `polygon`   | `137`      |
| Optimism  | `optimism`  | `10`       |
| Avalanche | `avalanche` | `43114`    |


## API Index

### Market Price APIs


| #   | API Name | API Endpoint | Description                      |
| --- | ------- | ------- | -------------------------------- |
| 1   | Price | POST `/trader/{router}/{version}/market/price` | Get single or batch token prices |
| 2   | Trades | GET `/trader/{router}/{version}/market/trades` | Get recent trades                |
| 3   | Kline | GET `/trader/{router}/{version}/market/kline` | Get K-line / candlestick data    |


### Index Price APIs


| #   | API Name | API Endpoint | Description                                        |
| --- | ------- | ------- | -------------------------------------------------- |
| 4   | Index | POST `/trader/{router}/{version}/market/current-price` | Get index price (aggregated from multiple sources) |


### Signal APIs


| #   | API Name      | API Endpoint | Description                                                 |
| --- | ------------ | ------------ | ----------------------------------------------------------- |
| 5   | SignalChains | GET `/trader/{router}/{version}/market/signal-chains` | Get supported chains for market signals                     |
| 6   | SignalList | POST `/trader/{router}/{version}/market/signal-list` | Get latest signal list (smart money / KOL / whale activity) |


### Meme Pump APIs

| #   | API Name         | API Endpoint | Description                                          |
| --- | --------------- | --------------- | ---------------------------------------------------- |
| 7   | MemepumpChains | GET `/trader/{router}/{version}/market/memepump-chains`  | Get supported chains and protocols for meme pump     |
| 8   | MemepumpTokens | POST `/trader/{router}/{version}/market/memepump-tokens`  | List meme pump tokens with advanced filtering        |


## Boundary: market vs token skill


| Need                                        | Use this skill (`opentrade-market`) | Use `opentrade-token` instead |
| ------------------------------------------- | ----------------------------- | ----------------------- |
| Real-time price (single value)              | Price command                 | -                       |
| Price + market cap + liquidity + 24h change | -                             | PriceInfo command       |
| K-line / candlestick chart                  | Kline command                 | -                       |
| Trade history (buy/sell log)                | Trades command                | -                       |
| Index price (multi-source aggregate)        | Index command                 | -                       |
| Token search by name/symbol                 | -                             | Search command          |
| Token metadata (decimals, logo)             | -                             | Info command            |
| Token ranking (trending)                    | -                             | Trending command        |
| Holder distribution                         | -                             | Holders command         |
| Smart money / whale / KOL signals           | SignalList command            | -                       |
| Signal-supported chains                     | SignalChains command          | -                       |
| Browse meme pump tokens                     | MemepumpTokens command        | -                       |
| Meme pump supported chains/protocols        | MemepumpChains command        | -                       |


**Rule of thumb**: `opentrade-market` = raw price feeds, charts, smart money signals & meme pump token scanning. `opentrade-token` = token discovery & enriched analytics.

## Cross-Skill Workflows

### Workflow A: Research Token Before Buying

> User: "Tell me about BONK, show me the chart, then buy if it looks good"

```
1. opentrade-token     → search BONK on Solana
2. opentrade-token     → get price-info (market cap, liquidity, 24h volume)
3. opentrade-token     → get holders (check holder distribution)
4. opentrade-market    → get K-line chart (visual trend)
       ↓ user decides to buy
5. opentrade-dex-swap  → get quote
6. opentrade-dex-swap  → get swap transaction
```

**Data handoff**: `tokenContractAddress` from step 1 is reused as `<address>` in steps 2-6.

### Workflow B: Price Monitoring / Alerts

```
1. opentrade-token     → get trending tokens by volume
       ↓ select tokens of interest
2. opentrade-market    → get current price for each
3. opentrade-market    → get K-line (hourly chart)
4. opentrade-market    → get index price (compare on-chain vs index)
```

### Workflow C: Signal-Driven Token Research & Buy

> User: "Show me what smart money is buying on Solana and buy if it looks good"

```
1. opentrade-market    → get signal-chains (confirm Solana supports signals)
2. opentrade-market    → get signal-list (latest smart money / whale / KOL buy signals)
       ↓ user picks a token from signal list
3. opentrade-token     → get price-info (enrich: market cap, liquidity, 24h volume)
4. opentrade-token     → get holders (check holder concentration risk)
5. opentrade-market    → get K-line (confirm momentum)
       ↓ user decides to buy
6. opentrade-dex-swap  → get quote
7. opentrade-dex-swap  → get swap transaction
```

**Data handoff**: `token.tokenAddress` from step 2 feeds directly into steps 3–7.

> User: "Filter signals to only show whale buys above $10k"

```
1. opentrade-market    → get signal-list (whale-only signals on Ethereum, min $10k)
2. opentrade-market    → get K-line (chart for chosen token)
```

### Workflow D: Meme Token Discovery

> User: \"Show me new meme tokens on Solana\"

```
1. opentrade-market    → get memepump-chains (discover supported chains & protocols)
2. opentrade-market    → get memepump-tokens (browse new tokens on Solana)
       ↓ pick an interesting token
3. opentrade-token     → get price-info (market cap, liquidity, 24h volume)
4. opentrade-market    → get K-line (view price chart)
       ↓ user decides to buy
5. opentrade-dex-swap  → get quote
6. opentrade-dex-swap  → get swap transaction
```

**Data handoff**: `tokenAddress` from step 2 is reused in steps 3–6.

## Operation Flow

### Step 1: Identify Intent

- Real-time price (single token) → Price API
- Trade history → Trades API
- K-line chart → Kline API
- Index price (current) → Index API
- Smart money / whale / KOL buy signals → SignalList API
- Chains supporting signals → SignalChains API
- Discover meme pump supported chains/protocols → MemepumpChains API
- Browse/filter meme tokens → MemepumpTokens API

### Step 2: Collect Parameters

- Missing chain → recommend XLayer (low gas, fast confirmation) as default, then ask which chain the user prefers; for signal queries, first call SignalChains to confirm the chain is supported; for meme pump queries, default to Solana
- Missing token address → use `opentrade-token` Search API first to resolve; for signal queries, `tokenAddress` is optional (omit to get all signals on the chain); for meme pump, use MemepumpTokens first to discover tokens
- K-line requests → confirm bar size and time range with user
- Signal filter params (`walletType`, `minAmountUsd`, etc.) → ask user for preferences if not specified; default to no filter (returns all signal types)
- Meme pump filter params (`sortBy`, `limit`, etc.) → ask user for preferences if not specified

### Step 3: Call and Display

- Call API endpoint with parameters
- Return formatted results
- Use appropriate precision: 2 decimals for high-value tokens, significant digits for low-value
- Show USD value alongside

### Step 4: Suggest Next Steps

After displaying results, suggest 2-3 relevant follow-up actions based on the command just executed:


| Just called     | Suggest                                                                                                                                        |
| --------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| Price           | 1. View K-line chart → Kline 2. Deeper analytics (market cap, liquidity, 24h volume) → `opentrade-token` 3. Buy/swap this token → `opentrade-dex-swap`     |
| Kline           | 1. Check recent trades → Trades 2. Buy/swap based on the chart → `opentrade-dex-swap`                                                                |
| Trades          | 1. View price chart for context → Kline 2. Execute a trade → `opentrade-dex-swap`                                                                    |
| Index           | 1. Compare with on-chain DEX price → Price 2. View full price chart → Kline                                                                    |
| SignalList      | 1. View price chart for a signal token → Kline 2. Deep token analytics (market cap, liquidity) → `opentrade-token` 3. Buy the token → `opentrade-dex-swap` |
| SignalChains    | 1. Fetch signals on a supported chain → SignalList                                                                                             |
| MemepumpChains  | 1. Browse tokens → MemepumpTokens                                                                                                               |
| MemepumpTokens  | 1. Pick a token for details → `opentrade-token` 2. View price chart → Kline 3. Buy the token → `opentrade-dex-swap`                                        |


Present conversationally, e.g.: "Would you like to see the K-line chart, or buy this token?" — never expose skill names or endpoint paths to the user.

## API Reference

### 1. Price

Get single or batch token prices.

```bash
# Single token
curl -s -X POST -H "Authorization: Bearer $OPEN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '[{"chainIndex":"<chain>","tokenContractAddress":"<address>"}]' \
  "https://ai.6551.io/open/trader/{router}/{version}/market/price"

# Multiple tokens
curl -s -X POST -H "Authorization: Bearer $OPEN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '[{"chainIndex":"1","tokenContractAddress":"0x..."},{"chainIndex":"501","tokenContractAddress":"..."}]' \
  "https://ai.6551.io/open/trader/{router}/{version}/market/price"
```

**Request body:** Array of objects


| Field                  | Required | Description            |
| ---------------------- | -------- | ---------------------- |
| `chainIndex`           | Yes      | Chain ID               |
| `tokenContractAddress` | Yes      | Token contract address |


**Response fields:**

- `chainIndex`: Chain identifier
- `tokenContractAddress`: Token contract address
- `time`: Timestamp (Unix milliseconds)
- `price`: Current price in USD

### 2. Trades

Get recent trades.

```bash
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/market/trades?chainIndex=<chain>&tokenContractAddress=<address>&limit=<n>"
```

**Parameters:**


| Param                  | Required | Default | Description                |
| ---------------------- | -------- | ------- | -------------------------- |
| `chainIndex`           | Yes      | -       | Chain ID                   |
| `tokenContractAddress` | Yes      | -       | Token contract address     |
| `limit`                | No       | `100`   | Number of trades (max 500) |


**Response fields:**

- `id`: Trade ID
- `type`: Trade direction: `buy` or `sell`
- `price`: Trade price in USD
- `volume`: Trade volume in USD
- `time`: Trade timestamp (Unix milliseconds)
- `dexName`: DEX name where trade occurred
- `txHashUrl`: Transaction hash explorer URL
- `userAddress`: Wallet address of the trader

### 3. Kline

Get K-line / candlestick data.

```bash
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/market/kline?chainIndex=<chain>&tokenContractAddress=<address>&bar=<bar>&limit=<n>"
```

**Parameters:**


| Param                  | Required | Default | Description                                                            |
| ---------------------- | -------- | ------- | ---------------------------------------------------------------------- |
| `chainIndex`           | Yes      | -       | Chain ID                                                               |
| `tokenContractAddress` | Yes      | -       | Token contract address                                                 |
| `bar`                  | No       | `1H`    | Bar size: `1s`, `1m`, `5m`, `15m`, `30m`, `1H`, `4H`, `1D`, `1W`, etc. |
| `limit`                | No       | `100`   | Number of data points (max 299)                                        |


**Response fields**: Each data point is an array with the following elements:

- `[0]` ts: Timestamp (Unix milliseconds)
- `[1]` open: Opening price
- `[2]` high: Highest price
- `[3]` low: Lowest price
- `[4]` close: Closing price
- `[5]` vol: Trading volume (token units)
- `[6]` volUsd: Trading volume (USD)
- `[7]` confirm: `"0"` = uncompleted candle, `"1"` = completed candle

### 4. Index

Get index price (aggregated from multiple sources).

```bash
curl -s -X POST -H "Authorization: Bearer $OPEN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '[{"chainIndex":"<chain>","tokenContractAddress":"<address>"}]' \
  "https://ai.6551.io/open/trader/{router}/{version}/market/current-price"
```

**Request body:** Array of objects


| Field                  | Required | Description                                                 |
| ---------------------- | -------- | ----------------------------------------------------------- |
| `chainIndex`           | Yes      | Chain ID                                                    |
| `tokenContractAddress` | Yes      | Token contract address (empty string `""` for native token) |


**Response fields:**

- `chainIndex`: Chain identifier
- `tokenContractAddress`: Token contract address
- `price`: Index price (aggregated from multiple sources)
- `time`: Timestamp (Unix milliseconds)

### 5. SignalChains

Get supported chains for market signals. No parameters required.

```bash
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/market/signal-chains"
```

**Response fields:**

- `chainIndex`: Chain identifier (e.g., `"1"`, `"501"`)
- `chainName`: Human-readable chain name (e.g., `"Ethereum"`, `"Solana"`)
- `chainLogo`: Chain logo image URL

> Call this first when a user wants signal data and you need to confirm chain support before calling SignalList.

### 6. SignalList

Get latest buy-direction token signals sorted descending by time.

```bash
curl -s -X POST -H "Authorization: Bearer $OPEN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"chainIndex":"<chain>","walletType":"<types>","minAmountUsd":"<min>","maxAmountUsd":"<max>","minAddressCount":"<min>","maxAddressCount":"<max>","tokenAddress":"<address>","minMarketCapUsd":"<min>","maxMarketCapUsd":"<max>","minLiquidityUsd":"<min>","maxLiquidityUsd":"<max>"}' \
  "https://ai.6551.io/open/trader/{router}/{version}/market/signal-list"
```

**Request body:**


| Field             | Required | Description                                                                                            |
| ----------------- | -------- | ------------------------------------------------------------------------------------------------------ |
| `chainIndex`      | Yes      | Chain ID                                                                                               |
| `walletType`      | No       | Wallet classification, comma-separated: `1`=Smart Money, `2`=KOL/Influencer, `3`=Whale (e.g., `"1,2"`) |
| `minAmountUsd`    | No       | Minimum transaction amount in USD                                                                      |
| `maxAmountUsd`    | No       | Maximum transaction amount in USD                                                                      |
| `minAddressCount` | No       | Minimum triggering wallet address count                                                                |
| `maxAddressCount` | No       | Maximum triggering wallet address count                                                                |
| `tokenAddress`    | No       | Token contract address (filter signals for a specific token)                                           |
| `minMarketCapUsd` | No       | Minimum token market cap in USD                                                                        |
| `maxMarketCapUsd` | No       | Maximum token market cap in USD                                                                        |
| `minLiquidityUsd` | No       | Minimum token liquidity in USD                                                                         |
| `maxLiquidityUsd` | No       | Maximum token liquidity in USD                                                                         |


**Response fields:**

- `timestamp`: Signal timestamp (Unix milliseconds)
- `chainIndex`: Chain identifier
- `price`: Token price at signal time (USD)
- `walletType`: Wallet classification: `SMART_MONEY`, `WHALE`, or `INFLUENCER`
- `triggerWalletCount`: Number of wallets that triggered this signal
- `triggerWalletAddress`: Comma-separated wallet addresses that triggered the signal
- `amountUsd`: Total transaction amount in USD
- `soldRatioPercent`: Percentage of tokens sold (lower = still holding)
- `token.tokenAddress`: Token contract address
- `token.symbol`: Token symbol
- `token.name`: Token name
- `token.logo`: Token logo URL
- `token.marketCapUsd`: Token market cap in USD
- `token.holders`: Number of token holders
- `token.top10HolderPercent`: Percentage of supply held by top 10 holders

### 7. MemepumpChains

Get supported chains and protocols for meme pump. No parameters required.

```bash
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/market/memepump-chains"
```

**Response fields:**

- `data[].chainIndex`: Chain identifier (e.g., `"501"` for Solana, `"56"` for BSC)
- `data[].chainName`: Human-readable chain name
- `data[].protocolList[].protocolId`: Protocol unique ID
- `data[].protocolList[].protocolName`: Protocol display name (e.g., `pumpfun`, `fourmeme`)

> Currently supports: Solana (501), BSC (56), X Layer (196), TRON (195).

### 8. MemepumpTokens

List meme pump tokens with advanced filtering. Returns up to 30 tokens per request.

```bash
curl -s -X POST -H "Authorization: Bearer $OPEN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"chainIndex":"<chain>","sortBy":"<field>","timeFrame":"<period>","limit":"<n>","offset":"<n>"}' \
  "https://ai.6551.io/open/trader/{router}/{version}/market/memepump-tokens"
```

**Request body:**

| Field       | Required | Description                                                                                    |
| ----------- | -------- | ---------------------------------------------------------------------------------------------- |
| `chainIndex`| Yes      | Chain ID (e.g., `"501"` for Solana)                                                            |
| `sortBy`    | No       | Sort field: `marketCap`, `volume1h`, `txCount1h`, `createdTimestamp`, `bondingPercent`         |
| `timeFrame` | No       | Time period for sorting (e.g., `1h`, `24h`)                                                    |
| `limit`     | No       | Number of tokens to return (max 30)                                                            |
| `offset`    | No       | Pagination offset                                                                              |

**Response fields:** Array of token objects with the following structure:

- `chainIndex`: Chain identifier
- `protocolId`: Protocol numeric ID
- `tokenAddress`: Token contract address
- `symbol`: Token symbol
- `name`: Token name
- `logoUrl`: Token logo URL
- `creatorAddress`: Token creator wallet address
- `createdTimestamp`: Creation timestamp (Unix ms)
- `market.marketCapUsd`: Market cap in USD
- `market.volumeUsd1h`: 1-hour volume in USD
- `market.txCount1h`: 1-hour transaction count
- `bondingPercent`: Bonding curve progress (0-100)

## Input / Output Examples

**User says:** "Check the current price of OKB on XLayer"

```bash
curl -s -X POST -H "Authorization: Bearer $OPEN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '[{"chainIndex":"196","tokenContractAddress":"0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"}]' \
  "https://ai.6551.io/open/trader/{router}/{version}/market/price"
# → Display: OKB current price $XX.XX
```

**User says:** "Show me hourly candles for USDC on XLayer"

```bash
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/market/kline?chainIndex=196&tokenContractAddress=0x74b7f16337b8972027f6196a17a631ac6de26d22&bar=1H&limit=24"
# → Display candlestick data (open/high/low/close/volume)
```

**User says:** "What are smart money wallets buying on Solana?"

```bash
curl -s -X POST -H "Authorization: Bearer $OPEN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"chainIndex":"501","walletType":"1"}' \
  "https://ai.6551.io/open/trader/{router}/{version}/market/signal-list"
# → Display smart money buy signals with token info
```

**User says:** "Show me whale buys above $10k on Ethereum"

```bash
curl -s -X POST -H "Authorization: Bearer $OPEN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"chainIndex":"1","walletType":"3","minAmountUsd":"10000"}' \
  "https://ai.6551.io/open/trader/{router}/{version}/market/signal-list"
# → Display whale-only signals, min $10k
```

**User says:** \"Show me new meme tokens on Solana\"

```bash
curl -s -X POST -H "Authorization: Bearer $OPEN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"chainIndex":"501","sortBy":"createdTimestamp","limit":"30"}' \
  "https://ai.6551.io/open/trader/{router}/{version}/market/memepump-tokens"
# → Display list of new meme pump tokens with market data
```

**User says:** \"Which chains support meme pump?\"

```bash
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/market/memepump-chains"
# → Display supported chains and protocols for meme pump
```

## Edge Cases

- **Invalid token address**: returns empty data or error — prompt user to verify, or use `opentrade-token` Search to resolve
- **Unsupported chain**: returns error — try a different chain
- **No candle data**: may be a new token or low liquidity — inform user
- **Unsupported chain for signals**: not all chains support signals — always verify with SignalChains first
- **Empty signal list**: no signals on this chain for the given filters — suggest relaxing `walletType`, `minAmountUsd`, or `minAddressCount`, or try a different chain
- **Unsupported chain for meme pump**: only Solana (501), BSC (56), X Layer (196), TRON (195) are supported — verify with MemepumpChains first
- **Empty meme pump token list**: no tokens found for the given filters — suggest adjusting `sortBy`, `limit`, or trying a different chain
- **Network error**: retry once, then prompt user to try again later

## Amount Display Rules

- Always display in UI units (`1.5 ETH`), never base units
- Show USD value alongside (`1.5 ETH ≈ $4,500`)
- Prices are strings — handle precision carefully

## Global Notes

- EVM contract addresses must be **all lowercase**
- Use `chainIndex` parameter (not `chainId`)
- All output is JSON format
- Get your API token at [https://6551.io/mcp](https://6551.io/mcp)
- Each request consumes 1 quota unit

