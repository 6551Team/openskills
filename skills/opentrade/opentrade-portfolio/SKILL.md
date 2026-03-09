---
name: opentrade-portfolio
description: "This skill should be used when the user asks to 'check my wallet balance', 'show my token holdings', 'how much OKB do I have', 'what tokens do I have', 'check my portfolio value', 'view my assets', 'how much is my portfolio worth', 'what's in my wallet', 'show transaction history', 'recent transactions', or mentions checking wallet balance, total assets, token holdings, portfolio value, remaining funds, transaction history, or multi-chain balance lookup. Supports XLayer, Solana, Ethereum, Base, BSC, Arbitrum, Polygon, and 20+ other chains. Do NOT use for general programming questions about balance variables or API documentation. Do NOT use when the user is asking how to build or integrate a balance feature into code."
license: Apache-2.0
metadata:
  author: 6551
  version: "1.0.0"
  homepage: "https://6551.io"
---

# OpenTrade Wallet Portfolio CLI

4 commands for supported chains, wallet total value, all token balances, and specific token balances.

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

## Skill Routing

- For token prices / K-lines → use `opentrade-market`
- For token search / metadata → use `opentrade-token`
- For swap execution → use `opentrade-dex-swap`
- For transaction broadcasting → use `opentrade-transaction`

## Quickstart

```bash
# Get supported chains for balance queries
opentrade portfolio chains --router okx --version v1

# Get total asset value on XLayer
opentrade portfolio total-value --address 0xYourWallet --chain xlayer --router okx --version v1

# Get all token balances
opentrade portfolio all-balances --address 0xYourWallet --chain xlayer --router okx --version v1

# Check specific tokens (native OKB + USDC on XLayer)
opentrade portfolio token-balances --address 0xYourWallet --tokens "196:,196:0x74b7f16337b8972027f6196a17a631ac6de26d22" --router okx --version v1
```

## Chain Name Support

The CLI accepts human-readable chain names and resolves them automatically.

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
| Moonbeam | `moonbeam` | `1284` |
| Moonriver | `moonriver` | `1285` |
| Celo | `celo` | `42220` |
| Aurora | `aurora` | `1313161554` |
| Harmony | `harmony` | `1666600000` |
| Boba | `boba` | `288` |
| Metis | `metis` | `1088` |
| Klaytn | `klaytn` | `8217` |
| Fuse | `fuse` | `122` |

## Command Index

```bash
opentrade portfolio chains          # List supported chains
opentrade portfolio total-value     # Get total portfolio value
opentrade portfolio all-balances    # Get all token balances
opentrade portfolio token-balances  # Get specific token balances
```

---

## 1. Get Supported Chains

List all chains supported for balance queries.

```bash
opentrade portfolio chains --router okx --version v1
```

**Parameters:**
- `--router` (required): Router name from router discovery
- `--version` (required): Router version from router discovery

**Response:**
```json
{
  "data": [
    {"chainIndex": "1", "chainName": "Ethereum"},
    {"chainIndex": "56", "chainName": "BSC"},
    {"chainIndex": "137", "chainName": "Polygon"},
    {"chainIndex": "196", "chainName": "XLayer"},
    {"chainIndex": "501", "chainName": "Solana"},
    {"chainIndex": "8453", "chainName": "Base"}
  ],
  "success": true
}
```

**Example:**
```bash
opentrade portfolio chains --router okx --version v1
# → Display: Ethereum (1), BSC (56), Polygon (137), XLayer (196), Solana (501), Base (8453), ...
```

---

## 2. Get Total Portfolio Value

Get the total USD value of all assets in a wallet.

```bash
opentrade portfolio total-value \
  --address 0xYourWalletAddress \
  --chain xlayer \
  --router okx \
  --version v1
```

**Parameters:**
- `--address` (required): Wallet address (EVM or Solana format)
- `--chain` (required): Chain name or chainIndex (e.g., `xlayer`, `196`, `ethereum`, `1`)
- `--router` (required): Router name from router discovery
- `--version` (required): Router version from router discovery
- `--asset-type` (optional): `0`=all (default), `1`=tokens only, `2`=DeFi only
- `--exclude-risk` (optional): Exclude risky/scam tokens (only works on ETH/BSC/SOL/BASE)

**Response:**
```json
{
  "data": {
    "totalValue": "12345.67"
  },
  "success": true
}
```

**Examples:**

```bash
# Get total value on XLayer
opentrade portfolio total-value --address 0xYourWallet --chain xlayer --router okx --version v1
# → Display: Total Portfolio Value: $12,345.67

# Get only token value (exclude DeFi positions)
opentrade portfolio total-value --address 0xYourWallet --chain ethereum --asset-type 1 --router okx --version v1
# → Display: Token Value: $8,500.00

# Get only DeFi positions
opentrade portfolio total-value --address 0xYourWallet --chain ethereum --asset-type 2 --router okx --version v1
# → Display: DeFi Value: $3,845.67

# Exclude risky tokens on Ethereum
opentrade portfolio total-value --address 0xYourWallet --chain ethereum --exclude-risk --router okx --version v1
# → Display: Total Portfolio Value (Safe Assets): $11,200.00
```

