//! # Get Account Information
//!
//! This example demonstrates how to retrieve account information using the HTTP API.
//!
//! ## Prerequisites
//!
//! Set environment variables:
//! - `ALPACA_API_KEY`: Your Alpaca API key
//! - `ALPACA_API_SECRET`: Your Alpaca secret key
//!
//! ## Usage
//!
//! ```bash
//! cargo run -p alpaca-http --example http_get_account
//! ```

use alpaca_base::Environment;
use alpaca_http::AlpacaHttpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Get Account Information ===\n");

    // Create client from environment variables (Paper trading)
    let client = AlpacaHttpClient::from_env(Environment::Paper)?;
    println!("Client created for Paper trading environment");

    // Get account information
    println!("\nFetching account...");
    match client.get_account().await {
        Ok(account) => {
            println!("\n--- Account Details ---");
            println!("  Account Number: {}", account.account_number);
            println!("  Status: {:?}", account.status);
            println!("  Currency: {}", account.currency);
            println!("  Crypto Status: {}", account.crypto_status);
            println!();
            println!("--- Balances ---");
            println!("  Cash: ${}", account.cash);
            println!("  Portfolio Value: ${}", account.portfolio_value);
            println!("  Buying Power: ${}", account.buying_power);
            println!("  Equity: ${}", account.equity);
            println!("  Last Equity: ${}", account.last_equity);
            println!();
            println!("--- Margin ---");
            println!("  RegT Buying Power: ${}", account.regt_buying_power);
            println!(
                "  Effective Buying Power: ${}",
                account.effective_buying_power
            );
            println!(
                "  Non-Marginable Buying Power: ${}",
                account.non_marginable_buying_power
            );
            println!("  Options Buying Power: ${}", account.options_buying_power);
            println!("  Initial Margin: ${}", account.initial_margin);
            println!("  Maintenance Margin: ${}", account.maintenance_margin);
            println!("  SMA: ${}", account.sma);
            println!();
            println!("--- Options ---");
            println!(
                "  Options Approved Level: {}",
                account.options_approved_level
            );
            println!("  Options Trading Level: {}", account.options_trading_level);
            println!();
            println!("--- Trading Status ---");
            println!(
                "  Pattern Day Trader: {}",
                account
                    .pattern_day_trader
                    .map_or("N/A".to_string(), |v| v.to_string())
            );
            println!(
                "  Day Trade Count: {}",
                account
                    .daytrade_count
                    .map_or("N/A".to_string(), |v| v.to_string())
            );
            println!("  Trading Blocked: {}", account.trading_blocked);
            println!("  Account Blocked: {}", account.account_blocked);
            println!("  Shorting Enabled: {}", account.shorting_enabled);
            println!();
            println!("--- Market Values ---");
            println!("  Long Market Value: ${}", account.long_market_value);
            println!("  Short Market Value: ${}", account.short_market_value);
            println!(
                "  Position Market Value: ${}",
                account.position_market_value
            );
        }
        Err(e) => {
            eprintln!("Error fetching account: {}", e);
            return Err(e.into());
        }
    }

    println!("\n=== Example Complete ===");
    Ok(())
}
