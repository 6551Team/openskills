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

## Quickstart

```bash
# 1. Get real-time ticker
curl -s "$BASE_URL/trader/newsliquid/v1/market/ticker?symbol=BTCUSDT&exchangeId=binance" \
  -H "$AUTH_HEADER"

# 2. Check account balance
curl -s "$BASE_URL/trader/newsliquid/v1/account/summary?exchangeId=binance" \
  -H "$AUTH_HEADER"

# 3. Place a limit buy order
curl -s -X POST "$BASE_URL/trader/newsliquid/v1/orders" \
  -H "$AUTH_HEADER" -H "Content-Type: application/json" \
  -d '{"symbol":"BTCUSDT","side":"buy","type":"limit","quantity":"0.001","price":"60000","exchangeId":"binance"}'

# 4. Check open orders
curl -s "$BASE_URL/trader/newsliquid/v1/orders/open?exchangeId=binance" \
  -H "$AUTH_HEADER"

# 5. Check current positions
curl -s "$BASE_URL/trader/newsliquid/v1/positions?exchangeId=binance" \
  -H "$AUTH_HEADER"

# 6. Close a position (market price)
curl -s -X POST "$BASE_URL/trader/newsliquid/v1/positions/close" \
  -H "$AUTH_HEADER" -H "Content-Type: application/json" \
  -d '{"symbol":"BTCUSDT","exchangeId":"binance"}'
```

## Command Index

### Market Data (no risk control)

| # | Endpoint | Method | Description |
|---|---|---|---|
| 1 | `/trader/newsliquid/v1/market/metadata` | GET | Get market metadata (trading pairs, precision, limits) |
| 2 | `/trader/newsliquid/v1/market/ticker` | GET | Get real-time ticker (last price, 24h change, volume) |
| 3 | `/trader/newsliquid/v1/market/klines` | GET | Get K-line / candlestick data |
| 4 | `/trader/newsliquid/v1/market/base-currencies` | GET | Get base currency list (USDT, BTC, etc.) |
| 5 | `/trader/newsliquid/v1/market/time` | GET | Get server time |

### Account (no risk control)

| # | Endpoint | Method | Description |
|---|---|---|---|
| 6 | `/trader/newsliquid/v1/account/summary` | GET | Account summary (balance, leverage, max position) |
| 7 | `/trader/newsliquid/v1/account/spot` | GET | Query specific spot asset |
| 8 | `/trader/newsliquid/v1/account/spots` | GET | Query all spot assets |

### Config (no risk control)

| # | Endpoint | Method | Description |
|---|---|---|---|
| 9 | `/trader/newsliquid/v1/config` | GET | Get trading config |
| 10 | `/trader/newsliquid/v1/config` | PUT | Update trading config |

### Orders (risk control on create/edit)

| # | Endpoint | Method | Risk | Description |
|---|---|---|---|---|
| 11 | `/trader/newsliquid/v1/orders` | POST | Yes | Place order (limit/market/stop-loss/take-profit) |
| 12 | `/trader/newsliquid/v1/orders/edit` | PUT | Yes | Edit existing order |
| 13 | `/trader/newsliquid/v1/orders/:orderId` | DELETE | No | Cancel order |
| 14 | `/trader/newsliquid/v1/orders/open` | GET | No | List open orders |
| 15 | `/trader/newsliquid/v1/orders/closed` | GET | No | List closed orders |

### Positions (risk control on close)

| # | Endpoint | Method | Risk | Description |
|---|---|---|---|---|
| 16 | `/trader/newsliquid/v1/positions` | GET | No | List current positions |
| 17 | `/trader/newsliquid/v1/positions/history` | GET | No | List historical positions |
| 18 | `/trader/newsliquid/v1/positions/close` | POST | Yes | Close position (market price) |

### Trades (no risk control)

| # | Endpoint | Method | Description |
|---|---|---|---|
| 19 | `/trader/newsliquid/v1/trades/history` | GET | Get trade execution history |

### Leverage & Margin (risk control on leverage change)

