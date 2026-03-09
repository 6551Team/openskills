use anyhow::Result;
use clap::Subcommand;
use serde_json::json;

use super::Context;
use crate::output;

#[derive(Subcommand)]
pub enum PortfolioCommand {
    /// Get all token balances for a wallet
    AllBalances {
        /// Wallet address
        #[arg(long)]
        address: String,
        /// Chain
        #[arg(long)]
        chain: String,
    },
    /// Get total portfolio value in USD
    TotalValue {
        /// Wallet address
        #[arg(long)]
        address: String,
        /// Chain
        #[arg(long)]
        chain: String,
    },
    /// Batch query specific token balances
    Balances {
        /// Wallet address
        #[arg(long)]
        address: String,
        /// Chain
        #[arg(long)]
        chain: String,
        /// Token addresses (comma-separated)
        #[arg(long)]
        tokens: String,
    },
    /// Get address transaction history
    Transactions {
        /// Wallet address
        #[arg(long)]
        address: String,
        /// Chain
        #[arg(long)]
        chain: String,
        /// Number of results
        #[arg(long)]
        limit: Option<String>,
        /// Offset for pagination
        #[arg(long)]
        offset: Option<String>,
    },
}

pub async fn execute(ctx: &Context, cmd: PortfolioCommand) -> Result<()> {
    match cmd {
        PortfolioCommand::AllBalances { address, chain } => {
            all_balances(ctx, &address, &chain).await
        }
        PortfolioCommand::TotalValue { address, chain } => {
            total_value(ctx, &address, &chain).await
        }
        PortfolioCommand::Balances {
            address,
            chain,
            tokens,
        } => balances(ctx, &address, &chain, &tokens).await,
        PortfolioCommand::Transactions {
            address,
            chain,
            limit,
            offset,
        } => {
            transactions(ctx, &address, &chain, limit.as_deref(), offset.as_deref()).await
        }
    }
}

/// GET /trader/{router}/{version}/balance/all-balances
async fn all_balances(ctx: &Context, address: &str, chain: &str) -> Result<()> {
    let chain_index = crate::chains::resolve_chain(chain);
    let client = ctx.client()?;
    let data = client
        .get(
            "/balance/all-balances",
            &[("address", address), ("chainIndex", chain_index.as_str())],
        )
        .await?;
    output::success(data);
    Ok(())
}

/// GET /trader/{router}/{version}/balance/total-value
async fn total_value(ctx: &Context, address: &str, chain: &str) -> Result<()> {
    let chain_index = crate::chains::resolve_chain(chain);
    let client = ctx.client()?;
    let data = client
        .get(
            "/balance/total-value",
            &[("address", address), ("chainIndex", chain_index.as_str())],
        )
        .await?;
    output::success(data);
    Ok(())
}

/// POST /trader/{router}/{version}/balance/token-balances
async fn balances(ctx: &Context, address: &str, chain: &str, tokens: &str) -> Result<()> {
    let chain_index = crate::chains::resolve_chain(chain);
    let client = ctx.client()?;
    let token_list: Vec<&str> = tokens.split(',').collect();
    let body = json!({
        "address": address,
        "chainIndex": chain_index,
        "tokenAddresses": token_list,
    });
    let data = client.post("/balance/token-balances", &body).await?;
    output::success(data);
    Ok(())
}

/// GET /trader/{router}/{version}/balance/history
async fn transactions(
    ctx: &Context,
    address: &str,
    chain: &str,
    limit: Option<&str>,
    offset: Option<&str>,
) -> Result<()> {
    let chain_index = crate::chains::resolve_chain(chain);
    let client = ctx.client()?;
    let mut query: Vec<(&str, &str)> =
        vec![("address", address), ("chainIndex", chain_index.as_str())];
    if let Some(l) = limit {
        query.push(("limit", l));
    }
    if let Some(o) = offset {
        query.push(("offset", o));
    }
    let data = client.get("/balance/history", &query).await?;
    output::success(data);
    Ok(())
}
