---
name: opentrade-newsliquid
description: "This skill should be used when the user asks to 'place a CEX order', 'trade on centralized exchange', 'buy BTC on CEX', 'sell ETH futures', 'open a long position', 'open a short position', 'close my position', 'set leverage', 'check my CEX balance', 'show my open orders', 'cancel my order', 'check CEX ticker', 'get K-line data', 'set margin mode', 'check my CEX positions', 'view trade history', 'manage wallet agent', or mentions CEX trading, futures, contracts, leverage, margin, limit orders, market orders, stop-loss, take-profit, or newsliquid. This is for centralized exchange operations only. Do NOT use for DEX swaps (use opentrade-dex-swap), on-chain balances (use opentrade-portfolio), on-chain market data (use opentrade-market), token search (use opentrade-token), custodial wallet (use opentrade-wallet), or transaction broadcasting (use opentrade-gateway)."
license: MIT
metadata:
  author: 6551
  version: "1.0.0"
  homepage: "https://6551.io"
---

# OpenTrade Newsliquid CEX Trading

29 API endpoints for centralized exchange trading — market data, account management, spot & futures orders, positions, leverage, and wallet agent.

> **IMPORTANT**: This is a **CEX (centralized exchange)** trading skill. All trades are executed server-side with built-in risk controls — no private key management or transaction signing required.
>
> **IMPORTANT**: Write operations (place order, edit order, close position, set leverage) are protected by a 4-layer risk engine: price deviation check, position limit, rate limit, and balance verification.

## Pre-flight Checks

Every time before running any newsliquid command, always follow these steps in order:

1. Find or create a `.env` file in the project root to load the API credentials:
  ```bash
  OPEN_TOKEN=your_token_here
  ```

  Get your API token at: https://6551.io/mcp

  **Security warning**: Never commit .env to git (add it to .gitignore) and never expose credentials in logs, screenshots, or chat messages.

2. Set the base URL and auth header:
  ```bash
  BASE_URL="https://ai.6551.io"
  AUTH_HEADER="Authorization: Bearer $OPEN_TOKEN"
  ```

## Skill Routing

- For DEX swaps / on-chain token exchange → use `opentrade-dex-swap`
- For on-chain wallet balances / portfolio → use `opentrade-portfolio`
- For on-chain market data / smart money signals → use `opentrade-market`
- For token search / holders / trending → use `opentrade-token`
- For custodial wallet (BSC/Solana) → use `opentrade-wallet`
- For transaction broadcasting / gas → use `opentrade-gateway`
- For CEX trading (spot, futures, leverage, orders, positions) → use this skill (`opentrade-newsliquid`)

## Supported Exchanges

| ExchangeID | Name |
|------------|------|
| `binance` | Binance |
| `bybit` | Bybit |
| `okx` | OKX |
| `hyperliquid` | Hyperliquid |
| `aster` | Aster |

## Quickstart

```bash
# 1. Get real-time ticker
curl -s "$BASE_URL/open/trader/newsliquid/v1/market/ticker?symbol=BTC/USDT&exchangeId=binance" \
  -H "$AUTH_HEADER"

# 2. Check account balance
curl -s "$BASE_URL/open/trader/newsliquid/v1/account/summary?exchangeId=binance&symbol=BTC/USDT:USDT" \
  -H "$AUTH_HEADER"

# 3. Place a limit buy order
curl -s -X POST "$BASE_URL/open/trader/newsliquid/v1/orders" \
  -H "$AUTH_HEADER" -H "Content-Type: application/json" \
  -d '{"symbol":"BTC/USDT:USDT","side":"buy","type":"limit","quantity":0.001,"price":60000,"exchangeId":"binance"}'

# 4. Check open orders
curl -s "$BASE_URL/open/trader/newsliquid/v1/orders/open?exchangeId=binance" \
  -H "$AUTH_HEADER"

# 5. Check current positions
curl -s "$BASE_URL/open/trader/newsliquid/v1/positions?exchangeId=binance" \
  -H "$AUTH_HEADER"

# 6. Close a position (market price)
curl -s -X POST "$BASE_URL/open/trader/newsliquid/v1/positions/close" \
  -H "$AUTH_HEADER" -H "Content-Type: application/json" \
  -d '{"symbol":"BTC/USDT:USDT","side":"long","quantity":0,"exchangeId":"binance"}'
```

> **Note**: Trading pair format follows CCXT standard: `BTC/USDT` for spot, `BTC/USDT:USDT` for USDT perpetual contracts.

## Command Index

### Market Data (no risk control)

| # | Endpoint | Method | Description |
|---|---|---|---|
| 1 | `/open/trader/newsliquid/v1/market/metadata` | GET | Get market metadata (trading pairs, precision, limits) |
| 2 | `/open/trader/newsliquid/v1/market/ticker` | GET | Get real-time ticker (last price, 24h change, volume) |
| 3 | `/open/trader/newsliquid/v1/market/klines` | GET | Get K-line / candlestick data |
| 4 | `/open/trader/newsliquid/v1/market/base-currencies` | GET | Get base currency list (USDT, BTC, etc.) |
| 5 | `/open/trader/newsliquid/v1/market/time` | GET | Get server time |

### Account (no risk control)

| # | Endpoint | Method | Description |
|---|---|---|---|
| 6 | `/open/trader/newsliquid/v1/account/summary` | GET | Account summary (balance, leverage, max position) |
| 7 | `/open/trader/newsliquid/v1/account/spot` | GET | Query specific spot asset |
| 8 | `/open/trader/newsliquid/v1/account/spots` | GET | Query all spot assets |

### Config (no risk control)

| # | Endpoint | Method | Description |
|---|---|---|---|
| 9 | `/open/trader/newsliquid/v1/config` | GET | Get trading config |
| 10 | `/open/trader/newsliquid/v1/config` | PUT | Update trading config |

### Orders (risk control on create/edit)

| # | Endpoint | Method | Risk | Description |
|---|---|---|---|---|
| 11 | `/open/trader/newsliquid/v1/orders` | POST | Yes | Place order (limit/market/stop-loss/take-profit) |
| 12 | `/open/trader/newsliquid/v1/orders/edit` | PUT | Yes | Edit existing order |
| 13 | `/open/trader/newsliquid/v1/orders/:orderId` | DELETE | No | Cancel order |
| 14 | `/open/trader/newsliquid/v1/orders/open` | GET | No | List open orders |
| 15 | `/open/trader/newsliquid/v1/orders/closed` | GET | No | List closed orders |

### Positions (risk control on close)

| # | Endpoint | Method | Risk | Description |
|---|---|---|---|---|
| 16 | `/open/trader/newsliquid/v1/positions` | GET | No | List current positions |
| 17 | `/open/trader/newsliquid/v1/positions/history` | GET | No | List historical positions |
| 18 | `/open/trader/newsliquid/v1/positions/close` | POST | Yes | Close position (market price) |

