use serde_json::json;
use serde_json;
use std::path::Path;
use std::fs::File;
extern crate serde_derive;
use std::ffi::{CString,CStr};
use core::convert::TryFrom;
use core::fmt::Debug;
use krakenrs::{BsType, MarketOrder, LimitOrder, OrderFlag};
use std::os::raw::c_char;
use structopt::StructOpt;
use displaydoc::Display;
use std::collections::BTreeSet;

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

// struct AccountBal {
//     coin1: String,
//     coin2: String,
// }

#[no_mangle]
pub extern "C" fn account_balance(credens: *const c_char) -> CString {
    let c_str = unsafe {
        assert!(!credens.is_null());

        CStr::from_ptr(credens)
    };
    let creden_C = c_str.to_str().unwrap();
    let creds:String = creden_C.to_string();
    let creds_path = Path::new(&creds);
    if !creds_path.is_file() {
        println!("File: {} does not exist", creds);
    }
    println!("Found your path at: {}", creds);

    let mut krc = krakenrs::KrakenRestConfig::default();
    let credentials =  krakenrs::KrakenCredentials::load_json_file(creds_path).unwrap();                                              //
    krc.creds = credentials;

    let api = krakenrs::KrakenRestAPI::try_from(krc).expect("could not create kraken api");
    let result = api.get_account_balance().expect("api call failed"); 
    //println!("{:?}",result);
    //let sorted_result = result.intoiter().collect::<BTreeMap<, _>>();
    let jso = json!(result);
    //println!("{:?}", jso.to_string());
    let retu = CString::new(jso.to_string()).unwrap();
    return retu;
}

#[no_mangle]
pub extern "C" fn get_orders(credens: *const c_char) -> CString {
    
    //---------------------------------------------//
    let c_str = unsafe {			               //
        assert!(!credens.is_null());	       	   //
        CStr::from_ptr(credens)		               //
    };						                       //
    let creden_c = c_str.to_str().unwrap();	       //	turns cstring input into Rust path &
    let creds:String = creden_c.to_string();       // 	Checks Creds File 
    let creds_path = Path::new(&creds);            //
    if !creds_path.is_file() {			           //      
        println!("File: {} does not exist", creds);//
        return CString::new("FAIL").unwrap();	   //
    }						                       //
    //---------------------------------------------//


    //--------------------------------------------------------------------------------//
    let mut krc = krakenrs::KrakenRestConfig::default();                              //
    let credentials = match krakenrs::KrakenCredentials::load_json_file(creds_path) { //  
        Ok(res) => res,							                                      //
        Err(e) => {								                                      //
            println!("error loading creds, {}", e);				                      //   Creates KrakenRestAPI object 
            return CString::new("FAIL").unwrap();				                      //   and loads the creds into it
        }									                                          //
    };										                                          //
    krc.creds = credentials; 							                              //
    //--------------------------------------------------------------------------------//

    //-------------------------------------------------------------------------------------//
    let api = krakenrs::KrakenRestAPI::try_from(krc).expect("could not create kraken api");//
    let result = api.get_open_orders(None).expect("api call failed"); 		           	   //	Creates api object, captures output to result,
    let jso = json!(result).to_string();					                          	   //	turns it into json and then returns it out as 
    return CString::new(jso).unwrap();							                           //	CString
    //-------------------------------------------------------------------------------------//
}

#[no_mangle]
pub extern "C" fn ticker(coin: *const c_char) -> CString {
    //----------------------------------//
    let c_str = unsafe {        //
        assert!(!coin.is_null());    //
        CStr::from_ptr(coin)        //    turns CString input into Rust String
    };                    //    for Kraken ticker call.
    let coiin = c_str.to_str().unwrap();//
    //----------------------------------//

    //-------------------------------------------------------------------------------------//
    let krc = krakenrs::KrakenRestConfig::default();                               //
    let api = krakenrs::KrakenRestAPI::try_from(krc).expect("could not create kraken api");//    creates kraken api object and makes ticker call
    let result = api.ticker(vec![coiin.to_string()]).expect("api call failed");       //    saving output and turning it into json to then 
    let jso = json!(result).to_string();                           //    return it as a CString
    
    return CString::new(jso).unwrap();                               //
    //-------------------------------------------------------------------------------------//
}

