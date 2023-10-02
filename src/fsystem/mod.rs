use std::fs::File;
use std::io::BufReader;
use std::io::Write; //Trait
use std::io::Read; // Trait
//use std::io::prelude::*; // raits in general.


pub fn write_hl() -> std::io::Result<()> {
    let mut file = File::create("fstest.txt")?;    
    file.write_all(b"Hello World!")?;
    Ok(())
}

pub fn read_hl() -> std::io::Result<String> {
   let  file = File::open("fstest.txt");
   let mut content = String::new();

   file?.read_to_string(&mut content)?;
   Ok(content)
}

pub fn read_buffered_hl() -> std::io::Result<String> {
    let file = File::open("fstest.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();

    buf_reader.read_to_string(&mut content)?;

    Ok(content)
}



