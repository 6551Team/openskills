---
name: opentrade-transaction
description: "This skill should be used when the user asks to 'broadcast transaction', 'send tx', 'estimate gas', 'simulate transaction', 'check tx status', 'track my transaction', 'get gas price', 'gas limit', 'broadcast signed tx', or mentions broadcasting transactions, sending transactions on-chain, gas estimation, transaction simulation, tracking broadcast orders, or checking transaction status. Covers gas price, gas limit estimation, transaction simulation, transaction broadcasting, and order tracking across XLayer, Solana, Ethereum, Base, BSC, Arbitrum, Polygon, and 20+ other chains. Do NOT use for swap quote or execution — use okx-dex-swap instead. Do NOT use for general programming questions about transaction handling."
license: Apache-2.0
metadata:
  author: 6551
  version: "1.0.0"
  homepage: "https://6551.io"
---

# OpenTrade Onchain Gateway Skill

6 API endpoints for gas estimation, transaction simulation, broadcasting, and order tracking via HTTP.

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

- For swap quote and execution → use `opentrade-dex-swap`
- For market prices → use `opentrade-market`
- For token search → use `opentrade-token`
- For wallet balances / portfolio → use `opentrade-wallet`
- For transaction broadcasting → use this skill (`opentrade-transaction`)

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
- If response contains `"router": "okx"` and `"version": "v1"`, use: `https://ai.6551.io/open/trader/okx/v1/transaction/gas-price`
- If response is empty, use: `https://ai.6551.io/open/trader/okx/v1/transaction/gas-price`

```bash
# Get current gas price on XLayer
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/gateway/gas?chainIndex=196"

# Estimate gas limit for a transaction
curl -s -X POST -H "Authorization: Bearer $OPEN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"chainIndex":"196","fromAddress":"0xYourWallet","toAddress":"0xRecipient","txAmount":"0"}' \
  "https://ai.6551.io/open/trader/{router}/{version}/gateway/gas-limit"

# Simulate a transaction (dry-run)
curl -s -X POST -H "Authorization: Bearer $OPEN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"chainIndex":"196","fromAddress":"0xYourWallet","toAddress":"0xContract","txAmount":"0","extJson":{"inputData":"0x..."}}' \
  "https://ai.6551.io/open/trader/{router}/{version}/gateway/simulate"

# Broadcast a signed transaction
curl -s -X POST -H "Authorization: Bearer $OPEN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"signedTx":"0xf86c...signed","chainIndex":"196","address":"0xYourWallet"}' \
  "https://ai.6551.io/open/trader/{router}/{version}/gateway/broadcast"

# Track order status
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/gateway/orders?address=0xYourWallet&chainIndex=196&orderId=123456789"
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
| 1 | Chains | GET `/trader/{router}/{version}/gateway/chains` | Get supported chains for gateway |
| 2 | Gas | GET `/trader/{router}/{version}/gateway/gas` | Get current gas prices for a chain |
| 3 | GasLimit | POST `/trader/{router}/{version}/gateway/gas-limit` | Estimate gas limit for a transaction |
| 4 | Simulate | POST `/trader/{router}/{version}/gateway/simulate` | Simulate a transaction (dry-run) |
| 5 | Broadcast | POST `/trader/{router}/{version}/gateway/broadcast` | Broadcast a signed transaction |
| 6 | Orders | GET `/trader/{router}/{version}/gateway/orders` | Track broadcast order status |

## Cross-Skill Workflows

This skill is the **final mile** — it takes a signed transaction and sends it on-chain. It pairs with swap (to get tx data).

### Workflow A: Swap → Broadcast → Track

> User: "Swap 1 ETH for USDC and broadcast it"

```
1. opentrade-dex-swap      → get swap transaction data
       ↓ user signs the tx locally
2. opentrade-transaction   → broadcast signed transaction
       ↓ orderId returned
3. opentrade-transaction   → track order status
```

**Data handoff**:
- `tx.data`, `tx.to`, `tx.value`, `tx.gas` from swap → user builds & signs → `signedTx` for broadcast
- `orderId` from broadcast → `orderId` param in orders query

### Workflow B: Simulate → Broadcast → Track

> User: "Simulate this transaction first, then broadcast if safe"

```
1. opentrade-transaction   → simulate transaction
       ↓ simulation passes (no revert)
