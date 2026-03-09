---
name: opentrade-transaction
description: "This skill should be used when the user asks to 'broadcast transaction', 'send tx', 'estimate gas', 'simulate transaction', 'check tx status', 'track my transaction', 'get gas price', 'gas limit', 'broadcast signed tx', or mentions broadcasting transactions, sending transactions on-chain, gas estimation, transaction simulation, tracking broadcast orders, or checking transaction status. Covers gas price, gas limit estimation, transaction simulation, transaction broadcasting, and order tracking across XLayer, Solana, Ethereum, Base, BSC, Arbitrum, Polygon, and 20+ other chains. Do NOT use for swap quote or execution — use opentrade-dex-swap instead. Do NOT use for general programming questions about transaction handling."
license: Apache-2.0
metadata:
  author: 6551
  version: "1.0.0"
  homepage: "https://6551.io"
---

# OpenTrade Onchain Gateway CLI

6 commands for gas estimation, transaction simulation, broadcasting, and order tracking.

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

- For swap quote and execution → use `opentrade-dex-swap`
- For market prices → use `opentrade-market`
- For token search → use `opentrade-token`
- For wallet balances / portfolio → use `opentrade-wallet`
- For transaction broadcasting → use this skill (`opentrade-transaction`)

## Quickstart

### Router Discovery

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

### Basic Commands

```bash
# Get current gas price on XLayer
opentrade gateway gas --chain xlayer --router okx --version v1

# Estimate gas limit for a transaction
opentrade gateway gas-limit --from 0xYourWallet --to 0xRecipient --chain xlayer --router okx --version v1

# Simulate a transaction (dry-run)
opentrade gateway simulate --from 0xYourWallet --to 0xContract --data 0x... --chain xlayer --router okx --version v1

# Broadcast a signed transaction
opentrade gateway broadcast --signed-tx 0xf86c...signed --address 0xYourWallet --chain xlayer --router okx --version v1

# Track order status
opentrade gateway orders --address 0xYourWallet --chain xlayer --order-id 123456789 --router okx --version v1
```

## Chain Name Support

The CLI accepts human-readable chain names and resolves them automatically.

| Chain | Name | chainIndex |
|---|---|---|
| XLayer | `xlayer` | 196 |
| Ethereum | `ethereum` | 1 |
| Solana | `solana` | 501 |
| Base | `base` | 8453 |
| BSC | `bsc` | 56 |
| Arbitrum | `arbitrum` | 42161 |
| Polygon | `polygon` | 137 |
| Optimism | `optimism` | 10 |
| Avalanche | `avalanche` | 43114 |
| Fantom | `fantom` | 250 |
| zkSync Era | `zksync` | 324 |
| Linea | `linea` | 59144 |
| Scroll | `scroll` | 534352 |
| Manta | `manta` | 169 |
| Mantle | `mantle` | 5000 |
| Blast | `blast` | 81457 |
| Merlin | `merlin` | 4200 |
| Mode | `mode` | 34443 |
| Berachain | `berachain` | 80084 |
| Taiko | `taiko` | 167000 |
| Zircuit | `zircuit` | 48900 |
| Ink | `ink` | 57073 |
| Corn | `corn` | 21000000 |
| Sonic | `sonic` | 146 |

Use the chain name in the `--chain` flag (e.g., `--chain xlayer`).

## Command Index

| Command | Description |
|---|---|
| `opentrade trade routers` | Discover available trading routers |
| `opentrade gateway gas` | Get current gas price |
| `opentrade gateway gas-limit` | Estimate gas limit for a transaction |
| `opentrade gateway simulate` | Simulate transaction execution (dry-run) |
| `opentrade gateway broadcast` | Broadcast a signed transaction |
| `opentrade gateway orders` | Query order status by order ID |

---

## 1. Router Discovery

Discover available trading routers for the authenticated user.

```bash
opentrade trade routers
```

**Output:**
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
- Use `router` and `version` values in subsequent commands
- If no routers are returned, use default: `router=okx`, `version=v1`

---

## 2. Gas Price

Get current gas price for a specific chain.

```bash
opentrade gateway gas --chain <chain_name> --router <router> --version <version>
```

**Parameters:**
- `--chain`: Chain name (e.g., `xlayer`, `ethereum`, `solana`)
- `--router`: Router name from router discovery (default: `okx`)
- `--version`: Router version from router discovery (default: `v1`)

**Example:**
```bash
opentrade gateway gas --chain xlayer --router okx --version v1
```

**Output:**
```json
{
  "code": "0",
  "data": [
    {
      "chainId": "196",
      "normal": {
        "maxFeePerGas": "100000000",
        "baseFee": "100000000",
        "gasPrice": "100000000",
        "priorityFeePerGas": "0"
      },
      "fast": {
        "maxFeePerGas": "100000000",
        "baseFee": "100000000",
        "gasPrice": "100000000",
        "priorityFeePerGas": "0"
      },
      "slow": {
        "maxFeePerGas": "100000000",
        "baseFee": "100000000",
        "gasPrice": "100000000",
        "priorityFeePerGas": "0"
      }
    }
  ],
  "msg": ""
}
```

