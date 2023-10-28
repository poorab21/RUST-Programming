// imports for web_scraper module
use reqwest::Client;
use std::collections::HashMap;
use crate::file_writer::scrape_to_file;
use tokio;
use std::sync::{ Arc , Mutex };
use std::time::Duration;

// function for returning url links of webpages associated with the input content
pub async fn links( content : &str ) -> Result< Vec<String> , Box< dyn std::error::Error > > {
    let query = format!("https://www.google.com/search?q={}",content);
    let mut links : Vec<String> = Vec::new();

    // google search on the input content
    let client = Client::new();
    let response = client.get( &query ).send().await?;

    // retrieval of url links present in the search result page
    let document =  scraper::Html::parse_document( response.text().await?.trim() );
    let links_selectors = scraper::Selector::parse("a")?;

    for link in document.select(&links_selectors) {
        let mut href = link.value().attr("href").unwrap().to_string();
        
        if href.starts_with("/url?q=") && links.len() < 5 {
            href = href.replace("/url?q=", "");
            links.push(href.split("&").nth(0).unwrap().to_string());
        }
    }
    // returning url links wrapped in Result eum
    Ok(links)
}

// function for retrieving content in a particular webpage
pub async fn link_content( query : &str ) -> Result< String , Box< dyn std::error::Error > > {
    let client = Client::new();
    let response = client.get( query ).timeout( Duration::from_secs(5) ).send().await?;

    Ok(response.text().await?)
}

// scrape the web for data associated with the specified input content
pub async fn scrape( content : &str ) -> Result< () , Box< dyn std::error::Error> > {
    // retrieve URLs of random webpages associated with the input content
    let links = links(content).await?;
    let scraped = Arc::new( Mutex::new( HashMap::<String,Vec<String>>::new() ) );
    let mut handles = Vec::new();
    
    // concurrently scrape the contents in each of the webpages which associate with the user input  
    for link in links {
        // create new reference to Mutex data ( i.e. HashMap )
        let scraped_reference = Arc::clone( &scraped );
        let content_clone = content.to_string();
        
        handles.push(
            // concurrently run scraping processes in the webpages and retrieve associated content
            tokio::spawn(
                async move {
                    let scraped_content = link_content( &link ).await;
                    let mut lines : Vec<String> = Vec::new();
                    
                    // handle error if there is an issue in processing the content in the webpage or the webpage itself
                    if let Err(e) = scraped_content {
                        println!("{}",e.to_string());
                        return;
                    }

                    // extract all paragraphs from the webpage 
                    let document = scraper::Html::parse_document( &scraped_content.unwrap() );
                    let selector = scraper::Selector::parse("p");
                    
                    if let Err(e) = selector {
                        eprintln!("{}",e.to_string());
                        std::process::exit(1);
                    }
 
                    // extract all lines containing the input content
                    for para in document.select(&selector.unwrap()) {
                        let line = para.text().collect::<String>();
                        
                        if line.trim().to_ascii_lowercase().contains( &content_clone.trim().to_ascii_lowercase() ) {
                            lines.push( para.text().collect::<String>().trim().to_string() );
                        }
                    }

                    let mut scraped_access = scraped_reference.lock().unwrap();

                    // insert the extracted content into the HashMap
                    scraped_access.insert( link , lines);
                }
            )
        );
    }

    // wait for all the concurrently running scraping processes on the different webpages to conclude
    for handle in handles {
        handle.await.unwrap();
    }

    // submit the entire result of the scraping process into the specified text file 
    scrape_to_file( &(*scraped.lock().unwrap()) , "Scraped.txt")?;

    Ok(())
}


// test functions for assessing the standalone functions in the web_scraper module
#[cfg(test)]
mod tests {
    use super::*;
    // assessment of the line_content() function
    #[tokio::test] 
    async fn check_link_content() -> Result< () , Box< dyn std::error::Error > > {
        link_content("https://www.google.com").await?;

        Ok(())
    }
}