2. opentrade-transaction   → broadcast signed transaction
3. opentrade-transaction   → track order status
```

### Workflow C: Gas Check → Swap → Broadcast

> User: "Check gas, swap for USDC, then send it"

```
1. opentrade-transaction   → get gas price
2. opentrade-dex-swap      → get swap transaction
       ↓ user signs
3. opentrade-transaction   → broadcast signed transaction
4. opentrade-transaction   → track order status
```

## Operation Flow

### Step 1: Identify Intent

- Estimate gas for a chain → Gas command
- Estimate gas limit for a specific tx → GasLimit command
- Test if a tx will succeed → Simulate command
- Broadcast a signed tx → Broadcast command
- Track a broadcast order → Orders command
- Check supported chains → Chains command

### Step 2: Collect Parameters

- Missing chain → recommend XLayer (low gas, fast confirmation) as default, then ask which chain the user prefers
- Missing `signedTx` → remind user to sign the transaction first (this API does NOT sign)
- Missing wallet address → ask user
- For gas-limit / simulate → need `fromAddress`, `toAddress`, optionally `extJson.inputData` (calldata)
- For orders query → need `address` and `chainIndex`, optionally `orderId`

### Step 3: Call and Display

- Call API endpoint with parameters
- Display results with appropriate formatting
- For gas prices: show in Gwei for readability
- For simulation: clearly indicate success or failure

### Step 4: Suggest Next Steps

After displaying results, suggest relevant follow-up actions:

| Just called | Suggest |
|---|---|
| Chains | 1. Check gas price → Gas 2. Get swap quote → `opentrade-dex-swap` |
| Gas | 1. Estimate gas limit → GasLimit 2. Execute swap → `opentrade-dex-swap` |
| GasLimit | 1. Simulate transaction → Simulate 2. Broadcast → Broadcast |
| Simulate | 1. Broadcast if successful → Broadcast 2. Adjust parameters if failed |
| Broadcast | 1. Track order status → Orders |
| Orders | 1. Check again if pending 2. View transaction on explorer |

Present conversationally, e.g.: "Transaction broadcast! Would you like to track its status?" — never expose skill names or endpoint paths to the user.

## API Reference

### 1. Chains

Get supported chains for gateway.

```bash
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/gateway/chains"
```

No parameters required.

**Response fields:**
- `chainIndex`: Chain identifier
- `chainName`: Human-readable chain name
- `chainLogo`: Chain logo URL

### 2. Gas

Get current gas prices for a chain.

```bash
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/gateway/gas?chainIndex=<chain>"
```

**Parameters:**
| Param | Required | Description |
|---|---|---|
| `chainIndex` | Yes | Chain ID (e.g., 1 for Ethereum) |

**Response fields:**
- `gasPrice`: Standard gas price (wei)
- `maxFeePerGas`: Maximum fee per gas (EIP-1559)
- `maxPriorityFeePerGas`: Maximum priority fee per gas (EIP-1559)

### 3. GasLimit

Estimate gas limit for a transaction.

```bash
curl -s -X POST -H "Authorization: Bearer $OPEN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"chainIndex":"<chain>","fromAddress":"<from>","toAddress":"<to>","txAmount":"<amount>","extJson":{"inputData":"<data>"}}' \
  "https://ai.6551.io/open/trader/{router}/{version}/gateway/gas-limit"
```

**Request body:**
| Field | Required | Description |
|---|---|---|
| `chainIndex` | Yes | Chain ID |
| `fromAddress` | Yes | Sender address |
| `toAddress` | Yes | Recipient/contract address |
| `txAmount` | Yes | Transfer value in minimal units (default "0") |
| `extJson.inputData` | No | Encoded calldata (hex, for contract interactions) |

**Response fields:**
- `gasLimit`: Estimated gas limit

### 4. Simulate

Simulate a transaction (dry-run).

```bash
curl -s -X POST -H "Authorization: Bearer $OPEN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"chainIndex":"<chain>","fromAddress":"<from>","toAddress":"<to>","txAmount":"<amount>","extJson":{"inputData":"<data>"}}' \
  "https://ai.6551.io/open/trader/{router}/{version}/gateway/simulate"