#[no_mangle]
pub extern "C" fn cancel_order(credens: *const c_char, idd: *const c_char) -> CString {

    //---------------------------------------------//
    let c_str = unsafe {                           //
        assert!(!credens.is_null());                  //
        CStr::from_ptr(credens)                       //
    };                                               //
    let creden_c = c_str.to_str().unwrap();           //    turns cstring input into Rust path &
    let creds:String = creden_c.to_string();       //     Checks Creds File 
    let creds_path = Path::new(&creds);            //
    if !creds_path.is_file() {                       //
        println!("File: {} does not exist", creds);//
        return CString::new("FAIL").unwrap();       //
    }                                               //
    //---------------------------------------------//

    //----------------------------------//
    let c_str = unsafe {                //
        assert!(!idd.is_null());        //
        CStr::from_ptr(idd)                //
    };                                    //
    let id = c_str.to_str().unwrap();   //
    //----------------------------------//

    //-------------------------------------------------------------------------------------//
    let krc = krakenrs::KrakenRestConfig::default();                                   //
    let api = krakenrs::KrakenRestAPI::try_from(krc).expect("could not create kraken api");//    creates kraken api object and makes ticker call
    let result = api.cancel_order(id.to_string()).expect("api call failed");               //    saving output and turning it into json to then 
    let jso = json!(result).to_string();                                                   //    return it as a CString
    return CString::new(jso).unwrap();                                                       //
    //-------------------------------------------------------------------------------------//
}

#[no_mangle]
pub extern "C" fn cancel_all_orders(credens: *const c_char) -> CString {

    //---------------------------------------------//
    let c_str = unsafe {                           //
        assert!(!credens.is_null());                  //
        CStr::from_ptr(credens)                       //
    };                                               //
    let creden_c = c_str.to_str().unwrap();           //    turns cstring input into Rust path &
    let creds:String = creden_c.to_string();       //     Checks Creds File 
    let creds_path = Path::new(&creds);            //
    if !creds_path.is_file() {                       //
        println!("File: {} does not exist", creds);//
        return CString::new("FAIL").unwrap();       //
    }                                               //
    //---------------------------------------------//

    //-------------------------------------------------------------------------------------//
    let krc = krakenrs::KrakenRestConfig::default();                                   //
    let api = krakenrs::KrakenRestAPI::try_from(krc).expect("could not create kraken api");//    creates kraken api object and makes ticker call
    let result = api.cancel_all_orders().expect("api call failed");               //    saving output and turning it into json to then 
    let jso = json!(result).to_string();                                                   //    return it as a CString
    return CString::new(jso).unwrap();                                                       //
    //-------------------------------------------------------------------------------------//
}

