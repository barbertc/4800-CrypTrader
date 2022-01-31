use std::process::{Command, Stdio};
use std::env;
use std::path::Path;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut arguments = Vec::new();

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
    let mut coin:String = "BTCUSD".to_string(); /* initialized coin to BTCUSD, bitcoin */

    /*
     *          0      1      2      3 
     *      ./cryptr creds  balance
     *      ./cryptr creds  prices
     *      ./cryptr creds  stat
     *      ./cryptr creds  ticker   coin
     */
    //arguments.push("krak".to_string());

    match args.len() { /* rust's version of a switch statement*/
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
            println!("\nIncorrect number of Arguments. [{}]\n", args.len());
            return;
        }
    }

    let creds_path = Path::new(&creds_path_input); 
    if !creds_path.is_file() {return;}
    println!("Found your path at: {}", creds_path_input);
    
    arguments.push(creds_path_input);
    arguments.push(subcommand.clone()); 
    if subcommand == "ticker".to_string() {
        arguments.push(coin);
    }

    println!("{:?}", arguments);
<<<<<<< HEAD
    let output = Command::new("krak").args(arguments).output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("{}", stdout);
    //fs::write("./out.json", stdout).expect("Unable to write file");
=======
    let output = Command::new("krak")
						.args(arguments)
						.stdout(Stdio::piped())
						.output()
						.expect("failed to execute krak command");
    let stdout = String::from_utf8_lossy(&output.stderr);
    //println!("{:?}", output);
	println!("{}", stdout);
    fs::write("./out.json", stdout.to_string()).expect("Unable to write file");
>>>>>>> 0edc55fce7f694d92d18d1f2b9bf839f8c98b5c9
}