**Display to user:**
- Convert wei to Gwei: `100000000 wei = 0.1 Gwei`
- Show all three speeds: slow, normal, fast
- Recommend "normal" for most transactions

---

## 3. Gas Limit Estimation

Estimate the gas limit required for a transaction.

```bash
opentrade gateway gas-limit \
  --from <sender_address> \
  --to <recipient_address> \
  --chain <chain_name> \
  --router <router> \
  --version <version> \
  [--value <amount_in_wei>] \
  [--data <hex_data>]
```

**Parameters:**
- `--from`: Sender wallet address (required)
- `--to`: Recipient address (required)
- `--chain`: Chain name (required)
- `--router`: Router name (default: `okx`)
- `--version`: Router version (default: `v1`)
- `--value`: Amount to send in wei (optional, default: `0`)
- `--data`: Transaction data in hex format (optional, default: `0x`)

**Example:**
```bash
opentrade gateway gas-limit \
  --from 0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb \
  --to 0x1234567890abcdef1234567890abcdef12345678 \
  --chain xlayer \
  --router okx \
  --version v1 \
  --value 1000000000000000000
```

**Output:**
```json
{
  "code": "0",
  "data": [
    {
      "gasLimit": "21000"
    }
  ],
  "msg": ""
}
```

**Display to user:**
- Show gas limit as integer: `21000`
- Estimate total gas cost: `gasLimit × gasPrice`
- Show USD equivalent if possible

---

## 4. Transaction Simulation

Simulate a transaction before broadcasting (dry-run).

```bash
opentrade gateway simulate \
  --from <sender_address> \
  --to <recipient_address> \
  --chain <chain_name> \
  --router <router> \
  --version <version> \
  [--value <amount_in_wei>] \
  [--data <hex_data>] \
  [--gas-limit <limit>] \
  [--gas-price <price_in_wei>]
```

**Parameters:**
- `--from`: Sender wallet address (required)
- `--to`: Recipient address (required)
- `--chain`: Chain name (required)
- `--router`: Router name (default: `okx`)
- `--version`: Router version (default: `v1`)
- `--value`: Amount to send in wei (optional, default: `0`)
- `--data`: Transaction data in hex format (optional, default: `0x`)
- `--gas-limit`: Gas limit (optional)
- `--gas-price`: Gas price in wei (optional)

**Example:**
```bash
opentrade gateway simulate \
  --from 0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb \
  --to 0x1234567890abcdef1234567890abcdef12345678 \
  --chain xlayer \
  --router okx \
  --version v1 \
  --value 1000000000000000000 \
  --gas-limit 21000 \
  --gas-price 100000000
```

**Output (Success):**
```json
{
  "code": "0",
  "data": [
    {
      "executionResult": {
        "used": 21000,
        "success": true,
        "gasLimit": 21000,
        "gasUsed": 21000
      }
    }
  ],
  "msg": ""
}
```

**Output (Failure):**
```json
{
  "code": "0",
  "data": [
    {
      "executionResult": {
        "used": 0,
        "success": false,
        "error": "execution reverted: insufficient balance"
      }
    }
  ],
  "msg": ""
}
```

**Display to user:**
- If `success: true`: "Simulation successful! Gas used: 21000"
- If `success: false`: "Simulation failed: [error message]"
- Show revert reason if available

---

## 5. Broadcast Transaction

Broadcast a signed transaction to the blockchain.

```bash
opentrade gateway broadcast \
  --signed-tx <signed_transaction_hex> \
  --address <sender_address> \
  --chain <chain_name> \
  --router <router> \
  --version <version>
```

**Parameters:**
- `--signed-tx`: Signed transaction in hex format (EVM) or base58 (Solana) (required)
- `--address`: Sender wallet address (required)
- `--chain`: Chain name (required)
- `--router`: Router name (default: `okx`)
- `--version`: Router version (default: `v1`)

**Example (EVM):**
```bash
opentrade gateway broadcast \
  --signed-tx 0xf86c808504a817c800825208941234567890abcdef1234567890abcdef12345678880de0b6b3a764000080820a96a0... \
  --address 0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb \
  --chain xlayer \
  --router okx \
  --version v1
```

**Example (Solana):**
```bash
opentrade gateway broadcast \
  --signed-tx 4hXTCkRzt9WyecNzV1XPgCDfGAZzQKNxLXgynz5QDuWWPSAZBZSHptvWRL3BjCvzUXRdKvHL2b7yGrRQcWyaqsaBCncVG7BFggS8w9snUts67BSh3EqKpXLUm5UMHfD7ZBe9GhARjbNQMLJ1QD3Spr6oMTBU6EhdB4RD8CP2xUxr2u3d6fos36PD98XS6oX8TQjLpsMwncs5DAMiD4nNnR8NBfyghGCWvCVifVwvA8B8TJxE1aHwYbgByseGmZJYDUSi5McBJdm7f9YY9d8FU7XMWoLM1gCWmoL92aPQuGHafb \
  --address YourSolanaWalletAddress \
  --chain solana \
  --router okx \
  --version v1
```