### Trades (no risk control)

| # | Endpoint | Method | Description |
|---|---|---|---|
| 19 | `/open/trader/newsliquid/v1/trades/history` | GET | Get trade execution history |

### Leverage & Margin (risk control on leverage change)

| # | Endpoint | Method | Risk | Description |
|---|---|---|---|---|
| 20 | `/open/trader/newsliquid/v1/leverage` | GET | No | Get available leverage tiers |
| 21 | `/open/trader/newsliquid/v1/leverage/current` | GET | No | Get current leverage setting |
| 22 | `/open/trader/newsliquid/v1/leverage/current` | PUT | Yes | Set leverage multiplier |
| 23 | `/open/trader/newsliquid/v1/margin/mode` | GET | No | Get margin mode |
| 24 | `/open/trader/newsliquid/v1/position/mode` | GET | No | Get position mode (one-way/hedge) |
| 25 | `/open/trader/newsliquid/v1/position/mode` | PUT | No | Set position mode |

### Wallet Agent (no risk control)

| # | Endpoint | Method | Description |
|---|---|---|---|
| 26 | `/open/trader/newsliquid/v1/walletagent/create` | POST | Create wallet agent |
| 27 | `/open/trader/newsliquid/v1/walletagent/list` | GET | List wallet agents |
| 28 | `/open/trader/newsliquid/v1/walletagent/address/:address` | GET | Query wallet agent by address |
| 29 | `/open/trader/newsliquid/v1/walletagent/authorize` | PUT | Authorize wallet agent |

## API Reference

### 1. Get Market Metadata

获取指定交易对在所有交易所的市场元数据信息（交易对列表、合约类型、杠杆范围、最小下单量等）。

```bash
curl -s "$BASE_URL/open/trader/newsliquid/v1/market/metadata?ticker=BTC" \
  -H "$AUTH_HEADER"
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `ticker` | String (query) | Yes | Base currency code (e.g., `BTC`, `ETH`, `SOL`) |

**Response:**
```json
{
  "success": true,
  "data": {
    "ticker": "BTC",
    "markets": [
      {
        "exchangeId": "binance",
        "id": "BTCUSDT",
        "symbol": "BTC/USDT",
        "displaySymbol": "BTC/USDT",
        "active": true,
        "leverageMin": 1,
        "leverageMax": 125,
        "baseCurrency": "BTC",
        "quoteCurrency": "USDT",
        "settleCurrency": "USDT",
        "type": "swap",
        "rawType": "swap",
        "costMin": 5.0,
        "amountMin": 0.001,
        "margin": true,
        "spot": false,
        "swap": true,
        "future": false,
        "option": false,
        "contract": true
      }
    ]
  },
  "usage": {"cost": 1, "quota": 99}
}
```

**Display to user:**
- List available exchanges and trading pairs for the requested ticker
- Highlight leverage range, min order size, and contract type (spot/swap/future)

---

### 2. Get Ticker

获取指定交易所和交易对的实时行情数据。

```bash
curl -s "$BASE_URL/open/trader/newsliquid/v1/market/ticker?symbol=BTC/USDT&exchangeId=binance" \
  -H "$AUTH_HEADER"
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `symbol` | String (query) | Yes | Trading pair in CCXT format (e.g., `BTC/USDT`) |
| `exchangeId` | String (query) | Yes | Exchange ID: `binance`, `bybit`, `okx`, `hyperliquid` |

**Response:**
```json
{
  "success": true,
  "data": {
    "symbol": "BTC/USDT",
    "last": 67890.50,
    "bid": 67889.00,
    "ask": 67891.00,
    "high": 68500.00,
    "low": 66800.00,
    "volume": 12345.678,
    "timestamp": "2026-03-21T10:30:00Z"
  },
  "usage": {"cost": 1, "quota": 99}
}
```

**Display to user:**
- "BTC/USDT: $67,890.50"
- "Bid: $67,889 | Ask: $67,891"
- "24h High: $68,500 | Low: $66,800 | Volume: 12,345.68 BTC"

---

### 3. Get K-Lines

获取 Binance K 线（蜡烛图）数据。

```bash
curl -s "$BASE_URL/open/trader/newsliquid/v1/market/klines?symbol=BTCUSDT&interval=1h&limit=100" \
  -H "$AUTH_HEADER"
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `symbol` | String (query) | No | Trading pair (default: `BTCUSDT`) |
| `interval` | String (query) | No | K-line interval (default: `1m`): `1m`, `5m`, `15m`, `1h`, `4h`, `1d`, etc. |
| `limit` | Integer (query) | No | Number of candles (default: 100) |

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "openTime": 1679400000000,
      "open": "67800.00",
      "high": "67900.00",
      "low": "67750.00",
      "close": "67850.00",
      "volume": "123.456",
      "closeTime": 1679400059999
    }
  ],
  "usage": {"cost": 1, "quota": 99}
}
```

**Display to user:**
- Summarize recent price action (e.g., "BTC rose from $67,800 to $67,850 in the last hour")
- Mention support/resistance levels if visible

---

### 4. Get Base Currencies

获取所有交易所支持的去重基础币种列表。

```bash
curl -s "$BASE_URL/open/trader/newsliquid/v1/market/base-currencies" \
  -H "$AUTH_HEADER"
```

**Parameters:** None

**Response:**
```json
{
  "success": true,
  "data": ["BTC", "ETH", "SOL", "DOGE", "XRP"],
  "usage": {"cost": 1, "quota": 99}
}
```

---

### 5. Get Server Time

获取 ai-bots-trading 服务器的当前时间。

```bash
curl -s "$BASE_URL/open/trader/newsliquid/v1/market/time" \
  -H "$AUTH_HEADER"
```

**Parameters:** None

**Response:**
```json
{
  "timestamp": 1679400000000,
  "time": "2026-03-21T10:30:00Z",
  "usage": {"cost": 1, "quota": 99}
}
```

---

### 6. Get Account Summary

获取指定交易所的账户余额摘要信息，包括总余额、可用余额、杠杆分析等。

