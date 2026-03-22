# OpenTrade Skills

A collection of skills for interacting with OKX DEX aggregator APIs via the 6551 platform.

## Overview

OpenTrade provides 7 specialized skills for comprehensive blockchain trading operations:

1. **opentrade-dex-swap** - DEX swap operations (quote, swap, approve, liquidity)
2. **opentrade-transaction** - Transaction management (gas, simulation, broadcast, tracking)
3. **opentrade-portfolio** - Wallet / Portfolio operations (balances, portfolio value, transaction history)
4. **opentrade-market** - Market data (prices, K-line, trades, smart money signals)
5. **opentrade-token** - Token information (search, info, holders, trending)
6. **opentrade-wallet** - Custodial wallet management (create wallet, get account, swap, withdraw). Supports BSC and Solana only.
7. **opentrade-newsliquid** - CEX trading via Newsliquid gateway (spot & futures orders, positions, leverage, account management, wallet agent). Server-side execution with built-in risk controls.

## Quick Start

### 1. Get Your API Token

Visit https://6551.io/mcp to obtain your API token.

### 2. Set Environment Variable

```bash
export OPEN_TOKEN="your_token_here"
```

Or add to your `.env` file:

```bash
OPEN_TOKEN=your_token_here
```

### 3. Test Connection

```bash
# Test with a simple API call
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/swap/chains"
```

## Skills Overview

### 🔄 opentrade-dex-swap

Execute DEX swaps across multiple chains.

**Key Features:**
- Get swap quotes
- Generate swap transactions
- Approve ERC-20 tokens
- Query supported chains and liquidity sources
- View swap history

**Example:**
```bash
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/swap/quote?chainIndex=1&fromTokenAddress=0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48&toTokenAddress=0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2&amount=1000000&swapMode=exactIn"
```

### 📡 opentrade-transaction

Manage blockchain transactions end-to-end.

**Key Features:**
- Get current gas prices
- Estimate gas limits
- Simulate transactions (dry-run)
- Broadcast signed transactions
- Track order status

**Example:**
```bash
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/transaction/gas-price?chainIndex=1"
```

### 💰 opentrade-portfolio

Query portfolio balances and transaction history.

**Key Features:**
- Get token balances (single or batch)
- Get all token balances
- Calculate total portfolio value
- View transaction history

**Example:**
```bash
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/wallet/balance?address=0x...&chainIndex=1"
```

### 📊 opentrade-market

Access comprehensive market data.

**Key Features:**
- Get token prices (single or batch)
- View K-line / candlestick data
- Get recent trades
- Monitor smart money / KOL / whale signals
- Get index prices

**Example:**
```bash
curl -s -X POST \
  -H "Authorization: Bearer $OPEN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '[{"chainIndex": "1", "tokenContractAddress": "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"}]' \
  "https://ai.6551.io/open/trader/{router}/{version}/market/price"
```

### 🪙 opentrade-token

Discover and analyze tokens.

**Key Features:**
- Search tokens by name/symbol/address
- Get token basic info
- View holder distribution
- Find trending tokens

**Example:**
```bash
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/token/search?chains=1,501&search=USDC"
```

### 🔐 opentrade-wallet

