use std::env;


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

}

fn create_bookmark(args: &Vec<String>) {

}

fn remove_bookmark(args: &Vec<String>) {

}

fn change_dir(args: &Vec<String>) {
//                println!("Please provide a valid subcommand. Use 'help' for usage information.")
}

