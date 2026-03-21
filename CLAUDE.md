# OpenSkills Project Guide

This repository contains AI coding assistant skills for crypto news, social media, and trading operations.

## Project Structure

```
openskills/
├── skills/
│   ├── opennews/          # Crypto news and AI ratings
│   ├── opentwitter/       # Twitter/X data and KOL tracking
│   └── opentrade/         # Trading operations
│       ├── opentrade-dex-swap/      # Swap execution
│       ├── opentrade-transaction/   # Transaction tracking
│       ├── opentrade-portfolio/      # Portfolio management
│       ├── opentrade-market/        # Market data
│       ├── opentrade-token/         # Token search
│       └── opentrade-newsliquid/    # CEX trading (spot, futures, leverage)
├── README.md
├── LICENSE
└── .env.example
```

## Skills Overview

### News & Social Media
- **opennews**: Crypto news search, AI ratings, trading signals
- **opentwitter**: Twitter/X profiles, tweets, followers, KOL tracking

### Trading Operations (opentrade)
- **opentrade-dex-swap**: DEX swap operations (quote, swap, approve, token pairs)
- **opentrade-transaction**: Transaction tracking (broadcast, query, history)
- **opentrade-portfolio**: Portfolio balance and value management
- **opentrade-market**: Market data (prices, liquidity, token search)
- **opentrade-token**: Token discovery (search, info, holders, trending)
- **opentrade-newsliquid**: CEX trading via Newsliquid gateway (spot & futures orders, positions, leverage, account, wallet agent)

## Authentication

All skills require an API token from https://6551.io/mcp

Set environment variable:
```bash
export OPEN_TOKEN="your-token"
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

## API Base URL

All skills use: `https://ai.6551.io`

## Common Workflows

### Trading Flow
1. Search token (`opentrade-token`)
2. Check balance (`opentrade-portfolio`)
3. Get quote (`opentrade-dex-swap`)
4. Execute swap (`opentrade-dex-swap`)
5. Broadcast transaction (`opentrade-transaction`)
6. Track status (`opentrade-transaction`)

### Portfolio Analysis
1. Get total value (`opentrade-portfolio`)
2. Get all balances (`opentrade-portfolio`)
3. Get prices (`opentrade-market`)
4. View history (`opentrade-portfolio`)

### News-Driven Trading
1. Search news (`opennews`)
2. Check Twitter sentiment (`opentwitter`)
3. Research token (`opentrade-market`)
4. Execute trade (`opentrade-dex-swap` + `opentrade-transaction`)

### CEX Trading Flow
1. Check ticker (`opentrade-newsliquid`)
2. Check account balance (`opentrade-newsliquid`)
3. Place order (`opentrade-newsliquid`)
4. Monitor positions (`opentrade-newsliquid`)
5. Close position (`opentrade-newsliquid`)

## Development Guidelines

- Each skill is independent and self-contained
- Skills communicate through data handoff (not direct calls)
- Use curl for HTTP requests (no external dependencies)
- Follow JWT Bearer token authentication
- Handle errors gracefully with user-friendly messages

## Security

- Never commit `.env` files
- Never expose tokens in logs or screenshots
- Use `.env.example` as template
- Keep tokens in environment variables

## Contributing

When adding new skills:
1. Create skill directory under `skills/`
2. Add `SKILL.md` with metadata and documentation
3. Follow existing skill structure and format
4. Update main README.md
5. Test thoroughly before committing

## License

MIT License - see LICENSE file for details
