//extern crate krakenrs;
use std::fs::File;
extern crate serde_json;
extern crate serde_derive;
use wasm_bindgen::prelude::*;
use krakenrs;

#[wasm_bindgen]
pub fn add_two_ints(a: u32, b: u32) -> u32 {
    a + b
}

#[wasm_bindgen]
pub fn fib(n: i32) -> i32 {
    if n == 0 || n == 1 {
        return n;
    }

    fib(n - 1) + fib(n - 2)
}

#[wasm_bindgen]
pub fn ticker(s: String) -> String {
    
    let coin: String = s.to_string();
    
    // ------------------------------------- Kraken Initialization ----------------------------------- \\
    let mut krc = krakenrs::KrakenRestConfig::default();
    let api = krakenrs::KrakenRestAPI::try_from(krc).expect("could not create kraken api"); /* API creation */
    let result = api.ticker(vec![coin]).expect("api call failed"); /* connects to external kraken wallet */
    // ------------------------------------- Kraken Initialization ----------------------------------- \\
    

    // --------------------------------------- File Writing Out -------------------------------------- \\
    let ofile = match File::create("ticker.json") {
        Ok(res) => res,
        Err(e) => {
            println!("error writing json, {}", e);
            return result;
        }
    };
    match serde_json::to_writer(ofile, &result) {
        Ok(res) => res, 
        Err(e) => {
            println!("error writing json, {}", e);
            return result;
        }
    };
    // --------------------------------------- File Writing Out --------------------------------------- \\
    return result;
}