---

## 3. Get All Token Balances

Get detailed balance information for all tokens in a wallet.

```bash
opentrade portfolio all-balances \
  --address 0xYourWalletAddress \
  --chain xlayer \
  --router okx \
  --version v1
```

**Parameters:**
- `--address` (required): Wallet address (EVM or Solana format)
- `--chain` (required): Chain name or chainIndex (e.g., `xlayer`, `196`)
- `--router` (required): Router name from router discovery
- `--version` (required): Router version from router discovery
- `--exclude-risk` (optional): Exclude risky/scam tokens (only works on ETH/BSC/SOL/BASE)

**Response:**
```json
{
  "data": [
    {
      "tokenAddress": "",
      "tokenSymbol": "OKB",
      "tokenName": "OKB",
      "balance": "10500000000000000000",
      "decimals": 18,
      "priceUsd": "48.50",
      "valueUsd": "509.25"
    },
    {
      "tokenAddress": "0x74b7f16337b8972027f6196a17a631ac6de26d22",
      "tokenSymbol": "USDC",
      "tokenName": "USD Coin",
      "balance": "2000000000",
      "decimals": 6,
      "priceUsd": "1.00",
      "valueUsd": "2000.00"
    }
  ],
  "success": true
}
```

**Examples:**

```bash
# Get all balances on XLayer
opentrade portfolio all-balances --address 0xYourWallet --chain xlayer --router okx --version v1
# → Display:
#   OKB:  10.5 ($509.25)
#   USDC: 2,000 ($2,000.00)
#   USDT: 1,500 ($1,500.00)
#   ...

# Get all balances excluding risky tokens
opentrade portfolio all-balances --address 0xYourWallet --chain ethereum --exclude-risk --router okx --version v1
# → Display: (only verified/safe tokens)

# Get balances on Solana
opentrade portfolio all-balances --address YourSolanaAddress --chain solana --router okx --version v1
# → Display:
#   SOL:  5.2 ($520.00)
#   USDC: 1,000 ($1,000.00)
#   ...
```

---

## 4. Get Specific Token Balances

Query balances for specific tokens only.

```bash
opentrade portfolio token-balances \
  --address 0xYourWalletAddress \
  --tokens "196:,196:0x74b7f16337b8972027f6196a17a631ac6de26d22" \
  --router okx \
  --version v1
```

**Parameters:**
- `--address` (required): Wallet address (EVM or Solana format)
- `--tokens` (required): Comma-separated list of `chainIndex:tokenAddress` pairs (max 20)
  - Native token: `chainIndex:` (empty address)
  - ERC-20 token: `chainIndex:0xContractAddress` (lowercase)
- `--router` (required): Router name from router discovery
- `--version` (required): Router version from router discovery

**Token Format:**
- `196:` → Native OKB on XLayer (chainIndex 196)
- `196:0x74b7f16337b8972027f6196a17a631ac6de26d22` → USDC on XLayer
- `1:` → Native ETH on Ethereum
- `1:0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48` → USDC on Ethereum
- `501:` → Native SOL on Solana

**Response:**
```json
{
  "data": [
    {
      "chainIndex": "196",
      "tokenAddress": "",
      "tokenSymbol": "OKB",
      "tokenName": "OKB",
      "balance": "10500000000000000000",
      "decimals": 18,
      "priceUsd": "48.50",
      "valueUsd": "509.25"
    },
    {
      "chainIndex": "196",
      "tokenAddress": "0x74b7f16337b8972027f6196a17a631ac6de26d22",
      "tokenSymbol": "USDC",
      "tokenName": "USD Coin",
      "balance": "2000000000",
      "decimals": 6,
      "priceUsd": "1.00",
      "valueUsd": "2000.00"
    }
  ],
  "success": true
}
```

**Examples:**

```bash
# Check native OKB and USDC on XLayer
opentrade portfolio token-balances --address 0xYourWallet --tokens "196:,196:0x74b7f16337b8972027f6196a17a631ac6de26d22" --router okx --version v1
# → Display:
#   OKB:  10.5 ($509.25)
#   USDC: 2,000 ($2,000.00)

# Check ETH and USDC on Ethereum
opentrade portfolio token-balances --address 0xYourWallet --tokens "1:,1:0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48" --router okx --version v1
# → Display:
#   ETH:  2.5 ($5,000.00)
#   USDC: 10,000 ($10,000.00)

# Check multiple tokens across chains
opentrade portfolio token-balances --address 0xYourWallet --tokens "196:,1:,56:0x55d398326f99059ff775485246999027b3197955" --router okx --version v1
# → Display:
#   OKB (XLayer):  10.5 ($509.25)
#   ETH (Ethereum): 2.5 ($5,000.00)
#   USDT (BSC):     5,000 ($5,000.00)
```