| # | Endpoint | Method | Risk | Description |
|---|---|---|---|---|
| 20 | `/trader/newsliquid/v1/leverage` | GET | No | Get available leverage tiers |
| 21 | `/trader/newsliquid/v1/leverage/current` | GET | No | Get current leverage setting |
| 22 | `/trader/newsliquid/v1/leverage/current` | PUT | Yes | Set leverage multiplier |
| 23 | `/trader/newsliquid/v1/margin/mode` | GET | No | Get margin mode |
| 24 | `/trader/newsliquid/v1/position/mode` | GET | No | Get position mode (one-way/hedge) |
| 25 | `/trader/newsliquid/v1/position/mode` | PUT | No | Set position mode |

### Wallet Agent (no risk control)

| # | Endpoint | Method | Description |
|---|---|---|---|
| 26 | `/trader/newsliquid/v1/walletagent/create` | POST | Create wallet agent |
| 27 | `/trader/newsliquid/v1/walletagent/list` | GET | List wallet agents |
| 28 | `/trader/newsliquid/v1/walletagent/address/:address` | GET | Query wallet agent by address |
| 29 | `/trader/newsliquid/v1/walletagent/authorize` | PUT | Authorize wallet agent |

## API Reference

### 1. Get Market Metadata

获取市场元数据，包括所有可交易的交易对、价格精度、数量精度、最小/最大下单量等。

```bash
curl -s "$BASE_URL/trader/newsliquid/v1/market/metadata?exchangeId=binance" \
  -H "$AUTH_HEADER"
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `exchangeId` | String (query) | Yes | Exchange identifier (e.g., `binance`) |

**Response:**
```json
{
  "success": true,
  "data": {
    "symbols": [
      {
        "symbol": "BTCUSDT",
        "baseAsset": "BTC",
        "quoteAsset": "USDT",
        "pricePrecision": 2,
        "quantityPrecision": 5,
        "minQty": "0.00001",
        "maxQty": "9000",
        "minNotional": "5"
      }
    ]
  }
}
```

**Display to user:**
- List available trading pairs with their precision and limits
- Highlight min/max order sizes for the user's target pair

---

### 2. Get Ticker

获取实时行情，包括最新价、24 小时涨跌幅、成交量等。

```bash
curl -s "$BASE_URL/trader/newsliquid/v1/market/ticker?symbol=BTCUSDT&exchangeId=binance" \
  -H "$AUTH_HEADER"
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `symbol` | String (query) | Yes | Trading pair (e.g., `BTCUSDT`) |
| `exchangeId` | String (query) | Yes | Exchange identifier |

**Response:**
```json
{
  "success": true,
  "data": {
    "symbol": "BTCUSDT",
    "lastPrice": "67500.50",
    "priceChange": "1250.30",
    "priceChangePercent": "1.89",
    "high": "68200.00",
    "low": "65800.00",
    "volume": "12345.678",
    "quoteVolume": "832456789.12"
  }
}
```

**Display to user:**
- "BTC/USDT: $67,500.50 (+1.89%)"
- "24h High: $68,200 | Low: $65,800"
- "24h Volume: 12,345.68 BTC"

---

### 3. Get K-Lines

获取 K 线（蜡烛图）数据，支持多种时间周期。

```bash
curl -s "$BASE_URL/trader/newsliquid/v1/market/klines?symbol=BTCUSDT&interval=1h&limit=100&exchangeId=binance" \
  -H "$AUTH_HEADER"
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `symbol` | String (query) | Yes | Trading pair |
| `interval` | String (query) | Yes | K-line interval: `1m`, `5m`, `15m`, `30m`, `1h`, `4h`, `1d`, `1w` |
| `limit` | String (query) | No | Number of candles (default: 100, max: 1500) |
| `exchangeId` | String (query) | Yes | Exchange identifier |

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "openTime": 1700000000000,
      "open": "67000.00",
      "high": "67500.00",
      "low": "66800.00",
      "close": "67300.00",
      "volume": "123.456",
      "closeTime": 1700003600000
    }
  ]
}
```