Manage custodial wallets powered by [Turnkey](https://www.turnkey.com/). **Only supports BSC and Solana networks.**

**Security Architecture:**
- Private keys are managed by **Turnkey** with **AWS KMS** (Key Management Service) as delegated custody
- **6551 does NOT store your private keys** — all signing operations are performed within Turnkey's secure infrastructure backed by AWS KMS
- Your wallet is non-extractable: no one (including 6551) can export the raw private key

**Key Features:**
- Create custodial wallet (BSC + Solana addresses)
- Get custodial account info
- Execute DEX swap (auto-sign + broadcast)
- Withdraw native tokens (BNB on BSC, SOL on Solana)

> **Note**: Newly created wallets have zero balance. You must deposit **BNB** (BSC network) or **SOL** (Solana network) before trading. Do NOT send tokens from other chains — funds will be lost.

**Example:**
```bash
# Create custodial wallet
curl -s -X POST -H "Authorization: Bearer $OPEN_TOKEN" \
  -H "Content-Type: application/json" \
  "https://ai.6551.io/trader/custodial/create"

# Get account
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/trader/custodial/account"

# Withdraw 1 BNB
curl -s -X POST -H "Authorization: Bearer $OPEN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"network":"bsc","to":"0xRecipient","amount":1000000000000000000}' \
  "https://ai.6551.io/trader/custodial/withdraw"
```

### 📈 opentrade-newsliquid

CEX (centralized exchange) trading via the Newsliquid gateway. Server-side execution with built-in risk controls — no private key management required.

**Key Features:**
- Market data: ticker, K-lines, trading pair metadata
- Account management: balance summary, spot assets
- Order management: place/edit/cancel limit, market, stop-loss, take-profit orders
- Position management: view, close, historical positions
- Leverage & margin: set leverage, margin mode, position mode
- Wallet agent: create and manage wallet agents for CEX↔chain fund transfers
- Built-in 4-layer risk engine (price deviation, position limit, rate limit, balance check)

**Example:**
```bash
# Get BTC ticker
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/newsliquid/v1/market/ticker?symbol=BTCUSDT&exchangeId=binance"

# Place a limit buy order
curl -s -X POST -H "Authorization: Bearer $OPEN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"symbol":"BTCUSDT","side":"buy","type":"limit","quantity":"0.001","price":"60000","exchangeId":"binance"}' \
  "https://ai.6551.io/open/trader/newsliquid/v1/orders"

# Check positions
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/newsliquid/v1/positions?exchangeId=binance"
```

## Common Workflows

### Complete Swap Flow

```bash
# 1. Get swap quote
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/swap/quote?chainIndex=1&fromTokenAddress=0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48&toTokenAddress=0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2&amount=1000000&swapMode=exactIn"

# 2. Get approval transaction (if needed)
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/swap/approve-transaction?chainIndex=1&tokenContractAddress=0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48&approveAmount=1000000"

# 3. Get swap transaction
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/swap/swap?chainIndex=1&fromTokenAddress=0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48&toTokenAddress=0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2&amount=1000000&slippagePercent=1&userWalletAddress=0x..."

# 4. Get gas price
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/transaction/gas-price?chainIndex=1"

# 5. Broadcast transaction (after signing)
curl -s -X POST \
  -H "Authorization: Bearer $OPEN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"signedTx": "0x...", "chainIndex": "1", "address": "0x..."}' \
  "https://ai.6551.io/open/trader/{router}/{version}/transaction/broadcast-transaction"

# 6. Track order status
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/transaction/orders?address=0x...&chainIndex=1&orderId=12345"
```

### Portfolio Analysis

```bash
# 1. Get all token balances
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/wallet/all-token-balances?address=0x...&chainIndex=1"

# 2. Get total portfolio value
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/wallet/total-value?address=0x...&chainIndex=1"

# 3. Get transaction history
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/wallet/address-transactions?address=0x...&chainIndex=1&limit=20"
```

### Market Research

```bash
# 1. Find trending tokens
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/market/token/toplist?chains=1&sortBy=5&timeFrame=4"

# 2. Get token details
curl -s -X POST \
  -H "Authorization: Bearer $OPEN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '[{"chainIndex": "1", "tokenContractAddress": "0x..."}]' \
  "https://ai.6551.io/open/trader/{router}/{version}/market/token/basic-info"

# 3. Get price data
curl -s -X POST \
  -H "Authorization: Bearer $OPEN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '[{"chainIndex": "1", "tokenContractAddress": "0x..."}]' \
  "https://ai.6551.io/open/trader/{router}/{version}/market/price-info"

# 4. Get K-line data
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/market/candles?chainIndex=1&tokenContractAddress=0x...&bar=1H&limit=24"

# 5. Monitor smart money signals
curl -s -X POST \
  -H "Authorization: Bearer $OPEN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"chainIndex": "1", "walletType": "1", "minAmountUsd": "50000"}' \
  "https://ai.6551.io/open/trader/{router}/{version}/market/signal/list"
```

## Supported Chains

| Chain | Chain ID |
|-------|----------|
| Ethereum | 1 |
| BSC | 56 |
| Polygon | 137 |
| Arbitrum | 42161 |
| Optimism | 10 |
| Avalanche | 43114 |
| Base | 8453 |
| XLayer | 196 |
| Solana | 501 |

## API Response Format

All endpoints return responses in this format:

```json
{
  "code": 200,
  "message": "success",
  "data": {
    // Response data
  },
  "usage": 1
}
```

## Error Handling

Common error codes:

| Code | Description |
|------|-------------|
| 200 | Success |
| 400 | Bad request (invalid parameters) |
| 401 | Unauthorized (invalid or missing token) |
| 403 | Forbidden (insufficient permissions) |
| 404 | Not found |
| 429 | Rate limit exceeded |
| 500 | Internal server error |
| 503 | Service unavailable |

## Rate Limits

- Each request consumes 1 quota unit
- Rate limits depend on your subscription plan
- Check your remaining quota in the response headers

## Best Practices

1. **Simulate transactions** before broadcasting
2. **Use batch queries** when fetching multiple token prices
3. **Implement retry logic** with exponential backoff
4. **Monitor smart money signals** for market insights
5. **Cache token info** to reduce API calls
6. **Use appropriate slippage** for volatile markets

## Documentation

- [API Mapping](./API_MAPPING.md) - Complete endpoint reference
- Individual skill documentation in each skill directory

## Support

- Platform: https://6551.io
- API Token: https://6551.io/mcp
- Documentation: https://docs.6551.io

## License

See main project LICENSE file.
