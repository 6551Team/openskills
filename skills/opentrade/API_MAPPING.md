# OpenTrade API Endpoint Mapping

This document maps the trading API endpoints to the corresponding skills in `openskills/skills/opentrade/`.

## Base URL

```
https://ai.6551.io/open/trader/{router}/{version}
```

**Router Discovery**: Use `https://ai.6551.io/open/trader/routers` to get available routers and versions.
**Default**: If no routers are returned, use `router=okx` and `version=v1`.

## Authentication

All endpoints require JWT Bearer Token authentication:
```
Authorization: Bearer <jwt_token>
```

---

## Skill: opentrade-dex-swap

DEX aggregator swap operations.

| Method | Endpoint | Description | Skill |
|--------|----------|-------------|-------|
| GET | `/{router}/{version}/swap/quote` | Get swap quote (read-only price estimate) | opentrade-dex-swap |
| GET | `/{router}/{version}/swap/swap` | Get swap transaction data | opentrade-dex-swap |
| GET | `/{router}/{version}/swap/approve` | Get ERC-20 approval transaction | opentrade-dex-swap |
| GET | `/{router}/{version}/swap/chains` | Get supported chains | opentrade-dex-swap |
| GET | `/{router}/{version}/swap/liquidity` | Get liquidity sources | opentrade-dex-swap |

---

## Skill: opentrade-transaction

Transaction gateway operations (gas, simulation, broadcast, tracking).

| Method | Endpoint | Description | Skill |
|--------|----------|-------------|-------|
| GET | `/{router}/{version}/transaction/gas-price` | Get current gas prices | opentrade-transaction |
| POST | `/{router}/{version}/transaction/gas-limit` | Estimate gas limit | opentrade-transaction |
| POST | `/{router}/{version}/transaction/simulate` | Simulate transaction (dry-run) | opentrade-transaction |
| GET | `/{router}/{version}/transaction/chains` | Get supported chains | opentrade-transaction |
| POST | `/{router}/{version}/transaction/broadcast` | Broadcast signed transaction | opentrade-transaction |
| GET | `/{router}/{version}/transaction/status` | Track transaction status | opentrade-transaction |

---

## Skill: opentrade-wallet

Wallet balance and transaction operations.

| Method | Endpoint | Description | Skill |
|--------|----------|-------------|-------|
| POST | `/{router}/{version}/wallet/balance` | Get token balance | opentrade-wallet |
| GET | `/{router}/{version}/wallet/balances` | Get all token balances | opentrade-wallet |
| GET | `/{router}/{version}/wallet/value` | Get total portfolio value | opentrade-wallet |
| GET | `/{router}/{version}/wallet/history` | Get address transaction history | opentrade-wallet |

---

## Skill: opentrade-market

Market data operations (prices, K-line, trades, signals).

| Method | Endpoint | Description | Skill |
|--------|----------|-------------|-------|
| POST | `/{router}/{version}/market/price` | Get token price (single or batch) | opentrade-market |
| GET | `/{router}/{version}/market/kline` | Get K-line / candlestick data | opentrade-market |
| GET | `/{router}/{version}/market/trades` | Get recent trades | opentrade-market |
| POST | `/{router}/{version}/market/current-price` | Get index price (aggregated) | opentrade-market |
| GET | `/{router}/{version}/market/signal-chains` | Get supported chains for signals | opentrade-market |
| POST | `/{router}/{version}/market/signal-list` | Get smart money / KOL / whale signals | opentrade-market |
| GET | `/{router}/{version}/market/memepump-chains` | Get meme pump supported chains | opentrade-market |
| POST | `/{router}/{version}/market/memepump-tokens` | Get meme pump tokens | opentrade-market |
| GET | `/{router}/{version}/market/memepump-protocols` | Get meme pump protocols | opentrade-market |

---

## Skill: opentrade-token

Token information operations (search, info, holders, trending).

| Method | Endpoint | Description | Skill |
|--------|----------|-------------|-------|
| GET | `/{router}/{version}/token/search` | Search tokens by name/symbol/address | opentrade-token |
| POST | `/{router}/{version}/token/info` | Get token basic info | opentrade-token |
| POST | `/{router}/{version}/token/price-info` | Get token price info | opentrade-token |
| GET | `/{router}/{version}/token/holders` | Get token holder distribution | opentrade-token |
| GET | `/{router}/{version}/token/trending` | Get trending / top tokens | opentrade-token |

---

## Summary Statistics

- **Total Skills**: 5
- **Total Endpoints**: 29
- **GET Endpoints**: 16
- **POST Endpoints**: 13

## Skill Distribution

| Skill | Endpoints | Primary Function |
|-------|-----------|------------------|
| opentrade-dex-swap | 5 | DEX swap operations |
| opentrade-transaction | 6 | Transaction management |
| opentrade-wallet | 4 | Wallet & balance queries |
| opentrade-market | 9 | Market data & signals |
| opentrade-token | 5 | Token information |

## Common Parameters

### Chain IDs

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

### Request/Response Format

All endpoints follow the same response format:

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

## Skill Collaboration Examples

### Complete Swap Workflow

```
opentrade-token → opentrade-market → opentrade-dex-swap → opentrade-transaction
```

1. **opentrade-token**: Search and discover tokens
2. **opentrade-market**: Analyze token prices and market data
3. **opentrade-dex-swap**: Get quote and swap transaction
4. **opentrade-transaction**: Broadcast and track transaction

### Portfolio Management Workflow

```
opentrade-wallet → opentrade-market → opentrade-token
```

1. **opentrade-wallet**: Get all token balances and total value
2. **opentrade-market**: Get current prices for held tokens
3. **opentrade-token**: Get detailed info for each token

### Market Analysis Workflow

```
opentrade-token → opentrade-market
```

1. **opentrade-token**: Find trending tokens
2. **opentrade-market**: Analyze price data, K-line, and smart money signals

## Notes

- All endpoints require `trader_router_okx` skill permission
- Use `chainIndex` instead of `chainId` for all parameters
- Amounts must be in minimal units (wei for EVM, lamports for Solana)
- Each request consumes 1 quota unit
- Get your API token at https://6551.io/mcp