**Display to user:**
- Summarize recent price action (e.g., "BTC rose from $67,000 to $67,300 in the last hour")
- Mention support/resistance levels if visible

---

### 4. Get Base Currencies

获取支持的基础货币列表（如 USDT、BTC、ETH 等报价币种）。

```bash
curl -s "$BASE_URL/trader/newsliquid/v1/market/base-currencies?exchangeId=binance" \
  -H "$AUTH_HEADER"
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `exchangeId` | String (query) | Yes | Exchange identifier |

**Response:**
```json
{
  "success": true,
  "data": ["USDT", "BTC", "ETH", "BNB", "BUSD"]
}
```

---

### 5. Get Server Time

获取服务器时间，可用于校验客户端时钟偏差。

```bash
curl -s "$BASE_URL/trader/newsliquid/v1/market/time" \
  -H "$AUTH_HEADER"
```

**Parameters:** None

**Response:**
```json
{
  "success": true,
  "data": {
    "serverTime": 1700000000000
  }
}
```

---

### 6. Get Account Summary

获取账户摘要信息，包括总余额、可用余额、已用保证金、杠杆、最大可开仓位等。

```bash
curl -s "$BASE_URL/trader/newsliquid/v1/account/summary?exchangeId=binance" \
  -H "$AUTH_HEADER"
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `exchangeId` | String (query) | Yes | Exchange identifier |

**Response:**
```json
{
  "success": true,
  "data": {
    "totalBalance": "10000.00",
    "availableBalance": "8500.00",
    "usedMargin": "1500.00",
    "unrealizedPnl": "250.00",
    "leverage": "10",
    "maxPosition": "100000.00"
  }
}
```

**Display to user:**
- "Total Balance: $10,000.00"
- "Available: $8,500.00 | Used Margin: $1,500.00"
- "Unrealized P&L: +$250.00"

---

### 7. Get Spot Asset

查询指定现货资产的余额详情。

```bash
curl -s "$BASE_URL/trader/newsliquid/v1/account/spot?asset=BTC&exchangeId=binance" \
  -H "$AUTH_HEADER"
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `asset` | String (query) | Yes | Asset symbol (e.g., `BTC`, `USDT`) |
| `exchangeId` | String (query) | Yes | Exchange identifier |

**Response:**
```json
{
  "success": true,
  "data": {
    "asset": "BTC",
    "free": "0.5",
    "locked": "0.1",
    "total": "0.6"
  }
}
```

---

### 8. Get All Spot Assets

查询所有现货资产余额。

```bash
curl -s "$BASE_URL/trader/newsliquid/v1/account/spots?exchangeId=binance" \
  -H "$AUTH_HEADER"
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `exchangeId` | String (query) | Yes | Exchange identifier |

**Response:**
```json
{
  "success": true,
  "data": [
    {"asset": "BTC", "free": "0.5", "locked": "0.1", "total": "0.6"},
    {"asset": "USDT", "free": "5000.00", "locked": "0", "total": "5000.00"},
    {"asset": "ETH", "free": "10.0", "locked": "0", "total": "10.0"}
  ]
}
```

**Display to user:**
- List all assets with non-zero balances
- Show total value in USDT equivalent if possible

---

### 9. Get Trading Config

获取当前交易配置。

```bash
curl -s "$BASE_URL/trader/newsliquid/v1/config?exchangeId=binance" \
  -H "$AUTH_HEADER"
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `exchangeId` | String (query) | Yes | Exchange identifier |

**Response:**
```json
{
  "success": true,
  "data": {
    "defaultLeverage": "10",
    "marginMode": "cross",
    "positionMode": "one-way"
  }
}
```

---

### 10. Update Trading Config

更新交易配置。

```bash
curl -s -X PUT "$BASE_URL/trader/newsliquid/v1/config" \
  -H "$AUTH_HEADER" -H "Content-Type: application/json" \
  -d '{"exchangeId":"binance","defaultLeverage":"20"}'
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `exchangeId` | String (body) | Yes | Exchange identifier |
| Other fields | varies | No | Config fields to update |

