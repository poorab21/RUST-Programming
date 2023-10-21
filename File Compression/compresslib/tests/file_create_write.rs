use compresslib::{ create , write_to_file };

#[test]
fn check_file_write() -> Result< () , Box< dyn std::error::Error > > {
    let _ = create("file2.txt")?;
    let _ = write_to_file("file2.txt","Hello there partner")?;
    
    Ok(())
}