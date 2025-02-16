use std::env;
use std::fs;
use dirs::home_dir;
use std::fs::OpenOptions;
use std::io::Write;


fn main() {

    let args: Vec<String>= env::args().collect();
   // let note: Note = Note::new(&args, full_path); 
    let config_dir: &str = "/.config/clinotes/notes/";
    match home_dir() {
        Some(home) => {

            let full_path = format!("{}{}", home.display(), config_dir);
            println!("{}", full_path);
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
    pub fn new(args: &[String], full_path: &String) -> Note{
    
        if args.len() < 3 {
            panic!("Poucos arguementos")
        }
       
        let file:String = args[1].clone();
        let message:String = args[2].clone();
        let file_full_path = format!("{}{}", full_path, file);
        println!("{}", file_full_path);
        let new_note = OpenOptions::new()
        .create(true) // Create the file if it doesn't exist
        .append(true) // Append to the file instead of overwriting
        .open(file_full_path);
        writeln!(new_note.expect("REASON"), "{}", message);
       

        Note {file, message}
    
    }

}
