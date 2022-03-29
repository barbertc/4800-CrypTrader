use serde_json::to_string_pretty;
use std::path::Path;
use std::fs::File;
extern crate serde_json;
extern crate serde_derive;
use core::convert::TryFrom;
use core::fmt::Debug;
use krakenrs::{BsType, KrakenCredentials, KrakenRestAPI, KrakenRestConfig, LimitOrder, MarketOrder, OrderFlag};
use log::Level;
use serde::Serialize;
use std::{
    collections::{BTreeMap, BTreeSet},
    io::Write,
    path::PathBuf,
};
extern crate colored;
use structopt::StructOpt;
use displaydoc::Display;
use colored::*;
use serde_json::to_string;
#[derive(StructOpt, Debug)]
struct KrakConfig {
    #[structopt(subcommand,)]
    command: Command,

    /// Credentials file, formatted in json. Required only for private APIs
    creds: String,

    /// Whether to pass "validate = true" with any orders (for testing)
    #[structopt(short, long)]
    validate: bool,
}

/// Commands supported by krak executable
#[derive(StructOpt, Display,Debug)]
enum Command {
    /// Get kraken system time
    Time,
    /// Get kraken system status
    SystemStatus,
    /// Get kraken's asset list
    Assets,
    /// Get kraken's asset pairs info: {pairs:?}
    AssetPairs { pairs: Vec<String> },
    /// Get kraken's ticker info: {pairs:?}
    Ticker { pairs: Vec<String> },
    /// Get account balance
    GetBalance,
    /// Get websockets token
    GetWebSocketsToken,
    /// Get open orders list
    GetOpenOrders,
    /// Cancel order: {id}
    CancelOrder { id: String },
    /// Cancel all orders
    CancelAllOrders,
    /// Cancel all orders after: {timeout}
    CancelAllOrdersAfter { timeout: u64 },
    /// Market buy order: {volume} {pair}
    MarketBuy { volume: String, pair: String },
    /// Market sell order: {volume} {pair}
    MarketSell { volume: String, pair: String },
    /// Limit buy order: {volume} {pair} @ {price}
    LimitBuy {
        volume: String,
        pair: String,
        price: String,
    },
    /// Limit sell order: {volume} {pair} @ {price}
    LimitSell {
        volume: String,
        pair: String,
        price: String,
    },
}

// pub extern fn main() {
// 	//account_balance("creds.json".to_string(), "stermonzl".to_string());
// 	ticker("creds.json".to_string(), "ETHUSD".to_string());
// 	mk_buy("creds.json".to_string(), "0.001".to_string(),"ETHUSD".to_string());
// 	//get_orders("creds.json".to_string(), "stermonzl".to_string());
// 	//mk_sell("creds.json".to_string(), "0.001".to_string(),"ETHUSD".to_string());
	
// }

#[no_mangle]
pub extern fn account_balance(creds: String, user: String) {
    
    //---------------------------------------------//
    let creds_path = Path::new(&creds);            //
    if !creds_path.is_file() {			   //    
        println!("File: {} does not exist", creds);// 	Checks Creds File 
        return;				   //
    }						   //
    println!("Found your path at: {}", creds);	   //
    //---------------------------------------------//


    //--------------------------------------------------------------------------------//
    let mut krc = krakenrs::KrakenRestConfig::default();                              //
    let credentials = match krakenrs::KrakenCredentials::load_json_file(creds_path) { //  
        Ok(res) => res,							      //
        Err(e) => {								      //
            println!("error loading creds, {}", e);				      //   Creates KrakenRestAPI object 
            return;								      //   and loads the creds into it
        }									      //
    };										      //
    krc.creds = credentials; 							      //
    //--------------------------------------------------------------------------------//


    let api = krakenrs::KrakenRestAPI::try_from(krc).expect("could not create kraken api"); // problem child!
    
    // This is where we make the actual call to the api to get our information.    
    let result = api.get_account_balance().expect("api call failed"); 

    //------------------------------------------------------//
                                                            //
    let ofile = match File::create("account_balance.json") {//
        Ok(res) => res,				    //
        Err(e) => {					    //
            println!("error writing json, {}", e);	    //
            return;					    //
        }						    //
    };							    //   File Writing Portion,
    match serde_json::to_writer(ofile, &result) {	    //   creates account balance.json and 
        Ok(res) => res, 				    //   uses to_writer to write it out 
        Err(e) => {					    //   as a nice json format
            println!("error writing json, {}", e);	    //
            return;					    //
        }						    //
    };							    //
    //------------------------------------------------------//
}