**Response:**
```json
{
  "success": true,
  "data": {
    "message": "Config updated successfully"
  }
}
```

---

### 11. Place Order (Risk Controlled)

创建订单。支持多种订单类型：limit（限价）、market（市价）、stop（止损）、take_profit（止盈）、oco（一取消另一个）等。

**This endpoint is protected by the risk engine** — orders that deviate too far from market price, exceed position limits, hit rate limits, or lack sufficient balance will be rejected.

```bash
# Limit order: buy 0.001 BTC at $60,000
curl -s -X POST "$BASE_URL/trader/newsliquid/v1/orders" \
  -H "$AUTH_HEADER" -H "Content-Type: application/json" \
  -d '{
    "symbol": "BTCUSDT",
    "side": "buy",
    "type": "limit",
    "quantity": "0.001",
    "price": "60000",
    "exchangeId": "binance"
  }'

# Market order: sell 0.5 ETH
curl -s -X POST "$BASE_URL/trader/newsliquid/v1/orders" \
  -H "$AUTH_HEADER" -H "Content-Type: application/json" \
  -d '{
    "symbol": "ETHUSDT",
    "side": "sell",
    "type": "market",
    "quantity": "0.5",
    "exchangeId": "binance"
  }'

# Stop-loss order
curl -s -X POST "$BASE_URL/trader/newsliquid/v1/orders" \
  -H "$AUTH_HEADER" -H "Content-Type: application/json" \
  -d '{
    "symbol": "BTCUSDT",
    "side": "sell",
    "type": "stop",
    "quantity": "0.001",
    "stopPrice": "58000",
    "exchangeId": "binance"
  }'
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `symbol` | String | Yes | Trading pair (e.g., `BTCUSDT`) |
| `side` | String | Yes | `buy` or `sell` |
| `type` | String | Yes | Order type: `limit`, `market`, `stop`, `take_profit`, `stop_limit`, `oco`, `trailing_stop` |
| `quantity` | String | Yes | Order quantity in base asset units |
| `price` | String | Conditional | Required for `limit`, `stop_limit`, `oco` orders |
| `stopPrice` | String | Conditional | Required for `stop`, `take_profit`, `stop_limit`, `oco` orders |
| `exchangeId` | String | Yes | Exchange identifier |

**Response:**
```json
{
  "success": true,
  "data": {
    "orderId": "12345678",
    "symbol": "BTCUSDT",
    "side": "buy",
    "type": "limit",
    "quantity": "0.001",
    "price": "60000",
    "status": "new"
  }
}
```

**Display to user:**
- "Order placed! ID: 12345678"
- "Buy 0.001 BTC @ $60,000 (Limit)"
- "Status: New"

---

### 12. Edit Order (Risk Controlled)

修改已存在的挂单。

**This endpoint is protected by the risk engine.**

```bash
curl -s -X PUT "$BASE_URL/trader/newsliquid/v1/orders/edit" \
  -H "$AUTH_HEADER" -H "Content-Type: application/json" \
  -d '{
    "orderId": "12345678",
    "symbol": "BTCUSDT",
    "price": "59500",
    "quantity": "0.002",
    "exchangeId": "binance"
  }'
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `orderId` | String | Yes | Order ID to edit |
| `symbol` | String | Yes | Trading pair |
| `price` | String | No | New price |
| `quantity` | String | No | New quantity |
| `exchangeId` | String | Yes | Exchange identifier |

**Response:**
```json
{
  "success": true,
  "data": {
    "orderId": "12345678",
    "status": "modified"
  }
}
```

---

### 13. Cancel Order

取消指定订单。

