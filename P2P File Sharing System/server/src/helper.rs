use std::path::Path;
use std::io::{Write, BufReader , BufRead, Error};
use std::fs::{ create_dir , OpenOptions, read_dir , File, DirEntry, remove_dir_all , metadata };
use p2p_lib::userfile::UserFile;
use std::thread::spawn;
use std::sync::{ Arc , Mutex };

pub fn folder_versioning( folder : &str , subfolder : &str ) -> Result< String , Box< dyn std::error::Error> > {
    let mut version = 0;
    let mut path = String::new();
    
    loop { 
        
        let result = if version == 0 {
            path = format!("Database/{}/{}",folder,subfolder);
            create_dir(
                path.as_str()
            )
        }
        else {
            path = format!("Database/{}/{} ({})",folder,subfolder,version);
            create_dir( 
                path.as_str()
            )
        };

        if let Ok(()) = result { break; }
        else if let Err(e) = result {  
            match e.to_string().as_str() {
                "Cannot create a file when that file already exists. (os error 183)" => {
                    version += 1;
                },
                _ => return Err(e.to_string().into())
            }
        }        
    };

    Ok(path)
}

pub fn upload_to_folder( userfile : UserFile ) -> Result< String , Box< dyn std::error::Error > > {
    let folder = userfile.ip_address();
    let subfolder = userfile.title().split(".").nth(0).unwrap();

    if !Path::new( format!("Database/{}",folder).as_str() ).exists() {
        create_dir(
            format!("Database/{}",folder)
        )?;
    }

    let path = folder_versioning(folder, subfolder.trim())?;

    OpenOptions::new().create(true).write(true).open(
        format!(
            "{}/{}",
            path,
            userfile.title().trim()        
        ).as_str()
    )?.write( userfile.content().as_bytes() )?;

    OpenOptions::new().create(true).write(true).open(
       format!(
            "{}/metadata.txt" ,
            path
        ).as_str()
    )?.write(
        format!(
            "Filename = {}\nDescription = {}\nFrom = {}\nAccessibility = {:?}" ,
            userfile.title().trim() ,
            userfile.description() ,
            userfile.ip_address() ,
            userfile.visibility()
        ).as_bytes()
    )?;

    Ok( 
        format!(
            "File was successfully uploaded to subfolder \"{}\" within folder \"{}\""
            ,
            path.split("/").last().unwrap() ,
            userfile.ip_address() ,
        ) 
    )
}

fn get_files( entry : Result< DirEntry , Error > , access : Arc<Mutex<Vec<String>>> ) -> Result< () , Error > {
    let entry = entry?;
    let files = read_dir( entry.path() )?;
    let mut content = String::from("Content:\n");
    let mut data_file = String::new();
    let mut metadata_file = String::new();

    for file in files {
        let file = format!("{:?}",file?.path().display()).replace(
            r"\\"
            , 
            "/"
        ).replace("\"", "");
        
        if file.contains("metadata.txt") { metadata_file = file; }
        else { data_file = file; }
    }
    
    let data_reader = BufReader::new(
        File::open(data_file.trim())?
    );
    let metadata_reader = BufReader::new(
        File::open(metadata_file.trim())?
    );

    for line in data_reader.lines() {
        content.push_str(
            format!(
                "{}\n" ,
                &line?
            ).as_str()
        );
    }
    content.push('\n');

    for line in metadata_reader.lines() {
        content.push_str(
            format!(
                "{}\n" ,
                &line?
            ).as_str()
        );
    }
    content.push_str(
        format!("Folder = {:?}\n",entry.file_name()).as_str()
    );

    access.lock().unwrap().push(content);
    Ok(())
}

pub fn retrieve_files( ip_address : &str ) -> Result< Vec<String> , Box< dyn std::error::Error > > {
    let directory = format!("Database/{}",ip_address);
    let subdirs = read_dir( Path::new( &directory ) )?;
    let mut tasks = Vec::new();
    let file_content = Arc::new( Mutex::new( Vec::<String>::new() ) );
    
    for entry in subdirs {
        let access = Arc::clone( &file_content );
        tasks.push(
            spawn(
                 move || get_files( entry , access ) 
            )
        )
    }

    for task in tasks {
        task.join().unwrap()?;
    }

    let data = file_content.lock().unwrap().clone().to_vec();
    Ok(data)
}

