use std::sync::{ Arc , Mutex };

use tokio::spawn;
use futures::future::join_all;
use crate::webpage::{ description , extract_allowed_links };
use crate::file_writer::{ record , clear };

async fn process_url( url : String , crawled_links : Arc<Mutex<Vec<String>>> , depth : usize ) -> Result< Vec<String> , Box< dyn std::error::Error > > {
    let child_links = extract_allowed_links(&url, crawled_links).await?;
    
    for link in &child_links {
        record( description( link.to_string() ).await? , depth, "crawled.txt")?;
    }

    Ok(child_links)
}

pub async fn crawl( root_link : String , depth : usize ) -> Result< &'static str , Box< dyn std::error::Error > > {
    let mut url_queue = vec![ root_link.clone() ];
    let mut i = 0;
    let crawled_links = Arc::new( Mutex::new( vec![ root_link.clone() ] ) );

    clear("crawled.txt")?;
    record( description(root_link).await? , i, "crawled.txt")?;
    i += 1;

    while i <= depth {
        let mut index = 0;
        let mut link_tasks = Vec::new();

        while let Some(link) = url_queue.get(index) {
            let already_crawled_links = Arc::clone( &crawled_links );
            let link = link.clone();

            link_tasks.push(
                spawn(
                    async move {
                        process_url(link , already_crawled_links,i)
                    }
                )
            );
            index += 1;
        }

        url_queue.clear();
        let child_links = join_all(link_tasks).await;

        for link in child_links { 
            let result = link?.await;
            
            if let Err(e) = result {
                println!("{}",e.to_string());
            }
            else if let Ok(links) = result {
                url_queue.extend( links );
            }
        }

        i += 1;
    }

    Ok("Crawled Data successfully recorded in crawled.txt")
}