#[no_mangle]
pub extern "C" fn mk_buy(credens: *const c_char, voll: *const c_char, parr: *const c_char) -> CString {
    
    //---------------------------------------------//
    let c_str1 = unsafe {		           //
        assert!(!voll.is_null());	           //
        CStr::from_ptr(voll)		           //	turns CString input into Rust String
    };					           //	for volume selection.
    let vol = c_str1.to_str().unwrap().to_string();//
    //---------------------------------------------//
    
    //---------------------------------------------//
    let c_str2 = unsafe {		           //
        assert!(!parr.is_null());	           //
        CStr::from_ptr(parr)		           //	turns CString input into Rust String
    };					           //	for coin selection.
    let par = c_str2.to_str().unwrap().to_string();//
    //---------------------------------------------//
    
    //---------------------------------------------//
    let c_str = unsafe {			   //
        assert!(!credens.is_null());		   //
        CStr::from_ptr(credens)		   //
    };						   //
    let creden_c = c_str.to_str().unwrap();	   //	turns cstring input into Rust path &
    let creds:String = creden_c.to_string();       // 	Checks Creds File 
    let creds_path = Path::new(&creds);            //
    if !creds_path.is_file() {			   //      
        println!("File: {} does not exist", creds);//
        return CString::new("FAIL").unwrap();	   //
    }						   //
    //---------------------------------------------//
    
    //------------------------------------------------------------------------//
    let come_on = Command::MarketBuy {volume: vol.clone(), pair: par.clone()};//
    let config = KrakConfig{					      //
    	command: come_on,						      //  This creates an order to then pass to add_market_order
    	creds: creds.clone(),						      //
    	validate: false,						      //
    };									      //
    //------------------------------------------------------------------------//
	
    //--------------------------------------------------------------------------------//
    let mut krc = krakenrs::KrakenRestConfig::default();                              //
    let credentials = match krakenrs::KrakenCredentials::load_json_file(creds_path) { //  
        Ok(res) => res,							      //
        Err(e) => {								      //
            println!("error loading creds, {}", e);				      //   Creates KrakenRestAPI object 
            return CString::new("FAIL").unwrap();				      //   and loads the creds into it
        }									      //
    };										      //
    krc.creds = credentials; 							      //
    //--------------------------------------------------------------------------------//

    
    //-------------------------------------------------------------------------------------//
    let api = krakenrs::KrakenRestAPI::try_from(krc).expect("could not create kraken api");// 
    let result = api.add_market_order(MarketOrder {					   //
    						bs_type: BsType::Buy,			   //
                        			volume: vol,				   //
                        			pair: par,				   //
                        			oflags: Default::default(),		   //	calls add_market_order and adds our order that we created earlier
                    				},					   //	turns result into a json and then  
                    			None,						   //	returns it as a CString
                    			config.validate,				   //
    					).expect(&"api call failed");			   //
    let jso = json!(result).to_string();						   //	
    return CString::new(jso).unwrap();							   //
    //-------------------------------------------------------------------------------------//
}

#[no_mangle]
pub extern "C" fn mk_sell(credens: *const c_char, voll: *const c_char, parr: *const c_char) -> CString {
    
    //---------------------------------------------//
    let c_str1 = unsafe {		           //
        assert!(!voll.is_null());	           //
        CStr::from_ptr(voll)		           //	turns CString input into Rust String
    };					           //	for volume selection.
    let vol = c_str1.to_str().unwrap().to_string();//
    //---------------------------------------------//
    
    //---------------------------------------------//
    let c_str2 = unsafe {		           //
        assert!(!parr.is_null());	           //
        CStr::from_ptr(parr)		           //	turns CString input into Rust String
    };					           //	for coin selection.
    let par = c_str2.to_str().unwrap().to_string();//
    //---------------------------------------------//
    
    //---------------------------------------------//
    let c_str = unsafe {			   //
        assert!(!credens.is_null());		   //
        CStr::from_ptr(credens)		   //
    };						   //
    let creden_c = c_str.to_str().unwrap();	   //	turns cstring input into Rust path &
    let creds:String = creden_c.to_string();       // 	Checks Creds File 
    let creds_path = Path::new(&creds);            //
    if !creds_path.is_file() {			   //      
        println!("File: {} does not exist", creds);//
        return CString::new("FAIL").unwrap();	   //
    }						   //
    //---------------------------------------------//
    
    //------------------------------------------------------------------------//
    let come_on = Command::MarketSell {volume: vol.clone(), pair: par.clone()};//
    let mut config = KrakConfig{					      //
    	command: come_on,						      //  This creates an order to then pass to add_market_order
    	creds: creds.clone(),						      //
    	validate: false,						      //
    };									      //
    //------------------------------------------------------------------------//
	
    //--------------------------------------------------------------------------------//
    let mut krc = krakenrs::KrakenRestConfig::default();                              //
    let credentials = match krakenrs::KrakenCredentials::load_json_file(creds_path) { //  
        Ok(res) => res,							      //
        Err(e) => {								      //
            println!("error loading creds, {}", e);				      //   Creates KrakenRestAPI object 
            return CString::new("FAIL").unwrap();				      //   and loads the creds into it
        }									      //
    };										      //
    krc.creds = credentials; 							      //
    //--------------------------------------------------------------------------------//

    
    //-------------------------------------------------------------------------------------//
    let api = krakenrs::KrakenRestAPI::try_from(krc).expect("could not create kraken api");// 
    let result = api.add_market_order(MarketOrder {					   //
    						bs_type: BsType::Sell,			   //
                        			volume: vol,				   //
                        			pair: par,				   //
                        			oflags: Default::default(),		   //	calls add_market_order and adds our order that we created earlier
                    				},					   //	turns result into a json and then  
                    			None,						   //	returns it as a CString
                    			config.validate,				   //
    					).expect(&"api call failed");			   //
    let jso = json!(result).to_string();						   //	
    return CString::new(jso).unwrap();							   //
    //-------------------------------------------------------------------------------------//
}

