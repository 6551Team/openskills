use anyhow::Result;
use clap::Subcommand;

use super::Context;
use crate::output;

#[derive(Subcommand)]
pub enum TradeCommand {
    /// Get supported trading routers
    Routers,
}

pub async fn execute(ctx: &Context, cmd: TradeCommand) -> Result<()> {
    match cmd {
        TradeCommand::Routers => routers(ctx).await,
    }
}

/// GET /open/trader/routers — get supported trading routers
async fn routers(ctx: &Context) -> Result<()> {
    let client = ctx.client()?;
    let data = client.get_trader_level("/routers").await?;
    output::success(data);
    Ok(())
}
