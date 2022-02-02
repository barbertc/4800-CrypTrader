use std::process::{Command, Stdio};
use std::env;
use std::path::Path;
use std::fs;
use krakenrs::{BsType, KrakenCredentials, KrakenRestAPI, KrakenRestConfig, LimitOrder, MarketOrder, OrderFlag};
use log::Level;
use serde::Serialize;
use std::{collections::{BTreeMap, BTreeSet},io::Write,path::PathBuf};
use std::fs::File;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut arguments = Vec::<String>::new();

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
    
    let mut subcommand:String = "".to_string(); /* mutable strings are able to be updated later */
    let mut creds_path_input:String = "creds.json".to_string();
    let mut coin:String = "BTCUSD".to_string(); /* initialized coin to BTCUSD, bitcoin */

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
    if !creds_path.is_file() {
        println!("File: {} does not exist", creds_path_input);
        return;
    }
    
    println!("Found your path at: {}", creds_path_input);
    let mut krc = krakenrs::KrakenRestConfig::default();

    let credentials = match krakenrs::KrakenCredentials::load_json_file(creds_path) {
        Ok(res) => res,
        Err(e) => {
            println!("error loading creds, {}", e);
            return;
        }
    };

    krc.creds = credentials;
    
    let api = krakenrs::KrakenRestAPI::try_from(krc).expect("could not create kraken api");
    let result = api.get_account_balance().expect("api call failed");

    let ofile = match File::create("account_balance.json") {
        Ok(res) => res,
        Err(e) => {
            println!("error writing json, {}", e);
            return;
        }
    };

    match serde_json::to_writer(ofile, &result) {
        Ok(res) => res,
        Err(e) => {
            println!("error writing json, {}", e);
            return;
        }
    };
    
}




 // /*
    //  *          0      1      2      3 
    //  *      ./cryptr creds  balance
    //  *      ./cryptr creds  prices
    //  *      ./cryptr creds  stat
    //  *      ./cryptr creds  ticker   coin
    //  */
    // //arguments.push("krak".to_string());

    // match args.len() { /* rust's version of a switch statement*/
    //     4 => { /* 3 arguments + default exec */
    //         coin = args[3].to_string();
    //         subcommand = args[2].to_string();
    //         creds_path_input = args[1].to_string();
    //     },
    //     3 => { /* 2 arguments + default exec */
    //         subcommand = args[2].to_string();
    //         creds_path_input = args[1].to_string();
    //     },
    //     _ => { /* Default case */
    //         println!("\nIncorrect number of Arguments. [{}]\n", args.len());
    //         return;
    //     }
    // }

    // let creds_path = Path::new(&creds_path_input); 
    // if !creds_path.is_file() {return;}
    // println!("Found your path at: {}", creds_path_input);
    
    // arguments.push(creds_path_input);
    // arguments.push(subcommand.clone()); 
    // if subcommand == "ticker".to_string() {
    //     arguments.push(coin);
    // }

    // println!("{:?}", arguments);
    // let output = Command::new("krak").args(arguments.clone()).output().unwrap();
    // let stdout = String::from_utf8_lossy(&output.stdout);
    // println!("{}", stdout);
    // //fs::write("./out.json", stdout).expect("Unable to write file");
    // let output = Command::new("krak").args(arguments).stdout(Stdio::piped()).output().expect("failed to execute krak command");
    // let stdout = String::from_utf8_lossy(&output.stderr);
    // //println!("{:?}", output);
	// println!("{}", stdout);
    // fs::write("./out.json", stdout.to_string()).expect("Unable to write file");

    