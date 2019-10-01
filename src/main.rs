use std::env;
use std::path::Path;
use std::fs;
use std::io::Read;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Please provide a subcommand and arguments. Use 'help' for usage information."); 
    } else {
        match args[1].to_ascii_lowercase().as_str() {
            "help" => {
                display_help();
            },
            "create" => {
                create_bookmark(&args);
            },
            "remove" => {
                remove_bookmark(&args);
            },
            _ => {
                change_dir(&args);
            },
        }
    }
    println!("Arguments provided: {:?}", args);
}

fn display_help() {
    println!("Available arguments: help, create, remove. Type the name of the bookmark as an argument to be taken to that directory.")
}

fn create_bookmark(args: &Vec<String>) {
    if args.len() < 4 {
        println!("Please provide a name for the directory bookmark as well as a valid path.")
    } else {
        let bookmark_dir = Path::new(&args[3]);
        if bookmark_dir.is_dir() != true {
            println!("Please provide a valid directory path.")
        } else {
            let bookmark_name = &args[2];
            let mut bookmarks = fs::OpenOptions::new()
                                    .read(true)
                                    .append(true)
                                    .create(true)
                                    .open("cdd.txt");
            match bookmarks {
                Ok(mut file) => {
                    modify_or_add(file, bookmark_name, bookmark_dir);
                },
                Err(e) => {
                    println!("Encountered error opening file: {:?}", e);
                },
            }
        }
    }
}

fn modify_or_add(mut file: fs::File, bname: &str, bdir: &Path) {
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    println!("{:?}", contents);
}

fn remove_bookmark(args: &Vec<String>) {

}

fn change_dir(args: &Vec<String>) {
//                println!("Please provide a valid subcommand. Use 'help' for usage information.")
}
