# OpenSkills

AI coding assistant skills for crypto news, social media, and trading operations via the 6551 platform API.

## Available Skills

### News & Social Media

| Skill | Description |
|-------|-------------|
| `opennews` | Crypto news search, AI ratings, trading signals, and real-time updates |
| `opentwitter` | Twitter/X data including user profiles, tweet search, and KOL tracking |

### Trading Operations (opentrade)

| Skill | Description | Endpoints |
|-------|-------------|-----------|
| `opentrade-dex-swap` | DEX swap operations including quote, swap transaction, approve, and token pairs | 5 |
| `opentrade-transaction` | Transaction management including gas estimation, simulation, broadcast, and tracking | 6 |
| `opentrade-portfolio` | Wallet balance queries, portfolio value, and transaction history | 4 |
| `opentrade-market` | Market data including prices, K-line, trades, and smart money signals | 9 |
| `opentrade-token` | Token discovery, search, basic info, holders, and trending tokens | 5 |

**Total**: 7 skills covering 29 API endpoints

## Prerequisites

All skills require an API token from the 6551 platform. Get your token at: https://6551.io/mcp

### Environment Variable (Recommended)

```bash
export OPEN_TOKEN="your-token-here"
```

Or add to your `.env` file:

```bash
OPEN_TOKEN=your-token-here
```

### Config File (Alternative)

You can also store your token in a config file:

**macOS/Linux:**
```bash
mkdir -p ~/.config/openskills
echo '{"token": "your-token-here"}' > ~/.config/openskills/credentials.json
```

**Windows:**
```powershell
mkdir $env:APPDATA\openskills
echo '{"token": "your-token-here"}' > $env:APPDATA\openskills\credentials.json
```

**Priority:** Environment variable (`OPEN_TOKEN`) takes precedence over config file.

**Security warning**: Never commit tokens to git (add `.env` to `.gitignore`) and never expose credentials in logs, screenshots, or chat messages.

## Installation

### Recommended

```bash
npx skills add your-org/openskills
```

Works with Claude Code, Cursor, Codex CLI, and OpenCode. Auto-detects your environment and installs accordingly.

### Claude Code

```bash
# Run in Claude Code
/plugin marketplace add your-org/openskills
/plugin install openskills
```

## Skill Workflows

The skills work together in typical crypto workflows:

### Complete Trading Flow
`opentrade-token` → `opentrade-market` → `opentrade-dex-swap` → `opentrade-transaction`

1. Search and discover tokens
2. Analyze market data and prices
3. Get swap quote and transaction
4. Broadcast and track transaction

### News-Driven Trading
`opennews` → `opentrade-market` → `opentrade-dex-swap` → `opentrade-transaction`

1. Find trading signals from news
2. Check current token prices
3. Execute swap transaction
4. Monitor transaction status

### Portfolio Management
`opentrade-portfolio` → `opentrade-market` → `opentrade-token` → `opentrade-dex-swap`

1. Check current balances and total value
2. Get current prices for held tokens
3. Research token fundamentals
4. Rebalance portfolio via swaps

### Token Research
`opentrade-token` → `opentrade-market` → `opentrade-portfolio` → `opentrade-dex-swap`

1. Search and discover trending tokens
2. Analyze price data and K-line charts
3. Check current holdings
4. Execute trades based on research

### Social Sentiment Analysis
`opentwitter` → `opennews` → `opentrade-market` → `opentrade-dex-swap`

1. Search Twitter mentions and KOL activity
2. Cross-reference with news signals
3. Verify price movements
4. Execute informed trades

## API Base URL

All skills use the base URL: `https://ai.6551.io`

## Authentication

All requests require Bearer token authentication:

```bash
Authorization: Bearer $OPEN_TOKEN
```

The token can be provided via:
1. Environment variable: `OPEN_TOKEN`
2. Config file: `~/.config/openskills/credentials.json` (macOS/Linux) or `%APPDATA%\openskills\credentials.json` (Windows)

## Response Format

All API responses follow this format:

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

```json
{
  "code": 400,
  "message": "error message",
  "error": "detailed error information"
}
```

## Rate Limits

- Rate limits apply to all endpoints
- Each request consumes quota units (typically 1 unit per request)
- Monitor your usage at https://6551.io/mcp

## Supported Chains (Trading Operations)

| Chain | Chain ID | Status |
|-------|----------|--------|
| Ethereum | 1 | ✅ |
| BSC | 56 | ✅ |
| Polygon | 137 | ✅ |
| Arbitrum | 42161 | ✅ |
| Optimism | 10 | ✅ |
| Avalanche | 43114 | ✅ |
| Base | 8453 | ✅ |
| XLayer | 196 | ✅ |
| Solana | 501 | ✅ |

**Note**: Use `chainIndex` parameter (not `chainId`) for all API requests.

## License

Apache-2.0

## Support

For issues and feature requests, please visit: https://6551.io/support
