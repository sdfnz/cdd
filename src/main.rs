use std::env;
use std::path::Path;
use std::fs;
use std::io::{Read, Write, ErrorKind};
use serde_json::{Result, Value, json};


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Please provide a directory or a subcommand and arguments. Use 'help' for usage information."); 
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
            let bookmarks = fs::File::open("cdd.json");
            match bookmarks {
                Ok(mut file) => {
                    let mut contents = String::new();
                    file.read_to_string(&mut contents);
                    add_bookmark(contents.as_str(), bookmark_name, bookmark_dir);
                },
                Err(e) => {
                    match e.kind() {
                        ErrorKind::NotFound => {
                            let mut m = serde_json::map::Map::new();
                            m.insert(bookmark_name.to_string(), json!(bookmark_dir.to_str()));
                            let mut contents = serde_json::to_string(&m)
                                .expect("Invalid value.");
                            add_bookmark(&contents, bookmark_name, bookmark_dir);
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
    let mut file = fs::File::create("cdd.json")
        .expect("Unable to create file.");
    let mut deser_file: Value = serde_json::from_str(contents)
        .expect("Unable to deserialize file.");
    let mut bkmk_JSON = deser_file.as_object_mut()
        .expect("Unable to convert to object.");
    bkmk_JSON.insert(bname.to_string(), json!(bdir.to_str()));
    let new_bookmarks = serde_json::to_string_pretty(&bkmk_JSON)
        .expect("Unable to serialize.");
    write!(file, "{b}", b=new_bookmarks)
        .expect("Invalid write.");
}

fn remove_bookmark(args: &Vec<String>) {
    if args.len() < 3 {
        println!("Please provide the name of a bookmark to delete.");
    } else {
        let bookmark_name = &args[2];
        let mut bookmarks = fs::File::open("cdd.json");
        match bookmarks {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents);
                let mut deser_file: Value = serde_json::from_str(&contents)
                    .expect("Unable to deserialize file.");
                let mut bkmk_JSON = deser_file.as_object_mut()
                    .expect("Unable to convert to object.");
                bkmk_JSON.remove(bookmark_name);
                let new_bookmarks = serde_json::to_string_pretty(&bkmk_JSON)
                    .expect("Unable to serialize.");
                fs::write("cdd.json", &new_bookmarks);
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
        let mut bat = fs::File::create("setDir.bat")
            .expect("Unable to create file.");
        write!(bat, "SET CD_PATH=\"{d}\"", d=directory.display())
            .expect("Invalid write.");
    } else {
        let bookmark_name = &args[1].as_str();
        let bookmarks = fs::File::open("cdd.json");
        match bookmarks {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents);
                let mut deser_file: Value = serde_json::from_str(&contents)
                    .expect("Unable to deserialize file.");
                let mut bkmk_JSON = deser_file.as_object_mut()
                    .expect("Unable to convert to object.");
                let destination = bkmk_JSON.get(&bookmark_name.to_string());
                match destination {
                    Some(dir) => {
                        let mut bat = fs::File::create("setDir.bat")
                            .expect("Unable to create file.");
                        write!(bat, "SET CD_PATH={d}", d=dir)
                            .expect("Invalid write.");
                    },
                    None => {
                        println!("Please provide a valid bookmark, directory, or subcommand.");
                        let mut bat = fs::File::create("setDir.bat")
                            .expect("Unable to create file.");
                        write!(bat, "SET CD_PATH=\"{d}\"", d=".")
                            .expect("Invalid write.");
                    },
                }
            },
            Err(e) => {
                println!("Encountered error opening file: {:?}", e);
            },
        }
    }
}