#[no_mangle]
pub extern "C" fn limit_buy(credens: *const c_char, voll: *const c_char, parr: *const c_char, pricee: *const c_char) -> CString {
    //---------------------------------------------//
    let c_str1 = unsafe {		                   //
        assert!(!voll.is_null());	               //
        CStr::from_ptr(voll)		               //	turns CString input into Rust String
    };					                           //	for volume selection.
    let vol = c_str1.to_str().unwrap().to_string();//
    //---------------------------------------------//
    
    //---------------------------------------------//
    let c_str2 = unsafe {		                   //
        assert!(!parr.is_null());	               //
        CStr::from_ptr(parr)		               //	turns CString input into Rust String
    };					                           //	for coin selection.
    let par = c_str2.to_str().unwrap().to_string();//
    //---------------------------------------------//

    //---------------------------------------------//
    let c_str3 = unsafe {		                   //
        assert!(!pricee.is_null());	               //
        CStr::from_ptr(pricee)		               //	turns CString input into Rust String
    };					                           //	for coin selection.
    let pryce = c_str3.to_str().unwrap().to_string();//
    //---------------------------------------------//
    
    //---------------------------------------------//
    let c_str = unsafe {			               //
        assert!(!credens.is_null());	    	   //
        CStr::from_ptr(credens)		               //
    };						                       //
    let creden_c = c_str.to_str().unwrap();	       //	turns cstring input into Rust path &
    let creds:String = creden_c.to_string();       // 	Checks Creds File 
    let creds_path = Path::new(&creds);            //
    if !creds_path.is_file() {			           //      
        println!("File: {} does not exist", creds);//
        return CString::new("FAIL").unwrap();	   //
    }						                       //
    //---------------------------------------------//
    
    //------------------------------------------------------------------------//
    let mut oflags = BTreeSet::new();
    oflags.insert(OrderFlag::Post);
    let come_on = Command::LimitBuy {volume: vol.clone(), pair: par.clone(), price: pryce.clone()};//
    let mut config = KrakConfig{					      //
    	command: come_on,						      //  This creates an order to then pass to add_market_order
    	creds: creds.clone(),						      //
    	validate: false,						      //
    };									      //
    //------------------------------------------------------------------------//
	
    //--------------------------------------------------------------------------------//
    let mut krc = krakenrs::KrakenRestConfig::default();                              //
    let credentials = match krakenrs::KrakenCredentials::load_json_file(creds_path) { //  
        Ok(res) => res,							      //
        Err(e) => {								      //
            println!("error loading creds, {}", e);				      //   Creates KrakenRestAPI object 
            return CString::new("FAIL").unwrap();				      //   and loads the creds into it
        }									      //
    };										      //
    krc.creds = credentials; 							      //
    //--------------------------------------------------------------------------------//

    
    //-------------------------------------------------------------------------------------//
    let api = krakenrs::KrakenRestAPI::try_from(krc).expect("could not create kraken api");// 
    let result = api.add_limit_order(LimitOrder {					   //
    						bs_type: BsType::Buy,			   //
                        			volume: vol,				   //
                        			pair: par,
                                    price: pryce,				   //
                        			oflags: Default::default(),		   //	calls add_market_order and adds our order that we created earlier
                    				},					   //	turns result into a json and then  
                    			None,						   //	returns it as a CString
                    			config.validate,				   //
    					).expect(&"api call failed");			   //
    let jso = json!(result).to_string();						   //	
    return CString::new(jso).unwrap();							   //
    //-------------------------------------------------------------------------------------//
}

