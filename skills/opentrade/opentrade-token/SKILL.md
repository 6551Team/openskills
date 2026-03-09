---
name: opentrade-token
description: "This skill should be used when the user asks to 'find a token', 'search for a token', 'look up PEPE', 'what's trending', 'top tokens', 'trending tokens on Solana', 'token rankings', 'who holds this token', 'holder distribution', 'is this token safe', 'token market cap', 'token liquidity', 'research a token', 'tell me about this token', 'token info', or mentions searching for tokens by name or address, discovering trending tokens, viewing token rankings, checking holder distribution, or analyzing token market cap and liquidity. Covers token search, metadata, market cap, liquidity, volume, trending token rankings, and holder analysis across XLayer, Solana, Ethereum, Base, BSC, Arbitrum, Polygon, and 20+ other chains. Do NOT use when the user says only a single generic word like 'tokens' or 'crypto' without specifying a token name, action, or question. For simple current price checks, price charts, candlestick data, or trade history, use opentrade-market instead."
license: Apache-2.0
metadata:
  author: 6551
  version: "1.0.1"
  homepage: "https://6551.io"
---

# OpenTrade DEX Token Info CLI

5 commands for token search, metadata, detailed pricing, rankings, and holder distribution.

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
4. Create a `.env` file in the project root to override the default API credentials (optional — skip this for quick start):
   ```
   OPEN_TOKEN=your_token_here
   ```
   Get your API token at: https://6551.io/mcp

## Skill Routing

- For real-time prices / K-lines / trade history → use `opentrade-market`
- For swap execution → use `opentrade-dex-swap`
- For transaction broadcasting → use `opentrade-transaction`
- For wallet balances / portfolio → use `opentrade-portfolio`

## Router Discovery

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
- **Default fallback**: If the API returns no data or is empty, use `router=okx` and `version=v1`

## Quickstart

```bash
# Search token
opentrade token search xETH --chains "ethereum,solana"

# Get detailed price info
opentrade token price-info 0xe7b000003a45145decf8a28fc755ad5ec5ea025a --chain xlayer

# What's trending on Solana by volume?
opentrade token toplist --chains solana --sort-by 5 --time-frame 4

# Check holder distribution
opentrade token holders 0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee --chain xlayer
```

## Chain Name Support

The CLI accepts human-readable chain names (e.g., `ethereum`, `solana`, `xlayer`) and resolves them automatically.

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
| Fantom | `fantom` | `250` |
| Cronos | `cronos` | `25` |
| Gnosis | `gnosis` | `100` |
| Klaytn | `klaytn` | `8217` |
| Aurora | `aurora` | `1313161554` |
| Harmony | `harmony` | `1666600000` |
| Moonbeam | `moonbeam` | `1284` |
| Moonriver | `moonriver` | `1285` |
| Celo | `celo` | `42220` |
| Fuse | `fuse` | `122` |
| OKC | `okc` | `66` |
| Heco | `heco` | `128` |
| Metis | `metis` | `1088` |
| Boba | `boba` | `288` |
| zkSync Era | `zksync` | `324` |
| Polygon zkEVM | `polygon-zkevm` | `1101` |
| Linea | `linea` | `59144` |
| Mantle | `mantle` | `5000` |
| Scroll | `scroll` | `534352` |
| Blast | `blast` | `81457` |

## Command Index

| Command | Description |
|---|---|
| `opentrade token search` | Search tokens by name/symbol/address |
| `opentrade token info` | Get basic token metadata |
| `opentrade token price-info` | Get detailed price, liquidity, volume |
| `opentrade token toplist` | Get trending/top tokens by various metrics |
| `opentrade token holders` | Get holder distribution and top holders |

---

## 1. Token Search

Search for tokens by name, symbol, or contract address across multiple chains.

### Command

```bash
opentrade token search <query> [options]
```

### Options

| Option | Type | Required | Description |
|---|---|---|---|
| `<query>` | string | Yes | Token name, symbol, or contract address |
| `--chains` | string | No | Comma-separated chain names (e.g., "ethereum,solana") |
| `--limit` | number | No | Max results (default: 20, max: 100) |

### Examples

```bash
# Search for USDC across all chains
opentrade token search USDC

# Search on specific chains
opentrade token search PEPE --chains "ethereum,base"

# Search by contract address
opentrade token search 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48 --chain ethereum

# Limit results
opentrade token search ETH --limit 5
```

