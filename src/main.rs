mod network;
mod nodes;
mod config;
mod protocol;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate rmp_serde as rmps;
extern crate dotenv;
extern crate sodiumoxide;
extern crate chrono;
extern crate futures;
#[macro_use]
extern crate tokio_core;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use sodiumoxide::crypto::box_;
use dotenv::dotenv;
use std::process::exit;
use chrono::prelude::*;
use std::{env, thread, time};
use std::sync::Mutex;
use futures::{Future, Poll};
use tokio_core::net::UdpSocket;
use tokio_core::reactor::Core;

fn control_loop(server_client: config::Config)
{
    /*
    let stack: network::NetworkStack = network::NetworkStack::new_clean();
    let (mut public, mut private) = box_::gen_keypair();
    let now = Utc::now();
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let their_node = nodes::NodeInfo::new(public, socket, now);  
    */
    //let mut handles = vec![];
    //let stack = network::NetworkStack::new_clean();
    /*
    for x in 0..1 {
        let handle = thread::spawn(move || {
            if x == 0 {
                loop {
                    println!("{:?}",stack.read_message());
                }
            } else if x == 1 {
                loop {
                    println!("{:?}",stack.read_message());
                }
            }
        });
        handles.push(handle);
    }
    /* A loop. */
    for h in handles {
        h.join().unwrap();
    }*/
    let mut l = Core::new().unwrap();
    let handle = l.handle();
    l.run(
        network::NetworkStack::new_clean(handle)
    ).unwrap();
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
        control_loop(node_config);
    }
}
