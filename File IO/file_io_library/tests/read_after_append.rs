use file_io_library::{ file_writer , file_reader };

#[test]
fn check_append_and_read() -> Result< () , Box< dyn std::error::Error > > {
    let _ = file_writer::append_to_file("file4.txt","Yes i am a cowboy ! you have a problem with that.")?;
    let content = file_reader::read_file("file4.txt")?;

    println!("{}",content);
    Ok(())
}