### Response Fields

- `tokenContractAddress`: Contract address (lowercase for EVM)
- `tokenSymbol`: Token symbol
- `tokenName`: Full token name
- `chainIndex`: Chain identifier
- `decimals`: Token decimals
- `totalSupply`: Total supply in base units
- `circulatingSupply`: Circulating supply
- `marketCap`: Market capitalization in USD
- `price`: Current price in USD
- `priceChange24h`: 24h price change percentage
- `volume24h`: 24h trading volume in USD
- `liquidity`: Total liquidity in USD
- `communityRecognized`: Whether token is verified/recognized

### Notes

- Returns up to 100 results
- EVM addresses must be lowercase
- Use contract address for exact match
- Symbol search may return multiple tokens

---

## 2. Token Info

Get basic metadata for a specific token.

### Command

```bash
opentrade token info <address> --chain <chain>
```

### Options

| Option | Type | Required | Description |
|---|---|---|---|
| `<address>` | string | Yes | Token contract address |
| `--chain` | string | Yes | Chain name (e.g., "ethereum", "solana") |

### Examples

```bash
# Get USDC info on Ethereum
opentrade token info 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48 --chain ethereum

# Get SOL info on Solana
opentrade token info So11111111111111111111111111111111111111112 --chain solana
```

### Response Fields

- `tokenContractAddress`: Contract address
- `tokenSymbol`: Token symbol
- `tokenName`: Full token name
- `chainIndex`: Chain identifier
- `decimals`: Token decimals
- `totalSupply`: Total supply
- `logoUrl`: Token logo URL
- `websiteUrl`: Official website
- `twitterUrl`: Twitter/X profile
- `telegramUrl`: Telegram group
- `discordUrl`: Discord server
- `communityRecognized`: Verification status

### Notes

- Returns single token metadata
- Use for detailed token information
- Logo URLs may be empty for unverified tokens

---

## 3. Token Price Info

Get detailed price, liquidity, volume, and market data for a token.

### Command

```bash
opentrade token price-info <address> --chain <chain>
```

### Options

| Option | Type | Required | Description |
|---|---|---|---|
| `<address>` | string | Yes | Token contract address |
| `--chain` | string | Yes | Chain name |

### Examples

```bash
# Get WETH price info on Ethereum
opentrade token price-info 0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2 --chain ethereum

# Get BONK price info on Solana
opentrade token price-info DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 --chain solana
```

### Response Fields

- `price`: Current price in USD
- `priceChange1h`: 1h price change %
- `priceChange4h`: 4h price change %
- `priceChange12h`: 12h price change %
- `priceChange24h`: 24h price change %
- `volume24h`: 24h trading volume
- `liquidity`: Total liquidity in USD
- `liquidityChange24h`: 24h liquidity change %
- `marketCap`: Market capitalization
- `fullyDilutedValuation`: FDV
- `holders`: Number of token holders
- `transactions24h`: 24h transaction count
- `buys24h`: 24h buy count
- `sells24h`: 24h sell count

### Notes

- More detailed than basic token info
- Includes time-series price changes
- Use for trading decisions

---

## 4. Token Top List (Trending)

Get trending or top-ranked tokens by various metrics.

### Command

```bash
opentrade token toplist [options]
```

### Options

| Option | Type | Required | Description |
|---|---|---|---|
| `--chains` | string | No | Comma-separated chain names |
| `--sort-by` | number | No | Sort metric (see table below) |
| `--time-frame` | number | No | Time window (see table below) |
| `--limit` | number | No | Max results (default: 10, max: 100) |

### Sort By Options

| Value | Metric |
|---|---|
| `1` | Market Cap |
| `2` | Price Change % |
| `3` | Liquidity |
| `4` | Holders |
| `5` | Volume |
| `6` | Transactions |

### Time Frame Options

| Value | Period |
|---|---|
| `1` | 1 hour |
| `2` | 4 hours |
| `3` | 12 hours |
| `4` | 24 hours |

### Examples

