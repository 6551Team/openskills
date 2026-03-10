use anyhow::Result;
use clap::Subcommand;
use serde_json::{json, Value};

use super::Context;
use crate::output;

#[derive(Subcommand)]
pub enum MarketCommand {
    /// Get single token price
    Price {
        /// Token contract address
        address: String,
        /// Chain (e.g. ethereum, solana, xlayer)
        #[arg(long)]
        chain: Option<String>,
    },
    /// Get batch prices for multiple tokens
    Prices {
        /// Comma-separated chainIndex:address pairs (e.g. "1:0xeee...,501:1111...")
        tokens: String,
        /// Default chain if not specified per token
        #[arg(long)]
        chain: Option<String>,
    },
    /// Get recent trades
    Trades {
        /// Token contract address
        address: String,
        /// Chain
        #[arg(long)]
        chain: Option<String>,
        /// Number of trades
        #[arg(long, default_value = "100")]
        limit: String,
    },
    /// Get K-line / candlestick data
    Kline {
        /// Token contract address
        address: String,
        /// Bar size: 1s, 1m, 5m, 15m, 30m, 1H, 4H, 1D, 1W, etc.
        #[arg(long, default_value = "1H")]
        bar: String,
        /// Number of data points
        #[arg(long, default_value = "100")]
        limit: String,
        /// Chain
        #[arg(long)]
        chain: Option<String>,
    },
    /// Get index price (aggregated from multiple sources)
    Index {
        /// Token contract address
        address: String,
        /// Chain
        #[arg(long)]
        chain: Option<String>,
    },
    /// Get supported chains for market signals
    SignalChains,
    /// Get latest signal list (smart money / KOL / whale activity)
    SignalList {
        /// Chain
        chain: String,
        /// Wallet type filter
        #[arg(long)]
        wallet_type: Option<String>,
        /// Minimum amount in USD
        #[arg(long)]
        min_amount_usd: Option<String>,
        /// Maximum amount in USD
        #[arg(long)]
        max_amount_usd: Option<String>,
        /// Minimum address count
        #[arg(long)]
        min_address_count: Option<String>,
        /// Maximum address count
        #[arg(long)]
        max_address_count: Option<String>,
        /// Token address filter
        #[arg(long)]
        token_address: Option<String>,
        /// Minimum market cap in USD
        #[arg(long)]
        min_market_cap_usd: Option<String>,
        /// Maximum market cap in USD
        #[arg(long)]
        max_market_cap_usd: Option<String>,
        /// Minimum liquidity in USD
        #[arg(long)]
        min_liquidity_usd: Option<String>,
        /// Maximum liquidity in USD
        #[arg(long)]
        max_liquidity_usd: Option<String>,
    },
    /// Get supported chains and protocols for meme pump
    MemepumpChains,
    /// List meme pump tokens with advanced filtering
    MemepumpTokens {
        /// Chain (e.g. solana, bsc). Required.
        chain: String,
        /// Protocol ID filter
        #[arg(long)]
        protocol_id: Option<String>,
        /// Token stage: NEW, MIGRATING, or MIGRATED (required by API)
        #[arg(long)]
        stage: String,
        /// Sort field: marketCap, volume1h, txCount1h, createdTimestamp, bondingPercent
        #[arg(long)]
        sort_by: Option<String>,
        /// Sort direction: asc or desc
        #[arg(long)]
        sort_order: Option<String>,
        /// Minimum token age in minutes
        #[arg(long)]
        min_age: Option<String>,
        /// Maximum token age in minutes
        #[arg(long)]
        max_age: Option<String>,
        /// Minimum market cap in USD
        #[arg(long)]
        min_market_cap: Option<String>,
        /// Maximum market cap in USD
        #[arg(long)]
        max_market_cap: Option<String>,
        /// Minimum 1h volume in USD
        #[arg(long)]
        min_volume: Option<String>,
        /// Maximum 1h volume in USD
        #[arg(long)]
        max_volume: Option<String>,
        /// Minimum 1h transaction count
        #[arg(long)]
        min_tx_count: Option<String>,
        /// Maximum 1h transaction count
        #[arg(long)]
        max_tx_count: Option<String>,
    },
    /// Get Meme Pump token details
    MemepumpTokenDetails {
        /// Token contract address
        address: String,
        /// Chain (e.g. solana, bsc)
        #[arg(long)]
        chain: Option<String>,
    },
    /// Get Meme Pump token developer info
    MemepumpTokenDevInfo {
        /// Token contract address
        address: String,
        /// Chain (e.g. solana, bsc)
        #[arg(long)]
        chain: Option<String>,
    },
    /// Get similar tokens for a Meme Pump token
    MemepumpSimilarTokens {
        /// Token contract address
        address: String,
        /// Chain (e.g. solana, bsc)
        #[arg(long)]
        chain: Option<String>,
    },
    /// Get Meme Pump token bundle (bundler/sniper) info
    MemepumpTokenBundleInfo {
        /// Token contract address
        address: String,
        /// Chain (e.g. solana, bsc)
        #[arg(long)]
        chain: Option<String>,
    },
    /// Get Meme Pump aped (co-invested) wallet data
    MemepumpApedWallet {
        /// Token contract address
        address: String,
        /// Chain (e.g. solana, bsc)
        #[arg(long)]
        chain: Option<String>,
    },
}

