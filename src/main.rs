use std::fs::{File, write};
use std::io::prelude::*;
use std::io::{BufReader};
use std::time::SystemTime;
use std::path::Path;
use std::fs;
use std::env;
use std::process;
use serde::{Serialize, Deserialize};

use serde_json::{Value};

#[derive(Serialize, Deserialize, Debug)]
struct Note {
    text: String,
    time_created: std::time::Duration
}

#[derive(Serialize, Deserialize, Debug)]
struct Notes {
    notes: Vec<Note>
}

fn load_file(fpath:&String, name: &String ) -> String{
    
    let mut file;
    
    let path = Path::new(&env::home_dir().unwrap()).join(fpath);
    let file_path = path.join(name);   
    // Check if the file exists
    if !path.exists(){
        fs::create_dir_all(&path);
        File::create(&file_path).unwrap();
        write(&file_path, r#"{"notes":[]}"#).expect("Unable to write");
    }
    
    file = File::open(&file_path).unwrap();
    
    
    
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    
    buf_reader.read_to_string(&mut contents);
    
    return contents    
    
}

fn write_to_file(data:String, fpath:&String, name: &String) -> std::io::Result<()>{
    let path = Path::new(&env::home_dir().unwrap()).join(fpath);
    let file_path = path.join(name);
    write(file_path, data);
    
    Ok(())
}

fn main(){

    let file_path:String = ".rq".to_string();
    let file_name:String = "database.json".to_string();
    
    let str_data = load_file(&file_path, &file_name);

    let mut data:Notes = serde_json::from_str(&str_data).unwrap();
    
    let args: Vec<String> = env::args().collect();

    
    if args.len() == 1 {
        if data.notes.is_empty() {
            println!("You have nothing left :)");

        } else {
            let note = &data.notes[0];
            println!("{}", note.text);
            data.notes.remove(0);
            let json = serde_json::to_string(&data).unwrap();
            write_to_file(json, &file_path, &file_name);
        }
        
    }else{
        let text = &args[1];
        let note = Note {
            text: text.to_string(),
            time_created: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap()
        };
        
        
        data.notes.push(note);
        let json = serde_json::to_string(&data).unwrap(); 
        
        write_to_file(json, &file_path, &file_name);
    }
}