```bash
curl -s "$BASE_URL/open/trader/newsliquid/v1/account/summary?exchangeId=binance&symbol=BTC/USDT:USDT&accountType=swap" \
  -H "$AUTH_HEADER"
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `exchangeId` | String (query) | Yes | Exchange ID |
| `symbol` | String (query) | No | Trading pair, for determining quote currency |
| `accountType` | String (query) | No | Account type (default: `spot`): `spot`, `swap`, `future`, `margin` |

**Response:**
```json
{
  "success": true,
  "data": {
    "exchangeId": "binance",
    "accountType": "swap",
    "balance": 10000.00,
    "available": 8500.00,
    "currency": "USDT",
    "leverage": 10,
    "maxPosition": 85000.00,
    "totals": {
      "USDT": 10000.00,
      "BTC": 0.05
    },
    "frees": {
      "USDT": 8500.00,
      "BTC": 0.05
    },
    "leverageAnalysis": {
      "symbol": "BTC/USDT:USDT",
      "exchangeId": "binance",
      "availableBalance": 8500.00,
      "balanceCurrency": "USDT",
      "leverageInfo": {
        "exchangeId": "binance",
        "symbol": "BTC/USDT:USDT",
        "tiers": [],
        "supportsDynamicLeverage": true,
        "baseCurrency": "BTC",
        "quoteCurrency": "USDT",
        "settleCurrency": "USDT",
        "generatedAt": 1679400000000
      },
      "marketPrice": 67890.50
    }
  },
  "usage": {"cost": 1, "quota": 99}
}
```

**Display to user:**
- "Total Balance: $10,000.00 USDT"
- "Available: $8,500.00 | Leverage: 10x"
- "Max Position: $85,000.00"

---

### 7. Get Spot Asset

查询指定交易所和交易对的现货资产持有信息。

```bash
curl -s "$BASE_URL/open/trader/newsliquid/v1/account/spot?exchangeId=binance&symbol=BTC/USDT" \
  -H "$AUTH_HEADER"
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `exchangeId` | String (query) | Yes | Exchange ID |
| `symbol` | String (query) | Yes | Trading pair (e.g., `BTC/USDT`) |

**Response:**
```json
{
  "success": true,
  "data": {
    "exchangeId": "binance",
    "symbol": "BTC/USDT",
    "baseAsset": "BTC",
    "quantity": 0.5,
    "free": 0.45,
    "locked": 0.05,
    "costPrice": 65000.00,
    "totalCost": 32500.00
  },
  "usage": {"cost": 1, "quota": 99}
}
```

**Display to user:**
- "BTC: 0.5 (Free: 0.45, Locked: 0.05)"
- "Avg Cost: $65,000 | Total Cost: $32,500"

---

### 8. Get All Spot Assets

获取指定交易所的所有现货资产列表。

```bash
curl -s "$BASE_URL/open/trader/newsliquid/v1/account/spots?exchangeId=binance" \
  -H "$AUTH_HEADER"
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `exchangeId` | String (query) | Yes | Exchange ID |
| `symbol` | String (query) | No | Filter by trading pair |

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "exchangeId": "binance",
      "symbol": null,
      "baseAsset": "BTC",
      "quantity": 0.5,
      "free": 0.45,
      "locked": 0.05,
      "costPrice": 65000.00,
      "totalCost": 32500.00
    },
    {
      "exchangeId": "binance",
      "symbol": null,
      "baseAsset": "USDT",
      "quantity": 10000.00,
      "free": 8500.00,
      "locked": 1500.00,
      "costPrice": 1.00,
      "totalCost": 10000.00
    }
  ],
  "usage": {"cost": 1, "quota": 99}
}
```

**Display to user:**
- List all assets with non-zero balances, showing free/locked split and cost basis

---

### 9. Get Trading Config

获取用户的交易配置摘要（不包含密钥敏感信息）。

```bash
curl -s "$BASE_URL/open/trader/newsliquid/v1/config" \
  -H "$AUTH_HEADER"
```

**Parameters:** None

**Response:**
```json
{
  "success": true,
  "data": {
    "defaultExchange": "binance",
    "defaultLeverage": 10,
    "defaultPosition": 100.0,
    "general": {},
    "binanceConfigured": true,
    "bybitConfigured": false,
    "okxConfigured": true,
    "hyperliquidConfigured": false,
    "asterConfigured": false
  },
  "usage": {"cost": 1, "quota": 99}
}
```

**Display to user:**
- "Default Exchange: binance | Leverage: 10x | Position: $100"
- List which exchanges are configured

---

### 10. Update Trading Config

更新用户的交易配置，包括默认交易所、杠杆和交易所凭证。

> **IMPORTANT**: Exchange API credentials (`apiKey`, `secret`, `password`) are sensitive. Never log or display them.

```bash
curl -s -X PUT "$BASE_URL/open/trader/newsliquid/v1/config" \
  -H "$AUTH_HEADER" -H "Content-Type: application/json" \
  -d '{
    "defaultExchange": "binance",
    "defaultLeverage": 10,
    "defaultPosition": 100.0,
    "binance": {
      "apiKey": "your-api-key",
      "secret": "your-api-secret",
      "password": ""
    }
  }'
```

**Parameters (body):**

| Field | Type | Required | Description |
|---|---|---|---|
| `defaultExchange` | String | No | Default exchange: `binance`, `bybit`, `okx`, `hyperliquid` |
| `defaultLeverage` | Integer | No | Default leverage (1-125) |
| `defaultPosition` | Float | No | Default position size |
| `general` | Object | No | General config |
| `binance` / `bybit` / `okx` / `hyperliquid` / `aster` | Object | No | Exchange credentials: `apiKey` (required), `secret` (required), `password` (optional, OKX requires) |

**Response:**
```json
{
  "success": true,
  "data": {
    "updated": true
  },
  "usage": {"cost": 1, "quota": 99}
}
```

---

### 11. Place Order (Risk Controlled)

在指定交易所下单。支持多种订单类型。

**This endpoint is protected by the risk engine** — orders that deviate too far from market price, exceed position limits, hit rate limits, or lack sufficient balance will be rejected.

```bash
# Limit order: buy 0.01 BTC at $65,000 with TP/SL
curl -s -X POST "$BASE_URL/open/trader/newsliquid/v1/orders" \
  -H "$AUTH_HEADER" -H "Content-Type: application/json" \
  -d '{
    "exchangeId": "binance",
    "symbol": "BTC/USDT:USDT",
    "side": "buy",
    "type": "limit",
    "quantity": 0.01,
    "price": 65000.00,
    "stopLossPrice": 64000.00,
    "takeProfitPrice": 70000.00
  }'

# Market order: sell 0.5 ETH
curl -s -X POST "$BASE_URL/open/trader/newsliquid/v1/orders" \
  -H "$AUTH_HEADER" -H "Content-Type: application/json" \
  -d '{
    "exchangeId": "binance",
    "symbol": "ETH/USDT:USDT",
    "side": "sell",
    "type": "market",
    "quantity": 0.5
  }'

# Stop-loss market order
curl -s -X POST "$BASE_URL/open/trader/newsliquid/v1/orders" \
  -H "$AUTH_HEADER" -H "Content-Type: application/json" \
  -d '{
    "exchangeId": "binance",
    "symbol": "BTC/USDT:USDT",
    "side": "sell",
    "type": "stop_market",
    "quantity": 0.01,
    "triggerPrice": 58000.00
  }'
```

**Parameters (body):**

