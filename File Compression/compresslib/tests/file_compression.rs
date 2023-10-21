use compresslib::{ create , write_to_file , compress , read_file };

#[test]
fn file_compression() -> Result< () , Box< dyn std::error::Error > > {
    let _ = create("file.txt")?;
    let _ = write_to_file("file.txt","Hoooowwwwdddddddyyyyyyy Thhhhheeeeeerrrrrrreeeeeee")?;
    let _ = compress("file.txt")?;
    let content = read_file("file.txt")?;

    if content == "Howdy There" { return Ok(()); }
    Err("Content not compressed as desired".into())
}
