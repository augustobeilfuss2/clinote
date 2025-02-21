use std::env;
use std::fs;
use dirs::home_dir;
use std::fs::OpenOptions;
use std::io::{Write, Read};
use open::that;
use std::process::Command;


pub struct Note {
    pub file: String,
    pub message: String, 
}

impl Note {
    pub fn new(args: &[String], full_path: &String){
    
        if args.len() == 2 {
            let file:String = args[1].clone();
            let file_full_path = format!("{}{}", full_path, file);
            println!("{}", file_full_path);

            let read_note = OpenOptions::new()
            .read(true)
            .open(file_full_path);
            let mut content = String::new();
            read_note.expect("REASON").read_to_string(&mut content);
            println!("---{}--- \n {}", file, content)
        }

        if args.len() == 3 {
            let file:String = args[1].clone();
            let message:String = args[2].clone();
            let file_full_path = format!("{}{}", full_path, file);
            println!("{}", file_full_path);

            let new_note = OpenOptions::new()
            .create(true) 
            .append(true)
            .open(file_full_path);
            writeln!(new_note.expect("REASON"), "{}", message);
           
        }

       

    
    }

    pub fn edit(args: &[String], full_path: &String ){
        //open a file with default editor
        //needs xdg-open
        let e:String = args[1].clone();
        let file:String = args[2].clone();
        let file_full_path = format!("{}{}", full_path, file);
        println!("open {}",file_full_path);
        let status = Command::new("xdg-open")
        .arg(&file_full_path) // Pass the file path as an argument to xdg-open
        .status(); // Execute the command and wait for it to finish


    }
}