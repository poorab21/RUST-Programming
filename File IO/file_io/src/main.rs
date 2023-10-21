extern crate file_io_library;
use file_io_library::{ file_reader , file_writer };
use file_reader::{ read_file , occurrences_of_words_in_file };
use file_writer::{ create_file , write_to_file , append_to_file };

fn main() {
    let result = create_file("file6.txt");

    if let Err(e) = result {
        println!("{:?}",e);
    }
    else if let Ok(message) = result {
        println!("{}",message);
    }

    let result = write_to_file("file6.txt","My name is Daniel Bryan");

    if let Err(e) = result {
        println!("{:?}",e);
    }
    else if let Ok(message) = result {
        println!("{}",message);
    }

    let result = append_to_file("file6.txt","good for you vegan");

    if let Err(e) = result {
        println!("{:?}",e);
    }
    else if let Ok(message) = result {
        println!("{}",message);
    }

    let result = read_file("file6.txt");

    if let Err(e) = result {
        println!("{:?}",e);
    }
    else if let Ok(content) = result {
        println!("{}",content);
    }

    let result = occurrences_of_words_in_file("file5.txt");

    if let Err(e) = result {
        println!("{:?}",e);
    }
    else if let Ok(word_content) = result {
        println!("{:?}",word_content);
    }
}
