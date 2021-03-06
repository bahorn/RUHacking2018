use protocol::MessageFormat;

use std::net::SocketAddr;
use std::collections::HashMap;
use sodiumoxide::crypto::{box_,sealedbox};
use serde::{Deserialize, Serialize};
use serde_json;//::{Value, Error};
use chrono::prelude::*;

/* Us */
#[derive(Serialize, Deserialize, Debug)]
pub struct OurNodeInfo {
    public: box_::PublicKey,
    private: box_::SecretKey,
}

impl OurNodeInfo {
    pub fn new() -> OurNodeInfo {
        // Generate our Key pair.
        let (public, private) = box_::gen_keypair();
        OurNodeInfo {public: public, private: private}
    }
    // Decrypt the messages sent to our node.
    pub fn decrypt(&self, message: &[u8]) -> Result<MessageFormat, ()> {
        let decrypted = sealedbox::open(message, &self.public, &self.private);
        match decrypted {
            Ok(decrypted_message) => {
                let x: MessageFormat = serde_json::from_str(&String::from_utf8(decrypted_message).unwrap()).unwrap();
                Ok(x)
            },
            Err(_) => return Err(())
        }
    }
}

/* Used to describe what nodes we are talking to */
#[derive(Serialize, Deserialize, Debug)]
pub struct NodeInfo {
    pub public: box_::PublicKey,
    pub address: SocketAddr,
    pub last_contact: DateTime<Utc>
}

impl NodeInfo {
    pub fn new(public_key: box_::PublicKey, address: SocketAddr,
               last_contact: DateTime<Utc>) -> NodeInfo {
        NodeInfo {public: public_key, address: address,
            last_contact: last_contact}
    }
    // Send an encrypted message to this node.
    pub fn encrypt(&self, message: MessageFormat) -> Vec<u8> {
        let buf: Vec<u8> = serde_json::to_string(&message).unwrap().into_bytes();
        sealedbox::seal(&buf, &self.public)
    }
}

/* List of every node we know of */
#[derive(Serialize, Deserialize, Debug)]
pub struct KnownNodes {
    pub nodes: HashMap<box_::PublicKey, NodeInfo>
}

impl KnownNodes {
    pub fn new() -> KnownNodes{
        KnownNodes {nodes: HashMap::new()}
    }
    
    pub fn add_node(&mut self, node_info: NodeInfo) {
        self.nodes.insert(node_info.public, node_info);
    }

    pub fn del_node(&mut self, public_key: box_::PublicKey) {
        self.nodes.remove(&public_key);
    }

    pub fn get(&self, public_key: box_::PublicKey) -> Option<&NodeInfo> { 
        self.nodes.get(&public_key)
    }
    pub fn get_length(&self) -> usize {
        self.nodes.len()
    }
    pub fn get_random_node(&self) -> Result<&NodeInfo, ()> {
        // Get the first element.
        for value in self.nodes.values() {
            return Ok(value);
        }
        return Err(());
    }
}