```bash
# Top tokens by market cap (default)
opentrade token toplist

# Trending on Solana by 24h volume
opentrade token toplist --chains solana --sort-by 5 --time-frame 4

# Top 5 tokens by liquidity on Ethereum
opentrade token toplist --chains ethereum --sort-by 3 --limit 5

# Biggest gainers in last 1h across all chains
opentrade token toplist --sort-by 2 --time-frame 1
```

### Response Fields

- `rank`: Position in ranking
- `tokenContractAddress`: Contract address
- `tokenSymbol`: Token symbol
- `tokenName`: Token name
- `chainIndex`: Chain identifier
- `price`: Current price
- `priceChange`: Price change % for time frame
- `volume`: Volume for time frame
- `liquidity`: Current liquidity
- `marketCap`: Market capitalization
- `holders`: Number of holders
- `transactions`: Transaction count for time frame

### Notes

- Default sort: market cap, 24h time frame
- Results are cached for 5 minutes
- Use for discovering trending tokens

---

## 5. Token Holders

Get holder distribution and top holders for a token.

### Command

```bash
opentrade token holders <address> --chain <chain> [options]
```

### Options

| Option | Type | Required | Description |
|---|---|---|---|
| `<address>` | string | Yes | Token contract address |
| `--chain` | string | Yes | Chain name |
| `--limit` | number | No | Max holders to return (default: 20, max: 100) |

### Examples

```bash
# Get top 20 holders
opentrade token holders 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48 --chain ethereum

# Get top 50 holders
opentrade token holders DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 --chain solana --limit 50
```

### Response Fields

- `totalHolders`: Total number of holders
- `top10Concentration`: % held by top 10 holders
- `top50Concentration`: % held by top 50 holders
- `top100Concentration`: % held by top 100 holders
- `holders`: Array of top holders:
  - `rank`: Holder rank
  - `address`: Wallet address
  - `balance`: Token balance (UI units)
  - `percentage`: % of total supply
  - `value`: USD value of holdings
  - `isContract`: Whether address is a contract
  - `tag`: Address tag (e.g., "DEX", "CEX", "Team")

### Notes

- High concentration = higher manipulation risk
- Contract holders may be liquidity pools
- Use for risk assessment

---

## Usage Examples

**User says:** "Find PEPE token"

```bash
opentrade token search PEPE
# → Display: Multiple PEPE tokens found across chains
#   1. PEPE on Ethereum (0x6982508145454ce325ddbe47a25d4ec3d2311933)
#   2. PEPE on Base (0x...)
#   ...
```

**User says:** "Show me BONK details"

```bash
# First search
opentrade token search BONK --chains solana

# Then get detailed info
opentrade token price-info DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263 --chain solana
# → Display: Price, volume, liquidity, market cap, holders, etc.
```

**User says:** "What's trending on Solana by volume?"

```bash
opentrade token toplist --chains solana --sort-by 5 --time-frame 4
# → Display top tokens sorted by 24h volume:
#   #1 SOL  - Vol: $1.2B | Change: +3.5% | MC: $80B
#   #2 BONK - Vol: $450M | Change: +12.8% | MC: $1.5B
#   ...
```

**User says:** "Who are the top holders of this token?"

```bash
opentrade token holders 0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee --chain xlayer
# → Display top 20 holders with amounts and addresses
```

## Edge Cases

- **Token not found**: suggest verifying the contract address (symbols can collide)
- **Same symbol on multiple chains**: show all matches with chain names
- **Unverified token**: `communityRecognized = false` — warn user about risk
- **Too many results**: name/symbol search caps at 100 — suggest using exact contract address
- **Network error**: retry once
- **Region restriction (error code 50125 or 80001)**: do NOT show the raw error code to the user. Instead, display a friendly message: `⚠️ Service is not available in your region. Please switch to a supported region and try again.`

## Amount Display Rules

- Use appropriate precision: 2 decimals for high-value, significant digits for low-value
- Market cap / liquidity in shorthand ($1.2B, $45M)
- 24h change with sign and color hint (+X% / -X%)

## Global Notes

- Use contract address as **primary identity** — symbols can collide across tokens
- `communityRecognized = true` means listed on Top 10 CEX or community verified
- The CLI resolves chain names automatically (e.g., `ethereum` → `1`, `solana` → `501`)
- EVM addresses must be **all lowercase**
- The CLI handles authentication internally via environment variables — see Pre-flight Checks step 4 for authentication setup
- Get your API token at https://6551.io/mcp
