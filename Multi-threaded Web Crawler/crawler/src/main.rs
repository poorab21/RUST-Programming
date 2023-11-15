use crawlerlib::crawler;
use tokio;

#[tokio::main]
async fn main() {
    println!("{:?}",crawler::crawl("https://www.lifewire.com/best-smart-glasses-4172796".to_string(), 0).await);
}
