use compresslib::{ read_file , create , write_to_file };

#[test]
fn check_file_read() -> Result< () , Box< dyn std::error::Error > > {
    let _ = create("file.txt")?;
    let _ = write_to_file("file.txt","Hello There Partner");
    let content = read_file("file.txt")?;

    if content == "Hello There Partner" { return Ok(()); }
    Err("Content retrieved and displayed does not equal content actually in file".into())
}