use std::env;
use std::path::Path;
use std::fs;
use std::io::{Read, Write, ErrorKind};


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
            let bookmarks = fs::File::open("cdd.txt");
            match bookmarks {
                Ok(mut file) => {
                    let mut contents = String::new();
                    file.read_to_string(&mut contents);
                    add_bookmark(contents.as_str(), bookmark_name, bookmark_dir);
                },
                Err(e) => {
                    match e.kind() {
                        ErrorKind::NotFound => {
                            let mut contents = "";
                            add_bookmark(contents, bookmark_name, bookmark_dir);
                        },
                        _ => {
                            println!("Encountered error opening file: {:?}", e);
                        },
                    }
                },
            }
        }
    }
}

fn add_bookmark(mut contents: &str, bname: &str, bdir: &Path) {
    let mut file = fs::File::create("cdd.txt").expect("Unable to create file.");
    let lines: Vec<&str> = contents.split(';').collect();
    println!("{:?}", lines);
    for l in lines.into_iter() {
        if l.contains(bname) {
            println!("Replacing directory for bookmark: {:?}", bname);
        } else if l == "" {
        } else {
            write!(file, "{b};", b=l).expect("Invalid write.");
        }
    }
    let new_bookmark = format!("\r\n{name}@{dir};", name=bname, dir=bdir.display());
    write!(file, "{b}", b=new_bookmark).expect("Invalid write.");
}

fn remove_bookmark(args: &Vec<String>) {
    if args.len() < 3 {
        println!("Please provide the name of a bookmark to delete.");
    } else {
        let bookmark_name = &args[2];
        let bookmarks = fs::File::open("cdd.txt");
        match bookmarks {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents);
                let lines: Vec<&str> = contents.as_str().split(';').collect();
                let mut new_file = fs::File::create("cdd.txt").expect("Unable to create file.");
                for l in lines.into_iter() {
                    if l.contains(bookmark_name) {
                        println!("Bookmark removed");
                    } else if l == "" {
                    } else {
                        write!(new_file, "{b};", b=l).expect("Invalid write.");
                    }
                }
            },
            Err(e) => {
                println!("Encountered error opening file: {:?}", e);
            },
        }
    }
}

fn change_dir(args: &Vec<String>) {
    let directory = Path::new(&args[1]);
    if directory.is_dir() == true {
        let mut bat = fs::File::create("setDir.bat").expect("Unable to create file.");
        write!(bat, "SET CD_PATH=\"{d}\"", d=directory.display()).expect("Invalid write.");
    } else {
        let bookmark_name = &args[1].as_str();
        let bookmarks = fs::File::open("cdd.txt");
        match bookmarks {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents);
                let lines: Vec<&str> = contents.split(';').collect();
                let mut destination = ".";
                for l in lines.into_iter() {
                    if l.contains(bookmark_name) {
                        let d: Vec<&str> = l.split('@').collect();
                        destination = d[1];
                    }
                }
                if destination == "." {
                    println!("Please provide a valid bookmark, directory, or subcommand.");
                    let mut bat = fs::File::create("setDir.bat").expect("Unable to create file.");
                    write!(bat, "SET CD_PATH=\"{d}\"", d=destination).expect("Invalid write.");
                } else {
                    let mut bat = fs::File::create("setDir.bat").expect("Unable to create file.");
                    write!(bat, "SET CD_PATH=\"{d}\"", d=destination).expect("Invalid write.");
                }
            },
            Err(e) => {
                println!("Encountered error opening file: {:?}", e);
            },
        }
    }
}
