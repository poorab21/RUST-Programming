use reqwest::Client;
use scraper::{ Html , Selector };
use texting_robots::{ get_robots_url , Robot };
use std::collections::HashSet;
use std::time::Duration;
use std::sync::{ Arc , Mutex };

pub async fn description( link : String ) -> Result< ( String , String ) , Box< dyn std::error::Error > > {
    let response = Client::new().get(&link).timeout( Duration::from_secs(5) ).send().await?;
    let document = Html::parse_document( &response.text().await? );
    let title_of_page = Selector::parse(" head > title")?;
    let mut title = document.select( &title_of_page );
    println!("title generated for {}",link);

    Ok(( link , title.nth(0).unwrap_or(
        Html::parse_document( "<p>None</p>" ).select( &Selector::parse("p")? ).nth(0).unwrap()
    ).inner_html() ))
}

pub async fn extract_allowed_links( url : &str , crawled_links : Arc< Mutex< Vec<String> > > ) -> Result< Vec<String> , Box< dyn std::error::Error > > {
    let robots_url = get_robots_url(url)?;
    let response = Client::new().get(robots_url).timeout( Duration::from_secs(5) ).send().await?;
    let mut links = extract_url_links(url).await?;

    let robot = Robot::new("*", response.text().await?.as_bytes() )?;

    let mut access = crawled_links.lock().unwrap();
    links = links.into_iter().filter( |link| robot.allowed(link) && !access.contains(link) ).collect();
    access.extend( links.clone() );

    println!("allowed links extracted from {} =  {:#?}",url,links);
    Ok(links)
}

pub async fn extract_url_links( url : &str ) -> Result< Vec< String > , Box< dyn std::error::Error > > {
    let response = Client::new().get(url).timeout( Duration::from_secs(5) ).send().await?;
    let body = response.text().await?;
    let mut links : Vec<String> = Vec::new();
    let limit = 20;
    let mut i = 0;
    let document = Html::parse_document(&body);

    for link in document.select( &Selector::parse("a[href]")? ) {
        let href = link.value().attr("href").unwrap().to_string();

        if ( href.starts_with("https://") || href.starts_with("http://")) && i < limit {
            links.push(link.value().attr("href").unwrap().to_string());
            i += 1;
        }
    }

    let links : Vec<String> = links.into_iter().collect::<HashSet<_>>().into_iter().collect();
    
    Ok(links)
}