#[no_mangle]
pub extern fn get_orders(creds: String, user: String) {
    
    //---------------------------------------------//
    let creds_path = Path::new(&creds);            //
    if !creds_path.is_file() {			   //    
        println!("File: {} does not exist", creds);// 	Checks Creds File 
        return;				   //
    }						   //
    println!("Found your path at: {}", creds);	   //
    //---------------------------------------------//


    //--------------------------------------------------------------------------------//
    let mut krc = krakenrs::KrakenRestConfig::default();                              //
    let credentials = match krakenrs::KrakenCredentials::load_json_file(creds_path) { //  
        Ok(res) => res,							      //
        Err(e) => {								      //
            println!("error loading creds, {}", e);				      //   Creates KrakenRestAPI object 
            return;								      //   and loads the creds into it
        }									      //
    };										      //
    krc.creds = credentials; 							      //
    //--------------------------------------------------------------------------------//


    let api = krakenrs::KrakenRestAPI::try_from(krc).expect("could not create kraken api"); // problem child!
    
    // This is where we make the actual call to the api to get our information.    
    let result = api.get_open_orders(None).expect("api call failed"); 
    println!("{:?}",result);
    //------------------------------------------------------//
                                                            //
    let ofile = match File::create("open_orders.json") {//
        Ok(res) => res,				    //
        Err(e) => {					    //
            println!("error writing json, {}", e);	    //
            return;					    //
        }						    //
    };							    //   File Writing Portion,
    match serde_json::to_writer(ofile, &result) {	    //   creates account balance.json and 
        Ok(res) => res, 				    //   uses to_writer to write it out 
        Err(e) => {					    //   as a nice json format
            println!("error writing json, {}", e);	    //
            return;					    //
        }						    //
    };							    //
    //------------------------------------------------------//
}

#[no_mangle]
pub extern fn ticker(creds: String, coin: String) {
    
    //---------------------------------------------//
    let creds_path = Path::new(&creds);            //
    if !creds_path.is_file() {			   //    
        println!("File: {} does not exist", creds);// 	Checks Creds File 
        return;				   //
    }						   //
    //println!("Found your path at: {}", creds);	   //
    //---------------------------------------------//


    //--------------------------------------------------------------------------------//
    let mut krc = krakenrs::KrakenRestConfig::default();                              //
    let credentials = match krakenrs::KrakenCredentials::load_json_file(creds_path) { //  
        Ok(res) => res,							      //
        Err(e) => {								      //
            println!("error loading creds, {}", e);				      //   Creates KrakenRestAPI object 
            return;								      //   and loads the creds into it
        }									      //
    };										      //
    krc.creds = credentials; 							      //
    //--------------------------------------------------------------------------------//


    let api = krakenrs::KrakenRestAPI::try_from(krc).expect("could not create kraken api"); // problem child!
    
    // This is where we make the actual call to the api to get our information.    
    let result = api.ticker(vec![coin]).expect("api call failed"); 

    //------------------------------------------------------//
                                                            //
    let ofile = match File::create("ticker.json") {//
        Ok(res) => res,				    //
        Err(e) => {					    //
            println!("error writing json, {}", e);	    //
            return;					    //
        }						    //
    };							    //   File Writing Portion,
    match serde_json::to_writer(ofile, &result) {	    //   creates account balance.json and 
        Ok(res) => res, 				    //   uses to_writer to write it out 
        Err(e) => {					    //   as a nice json format
            println!("error writing json, {}", e);	    //
            return;					    //
        }						    //
    };							    //
    //------------------------------------------------------//
}

