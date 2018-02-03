mod network;
mod nodes;
mod config;


#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate rmp_serde as rmps;
extern crate dotenv;
extern crate sodiumoxide;
extern crate chrono;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use sodiumoxide::crypto::box_;
use dotenv::dotenv;
use std::process::exit;
use chrono::prelude::*;
use std::{env, thread, time};
use std::sync::{Mutex, Arc};


fn main_loop(server_client: config::Config)
{
    let stack = network::NetworkStack::new_clean();
    /*
    let stack: network::NetworkStack = network::NetworkStack::new_clean();
    let (mut public, mut private) = box_::gen_keypair();
    let now = Utc::now();
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let their_node = nodes::NodeInfo::new(public, socket, now);  
    */

    let handle = thread::spawn(move || {
        loop {
            println!("{:?}",stack.read_message());
        }
    });
    /* A loop. */
    handle.join();
}

fn main()
{
    let mut node_config: config::Config;
    let mut config_path: String;
    let mut make_configs = false;
    
    println!("[!] p2p backend");
    println!("[!] Processing Configuration");
    dotenv().ok();
    // Reading the config.
    let config_file = env::var("RU_CONFIG_FILE");
    match config_file {
        Ok(config_file_path) => {
            config_path = config_file_path;
        },
        Err(_) => {
            config_path = "./config.json".to_string();
        }
    }
    // Quick hack to parse generate configs
    for argument in env::args() {
        if argument == "-c" {
            make_configs = true;
        }
    }

    if make_configs == true {
        node_config = config::Config::read_config_file(&config_path);
        node_config.save(&config_path);
        exit(0);
    } else {
        node_config = config::Config::read_config_file(&config_path);
        main_loop(node_config);
    }
}
