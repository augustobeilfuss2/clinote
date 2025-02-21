use std::env;
use std::fs;
use dirs::home_dir;
use std::fs::OpenOptions;
use std::io::{Write, Read};
use clinote::Note;

use clap::{Arg, Command};

fn main() {

    let args: Vec<String>= env::args().collect();
    let config_dir: &str = "/.config/clinotes/notes/";

    let full_path = match home_dir() {
        Some(home) => {
            format!("{}{}", home.display(), config_dir)
        }
        None => {
            println!("Erro ao criar diretorio em ~.config");
            String::from("default_path")
        }
    };


    if args[1].starts_with("-"){
    let matches = Command::new("clinote")
    .version("0.1.0")
    .about("Edit the file with your default editor")
    .arg(
        Arg::new("edit")
            .short('e')
            .long("edit")
    ).get_matches();



        if let Some(edit_value) = matches.get_one::<String>("edit") {
            Note::edit(&args, &full_path)
        } 
        else {
        println!("No edit argument provided");
        }
    } 

    else {
        fs::create_dir_all(&full_path);
        Note::new(&args, &full_path);    
       

   }



}

