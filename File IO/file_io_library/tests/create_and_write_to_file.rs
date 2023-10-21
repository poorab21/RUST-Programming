use file_io_library::{ file_writer };

#[test]
fn create_and_write_to_file() -> Result< () , Box< dyn std::error::Error > > {
    let _ = file_writer::create_file("file4.txt")?;
    
    let _ = file_writer::write_to_file("file4.txt","Howdy there partner\nHowdy ? are you a cowboy or something")?;

    Ok(())
}