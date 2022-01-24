use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("\n==================");
    println!("|                |");
    println!("|  Cryp Traderz  |");
    println!("|                |");
    println!("==================\n\n");
    println!("USAGE:");
    println!("\t./cryptr <creds_path> SUBCOMMAND\n");
    println!("SUBCOMMANDS:");
    println!("\tbalance");
    println!("\tprices");
    println!("\tstat");
    println!("\tticker <coin>");
    println!();

    let mut subcommand:String; /* mutable strings are able to be updated later */
    let mut creds_path_input:String;
    let mut coin:String;

    /*
     *          0      1      2      3 
     *      ./cryptr creds  balance
     *      ./cryptr creds  prices
     *      ./cryptr creds  stat
     *      ./cryptr creds  ticker   coin
     */
    match args.len() {
        4 => { /* 3 arguments + default exec */
            coin = args[3].to_string();
            subcommand = args[2].to_string();
            creds_path_input = args[1].to_string();            
        },
        3 => { /* 2 arguments + default exec */
            subcommand = args[2].to_string();
            creds_path_input = args[1].to_string();
        },
        _ => { /* Default case */
            println!("\nIncorrect number of Arguments.\n");
            return;
        }
    }

    let creds_path = Path::new(&creds_path_input); 
    if !creds_path.is_file() {return;}

    println!("Found your path at : {}", creds_path_input);
}