```bash
curl -s -X DELETE "$BASE_URL/trader/newsliquid/v1/orders/12345678?exchangeId=binance" \
  -H "$AUTH_HEADER"
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `orderId` | String (path) | Yes | Order ID to cancel (in URL path) |
| `exchangeId` | String (query) | Yes | Exchange identifier |

**Response:**
```json
{
  "success": true,
  "data": {
    "orderId": "12345678",
    "status": "cancelled"
  }
}
```

---

### 14. List Open Orders

查询当前所有未完成的挂单。

```bash
curl -s "$BASE_URL/trader/newsliquid/v1/orders/open?exchangeId=binance&symbol=BTCUSDT" \
  -H "$AUTH_HEADER"
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `exchangeId` | String (query) | Yes | Exchange identifier |
| `symbol` | String (query) | No | Filter by trading pair |

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "orderId": "12345678",
      "symbol": "BTCUSDT",
      "side": "buy",
      "type": "limit",
      "quantity": "0.001",
      "price": "60000",
      "status": "new",
      "createdAt": "2025-01-01T00:00:00Z"
    }
  ]
}
```

**Display to user:**
- Table of open orders with ID, pair, side, type, quantity, price, status

---

### 15. List Closed Orders

查询已完成（成交、取消、过期）的订单。

```bash
curl -s "$BASE_URL/trader/newsliquid/v1/orders/closed?exchangeId=binance&symbol=BTCUSDT&limit=20" \
  -H "$AUTH_HEADER"
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `exchangeId` | String (query) | Yes | Exchange identifier |
| `symbol` | String (query) | No | Filter by trading pair |
| `limit` | String (query) | No | Max results (default: 20) |

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "orderId": "12345678",
      "symbol": "BTCUSDT",
      "side": "buy",
      "type": "limit",
      "quantity": "0.001",
      "executedQty": "0.001",
      "price": "60000",
      "avgPrice": "59980",
      "status": "filled",
      "createdAt": "2025-01-01T00:00:00Z"
    }
  ]
}
```

---

### 16. List Current Positions

查询当前持仓。

```bash
curl -s "$BASE_URL/trader/newsliquid/v1/positions?exchangeId=binance" \
  -H "$AUTH_HEADER"
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `exchangeId` | String (query) | Yes | Exchange identifier |

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "symbol": "BTCUSDT",
      "side": "long",
      "quantity": "0.1",
      "entryPrice": "65000.00",
      "markPrice": "67500.00",
      "unrealizedPnl": "250.00",
      "leverage": "10",
      "notionalValue": "6750.00",
      "marginUsed": "675.00"
    }
  ]
}
```

**Display to user:**
- "BTCUSDT Long 0.1 BTC"
- "Entry: $65,000 | Mark: $67,500"
- "P&L: +$250.00 (+3.85%)"
- "Leverage: 10x | Margin: $675"

---

### 17. List Historical Positions

查询历史已平仓的持仓记录。

```bash
curl -s "$BASE_URL/trader/newsliquid/v1/positions/history?exchangeId=binance&limit=20" \
  -H "$AUTH_HEADER"
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `exchangeId` | String (query) | Yes | Exchange identifier |
| `symbol` | String (query) | No | Filter by trading pair |
| `limit` | String (query) | No | Max results (default: 20) |

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "symbol": "BTCUSDT",
      "side": "long",
      "quantity": "0.1",
      "entryPrice": "65000.00",
      "exitPrice": "67500.00",
      "realizedPnl": "250.00",
      "closedAt": "2025-01-02T12:00:00Z"
    }
  ]
}
```

---

### 18. Close Position (Risk Controlled)

以市价平仓。

**This endpoint is protected by the risk engine.**

```bash
curl -s -X POST "$BASE_URL/trader/newsliquid/v1/positions/close" \
  -H "$AUTH_HEADER" -H "Content-Type: application/json" \
  -d '{"symbol":"BTCUSDT","exchangeId":"binance"}'
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `symbol` | String | Yes | Trading pair to close |
| `exchangeId` | String | Yes | Exchange identifier |

**Response:**
```json
{
  "success": true,
  "data": {
    "symbol": "BTCUSDT",
    "closedQuantity": "0.1",
    "closePrice": "67500.00",
    "realizedPnl": "250.00"
  }
}
```

**Display to user:**
- "Position closed! BTCUSDT"
- "Closed 0.1 BTC @ $67,500"
- "Realized P&L: +$250.00"

