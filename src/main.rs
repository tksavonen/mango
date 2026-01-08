use std::{io};
use std::env;
use chrono::Local;
use chrono::{DateTime, Utc};
use finnhub::models::stock::{RecommendationTrend};
use finnhub::{FinnhubClient};

mod display;
use display::{print_price, print_percent, print_i32_value};

// MAIN FUNCTION
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("FINNHUB_API_KEY")
    .expect("FINNHUB_API_KEY environment variable not set");
    let client = FinnhubClient::new(&api_key);

    // Welcoming messages
    println!("\nrunning mango v.1.0");
    let date = Local::now();
    println!("It is {} on {}", date.format("%H:%M"), date.format("%d.%m.%Y"));

    // Enter stock
    println!("\nEnter a stock name to get information: (e.g. AAPL)");
    let mut stock_name = String::new();
    io::stdin().read_line(&mut stock_name)?;
    let stock_name = stock_name.trim();
    
    // Get stock info
    let quote = client.stock().quote(&stock_name).await?;
    let rec: Vec<RecommendationTrend> = client.stock().recommendations(&stock_name).await?;

    // Present stock info
    println!("\nPRICE TRENDS");
    print_price("Current price", quote.current_price);
    print_price("Previous close", quote.previous_close);
    print_percent("Change after last close", quote.percent_change);
    print_price("Open", quote.open);
    print_price("Low", quote.low);
    print_price("High", quote.high);

    if let Some(latest) = rec.first() {
        println!("\nANALYST RECOMMENDATIONS");
        print_i32_value("Hold", latest.hold);
        print_i32_value("Buy", latest.buy);
        print_i32_value("Strong buy", latest.strong_buy);
        print_i32_value("Sell", latest.sell);
        print_i32_value("Sell", latest.strong_sell);
    }
    else {
        println!("No recommendation data available.");
    }

    let dt = DateTime::<Utc>::from_timestamp(quote.timestamp as i64, 0) 
        .expect("invalid timestamp"); 
    println!("\nLast updated: {} UTC", dt.format("%H:%M:%S"));
    println!("");

    Ok(())
}