pub async fn execute(ctx: &Context, cmd: MarketCommand) -> Result<()> {
    match cmd {
        MarketCommand::Price { address, chain } => price(ctx, &address, chain).await,
        MarketCommand::Prices { tokens, chain } => prices(ctx, &tokens, chain).await,
        MarketCommand::Trades {
            address,
            chain,
            limit,
        } => trades(ctx, &address, chain, &limit).await,
        MarketCommand::Kline {
            address,
            chain,
            bar,
            limit,
        } => kline(ctx, &address, chain, &bar, &limit).await,
        MarketCommand::Index { address, chain } => index(ctx, &address, chain).await,
        MarketCommand::SignalChains => signal_chains(ctx).await,
        MarketCommand::SignalList {
            chain,
            wallet_type,
            min_amount_usd,
            max_amount_usd,
            min_address_count,
            max_address_count,
            token_address,
            min_market_cap_usd,
            max_market_cap_usd,
            min_liquidity_usd,
            max_liquidity_usd,
        } => {
            signal_list(
                ctx,
                &chain,
                wallet_type,
                min_amount_usd,
                max_amount_usd,
                min_address_count,
                max_address_count,
                token_address,
                min_market_cap_usd,
                max_market_cap_usd,
                min_liquidity_usd,
                max_liquidity_usd,
            )
            .await
        }
        MarketCommand::MemepumpChains => memepump_chains(ctx).await,
        MarketCommand::MemepumpTokens {
            chain,
            protocol_id,
            stage,
            sort_by,
            sort_order,
            min_age,
            max_age,
            min_market_cap,
            max_market_cap,
            min_volume,
            max_volume,
            min_tx_count,
            max_tx_count,
        } => {
            memepump_token_list(
                ctx,
                MemepumpTokenListParams {
                    chain,
                    protocol_id,
                    stage,
                    sort_by,
                    sort_order,
                    min_age,
                    max_age,
                    min_market_cap,
                    max_market_cap,
                    min_volume,
                    max_volume,
                    min_tx_count,
                    max_tx_count,
                },
            )
            .await
        }
        MarketCommand::MemepumpTokenDetails { address, chain } => {
            memepump_by_address(ctx, "/market/memepump-token-details", &address, chain).await
        }
        MarketCommand::MemepumpTokenDevInfo { address, chain } => {
            memepump_by_address(ctx, "/market/memepump-token-dev-info", &address, chain).await
        }
        MarketCommand::MemepumpSimilarTokens { address, chain } => {
            memepump_by_address(ctx, "/market/memepump-similar-tokens", &address, chain).await
        }
        MarketCommand::MemepumpTokenBundleInfo { address, chain } => {
            memepump_by_address(ctx, "/market/memepump-token-bundle-info", &address, chain).await
        }
        MarketCommand::MemepumpApedWallet { address, chain } => {
            memepump_by_address(ctx, "/market/memepump-aped-wallet", &address, chain).await
        }
    }
}