#[no_mangle]
pub extern fn mk_buy(creds: String, vol: String, par: String) {
    
    //---------------------------------------------//
    let creds_path = Path::new(&creds);            //
    if !creds_path.is_file() {			   //    
        println!("File: {} does not exist", creds);// 	Checks Creds File 
        return;				   //
    }						   //
    //println!(" | Found your path at: {} | ", creds.clone());	   //
    //---------------------------------------------//
    let come_on = Command::MarketBuy {volume: vol.clone(), pair: par.clone()};
    let mut config = KrakConfig{
    	command: come_on,
    	creds: creds.clone(),
    	validate: false,
    };

    //let b = par.clone();
    println!(" | {:?}",config);
    //config.validate = true;
    //config.creds = creds;
    
    //println!("config.validate");
    //--------------------------------------------------------------------------------//
    let mut krc = krakenrs::KrakenRestConfig::default();                              //
    let credentials = match krakenrs::KrakenCredentials::load_json_file(creds_path) { //  
        Ok(res) => res,							      //
        Err(e) => {								      //
            println!("error loading creds, {}", e);				      //   Creates KrakenRestAPI object 
            return;								      //   and loads the creds into it
        }									      //
    };										      //
    krc.creds = credentials; 							      //
    //--------------------------------------------------------------------------------//
    println!(" | loaded credentials | ");

    let api = krakenrs::KrakenRestAPI::try_from(krc).expect("could not create kraken api"); // problem child!
    println!(" | api loaded | ");
    // This is where we make the actual call to the api to get our information.    
    let result = api.add_market_order(MarketOrder {
    						bs_type: BsType::Buy,
                        			volume: vol,
                        			pair: par,
                        			oflags: Default::default(),
                    				},
                    			None,
                    			config.validate,
    					).expect(&"api call failed");
    println!(" | sale successful | ");
    //------------------------------------------------------//
                                                            //
    let ofile = match File::create("buy.json") {//
        Ok(res) => res,				    //
        Err(e) => {					    //
            println!("error writing json, {}", e);	    //
            return;					    //
        }						    //
    };							    //   File Writing Portion,
    match serde_json::to_writer(ofile, &result) {	    //   creates account balance.json and 
        Ok(res) => res, 				    //   uses to_writer to write it out 
        Err(e) => {					    //   as a nice json format
            println!("error writing json, {}", e);	    //
            return;					    //
        }						    //
    };							    //
    //------------------------------------------------------//
    println!(" | write successful | ");
}

#[no_mangle]
pub extern fn mk_sell(creds: String, vol: String, par: String) {
    
    //---------------------------------------------//
    let creds_path = Path::new(&creds);            //
    if !creds_path.is_file() {			   //    
        println!("File: {} does not exist", creds);// 	Checks Creds File 
        return;				   //
    }						   //
    //println!(" | Found your path at: {} | ", creds.clone());	   //
    //---------------------------------------------//
    let come_on = Command::MarketSell {volume: vol.clone(), pair: par.clone()};
    let mut config = KrakConfig{
    	command: come_on,
    	creds: creds.clone(),
    	validate: false,
    };

    //let b = par.clone();
    println!(" | {:?}",config);
    //config.validate = true;
    //config.creds = creds;
    
    //println!("config.validate");
    //--------------------------------------------------------------------------------//
    let mut krc = krakenrs::KrakenRestConfig::default();                              //
    let credentials = match krakenrs::KrakenCredentials::load_json_file(creds_path) { //  
        Ok(res) => res,							      //
        Err(e) => {								      //
            println!("error loading creds, {}", e);				      //   Creates KrakenRestAPI object 
            return;								      //   and loads the creds into it
        }									      //
    };										      //
    krc.creds = credentials; 							      //
    //--------------------------------------------------------------------------------//
    println!(" | loaded credentials | ");

    let api = krakenrs::KrakenRestAPI::try_from(krc).expect("could not create kraken api"); // problem child!
    println!(" | api loaded | ");
    // This is where we make the actual call to the api to get our information.    
    let result = api.add_market_order(MarketOrder {
    						bs_type: BsType::Sell,
                        			volume: vol,
                        			pair: par,
                        			oflags: Default::default(),
                    				},
                    			None,
                    			config.validate,
    					).expect(&"api call failed");
    println!(" | sale successful | ");
    //------------------------------------------------------//
                                                            //
    let ofile = match File::create("sell.json") {//
        Ok(res) => res,				    //
        Err(e) => {					    //
            println!("error writing json, {}", e);	    //
            return;					    //
        }						    //
    };							    //   File Writing Portion,
    match serde_json::to_writer(ofile, &result) {	    //   creates account balance.json and 
        Ok(res) => res, 				    //   uses to_writer to write it out 
        Err(e) => {					    //   as a nice json format
            println!("error writing json, {}", e);	    //
            return;					    //
        }						    //
    };							    //
    //------------------------------------------------------//
    println!(" | write successful | ");
}