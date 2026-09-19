// use std::{fs::File, io::{Read, Write}};

// fn main() {
//     let file_path = "output.txt";

//     // create file
//     let mut file = File::create(file_path).expect("File cant be found or created");
    
//     // write to file
//     file.write_all(b"first try").expect("Content couldnt be written");
//     println!("Content was written to file");

//     let mut content = String::new();
//     file.read_to_string(&mut content).expect("Cant read contents of file");

//     println!("Content: {}", content);
// }

use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Error, Write};
//use clap;
use serde::Serialize; 

#[derive(Serialize)]
struct User {
    name: String,
    age: u8,
} 

fn main() -> Result<(), Error> {
    let user = User {
        name: "Bob".to_string(),
        age: 30,
    };

    let path = "output.txt";
    let json_data = serde_json::to_string(&user).expect("Failed to serialize user");
 
    let file = File::create(path)?;
    let mut json_file = File::create("user.json").expect("Failed to create json file");

    json_file.write_all(json_data.as_bytes()).expect("Cant write to json file");
    println!("Json Data is written");
    
    let mut writer = BufWriter::new(file);
    
    // These writes go into an in-memory buffer, not directly to disk.
    writeln!(writer, "line one")?;
    writeln!(writer, "line two")?;
    writeln!(writer, "line three")?;    
    writeln!(writer, "line four")?;

    writer.flush()?;

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line?);
    }

    Ok(())
}