---

### 19. Get Trade History

查询成交历史记录。

```bash
curl -s "$BASE_URL/trader/newsliquid/v1/trades/history?exchangeId=binance&symbol=BTCUSDT&limit=50" \
  -H "$AUTH_HEADER"
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `exchangeId` | String (query) | Yes | Exchange identifier |
| `symbol` | String (query) | No | Filter by trading pair |
| `limit` | String (query) | No | Max results (default: 50) |

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "tradeId": "98765",
      "orderId": "12345678",
      "symbol": "BTCUSDT",
      "side": "buy",
      "price": "60000.00",
      "quantity": "0.001",
      "fee": "0.06",
      "feeAsset": "USDT",
      "time": "2025-01-01T00:00:00Z"
    }
  ]
}
```

---

### 20. Get Leverage Tiers

获取交易对的可用杠杆档位信息。

```bash
curl -s "$BASE_URL/trader/newsliquid/v1/leverage?symbol=BTCUSDT&exchangeId=binance" \
  -H "$AUTH_HEADER"
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `symbol` | String (query) | Yes | Trading pair |
| `exchangeId` | String (query) | Yes | Exchange identifier |

**Response:**
```json
{
  "success": true,
  "data": [
    {"tier": 1, "maxLeverage": 125, "maxNotional": "50000"},
    {"tier": 2, "maxLeverage": 100, "maxNotional": "250000"},
    {"tier": 3, "maxLeverage": 50, "maxNotional": "1000000"}
  ]
}
```

---

### 21. Get Current Leverage

获取当前杠杆倍数设置。

```bash
curl -s "$BASE_URL/trader/newsliquid/v1/leverage/current?symbol=BTCUSDT&exchangeId=binance" \
  -H "$AUTH_HEADER"
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `symbol` | String (query) | Yes | Trading pair |
| `exchangeId` | String (query) | Yes | Exchange identifier |

**Response:**
```json
{
  "success": true,
  "data": {
    "symbol": "BTCUSDT",
    "leverage": "10"
  }
}
```

---

### 22. Set Leverage (Risk Controlled)

修改杠杆倍数。

**This endpoint is protected by the risk engine.**

```bash
curl -s -X PUT "$BASE_URL/trader/newsliquid/v1/leverage/current" \
  -H "$AUTH_HEADER" -H "Content-Type: application/json" \
  -d '{"symbol":"BTCUSDT","leverage":"20","exchangeId":"binance"}'
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `symbol` | String | Yes | Trading pair |
| `leverage` | String | Yes | New leverage multiplier (e.g., `"10"`, `"20"`, `"50"`) |
| `exchangeId` | String | Yes | Exchange identifier |

**Response:**
```json
{
  "success": true,
  "data": {
    "symbol": "BTCUSDT",
    "leverage": "20"
  }
}
```

**Display to user:**
- "Leverage updated! BTCUSDT: 20x"

---

### 23. Get Margin Mode

获取当前保证金模式（cross 全仓 / isolated 逐仓）。

```bash
curl -s "$BASE_URL/trader/newsliquid/v1/margin/mode?symbol=BTCUSDT&exchangeId=binance" \
  -H "$AUTH_HEADER"
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `symbol` | String (query) | No | Trading pair (if applicable) |
| `exchangeId` | String (query) | Yes | Exchange identifier |

**Response:**
```json
{
  "success": true,
  "data": {
    "marginMode": "cross"
  }
}
```

---

### 24. Get Position Mode

获取持仓模式（one-way 单向 / hedge 双向）。

```bash
curl -s "$BASE_URL/trader/newsliquid/v1/position/mode?exchangeId=binance" \
  -H "$AUTH_HEADER"
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `exchangeId` | String (query) | Yes | Exchange identifier |

**Response:**
```json
{
  "success": true,
  "data": {
    "positionMode": "one-way"
  }
}
```

---

### 25. Set Position Mode

设置持仓模式。

```bash
curl -s -X PUT "$BASE_URL/trader/newsliquid/v1/position/mode" \
  -H "$AUTH_HEADER" -H "Content-Type: application/json" \
  -d '{"positionMode":"hedge","exchangeId":"binance"}'
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `positionMode` | String | Yes | `one-way` or `hedge` |
| `exchangeId` | String | Yes | Exchange identifier |