| Field | Type | Required | Description |
|---|---|---|---|
| `exchangeId` | String | No | Exchange ID (default: `binance`) |
| `symbol` | String | Yes | Trading pair in CCXT format (e.g., `BTC/USDT:USDT`) |
| `side` | String | Yes | `buy` or `sell` |
| `type` | String | Yes | Order type (see below) |
| `quantity` | Float | Conditional | Base currency quantity |
| `quoteAmount` | Float | Conditional | Quote currency amount (e.g., USDT) |
| `price` | Float | Conditional | Limit price, required for `limit`, `stop_limit`, `take_profit_limit` |
| `triggerPrice` | Float | Conditional | Trigger price, required for `stop_market`, `stop_limit`, `take_profit_market`, `take_profit_limit` |
| `hedged` | Boolean | No | Hedge mode (default: `false`) |
| `stopLossPrice` | Float | No | Attached stop-loss trigger price |
| `takeProfitPrice` | Float | No | Attached take-profit trigger price |

**Order types:**
- `market` — Market order
- `limit` — Limit order
- `oco` — OCO order
- `stop_market` — Stop-loss market order
- `stop_limit` — Stop-loss limit order
- `take_profit_market` — Take-profit market order
- `take_profit_limit` — Take-profit limit order

**Response:**
```json
{
  "success": true,
  "data": {
    "exchange": "binance",
    "orderId": "123456789",
    "symbol": "BTC/USDT:USDT",
    "side": "buy",
    "type": "limit",
    "amount": 0.01,
    "price": 65000.00,
    "triggerPrice": 0,
    "status": "open",
    "filledQty": 0,
    "avgPrice": 0,
    "reduceOnly": false,
    "createdAt": "2026-03-21T10:30:00Z",
    "tpsl": {
      "tpTriggerPx": 70000.00,
      "tpTriggerPxType": "last",
      "tpOrdPx": -1,
      "slTriggerPx": 64000.00,
      "slTriggerPxType": "last",
      "slOrdPx": -1,
      "attachId": "attach_001",
      "closePosition": false
    }
  },
  "usage": {"cost": 1, "quota": 99}
}
```

**Display to user:**
- "Order placed! ID: 123456789"
- "Buy 0.01 BTC @ $65,000 (Limit) on Binance"
- "TP: $70,000 | SL: $64,000"
- "Status: Open"

---

### 12. Edit Order (Risk Controlled)

修改已存在的挂单参数。

**This endpoint is protected by the risk engine.**

```bash
curl -s -X PUT "$BASE_URL/open/trader/newsliquid/v1/orders/edit" \
  -H "$AUTH_HEADER" -H "Content-Type: application/json" \
  -d '{
    "exchangeId": "binance",
    "orderId": "123456789",
    "symbol": "BTC/USDT:USDT",
    "type": "limit",
    "side": "buy",
    "quantity": 0.02,
    "price": 64500.00
  }'
```

**Parameters (body):**

| Field | Type | Required | Description |
|---|---|---|---|
| `exchangeId` | String | Yes | Exchange ID |
| `orderId` | String | No | Order ID (Binance TP/SL orders can omit) |
| `symbol` | String | Yes | Trading pair |
| `type` | String | No | Order type |
| `side` | String | No | Direction |
| `quantity` | Float | No | New quantity |
| `price` | Float | No | New limit price |
| `triggerPrice` | Float | No | New trigger price |
| `stopLossPrice` | Float | No | New stop-loss price |
| `takeProfitPrice` | Float | No | New take-profit price |
| `hedged` | Boolean | No | Hedge mode |

**Response:** Same `OrderResponse` structure as Place Order.

---

### 13. Cancel Order

取消指定的挂单。

```bash
curl -s -X DELETE "$BASE_URL/open/trader/newsliquid/v1/orders/123456789?exchangeId=binance&symbol=BTC/USDT:USDT" \
  -H "$AUTH_HEADER"
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `orderId` | String (path) | Yes | Order ID (in URL path) |
| `exchangeId` | String (query) | Yes | Exchange ID |
| `symbol` | String (query) | Yes | Trading pair |
| `type` | String (query) | No | Order type (required for TP/SL orders) |

**Response:**
```json
{
  "success": true,
  "data": {
    "cancelled": true
  },
  "usage": {"cost": 1, "quota": 99}
}
```

---

### 14. List Open Orders

获取当前所有未成交的挂单。

```bash
curl -s "$BASE_URL/open/trader/newsliquid/v1/orders/open?exchangeId=binance&symbol=BTC/USDT:USDT" \
  -H "$AUTH_HEADER"
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `exchangeId` | String (query) | No | Exchange ID (omit for all exchanges) |
| `symbol` | String (query) | No | Filter by trading pair |
| `days` | Integer (query) | No | Filter by creation time (days) |

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "exchange": "binance",
      "orderId": "123456789",
      "symbol": "BTC/USDT:USDT",
      "side": "buy",
      "type": "limit",
      "amount": 0.01,
      "price": 65000.00,
      "triggerPrice": 0,
      "status": "open",
      "filledQty": 0,
      "avgPrice": 0,
      "reduceOnly": false,
      "createdAt": "2026-03-21T10:30:00Z",
      "tpsl": null
    }
  ],
  "usage": {"cost": 1, "quota": 99}
}
```

**Display to user:**
- Table of open orders with exchange, ID, pair, side, type, amount, price, status

---

### 15. List Closed Orders

获取已完成（成交/取消）的历史订单。

```bash
curl -s "$BASE_URL/open/trader/newsliquid/v1/orders/closed?exchangeId=binance&symbol=BTC/USDT:USDT&days=7" \
  -H "$AUTH_HEADER"
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `exchangeId` | String (query) | No | Exchange ID (omit for all exchanges) |
| `symbol` | String (query) | No | Filter by trading pair |
| `days` | Integer (query) | No | Filter by days (default: 7) |

**Response:** Same `[]OrderResponse` structure as List Open Orders.

---

### 16. List Current Positions

获取当前所有持仓信息。

```bash
curl -s "$BASE_URL/open/trader/newsliquid/v1/positions?exchangeId=binance" \
  -H "$AUTH_HEADER"
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `exchangeId` | String (query) | No | Exchange ID (omit for all exchanges) |
| `symbol` | String (query) | No | Filter by trading pair |

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "exchange": "binance",
      "symbol": "BTC/USDT:USDT",
      "side": "long",
      "contracts": 0.01,
      "entryPrice": 65000.00,
      "markPrice": 67890.50,
      "unrealizedPnl": 28.905,
      "leverage": 10,
      "liquidationPrice": 58500.00,
      "marginMode": "cross",
      "hedged": false,
      "timestamp": 1679400000000
    }
  ],
  "usage": {"cost": 1, "quota": 99}
}
```

**Display to user:**
- "BTC/USDT:USDT Long 0.01 BTC (Binance)"
- "Entry: $65,000 | Mark: $67,890.50"
- "P&L: +$28.91 (+4.45%)"
- "Leverage: 10x Cross | Liq: $58,500"

