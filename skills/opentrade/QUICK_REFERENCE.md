# OpenTrade Quick Reference

Quick reference guide for trading operations via 6551 platform.

## Setup

```bash
export OPEN_TOKEN="your_token_here"
```

Get your token at: https://6551.io/mcp

## Base URL

```
https://ai.6551.io/open/trader/{router}/{version}
```

**Router Discovery**: Call `https://ai.6551.io/open/trader/routers` to get available routers.
**Default**: Use `router=okx` and `version=v1` if no routers are returned.

## Skills Overview

| Skill | Emoji | Endpoints | Primary Use |
|-------|-------|-----------|-------------|
| opentrade-dex-swap | 🔄 | 5 | Execute swaps |
| opentrade-transaction | 📡 | 6 | Manage transactions |
| opentrade-portfolio | 💰 | 4 | Check balances |
| opentrade-market | 📊 | 9 | Market data |
| opentrade-token | 🪙 | 5 | Token info |

## Common Commands

### Check Supported Chains

```bash
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/swap/supported/chain"
```

### Get Token Price

```bash
curl -s -X POST \
  -H "Authorization: Bearer $OPEN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '[{"chainIndex": "1", "tokenContractAddress": "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"}]' \
  "https://ai.6551.io/open/trader/{router}/{version}/market/price"
```

### Get Swap Quote

```bash
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/swap/quote?chainIndex=1&fromTokenAddress=0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48&toTokenAddress=0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2&amount=1000000&swapMode=exactIn"
```

### Get Wallet Balances

```bash
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/wallet/all-token-balances?address=0x...&chainIndex=1"
```

### Search Tokens

```bash
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/market/token/search?chains=1,501&search=USDC"
```

### Get Gas Price

```bash
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/transaction/gas-price?chainIndex=1"
```

### Broadcast Transaction

```bash
curl -s -X POST \
  -H "Authorization: Bearer $OPEN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "signedTx": "0x...",
    "chainIndex": "1",
    "address": "0x..."
  }' \
  "https://ai.6551.io/open/trader/{router}/{version}/transaction/broadcast-transaction"
```

### Track Order Status

```bash
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/transaction/orders?address=0x...&chainIndex=1&orderId=12345"
```

## Chain IDs

| Chain | ID |
|-------|----|
| Ethereum | 1 |
| BSC | 56 |
| Polygon | 137 |
| Arbitrum | 42161 |
| Optimism | 10 |
| Avalanche | 43114 |
| Base | 8453 |
| XLayer | 196 |
| Solana | 501 |

## Response Format

```json
{
  "code": 200,
  "message": "success",
  "data": { ... },
  "usage": 1
}
```

## Error Codes

| Code | Meaning |
|------|---------|
| 200 | Success |
| 400 | Bad request |
| 401 | Unauthorized |
| 403 | Forbidden |
| 404 | Not found |
| 429 | Rate limit |
| 500 | Server error |
| 503 | Unavailable |

## Workflow Examples

### Complete Swap

```bash
# 1. Get quote
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/swap/quote?..."

# 2. Get approval (if needed)
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/swap/approve-transaction?..."

# 3. Get swap transaction
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/swap/swap?..."

# 4. Get gas price
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/transaction/gas-price?chainIndex=1"

# 5. Broadcast (after signing)
curl -s -X POST -H "Authorization: Bearer $OPEN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"signedTx": "0x...", "chainIndex": "1", "address": "0x..."}' \
  "https://ai.6551.io/open/trader/{router}/{version}/transaction/broadcast-transaction"

# 6. Track status
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/transaction/orders?..."
```

### Portfolio Check

```bash
# 1. Get all balances
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/wallet/all-token-balances?address=0x...&chainIndex=1"

# 2. Get total value
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/wallet/total-value?address=0x...&chainIndex=1"

# 3. Get transaction history
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/wallet/address-transactions?address=0x...&chainIndex=1&limit=20"
```

## Tips

- Use batch queries for multiple tokens
- Simulate transactions before broadcasting
- Monitor smart money signals for insights
- Cache token info to reduce API calls
- Implement retry logic with exponential backoff

## Documentation

- [Full Documentation](./README.md)
- [API Mapping](./API_MAPPING.md)
- Individual skill docs in each skill directory

## Support

- Platform: https://6551.io
- API Token: https://6551.io/mcp
- Documentation: https://docs.6551.io
