mod network;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate rmp_serde as rmps;
extern crate dotenv;
extern crate sodiumoxide;
extern crate futures;
extern crate hyper;
extern crate tokio_core;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use sodiumoxide::crypto::box_;
use std::time::Instant;
use dotenv::dotenv;
use std::env;
//use std::thread;

fn main() {
    let stack: network::NetworkStack;
    //let data_raw = vec![0,1,2,3];
    //let message = network::MessageFormat {message_type: 3, data: data_raw};
    let now = Instant::now();
    let (public, _) = box_::gen_keypair();
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let their_node = network::NodeInfo::new(public, socket, now);  
    
    dotenv().ok();
    /* Setup and connect */
    let bootstrap_node = env::var("BOOTSTRAP_NODE");
    match bootstrap_node {
        Ok(node) => {
            println!("{:?}",node);
            stack = network::NetworkStack::new(their_node);
        },
        Err(_) => {
            // Basically, just start listening.
            stack = network::NetworkStack::new_clean();
        }
    }
    /* Setup workers */

    /* Main loop */
    loop {
        println!("{:?}",stack.read_message());
    }
}