/// POST /trader/{router}/{version}/market/price
async fn price(ctx: &Context, address: &str, chain: Option<String>) -> Result<()> {
    let chain_index = chain
        .map(|c| crate::chains::resolve_chain(&c).to_string())
        .unwrap_or_else(|| ctx.chain_index_or("ethereum"));
    let client = ctx.client()?;
    let body = json!([{
        "chainIndex": chain_index,
        "tokenContractAddress": address
    }]);
    let data = client.post("/market/price", &body).await?;
    output::success(data);
    Ok(())
}

/// POST /trader/{router}/{version}/market/price — batch query
async fn prices(ctx: &Context, tokens: &str, chain: Option<String>) -> Result<()> {
    let default_chain = chain
        .map(|c| crate::chains::resolve_chain(&c).to_string())
        .unwrap_or_else(|| ctx.chain_index_or("ethereum"));
    let mut items: Vec<Value> = Vec::new();
    for pair in tokens.split(',') {
        let pair = pair.trim();
        if let Some((chain_part, addr)) = pair.split_once(':') {
            items.push(json!({
                "chainIndex": crate::chains::resolve_chain(chain_part),
                "tokenContractAddress": addr
            }));
        } else {
            items.push(json!({
                "chainIndex": &default_chain,
                "tokenContractAddress": pair
            }));
        }
    }
    let client = ctx.client()?;
    let data = client.post("/market/price", &Value::Array(items)).await?;
    output::success(data);
    Ok(())
}

/// GET /trader/{router}/{version}/market/trades
async fn trades(ctx: &Context, address: &str, chain: Option<String>, limit: &str) -> Result<()> {
    let chain_index = chain
        .map(|c| crate::chains::resolve_chain(&c).to_string())
        .unwrap_or_else(|| ctx.chain_index_or("ethereum"));
    let client = ctx.client()?;
    let data = client
        .get(
            "/market/trades",
            &[
                ("chainIndex", chain_index.as_str()),
                ("tokenContractAddress", address),
                ("limit", limit),
            ],
        )
        .await?;
    output::success(data);
    Ok(())
}

/// GET /trader/{router}/{version}/market/kline
async fn kline(
    ctx: &Context,
    address: &str,
    chain: Option<String>,
    bar: &str,
    limit: &str,
) -> Result<()> {
    let chain_index = chain
        .map(|c| crate::chains::resolve_chain(&c).to_string())
        .unwrap_or_else(|| ctx.chain_index_or("ethereum"));
    let client = ctx.client()?;
    let data = client
        .get(
            "/market/kline",
            &[
                ("chainIndex", chain_index.as_str()),
                ("tokenContractAddress", address),
                ("bar", bar),
                ("limit", limit),
            ],
        )
        .await?;
    output::success(data);
    Ok(())
}

/// POST /trader/{router}/{version}/market/current-price
async fn index(ctx: &Context, address: &str, chain: Option<String>) -> Result<()> {
    let chain_index = chain
        .map(|c| crate::chains::resolve_chain(&c).to_string())
        .unwrap_or_else(|| ctx.chain_index_or("ethereum"));
    let client = ctx.client()?;
    let body = json!([{
        "chainIndex": chain_index,
        "tokenContractAddress": address
    }]);
    let data = client.post("/market/current-price", &body).await?;
    output::success(data);
    Ok(())
}

/// GET /trader/{router}/{version}/market/signal-chains
async fn signal_chains(ctx: &Context) -> Result<()> {
    let client = ctx.client()?;
    let data = client.get("/market/signal-chains", &[]).await?;
    output::success(data);
    Ok(())
}

