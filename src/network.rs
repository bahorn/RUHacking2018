use std::time::Instant;
use std::net::{SocketAddr, UdpSocket};
use std::collections::HashMap;
use sodiumoxide::crypto::{box_,sealedbox};
use rmps;

/* How we describe messages sent over the wire */
#[derive(Serialize, Deserialize, Debug)]
pub struct MessageFormat {
    pub message_type: i32,
    pub data: Vec<u8>
}

/* Us */
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
                match rmps::from_slice(&decrypted_message) {
                    Ok(deserialized_message_pack) => {
                        return Ok(deserialized_message_pack);
                    },
                    Err(_) => return Err(())
                }
            },
            Err(_) => return Err(())
        }
    }
}

/* Used to describe what nodes we are talking to */
pub struct NodeInfo {
    public: box_::PublicKey,
    address: SocketAddr,
    last_contact: Instant
}

impl NodeInfo {
    pub fn new(public_key: box_::PublicKey, address: SocketAddr,
               last_contact: Instant) -> NodeInfo {
        NodeInfo {public: public_key, address: address,
            last_contact: last_contact}
    }
    // Send an encrypted message to this node.
    fn encrypt(&self, message: MessageFormat) -> Vec<u8> {
        let buf = rmps::to_vec(&message).unwrap();
        sealedbox::seal(&buf, &self.public)
    }
}

/* List of every node we know of */
pub struct KnownNodes {
    nodes: HashMap<box_::PublicKey, NodeInfo>
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
}

/* Implement the main network state */
pub struct NetworkStack {
    our_node: OurNodeInfo,
    known_nodes: KnownNodes,
    socket: UdpSocket
}

impl NetworkStack {
    // Setup our network.
    pub fn new(bootstrap_node: NodeInfo) -> NetworkStack {
        let mut network_stack = NetworkStack {
            our_node: OurNodeInfo::new(),
            known_nodes: KnownNodes::new(),
            socket: UdpSocket::bind("0.0.0.0:1337")
                .expect("couldn't bind to address")
        };
        // Install a bootstrap node.
        network_stack.known_nodes.add_node(bootstrap_node);
        // We should attempt to get a list of peers from this node.
        network_stack
    }
    /* In case we don't want to provide a bootstrap node */
    pub fn new_clean() -> NetworkStack {
        let network_stack = NetworkStack {
            our_node: OurNodeInfo::new(),
            known_nodes: KnownNodes::new(),
            socket: UdpSocket::bind("0.0.0.0:1337")
                .expect("couldn't bind to address")
        };
        network_stack
    }
    /* Talk to a known node */
    pub fn send_message(&self, public_key: box_::PublicKey,
                        message: MessageFormat) -> Result<(), &'static str> {
        let node = self.known_nodes.get(public_key);
        let user_node: &NodeInfo;
        match node {
            Some(found_node) => {
                user_node = found_node;
            },
            None => {
                return Err("Unable to find Node");
            }
        }
        let encrypted_message = user_node.encrypt(message);
        // Now attempt to send this out.
        self.socket.connect(user_node.address)
            .expect("connect function failed");
        self.socket.send(&encrypted_message)
            .expect("couldn't send message");
        Ok(())
    }
    /* Read a message from the UDP socket. */
    pub fn read_message(&self) -> Result<(MessageFormat, SocketAddr), ()> {
        // search by SocketAddr to find public key.
        let mut buf = Vec::new();
        let socket_address: SocketAddr;
        match self.socket.recv_from(&mut buf) {
            Ok((size, socket_addr)) => {
                socket_address = socket_addr;
                println!("Got message!");
            },
            Err(_) => return Err(())
        }
        let message = self.our_node.decrypt(&buf);
        match message {
            Ok(message_format) => {
                return Ok((message_format, socket_address));
            },
            Err(_) => {
                return Err(());
            }
        }
    }

    pub fn add_node(&mut self, node: NodeInfo) {
        self.known_nodes.add_node(node);
    }
}