---

### 17. List Historical Positions

获取已平仓的历史持仓记录（包含关联的交易明细）。

```bash
curl -s "$BASE_URL/open/trader/newsliquid/v1/positions/history?exchangeId=binance&days=7" \
  -H "$AUTH_HEADER"
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `exchangeId` | String (query) | No | Exchange ID |
| `symbol` | String (query) | No | Filter by trading pair |
| `days` | Integer (query) | No | Filter by days (default: 0 = all) |

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "exchange": "binance",
      "symbol": "BTC/USDT:USDT",
      "side": "long",
      "marginMode": "cross",
      "leverage": 10,
      "entryPrice": 65000.00,
      "openDateTime": "2026-03-20T08:00:00Z",
      "openTimestamp": 1679313600000,
      "closePrice": 67500.00,
      "fullyClosed": true,
      "closeDateTime": "2026-03-21T10:30:00Z",
      "closeTimestamp": 1679400600000,
      "closedAmount": 0.01,
      "totalAmount": 0.01,
      "realizedPnl": 25.00,
      "totalFee": 1.30,
      "tradeCount": 2,
      "openTrades": 1,
      "closeTrades": 1,
      "trades": [
        {
          "exchange": "binance",
          "tradeId": "trade_001",
          "orderId": "order_001",
          "symbol": "BTC/USDT:USDT",
          "side": "buy",
          "price": 65000.00,
          "amount": 0.01,
          "cost": 650.00,
          "fee": 0.65,
          "pnl": 0,
          "reduceOnly": false,
          "timestamp": 1679313600000,
          "datetime": "2026-03-20T08:00:00Z"
        },
        {
          "exchange": "binance",
          "tradeId": "trade_002",
          "orderId": "order_002",
          "symbol": "BTC/USDT:USDT",
          "side": "sell",
          "price": 67500.00,
          "amount": 0.01,
          "cost": 675.00,
          "fee": 0.65,
          "pnl": 25.00,
          "reduceOnly": true,
          "timestamp": 1679400600000,
          "datetime": "2026-03-21T10:30:00Z"
        }
      ]
    }
  ],
  "usage": {"cost": 1, "quota": 99}
}
```

**Display to user:**
- "BTC/USDT:USDT Long — Closed"
- "Entry: $65,000 → Exit: $67,500"
- "Realized P&L: +$25.00 (Fee: $1.30)"
- "Duration: ~26.5 hours"

---

### 18. Close Position (Risk Controlled)

关闭指定的持仓（全部或部分平仓）。

**This endpoint is protected by the risk engine.**

```bash
curl -s -X POST "$BASE_URL/open/trader/newsliquid/v1/positions/close" \
  -H "$AUTH_HEADER" -H "Content-Type: application/json" \
  -d '{
    "exchangeId": "binance",
    "symbol": "BTC/USDT:USDT",
    "side": "long",
    "quantity": 0,
    "hedged": false
  }'
```

**Parameters (body):**

| Field | Type | Required | Description |
|---|---|---|---|
| `exchangeId` | String | Yes | Exchange ID |
| `symbol` | String | Yes | Trading pair |
| `side` | String | Yes | Position side: `long` or `short` |
| `quantity` | Float | No | Close quantity (0 = close all) |
| `hedged` | Boolean | No | Hedge mode |
| `price` | Float | No | Market price for Hyperliquid |

**Response:** Same `OrderResponse` structure as Place Order.

**Display to user:**
- "Position closed! BTC/USDT:USDT Long"
- Show realized P&L from the order response

---

### 19. Get Trade History

获取历史成交记录。

```bash
curl -s "$BASE_URL/open/trader/newsliquid/v1/trades/history?exchangeId=binance&symbol=BTC/USDT:USDT&days=7" \
  -H "$AUTH_HEADER"
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `exchangeId` | String (query) | No | Exchange ID |
| `symbol` | String (query) | No | Filter by trading pair |
| `days` | Integer (query) | No | Filter by days (default: 7) |

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "exchange": "binance",
      "tradeId": "trade_001",
      "orderId": "order_001",
      "symbol": "BTC/USDT:USDT",
      "side": "buy",
      "price": 65000.00,
      "amount": 0.01,
      "cost": 650.00,
      "fee": 0.65,
      "pnl": 0,
      "reduceOnly": false,
      "timestamp": 1679313600000,
      "datetime": "2026-03-20T08:00:00Z"
    }
  ],
  "usage": {"cost": 1, "quota": 99}
}
```

**Display to user:**
- Table of trades with exchange, pair, side, price, amount, cost, fee, P&L

---

### 20. Get Leverage Tiers

获取指定交易对的杠杆档位（梯度）信息。

```bash
curl -s "$BASE_URL/open/trader/newsliquid/v1/leverage?symbol=BTC/USDT:USDT&exchangeId=binance" \
  -H "$AUTH_HEADER"
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `symbol` | String (query) | Yes | Trading pair |
| `exchangeId` | String (query) | Yes | Exchange ID |

**Response:**
```json
{
  "success": true,
  "data": {
    "exchangeId": "binance",
    "symbol": "BTC/USDT:USDT",
    "tiers": [
      {
        "tier": 1,
        "minNotional": 0,
        "maxNotional": 50000,
        "maintenanceMarginRate": 0.004,
        "initialMarginRate": 0.01,
        "maxLeverage": 100
      },
      {
        "tier": 2,
        "minNotional": 50000,
        "maxNotional": 250000,
        "maintenanceMarginRate": 0.005,
        "initialMarginRate": 0.02,
        "maxLeverage": 50
      }
    ],
    "supportsDynamicLeverage": true,
    "baseCurrency": "BTC",
    "quoteCurrency": "USDT",
    "settleCurrency": "USDT",
    "minNotional": 5.0,
    "generatedAt": 1679400000000
  },
  "usage": {"cost": 1, "quota": 99}
}
```

**Display to user:**
- Table of leverage tiers with max leverage, notional range, and margin rates

---

### 21. Get Current Leverage

获取指定交易对当前设置的杠杆倍数和保证金模式。

```bash
curl -s "$BASE_URL/open/trader/newsliquid/v1/leverage/current?symbol=BTC/USDT:USDT&exchangeId=binance" \
  -H "$AUTH_HEADER"
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `symbol` | String (query) | Yes | Trading pair |
| `exchangeId` | String (query) | Yes | Exchange ID |

**Response:**
```json
{
  "success": true,
  "data": {
    "leverage": 10,
    "marginMode": "cross"
  },
  "usage": {"cost": 1, "quota": 99}
}
```

**Display to user:**
- "BTC/USDT:USDT: 10x leverage, Cross margin"

---

### 22. Set Leverage (Risk Controlled)

设置指定交易对的杠杆倍数。

**This endpoint is protected by the risk engine.**