**Output:**
```json
{
  "code": "0",
  "data": [
    {
      "orderId": "123456789",
      "txHash": "0xabc123def456..."
    }
  ],
  "msg": ""
}
```

**Display to user:**
- "Transaction broadcast successfully!"
- "Order ID: 123456789"
- "Tx Hash: 0xabc123def456..."
- "Track status with: opentrade gateway orders --order-id 123456789"

---

## 6. Query Order Status

Query the status of a broadcast transaction by order ID.

```bash
opentrade gateway orders \
  --address <wallet_address> \
  --chain <chain_name> \
  --order-id <order_id> \
  --router <router> \
  --version <version>
```

**Parameters:**
- `--address`: Wallet address (required)
- `--chain`: Chain name (required)
- `--order-id`: Order ID from broadcast response (required)
- `--router`: Router name (default: `okx`)
- `--version`: Router version (default: `v1`)

**Example:**
```bash
opentrade gateway orders \
  --address 0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb \
  --chain xlayer \
  --order-id 123456789 \
  --router okx \
  --version v1
```

**Output:**
```json
{
  "code": "0",
  "data": [
    {
      "orderId": "123456789",
      "txHash": "0xabc123def456...",
      "txStatus": "2",
      "chainId": "196",
      "blockNumber": "12345678",
      "timestamp": "1709876543"
    }
  ],
  "msg": ""
}
```

**Transaction Status Codes:**
- `0`: Pending (not yet confirmed)
- `1`: Failed (transaction reverted or rejected)
- `2`: Success (confirmed on-chain)

**Display to user:**
- "Order 123456789: Success"
- "Tx Hash: 0xabc123def456..."
- "Block: 12345678"
- "Status: Confirmed on-chain"

---

## Workflow Examples

**User says:** "What's the gas price on XLayer?"

```bash
opentrade gateway gas --chain xlayer --router okx --version v1
# → Display: Normal: 0.1 Gwei, Fast: 0.1 Gwei, Slow: 0.1 Gwei
```

**User says:** "How much gas do I need to send 1 ETH?"

```bash
opentrade gateway gas-limit \
  --from 0xYourWallet \
  --to 0xRecipient \
  --chain ethereum \
  --router okx \
  --version v1 \
  --value 1000000000000000000
# → Display: Gas limit: 21000, Estimated cost: ~0.002 ETH ($5.50)
```

**User says:** "Simulate this transaction before I send it"

```bash
opentrade gateway simulate \
  --from 0xYourWallet \
  --to 0xContract \
  --chain xlayer \
  --router okx \
  --version v1 \
  --data 0xa9059cbb... \
  --gas-limit 100000
# → Display: Simulation successful! Gas used: 65432
```

**User says:** "Broadcast this signed transaction"

```bash
opentrade gateway broadcast \
  --signed-tx 0xf86c...signed \
  --address 0xYourWallet \
  --chain xlayer \
  --router okx \
  --version v1
# → Display: Transaction broadcast! Order ID: 123456789, Track status with Orders command
```

**User says:** "Check status of order 123456789"

```bash
opentrade gateway orders \
  --address 0xYourWallet \
  --chain xlayer \
  --order-id 123456789 \
  --router okx \
  --version v1
# → Display: Status: confirmed, TxHash: 0xabc..., Block: 12345678
```

## Edge Cases

- **MEV protection**: Broadcasting through OpenTrade nodes may offer MEV protection on supported chains.
- **Solana special handling**: Solana signed transactions use **base58** encoding (not hex). Ensure the `--signed-tx` format matches the chain.
- **Chain not supported**: call `opentrade trade routers` first to verify.
- **Node return failed**: the underlying blockchain node rejected the transaction. Common causes: insufficient gas, nonce too low, contract revert. Retry with corrected parameters.
- **Wallet type mismatch**: the address format does not match the chain (e.g., EVM address on Solana chain).
- **Network error**: retry once, then prompt user to try again later
- **Region restriction (error code 50125 or 80001)**: do NOT show the raw error code to the user. Instead, display a friendly message: `⚠️ Service is not available in your region. Please switch to a supported region and try again.`
- **Transaction already broadcast**: if the same `--signed-tx` is broadcast twice, the API may return an error or the same `txHash` — handle idempotently.

## Amount Display Rules

- Gas prices in Gwei for EVM chains (`18.5 Gwei`), never raw wei
- Gas limit as integer (`21000`, `145000`)
- USD gas cost estimate when possible
- Transaction values in UI units (`1.5 ETH`), never base units

## Global Notes

- **This skill does NOT sign transactions** — it only broadcasts pre-signed transactions
- Amounts in parameters use **minimal units** (wei/lamports)
- Gas price fields: use `normal` for most transactions, `fast` for urgent, `slow` for low priority
- EVM contract addresses must be **all lowercase**
- The CLI resolves chain names automatically (e.g., `ethereum` → `1`, `solana` → `501`)
- The CLI handles authentication internally via environment variables — set `OPEN_TOKEN` in `.env` file or environment
- Get your API token at: https://6551.io/mcp
- Each request consumes 1 quota unit