**Response:**
```json
{
  "success": true,
  "data": {
    "positionMode": "hedge"
  }
}
```

**Display to user:**
- "Position mode updated to Hedge (two-way)"

---

### 26. Create Wallet Agent

创建钱包代理，用于管理 CEX 与链上之间的资金流转。

```bash
curl -s -X POST "$BASE_URL/trader/newsliquid/v1/walletagent/create" \
  -H "$AUTH_HEADER" -H "Content-Type: application/json" \
  -d '{"exchangeId":"binance"}'
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `exchangeId` | String | Yes | Exchange identifier |

**Response:**
```json
{
  "success": true,
  "data": {
    "address": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb",
    "status": "created"
  }
}
```

---

### 27. List Wallet Agents

获取钱包代理列表。

```bash
curl -s "$BASE_URL/trader/newsliquid/v1/walletagent/list?exchangeId=binance" \
  -H "$AUTH_HEADER"
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `exchangeId` | String (query) | Yes | Exchange identifier |

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "address": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb",
      "authorized": true,
      "createdAt": "2025-01-01T00:00:00Z"
    }
  ]
}
```

---

### 28. Query Wallet Agent by Address

按地址查询钱包代理详情。

```bash
curl -s "$BASE_URL/trader/newsliquid/v1/walletagent/address/0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb?exchangeId=binance" \
  -H "$AUTH_HEADER"
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `address` | String (path) | Yes | Wallet address (in URL path) |
| `exchangeId` | String (query) | Yes | Exchange identifier |

**Response:**
```json
{
  "success": true,
  "data": {
    "address": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb",
    "authorized": true,
    "createdAt": "2025-01-01T00:00:00Z"
  }
}
```

---

### 29. Authorize Wallet Agent

设置钱包代理的授权状态。

```bash
curl -s -X PUT "$BASE_URL/trader/newsliquid/v1/walletagent/authorize" \
  -H "$AUTH_HEADER" -H "Content-Type: application/json" \
  -d '{"address":"0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb","authorized":true,"exchangeId":"binance"}'
```

**Parameters:**

| Field | Type | Required | Description |
|---|---|---|---|
| `address` | String | Yes | Wallet agent address |
| `authorized` | Boolean | Yes | `true` to authorize, `false` to revoke |
| `exchangeId` | String | Yes | Exchange identifier |

**Response:**
```json
{
  "success": true,
  "data": {
    "address": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb",
    "authorized": true
  }
}
```

## Cross-Skill Workflows

### Workflow A: CEX Spot Trading

> User: "Buy 0.1 BTC on Binance"

```
1. opentrade-newsliquid  GET /market/ticker?symbol=BTCUSDT           → check current price
2. opentrade-newsliquid  GET /account/summary                        → verify available balance
3. opentrade-newsliquid  GET /market/metadata                        → confirm pair precision/limits
4. opentrade-newsliquid  POST /orders                                → place order
       {"symbol":"BTCUSDT","side":"buy","type":"market","quantity":"0.1","exchangeId":"binance"}
5. opentrade-newsliquid  GET /orders/open                            → confirm order status
```

**Data handoff**:
- `lastPrice` from step 1 → helps user decide order type (market vs limit)
- `availableBalance` from step 2 → validates user can afford the order
- Precision info from step 3 → formats quantity/price correctly

### Workflow B: CEX Futures Trading

> User: "Open a 10x long on ETH with $1000"