```bash
curl -s -X PUT "$BASE_URL/open/trader/newsliquid/v1/leverage/current" \
  -H "$AUTH_HEADER" -H "Content-Type: application/json" \
  -d '{"exchangeId":"binance","symbol":"BTC/USDT:USDT","leverage":20}'
```

**Parameters (body):**

| Field | Type | Required | Description |
|---|---|---|---|
| `exchangeId` | String | Yes | Exchange ID |
| `symbol` | String | Yes | Trading pair |
| `leverage` | Integer | Yes | Leverage multiplier (min: 1) |

**Response:**
```json
{
  "success": true,
  "data": {
    "updated": true
  },
  "usage": {"cost": 1, "quota": 99}
}
```

**Display to user:**
- "Leverage updated! BTC/USDT:USDT: 20x"

---

### 23. Get Margin Mode

获取指定交易对的保证金模式（cross 全仓 / isolated 逐仓）。

```bash
curl -s "$BASE_URL/open/trader/newsliquid/v1/margin/mode?symbol=BTC/USDT:USDT&exchangeId=binance" \
  -H "$AUTH_HEADER"
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `symbol` | String (query) | Yes | Trading pair |
| `exchangeId` | String (query) | Yes | Exchange ID |

**Response:**
```json
{
  "success": true,
  "data": {
    "marginMode": "cross"
  },
  "usage": {"cost": 1, "quota": 99}
}
```

---

### 24. Get Position Mode

获取指定交易对的持仓模式（单向/双向）。

```bash
curl -s "$BASE_URL/open/trader/newsliquid/v1/position/mode?symbol=BTC/USDT:USDT&exchangeId=binance" \
  -H "$AUTH_HEADER"
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `symbol` | String (query) | Yes | Trading pair |
| `exchangeId` | String (query) | Yes | Exchange ID |

**Response:**
```json
{
  "success": true,
  "data": {
    "hedged": false,
    "positionMode": "one_way"
  },
  "usage": {"cost": 1, "quota": 99}
}
```

---

### 25. Set Position Mode

设置指定交易对的持仓模式。

```bash
curl -s -X PUT "$BASE_URL/open/trader/newsliquid/v1/position/mode" \
  -H "$AUTH_HEADER" -H "Content-Type: application/json" \
  -d '{"exchangeId":"binance","symbol":"BTC/USDT:USDT","hedged":true}'
```

**Parameters (body):**

| Field | Type | Required | Description |
|---|---|---|---|
| `exchangeId` | String | Yes | Exchange ID |
| `symbol` | String | Yes | Trading pair |
| `hedged` | Boolean | Yes | `true` = Hedge Mode (two-way), `false` = One-way Mode |

**Response:**
```json
{
  "success": true,
  "data": {
    "updated": true
  },
  "usage": {"cost": 1, "quota": 99}
}
```

**Display to user:**
- "Position mode updated to Hedge (two-way)"

---

### 26. Create Wallet Agent

创建一个新的以太坊钱包代理（用于 Aster 或 Hyperliquid 交易）。

```bash
curl -s -X POST "$BASE_URL/open/trader/newsliquid/v1/walletagent/create" \
  -H "$AUTH_HEADER" -H "Content-Type: application/json" \
  -d '{"exchange":"hyperliquid"}'
```

**Parameters (body):**

| Field | Type | Required | Description |
|---|---|---|---|
| `exchange` | String | Yes | Exchange type: `aster` or `hyperliquid` |

**Response:**
```json
{
  "success": true,
  "data": {
    "id": 1,
    "userId": "user_001",
    "agentAddress": "0x1234567890abcdef...",
    "userAddress": "",
    "exchange": "hyperliquid",
    "authorized": false,
    "createdAt": "2026-03-21T10:30:00Z",
    "updatedAt": "2026-03-21T10:30:00Z"
  },
  "usage": {"cost": 1, "quota": 99}
}
```

**Display to user:**
- "Wallet agent created!"
- "Agent Address: 0x1234..."
- "Exchange: Hyperliquid"
- "Status: Not authorized (run authorize to activate)"

---

### 27. List Wallet Agents

获取当前用户的所有钱包代理列表。

```bash
curl -s "$BASE_URL/open/trader/newsliquid/v1/walletagent/list?exchange=hyperliquid" \
  -H "$AUTH_HEADER"
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `exchange` | String (query) | No | Filter by exchange type: `aster` or `hyperliquid` |

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "id": 1,
      "userId": "user_001",
      "agentAddress": "0x1234567890abcdef...",
      "userAddress": "0xabcdef1234567890...",
      "exchange": "hyperliquid",
      "authorized": true,
      "expiredAt": 1681992600000,
      "createdAt": "2026-03-21T10:30:00Z",
      "updatedAt": "2026-03-21T10:30:00Z"
    }
  ],
  "usage": {"cost": 1, "quota": 99}
}
```

---

### 28. Query Wallet Agent by Address

根据钱包地址获取钱包代理信息。

```bash
curl -s "$BASE_URL/open/trader/newsliquid/v1/walletagent/address/0x1234567890abcdef?exchange=hyperliquid" \
  -H "$AUTH_HEADER"
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `address` | String (path) | Yes | Wallet address (in URL path) |
| `exchange` | String (query) | No | Filter by exchange type |

**Response:**
```json
{
  "success": true,
  "data": {
    "id": 1,
    "userId": "user_001",
    "agentAddress": "0x1234567890abcdef...",
    "userAddress": "0xabcdef1234567890...",
    "exchange": "hyperliquid",
    "authorized": true,
    "expiredAt": 1681992600000,
    "createdAt": "2026-03-21T10:30:00Z",
    "updatedAt": "2026-03-21T10:30:00Z"
  },
  "usage": {"cost": 1, "quota": 99}
}
```

---

### 29. Authorize Wallet Agent

设置钱包代理的授权状态。

```bash
curl -s -X PUT "$BASE_URL/open/trader/newsliquid/v1/walletagent/authorize" \
  -H "$AUTH_HEADER" -H "Content-Type: application/json" \
  -d '{
    "agentAddress": "0x1234567890abcdef...",
    "exchange": "hyperliquid",
    "userAddress": "0xabcdef1234567890...",
    "authorized": true,
    "expiredAt": 1681992600000
  }'