---

## Workflow Examples

### Check Portfolio Value

**User says:** "What's my total portfolio worth on XLayer?"

```bash
opentrade portfolio total-value --address 0xYourWallet --chain xlayer --router okx --version v1
# → Display: Total Portfolio Value: $12,345.67
```

### View All Holdings

**User says:** "Show me all my tokens on Ethereum"

```bash
opentrade portfolio all-balances --address 0xYourWallet --chain ethereum --router okx --version v1
# → Display:
#   ETH:  2.5 ($5,000.00)
#   USDC: 10,000 ($10,000.00)
#   USDT: 5,000 ($5,000.00)
#   DAI:  3,000 ($3,000.00)
#   ...
```

### Check Specific Tokens

**User says:** "How much OKB and USDC do I have on XLayer?"

```bash
opentrade portfolio token-balances --address 0xYourWallet --tokens "196:,196:0x74b7f16337b8972027f6196a17a631ac6de26d22" --router okx --version v1
# → Display:
#   OKB:  10.5 ($509.25)
#   USDC: 2,000 ($2,000.00)
```

### Multi-Chain Portfolio

**User says:** "What's my total portfolio across all chains?"

```bash
# Check XLayer
opentrade portfolio total-value --address 0xYourWallet --chain xlayer --router okx --version v1
# → $12,345.67

# Check Ethereum
opentrade portfolio total-value --address 0xYourWallet --chain ethereum --router okx --version v1
# → $25,000.00

# Check BSC
opentrade portfolio total-value --address 0xYourWallet --chain bsc --router okx --version v1
# → $8,500.00

# Display: Total Cross-Chain Portfolio: $45,845.67
```

### Safe Assets Only

**User says:** "Show my portfolio value excluding risky tokens on Ethereum"

```bash
opentrade portfolio total-value --address 0xYourWallet --chain ethereum --exclude-risk --router okx --version v1
# → Display: Total Portfolio Value (Safe Assets): $23,500.00
```

### DeFi Positions

**User says:** "How much do I have in DeFi on Ethereum?"

```bash
opentrade portfolio total-value --address 0xYourWallet --chain ethereum --asset-type 2 --router okx --version v1
# → Display: DeFi Value: $15,000.00
```

### Token-Only Value

**User says:** "What's my token balance worth (not DeFi)?"

```bash
opentrade portfolio total-value --address 0xYourWallet --chain ethereum --asset-type 1 --router okx --version v1
# → Display: Token Value: $10,000.00
```

---

## Edge Cases

- **Zero balance**: valid state — display `$0.00`, not an error
- **Unsupported chain**: call `opentrade portfolio chains` first to confirm
- **`--exclude-risk` not working**: only supported on ETH(`1`)/BSC(`56`)/SOL(`501`)/BASE(`8453`)
- **DeFi positions**: use `--asset-type 2` to query DeFi holdings separately
- **Address format mismatch**: EVM address on Solana chain will return empty data — do NOT mix
- **Network error**: retry once, then prompt user to try again later
- **Region restriction (error code 50125 or 80001)**: do NOT show the raw error code to the user. Instead, display a friendly message: `⚠️ Service is not available in your region. Please switch to a supported region and try again.`
- **Max 20 tokens**: `token-balances` supports max 20 token entries in `--tokens` parameter

## Amount Display Rules

- Token amounts in UI units (`1.5 ETH`), never base units (`1500000000000000000`)
- USD values with 2 decimal places (`$1,234.56`)
- Large amounts in shorthand (`$1.2M`, `$5.3K`)
- Sort by USD value descending in portfolio view
- Show percentage of total portfolio for each token
- Highlight tokens with significant value (>1% of portfolio)

## Global Notes

- `--chain` supports single chain name or chainIndex (e.g., `xlayer` or `196`)
- `--asset-type`: `0`=all `1`=tokens only `2`=DeFi only (only for `total-value`)
- `--exclude-risk` only works on ETH(`1`)/BSC(`56`)/SOL(`501`)/BASE(`8453`)
- `token-balances` supports max **20** token entries
- The CLI resolves chain names automatically (e.g., `ethereum` → `1`, `solana` → `501`)
- The CLI handles authentication internally via environment variables — see Pre-flight Checks step 4
- EVM contract addresses must be **all lowercase**
- Native token (ETH, BNB, OKB, etc.) is represented with empty string in token format: `chainIndex:`
- Balances are returned in minimal units (wei for EVM, lamports for Solana)
- Values are calculated in USD based on current market prices
- Get your API token at https://6551.io/mcp
- Each request consumes 1 quota unit
