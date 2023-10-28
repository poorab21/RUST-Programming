use web_scrape_lib::web_scraper;
use web_scraper::scrape;

mod tests {

    use super::*;
    // assessment of the web scraping process
    #[tokio::test]
    async fn check_scrape() -> Result< () , Box< dyn std::error::Error > > {
        scrape("Doctor").await?;

        Ok(())
    }
}