```
1. opentrade-newsliquid  GET /market/ticker?symbol=ETHUSDT           → check ETH price
2. opentrade-newsliquid  GET /account/summary                        → check margin balance
3. opentrade-newsliquid  GET /leverage/current?symbol=ETHUSDT        → check current leverage
4. opentrade-newsliquid  PUT /leverage/current                       → set leverage to 10x (if needed)
       {"symbol":"ETHUSDT","leverage":"10","exchangeId":"binance"}
5. opentrade-newsliquid  POST /orders                                → open long position
       {"symbol":"ETHUSDT","side":"buy","type":"market","quantity":"<calculated>","exchangeId":"binance"}
6. opentrade-newsliquid  GET /positions                              → verify position opened
```

**Data handoff**:
- `lastPrice` from step 1 → calculate quantity: `$1000 / ETH_price`
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
1. opentrade-newsliquid   GET /market/ticker?symbol=BTCUSDT          → CEX price
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
       {"symbol":"ETHUSDT","side":"sell","type":"market","quantity":"10","exchangeId":"binance"}
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

- **Missing exchangeId** → ask user which exchange (e.g., `binance`)
- **Missing symbol** → ask user which trading pair (e.g., `BTCUSDT`)
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
curl -s "$BASE_URL/trader/newsliquid/v1/market/ticker?symbol=BTCUSDT&exchangeId=binance" -H "$AUTH_HEADER"
# → BTC/USDT: $67,500.50 (+1.89%)
```

**User says:** "Buy 0.01 BTC at $65,000"

```bash
curl -s -X POST "$BASE_URL/trader/newsliquid/v1/orders" \
  -H "$AUTH_HEADER" -H "Content-Type: application/json" \
  -d '{"symbol":"BTCUSDT","side":"buy","type":"limit","quantity":"0.01","price":"65000","exchangeId":"binance"}'
# → Order placed! Buy 0.01 BTC @ $65,000 (Limit) — ID: 12345678
```

**User says:** "Close my BTC position"

```bash
curl -s -X POST "$BASE_URL/trader/newsliquid/v1/positions/close" \
  -H "$AUTH_HEADER" -H "Content-Type: application/json" \
  -d '{"symbol":"BTCUSDT","exchangeId":"binance"}'
# → Position closed! Realized P&L: +$250.00
```

**User says:** "Set my ETH leverage to 20x"

```bash
curl -s -X PUT "$BASE_URL/trader/newsliquid/v1/leverage/current" \
  -H "$AUTH_HEADER" -H "Content-Type: application/json" \
  -d '{"symbol":"ETHUSDT","leverage":"20","exchangeId":"binance"}'
# → Leverage updated! ETHUSDT: 20x
```

## Edge Cases

- **Risk engine rejects order**: Display the rejection reason clearly. Common causes: price too far from market (>10%), position too large, rate limited, insufficient balance. Suggest the user adjust parameters and retry.
- **Insufficient balance**: Check balance with `GET /account/summary` first. For futures, consider leverage — required margin = order value / leverage.
- **Rate limited**: If 30+ requests in 1 minute, wait 60 seconds before retrying. Inform the user about the cooldown.
- **Invalid trading pair**: Call `GET /market/metadata` to verify the symbol exists on the exchange.
- **Position mode conflict**: Cannot switch position mode while holding open positions. Close all positions first.
- **Leverage change with open positions**: Some exchanges restrict leverage changes when positions are open. Close positions or reduce size first.
- **Order quantity precision**: Use `GET /market/metadata` to check `quantityPrecision` and `pricePrecision`. Round values accordingly.
- **Minimum notional**: Orders below the minimum notional value (e.g., $5) will be rejected by the exchange.
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
- All endpoints accept `exchangeId` parameter to specify the exchange (e.g., `binance`)
- The API routes through the Newsliquid gateway with built-in risk controls — trades execute server-side
- No private keys or transaction signing involved — this is CEX trading via API
- CEX uses **standard amount units** (e.g., `0.1 BTC`), unlike DEX which uses minimal units (wei/lamports)
- Risk-controlled endpoints may reject requests — always display the rejection reason to the user
- Query parameters go in the URL, body parameters go in JSON request body
- Response format: `{"success": true/false, "data": {...}}` or `{"success": false, "error": "..."}`
- The skill uses the same `OPEN_TOKEN` as all other opentrade skills — no additional configuration needed