```

**Parameters (body):**

| Field | Type | Required | Description |
|---|---|---|---|
| `agentAddress` | String | Yes | Agent wallet address |
| `exchange` | String | Yes | Exchange type: `aster` or `hyperliquid` |
| `userAddress` | String | Yes | User's main wallet address |
| `authorized` | Boolean | Yes | `true` to authorize, `false` to revoke |
| `expiredAt` | Integer | No | Authorization expiry (Unix milliseconds) |

**Response:**
```json
{
  "success": true,
  "data": {
    "id": 1,
    "userId": "user_001",
    "agentAddress": "0x1234567890abcdef...",
    "userAddress": "0xabcdef1234567890...",
    "exchange": "hyperliquid",
    "authorized": true,
    "expiredAt": 1681992600000,
    "createdAt": "2026-03-21T10:30:00Z",
    "updatedAt": "2026-03-21T11:00:00Z"
  },
  "usage": {"cost": 1, "quota": 99}
}
```

## Cross-Skill Workflows

### Workflow A: CEX Spot Trading

> User: "Buy 0.1 BTC on Binance"

```
1. opentrade-newsliquid  GET /market/ticker?symbol=BTC/USDT&exchangeId=binance  → check current price
2. opentrade-newsliquid  GET /account/summary?exchangeId=binance                → verify available balance
3. opentrade-newsliquid  GET /market/metadata?ticker=BTC                        → confirm pair info/limits
4. opentrade-newsliquid  POST /orders                                           → place order
       {"symbol":"BTC/USDT:USDT","side":"buy","type":"market","quantity":0.1,"exchangeId":"binance"}
5. opentrade-newsliquid  GET /orders/open                                       → confirm order status
```

**Data handoff**:
- `last` price from step 1 → helps user decide order type (market vs limit)
- `available` balance from step 2 → validates user can afford the order
- Market info from step 3 → confirms pair exists and shows limits

### Workflow B: CEX Futures Trading

> User: "Open a 10x long on ETH with $1000"

```
1. opentrade-newsliquid  GET /market/ticker?symbol=ETH/USDT&exchangeId=binance  → check ETH price
2. opentrade-newsliquid  GET /account/summary?exchangeId=binance&symbol=ETH/USDT:USDT&accountType=swap  → check margin balance
3. opentrade-newsliquid  GET /leverage/current?symbol=ETH/USDT:USDT&exchangeId=binance  → check current leverage
4. opentrade-newsliquid  PUT /leverage/current                       → set leverage to 10x (if needed)
       {"symbol":"ETH/USDT:USDT","leverage":10,"exchangeId":"binance"}
5. opentrade-newsliquid  POST /orders                                → open long position
       {"symbol":"ETH/USDT:USDT","side":"buy","type":"market","quantity":<calculated>,"exchangeId":"binance"}
6. opentrade-newsliquid  GET /positions                              → verify position opened
```

**Data handoff**:
- `last` price from step 1 → calculate quantity: `$1000 / ETH_price`
- Leverage 10x means only $100 margin needed for $1000 position

### Workflow C: News-Driven CEX Trading

> User: "Check latest crypto news and trade accordingly"

```
1. [opennews]             Search crypto news → get AI ratings and trade signals
2. [opentwitter]          Check KOL sentiment on the target token
3. opentrade-newsliquid   GET /market/ticker                         → check CEX price
4. opentrade-newsliquid   GET /market/klines?interval=1h             → check recent trend
5. opentrade-newsliquid   GET /account/summary                       → check balance
6. opentrade-newsliquid   POST /orders                               → execute trade
7. opentrade-newsliquid   GET /positions                             → monitor position
```

### Workflow D: CEX-DEX Price Arbitrage

> User: "Compare BTC price between CEX and DEX"

```
1. opentrade-newsliquid   GET /market/ticker?symbol=BTC/USDT&exchangeId=binance   → CEX price
2. [opentrade-market]     GET /market/price (on-chain)               → DEX price
3. Compare prices → identify arbitrage opportunity
4a. CEX cheaper → opentrade-newsliquid POST /orders (CEX buy) + [opentrade-dex-swap] (DEX sell)
4b. DEX cheaper → [opentrade-dex-swap] (DEX buy) + opentrade-newsliquid POST /orders (CEX sell)
5. Confirm both sides filled → calculate profit
```

### Workflow E: CEX Hedge + DEX Spot Holdings

> User: "Hedge my on-chain ETH holdings with a CEX short"

```
1. [opentrade-portfolio]  Check on-chain ETH balance                 → e.g., 10 ETH
2. opentrade-newsliquid   GET /account/summary                       → check CEX margin
3. opentrade-newsliquid   PUT /leverage/current                      → set leverage
4. opentrade-newsliquid   POST /orders                               → open short position
       {"symbol":"ETH/USDT:USDT","side":"sell","type":"market","quantity":10,"exchangeId":"binance"}
5. opentrade-newsliquid   GET /positions                             → confirm hedge position
```

### Workflow F: DEX Discovery + CEX Execution

> User: "Find trending tokens and trade on CEX"

```
1. [opentrade-token]      Search trending tokens                     → discover hot tokens
2. [opentrade-market]     Check on-chain trading activity            → smart money signals
3. opentrade-newsliquid   GET /market/metadata                       → check if listed on CEX
4. If CEX listed → opentrade-newsliquid POST /orders                 → trade on CEX (lower fees)
   If not listed → [opentrade-dex-swap]                              → trade on DEX