```

**Request body:**
| Field | Required | Description |
|---|---|---|
| `chainIndex` | Yes | Chain ID |
| `fromAddress` | Yes | Sender address |
| `toAddress` | Yes | Recipient/contract address |
| `txAmount` | Yes | Transfer value in minimal units |
| `extJson.inputData` | Yes | Encoded calldata (hex) |

**Response fields:**
- `success`: Whether simulation succeeded
- `gasUsed`: Estimated gas used
- `returnData`: Return data from simulation
- `error`: Error message if simulation failed

### 5. Broadcast

Broadcast a signed transaction.

```bash
curl -s -X POST -H "Authorization: Bearer $OPEN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"signedTx":"<signed_tx>","chainIndex":"<chain>","address":"<address>"}' \
  "https://ai.6551.io/open/trader/{router}/{version}/gateway/broadcast"
```

**Request body:**
| Field | Required | Description |
|---|---|---|
| `signedTx` | Yes | Fully signed transaction (hex for EVM, base58 for Solana) |
| `chainIndex` | Yes | Chain ID |
| `address` | Yes | Sender wallet address |

**Response fields:**
- `orderId`: Order ID for tracking
- `txHash`: Transaction hash (if immediately available)

### 6. Orders

Track broadcast order status.

```bash
# Get all orders for an address
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/gateway/orders?address=<address>&chainIndex=<chain>"

# Get specific order by ID
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/gateway/orders?address=<address>&chainIndex=<chain>&orderId=<order_id>"
```

**Parameters:**
| Param | Required | Description |
|---|---|---|
| `address` | Yes | Wallet address |
| `chainIndex` | Yes | Chain ID |
| `orderId` | No | Specific order ID (from broadcast response) |

**Response fields:**
- `orderId`: Order identifier
- `txHash`: Transaction hash
- `status`: Order status (pending, confirmed, failed)
- `blockNumber`: Block number (if confirmed)
- `timestamp`: Timestamp

## Input / Output Examples

**User says:** "Get gas price on Ethereum"

```bash
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/gateway/gas?chainIndex=1"
# → Display: Gas price: 30 Gwei, Max fee: 35 Gwei
```

**User says:** "Broadcast this signed transaction on XLayer"

```bash
curl -s -X POST -H "Authorization: Bearer $OPEN_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"signedTx":"0xf86c...","chainIndex":"196","address":"0x..."}' \
  "https://ai.6551.io/open/trader/{router}/{version}/gateway/broadcast"
# → Display: Transaction broadcast! Order ID: 123456789, Track status with Orders command
```

**User says:** "Check status of order 123456789"

```bash
curl -s -H "Authorization: Bearer $OPEN_TOKEN" \
  "https://ai.6551.io/open/trader/{router}/{version}/gateway/orders?address=0x...&chainIndex=196&orderId=123456789"
# → Display: Status: confirmed, TxHash: 0xabc..., Block: 12345678
```

## Edge Cases

- **Invalid signed transaction**: returns error — verify transaction format
- **Insufficient gas**: simulation fails — increase gas limit
- **Transaction reverts**: simulation shows revert reason — fix contract interaction
- **Nonce too low**: broadcast fails — check wallet nonce
- **Order not found**: may still be processing — wait and retry
- **Network congestion**: transaction pending — monitor gas prices
- **Network error**: retry once, then prompt user to try again later
- **Region restriction (error code 50125 or 80001)**: do NOT show the raw error code to the user. Instead, display a friendly message: `⚠️ Service is not available in your region. Please switch to a supported region and try again.`

## Amount Display Rules

- Gas prices: display in Gwei for readability (1 Gwei = 10^9 wei)
- Transaction values: display in UI units with token symbol
- Always show USD equivalent when possible

## Global Notes

- EVM contract addresses must be **all lowercase**
- Use `chainIndex` parameter (not `chainId`)
- This API does NOT sign transactions — user must sign locally
- For Solana: use base58-encoded signed transactions
- All output is JSON format
- Get your API token at https://6551.io/mcp
- Each request consumes 1 quota unit
- Base URL: `https://ai.6551.io/open`
- Authentication: Bearer token in Authorization header
