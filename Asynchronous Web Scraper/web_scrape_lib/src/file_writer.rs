// imports for the file_writer module
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

// function for inserting scraped content into a newly created file with specified input name 
pub fn scrape_to_file( content : &HashMap< String , Vec<String> > , filename : &str ) -> Result< String , Box< dyn std::error::Error > > {
    let mut file = File::create(filename)?;

    for ( link , scraped_content ) in content {
        if !scraped_content.is_empty() {
            file.write( format!("Web Resource = {} \n{:#?}\n\n",link,scraped_content).as_bytes() )?;
        } 
        else {
            file.write( format!("Web Resource = {}\n( No associated content found )\n\n",link).as_bytes() )?;
        }
    }

    Ok( format!("Scraped data successfully saved to {}",filename) )
} 

// test functions for assessing independent functions of the file_writer module
#[cfg(test)]
mod tests {

    use super::*;
    // assessment of the scrape_to_file() function
    #[test]
    fn check_scrape_to_file() -> Result< () , Box< dyn std::error::Error > > {
        scrape_to_file( &HashMap::from(
            [
                ( "Link 1".to_string() , Vec::from([ "Link 1 scraped content".to_string() ])  ) ,
                ( "link 2".to_string() , Vec::from([ "Link 2 scraped content".to_string() ]) ) ,
                ( "link 3".to_string() , Vec::from([ "Link 3 scraped content".to_string() ]) ) ,
                ( "link 4".to_string() , Vec::from([ "Link 4 scraped content".to_string() ]) ) ,
                ( "link 5".to_string() , Vec::from([ "Link 5 scraped content".to_string() ]) ) ,
                
            ]
        ) , "Scraped.txt" )?;

        Ok(())
    }
}
