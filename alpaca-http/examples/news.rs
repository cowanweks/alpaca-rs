use alpaca_http::{AlpacaHttpClient, Credentials, Environment};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let creds = Credentials::from_env()?;
    let env = Environment::Paper;

    let client = AlpacaHttpClient::new(creds, env)?;

    let account = client.get_account().await?;
    println!("{:?}", account);

    let response = client
        .get_enhanced_news_for_symbols("AAPL,MSFT", 10)
        .await?;
    println!("{:?}", response);

    Ok(())
}