```

### Workflow G: Cross-Venue Portfolio Overview

> User: "Show me all my assets across CEX and DEX"

```
1. opentrade-newsliquid   GET /account/summary                       → CEX total balance
2. opentrade-newsliquid   GET /account/spots                         → CEX spot assets
3. opentrade-newsliquid   GET /positions                             → CEX open positions
4. [opentrade-portfolio]  Get on-chain wallet balances               → DEX holdings
5. Combine and present unified portfolio report
```

## Operation Flow

### Step 1: Identify Intent

| User wants to... | Action |
|---|---|
| Check CEX market price | `GET /market/ticker` |
| View K-line / chart data | `GET /market/klines` |
| Check account balance | `GET /account/summary` or `GET /account/spots` |
| Place a buy/sell order | `POST /orders` |
| Modify an existing order | `PUT /orders/edit` |
| Cancel an order | `DELETE /orders/:orderId` |
| View open orders | `GET /orders/open` |
| View closed orders | `GET /orders/closed` |
| Check current positions | `GET /positions` |
| Close a position | `POST /positions/close` |
| Set leverage | `PUT /leverage/current` |
| Check leverage / margin | `GET /leverage/current`, `GET /margin/mode` |
| View trade history | `GET /trades/history` |
| Manage wallet agents | `POST/GET/PUT /walletagent/*` |

### Step 2: Collect Parameters

- **Missing exchangeId** → ask user which exchange (e.g., `binance`, `bybit`, `okx`, `hyperliquid`)
- **Missing symbol** → ask user which trading pair; format as CCXT standard (e.g., `BTC/USDT` for spot, `BTC/USDT:USDT` for perpetual)
- **Missing side** → ask user: buy or sell?
- **Missing order type** → suggest `market` for immediate execution, `limit` for price control
- **Missing quantity** → ask user; for futures, help calculate based on notional value and leverage
- **Missing price** → required for limit orders; call `GET /market/ticker` to show current price as reference
- **Missing leverage** → check current setting with `GET /leverage/current`; suggest 1x-10x for beginners

### Step 3: Execute & Display

- Run the API call
- Parse JSON response
- Display human-readable summary with prices, quantities, P&L in friendly format
- **For order creation**: show order ID, pair, side, type, quantity, price, status
- **For positions**: show pair, side, entry/mark price, P&L, leverage
- **For account**: show total/available balance, unrealized P&L

### Step 4: Suggest Next Steps

| Just completed | Suggest |
|---|---|
| Checked ticker | 1. Place an order 2. View K-line for trend analysis |
| Checked balance | 1. Place an order 2. Check positions |
| Placed order | 1. Check open orders 2. View positions 3. Set stop-loss |
| Order filled | 1. View positions 2. Set take-profit/stop-loss |
| Position opened | 1. Monitor with ticker 2. Set stop-loss order 3. Close position |
| Position closed | 1. Check realized P&L in trade history 2. Check updated balance |
| Set leverage | 1. Place an order 2. Check position limits |

Present conversationally — never expose endpoint paths to the user.

## Risk Engine Notes

All risk-controlled endpoints (marked with "Risk: Yes" in Command Index) pass through a 4-layer risk engine before execution:

| Rule | Name | Trigger | Default Threshold |
|---|---|---|---|
| 1 | Price Deviation Check | Limit orders | Max 10% deviation from market price |
| 2 | Position Size Limit | Order creation, position close | Single: 20% of balance, Total: 80% of balance |
| 3 | Rate Limit | All risk-controlled endpoints | 30 requests per minute |
| 4 | Balance Check | Order creation | Min 5% balance reserve |

**What happens when risk check fails:**
- The API returns an error with a description of which rule was triggered
- The order/action is NOT executed
- Display the reason to the user clearly (e.g., "Order rejected: price deviates 15% from market price, max allowed is 10%")
- Suggest corrective action (adjust price, reduce quantity, wait and retry, etc.)

**Risk engine behavior:**
- Rules are checked in priority order (rate limit → price deviation → position limit → balance check)
- First failure stops the chain — remaining rules are not checked
- The risk engine uses a **fail-open** strategy: if market data or Redis is unavailable, the check passes (prioritizing availability)

## Input / Output Examples

**User says:** "What's the BTC price on Binance?"

```bash
curl -s "$BASE_URL/open/trader/newsliquid/v1/market/ticker?symbol=BTC/USDT&exchangeId=binance" -H "$AUTH_HEADER"
# → BTC/USDT: $67,890.50 (Bid: $67,889 | Ask: $67,891)
```

**User says:** "Buy 0.01 BTC at $65,000"

```bash
curl -s -X POST "$BASE_URL/open/trader/newsliquid/v1/orders" \
  -H "$AUTH_HEADER" -H "Content-Type: application/json" \
  -d '{"symbol":"BTC/USDT:USDT","side":"buy","type":"limit","quantity":0.01,"price":65000,"exchangeId":"binance"}'
# → Order placed! Buy 0.01 BTC @ $65,000 (Limit) — ID: 123456789
```

**User says:** "Close my BTC position"

```bash
curl -s -X POST "$BASE_URL/open/trader/newsliquid/v1/positions/close" \
  -H "$AUTH_HEADER" -H "Content-Type: application/json" \
  -d '{"symbol":"BTC/USDT:USDT","side":"long","quantity":0,"exchangeId":"binance"}'
# → Position closed! Realized P&L: +$25.00
```

**User says:** "Set my ETH leverage to 20x"

```bash
curl -s -X PUT "$BASE_URL/open/trader/newsliquid/v1/leverage/current" \
  -H "$AUTH_HEADER" -H "Content-Type: application/json" \
  -d '{"symbol":"ETH/USDT:USDT","leverage":20,"exchangeId":"binance"}'
# → Leverage updated! ETH/USDT:USDT: 20x
```

## Edge Cases

- **Risk engine rejects order**: Display the rejection reason clearly. Common causes: price too far from market (>10%), position too large, rate limited, insufficient balance. Suggest the user adjust parameters and retry.
- **Insufficient balance**: Check balance with `GET /account/summary` first. For futures, consider leverage — required margin = order value / leverage.
- **Rate limited**: If 30+ requests in 1 minute, wait 60 seconds before retrying. Inform the user about the cooldown.
- **Invalid trading pair**: Call `GET /market/metadata` to verify the symbol exists on the exchange.
- **Position mode conflict**: Cannot switch position mode while holding open positions. Close all positions first.
- **Leverage change with open positions**: Some exchanges restrict leverage changes when positions are open. Close positions or reduce size first.
- **Order quantity precision**: Use `GET /market/metadata` to check `amountMin` and `costMin`. Ensure order meets minimum requirements.
- **Minimum notional**: Orders below the minimum cost (e.g., $5 `costMin`) will be rejected by the exchange.
- **Network error**: Retry once, then prompt user to try again later.
- **Region restriction (error code 50125 or 80001)**: Do NOT show the raw error code to the user. Instead, display: `Service is not available in your region. Please switch to a supported region and try again.`

## Amount Display Rules

- CEX amounts use **standard units** (e.g., `0.1 BTC`, `100 USDT`) — NOT minimal units like DEX
- Always show currency symbol alongside amounts
- Format large numbers with commas (e.g., `$67,500.50`)
- Show P&L with sign and color hint: positive (+$250.00), negative (-$100.00)
- Show percentage changes with sign (e.g., +1.89%, -0.52%)
- Leverage shown as multiplier (e.g., 10x, 20x)

## Global Notes

- All endpoints require `Authorization: Bearer <token>` header
- Supported exchanges: `binance`, `bybit`, `okx`, `hyperliquid`, `aster`
- Trading pair format follows **CCXT standard**: `BTC/USDT` for spot, `BTC/USDT:USDT` for USDT perpetual contracts
- The API routes through the Newsliquid gateway with built-in risk controls — trades execute server-side
- No private keys or transaction signing involved — this is CEX trading via API
- CEX uses **standard amount units** (e.g., `0.1 BTC`), unlike DEX which uses minimal units (wei/lamports)
- Numeric values in request bodies use native types (numbers, not strings): `"quantity": 0.01`, `"price": 65000.00`
- Risk-controlled endpoints may reject requests — always display the rejection reason to the user
- Query parameters go in the URL, body parameters go in JSON request body
- Wallet Agent feature only supports `aster` and `hyperliquid` exchanges
- Each API request consumes 1 quota unit (shown in `usage` field of response)
- **Success response**: `{"success": true, "data": {...}, "usage": {"cost": 1, "quota": 99}}`
- **Error response (upstream)**: `{"success": false, "code": "INVALID_REQUEST", "error": "message"}`
- **Error response (gateway)**: `{"code": 400, "message": "error message", "error": "details"}`
- The skill uses the same `OPEN_TOKEN` as all other opentrade skills — no additional configuration needed