pub fn search_subfolder( subfolder : String , result_files : Arc<Mutex<Vec<String>>> , filename : String ) -> Result< () , std::io::Error > {
    let file_dirs = read_dir(
        format!("Database/{}",subfolder)
    )?;

    'outerloop : for file_dir in file_dirs {
        let file_dir = format!("{:?}",file_dir?.file_name()).replace(
            r"\\", 
            r"/"
        ).replace("\"", "");
        
        if file_dir.trim().contains(filename.trim()) {
            let metadata_file = read_dir(
                format!("Database/{}/{}",subfolder,file_dir)
            )?.filter( |file| {
                if let Ok(file) = file { file.path().ends_with("metadata.txt") }
                else { false } 
            }).nth(0).unwrap()?.path();

            let metadata = format!(
                "{:?}" ,
                metadata_file
            ).
            replace("\"", "").
            replace("\\", "/").
            replace(
                r"//", 
                r"/"
            );
            
            let file = OpenOptions::new().read(true).open( Path::new(&metadata) )?;
            let reader = BufReader::new( file );
            let mut content = String::new();
            
            for line in reader.lines() {
                let line = format!("{}\n",line?);

                if line.contains("Accessibility") && line.contains("PRIVATE") {
                    continue 'outerloop;
                }
                else if line.contains("Accessibility") {
                    continue;
                }

                content.push_str(&line);
            }
            content.push_str(
                format!("Folder = {}\n",file_dir).as_str()
            );
            result_files.lock().unwrap().push(content);
        }
    }

    Ok(())
}

pub fn search_file( filename : String , ip_address : &str ) -> Result< Vec<String> , Box< dyn std::error::Error > > {
    let directory = format!("Database");
    let subdirs = read_dir( Path::new( &directory ) )?;
    let result_files = Arc::new( Mutex::new( Vec::<String>::new() ) );
    let mut tasks = Vec::new();

    for subdir in subdirs {
        let subdir = subdir?.file_name().into_string().unwrap();

        if &subdir != ip_address {
            let access = Arc::clone(&result_files);
            let search_file = filename.clone();

            tasks.push(
                spawn(
                    move || search_subfolder( subdir , access , search_file )
                )
            );
        }
    }

    for task in tasks {
        task.join().unwrap()?; 
    }

    let result = result_files.lock().unwrap().clone().to_vec();
    Ok( result )
}

pub fn download( folder : &str , filename : &str , src_ip : &str , dest_ip : &str ) -> Result< String , Box< dyn std::error::Error > > {
    let file = OpenOptions::new().read(true).open(
        format!("Database/{}/{}/{}",src_ip,folder,filename)
    )?;
    let metadata_file = OpenOptions::new().read(true).open(
        format!("Database/{}/{}/metadata.txt",src_ip,folder)
    )?;

    let mut file_content = String::new();
    let mut metadata_content = String::new();
    let reader = BufReader::new(file);
    let metadata_reader = BufReader::new(metadata_file);

    for line in reader.lines() {
        let line = format!("{}\n",line?);
        file_content += line.as_str();
    }

    for line in metadata_reader.lines() {
        let line = format!("{}\n",line?);

        if line.contains("From") {
            metadata_content.push_str(
                format!("From = {}\n",dest_ip).as_str()
            );
        }
        else {
            metadata_content.push_str(&line);
        }
    }

    let destination = metadata(
        format!("Database/{}",dest_ip)
    );

    if !destination.is_ok() || !destination?.is_dir() {
        create_dir(
            format!("Database/{}",dest_ip)
        )?;
    }
    
    let folder = folder_versioning(dest_ip,folder)?;
    
    OpenOptions::new().create(true).write(true).open(
        format!("{}/{}",folder,filename)
    )?.write_all( file_content.as_bytes() )?;

    OpenOptions::new().create(true).write(true).open(
        format!("{}/metadata.txt",folder)
    )?.write_all( metadata_content.as_bytes() )?;

    Ok(
        file_content
    )
}

pub fn delete( ip_address : &str , foldername : &str ) -> Result< () , Box< dyn std::error::Error > > {
    
    let result = remove_dir_all(
        format!("Database/{}/{}",ip_address,foldername).as_str()
    )?;

    Ok(result)
}