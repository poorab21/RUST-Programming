use web_scrape_lib::web_scraper;
use web_scraper::scrape;
use tokio;

#[tokio::main]
async fn main() {
    // content to perform web scraping on
    let content = "Shawshank Redemption".to_string();

    let scraped_content: Result<(), Box<dyn std::error::Error>> = scrape(&content).await;

    // handle the result of the scraping process
    if let Err(e) = scraped_content {
        println!("{}",e.to_string());
    }
    else if let Ok(_) = scraped_content {
        println!("Data successfully scraped to Scraped.txt");
    }
}