/// POST /trader/{router}/{version}/market/signal-list
#[allow(clippy::too_many_arguments)]
async fn signal_list(
    ctx: &Context,
    chain: &str,
    wallet_type: Option<String>,
    min_amount_usd: Option<String>,
    max_amount_usd: Option<String>,
    min_address_count: Option<String>,
    max_address_count: Option<String>,
    token_address: Option<String>,
    min_market_cap_usd: Option<String>,
    max_market_cap_usd: Option<String>,
    min_liquidity_usd: Option<String>,
    max_liquidity_usd: Option<String>,
) -> Result<()> {
    let chain_index = crate::chains::resolve_chain(chain);
    let client = ctx.client()?;
    let mut body = json!({
        "chainIndex": chain_index
    });
    let obj = body.as_object_mut().unwrap();
    if let Some(v) = wallet_type {
        obj.insert("walletType".into(), Value::String(v));
    }
    if let Some(v) = min_amount_usd {
        obj.insert("minAmountUsd".into(), Value::String(v));
    }
    if let Some(v) = max_amount_usd {
        obj.insert("maxAmountUsd".into(), Value::String(v));
    }
    if let Some(v) = min_address_count {
        obj.insert("minAddressCount".into(), Value::String(v));
    }
    if let Some(v) = max_address_count {
        obj.insert("maxAddressCount".into(), Value::String(v));
    }
    if let Some(v) = token_address {
        obj.insert("tokenAddress".into(), Value::String(v));
    }
    if let Some(v) = min_market_cap_usd {
        obj.insert("minMarketCapUsd".into(), Value::String(v));
    }
    if let Some(v) = max_market_cap_usd {
        obj.insert("maxMarketCapUsd".into(), Value::String(v));
    }
    if let Some(v) = min_liquidity_usd {
        obj.insert("minLiquidityUsd".into(), Value::String(v));
    }
    if let Some(v) = max_liquidity_usd {
        obj.insert("maxLiquidityUsd".into(), Value::String(v));
    }

    let data = client.post("/market/signal-list", &body).await?;
    output::success(data);
    Ok(())
}

/// GET /trader/{router}/{version}/market/memepump-chains
async fn memepump_chains(ctx: &Context) -> Result<()> {
    let client = ctx.client()?;
    let data = client.get("/market/memepump-chains", &[]).await?;
    output::success(data);
    Ok(())
}

/// Parameters for the memepump token list query.
struct MemepumpTokenListParams {
    chain: String,
    protocol_id: Option<String>,
    stage: String,
    sort_by: Option<String>,
    sort_order: Option<String>,
    min_age: Option<String>,
    max_age: Option<String>,
    min_market_cap: Option<String>,
    max_market_cap: Option<String>,
    min_volume: Option<String>,
    max_volume: Option<String>,
    min_tx_count: Option<String>,
    max_tx_count: Option<String>,
}

/// GET /trader/{router}/{version}/market/memepump-tokens
async fn memepump_token_list(ctx: &Context, params: MemepumpTokenListParams) -> Result<()> {
    let chain_index = crate::chains::resolve_chain(&params.chain).to_string();
    let client = ctx.client()?;

    let protocol_id = params.protocol_id.unwrap_or_default();
    let sort_by = params.sort_by.unwrap_or_default();
    let sort_order = params.sort_order.unwrap_or_default();
    let min_age = params.min_age.unwrap_or_default();
    let max_age = params.max_age.unwrap_or_default();
    let min_market_cap = params.min_market_cap.unwrap_or_default();
    let max_market_cap = params.max_market_cap.unwrap_or_default();
    let min_volume = params.min_volume.unwrap_or_default();
    let max_volume = params.max_volume.unwrap_or_default();
    let min_tx_count = params.min_tx_count.unwrap_or_default();
    let max_tx_count = params.max_tx_count.unwrap_or_default();

    let data = client
        .get(
            "/market/memepump-tokens",
            &[
                ("chainIndex", chain_index.as_str()),
                ("protocolId", &protocol_id),
                ("stage", &params.stage),
                ("sortField", &sort_by),
                ("sortOrder", &sort_order),
                ("minAge", &min_age),
                ("maxAge", &max_age),
                ("minMarketCapUsd", &min_market_cap),
                ("maxMarketCapUsd", &max_market_cap),
                ("minVolumeUsd", &min_volume),
                ("maxVolumeUsd", &max_volume),
                ("minTxCount", &min_tx_count),
                ("maxTxCount", &max_tx_count),
            ],
        )
        .await?;
    output::success(data);
    Ok(())
}

/// Shared helper for memepump endpoints that take (chainIndex, tokenContractAddress).
async fn memepump_by_address(
    ctx: &Context,
    path: &str,
    address: &str,
    chain: Option<String>,
) -> Result<()> {
    let chain_index = chain
        .map(|c| crate::chains::resolve_chain(&c).to_string())
        .unwrap_or_else(|| ctx.chain_index_or("solana"));
    let client = ctx.client()?;
    let data = client
        .get(
            path,
            &[
                ("chainIndex", chain_index.as_str()),
                ("tokenContractAddress", address),
            ],
        )
        .await?;
    output::success(data);
    Ok(())
}
