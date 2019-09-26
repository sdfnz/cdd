use std::env;
use std::path::Path;
use std::fs;
use std::io::{ErrorKind};

#[macro_use]
extern crate json;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Please provide a subcommand and arguments. Use 'help' for usage information."); 
    } else {
        match args[1].as_str() {
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
        let bookmark_name = &args[2];
        let bookmark_dir = Path::new(&args[3]);
        if bookmark_dir.is_dir() != true {
            println!("Please provide a valid directory path.")
        } else {
            let mut bookmarks = check_for_file("create");
        }
    }
}

fn remove_bookmark(args: &Vec<String>) {

}

fn change_dir(args: &Vec<String>) {
//                println!("Please provide a valid subcommand. Use 'help' for usage information.")
}

fn check_for_file(operation: &str) -> fs::File {
    let mut bookmarks = fs::File::open("cdd.json");
    match bookmarks {
        Ok(f) => {
            return f
        },
        Err(e) => {
            match e {
                NotFound => {
                    if operation == "create" {
                        let mut bookmarks = fs::File::create("cdd.json");
                        return bookmarks.unwrap()
                    } else {
                        panic!("Bookmark file not found. Please run with the create command to create a new one.")
                    }
                },
                _ => {
                    panic!("Encountered error: {:?}", e);
                },
            }
        },
    }
}