#[no_mangle]
pub extern "C" fn limit_sell(credens: *const c_char, voll: *const c_char, parr: *const c_char, pricee: *const c_char) -> CString {
    //---------------------------------------------//
    let c_str1 = unsafe {		                   //
        assert!(!voll.is_null());	               //
        CStr::from_ptr(voll)		               //	turns CString input into Rust String
    };					                           //	for volume selection.
    let vol = c_str1.to_str().unwrap().to_string();//
    //---------------------------------------------//
    
    //---------------------------------------------//
    let c_str2 = unsafe {		                   //
        assert!(!parr.is_null());	               //
        CStr::from_ptr(parr)		               //	turns CString input into Rust String
    };					                           //	for coin selection.
    let par = c_str2.to_str().unwrap().to_string();//
    //---------------------------------------------//

    //---------------------------------------------//
    let c_str3 = unsafe {		                   //
        assert!(!pricee.is_null());	               //
        CStr::from_ptr(pricee)		               //	turns CString input into Rust String
    };					                           //	for coin selection.
    let pryce = c_str3.to_str().unwrap().to_string();//
    //---------------------------------------------//
    
    //---------------------------------------------//
    let c_str = unsafe {			               //
        assert!(!credens.is_null());	    	   //
        CStr::from_ptr(credens)		               //
    };						                       //
    let creden_c = c_str.to_str().unwrap();	       //	turns cstring input into Rust path &
    let creds:String = creden_c.to_string();       // 	Checks Creds File 
    let creds_path = Path::new(&creds);            //
    if !creds_path.is_file() {			           //      
        println!("File: {} does not exist", creds);//
        return CString::new("FAIL").unwrap();	   //
    }						                       //
    //---------------------------------------------//
    
    //------------------------------------------------------------------------//
    let mut oflags = BTreeSet::new();
    oflags.insert(OrderFlag::Post);
    let come_on = Command::LimitSell {volume: vol.clone(), pair: par.clone(), price: pryce.clone()};//
    let mut config = KrakConfig{					      //
    	command: come_on,						      //  This creates an order to then pass to add_market_order
    	creds: creds.clone(),						      //
    	validate: false,						      //
    };									      //
    //------------------------------------------------------------------------//
	
    //--------------------------------------------------------------------------------//
    let mut krc = krakenrs::KrakenRestConfig::default();                              //
    let credentials = match krakenrs::KrakenCredentials::load_json_file(creds_path) { //  
        Ok(res) => res,							      //
        Err(e) => {								      //
            println!("error loading creds, {}", e);				      //   Creates KrakenRestAPI object 
            return CString::new("FAIL").unwrap();				      //   and loads the creds into it
        }									      //
    };										      //
    krc.creds = credentials; 							      //
    //--------------------------------------------------------------------------------//

    
    //-------------------------------------------------------------------------------------//
    let api = krakenrs::KrakenRestAPI::try_from(krc).expect("could not create kraken api");// 
    let result = api.add_limit_order(LimitOrder {					   //
    						bs_type: BsType::Sell,			   //
                        			volume: vol,				   //
                        			pair: par,
                                    price: pryce,				   //
                        			oflags: Default::default(),		   //	calls add_market_order and adds our order that we created earlier
                    				},					   //	turns result into a json and then  
                    			None,						   //	returns it as a CString
                    			config.validate,				   //
    					).expect(&"api call failed");			   //
    let jso = json!(result).to_string();						   //	
    return CString::new(jso).unwrap();							   //
    //-------------------------------------------------------------------------------------//
}
