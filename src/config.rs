// Used to parse Config JSON files.
use nodes::{OurNodeInfo, NodeInfo, KnownNodes};

use serde_json;

use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    known_nodes: KnownNodes,
    our_node: OurNodeInfo
}

impl Config {
    // Read the config from a JSON file.
    pub fn read_config_file(filename: &String) -> Config {
        let mut temp_buffer = String::new();
        let mut file = File::open(filename).expect("Unable to open.");

        file.read_to_string(&mut temp_buffer).expect("Unable to read");
        let loaded_config: Config = serde_json::from_str(&temp_buffer)
            .expect("Unable to parse.");
        loaded_config
    }
    // Create an empty config.
    pub fn new() -> Config {
        Config {
            known_nodes: KnownNodes::new(),
            our_node: OurNodeInfo::new()
        }
    }
    // Save the config to a file.
    pub fn save(&self, filename: &String) {
       let mut file = File::create(filename).expect("Unable to open."); 
       let config_json = serde_json::to_string(self).expect("Work");
       file.write_fmt(format_args!("{}",config_json));
    }
}
