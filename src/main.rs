use std::env;
use std::fs;
use dirs::home_dir;
use std::fs::OpenOptions;
use std::io::{Write, Read};


fn main() {

    let args: Vec<String>= env::args().collect();
    let config_dir: &str = "/.config/clinotes/notes/";
    match home_dir() {
        Some(home) => {

            let full_path = format!("{}{}", home.display(), config_dir);
            //println!("a{}", full_path);
            fs::create_dir_all(&full_path);
            Note::new(&args, &full_path);
        }
        None => println!("Erro ao criar diretorio em ~.config"),
    }

    
   
}

struct Note {
    file: String,
    message: String, 
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

}
