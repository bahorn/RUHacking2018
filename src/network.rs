use nodes::{OurNodeInfo, NodeInfo, KnownNodes};

use std::net::{SocketAddr, UdpSocket};
use sodiumoxide::crypto::box_;

/* How we describe messages sent over the wire */
#[derive(Serialize, Deserialize, Debug)]
pub struct MessageFormat {
    pub message_type: i32,
    pub data: Vec<u8>
}

/* Implement the main network state */
pub struct NetworkStack {
    our_node: OurNodeInfo,
    known_nodes: KnownNodes,
    socket: UdpSocket,
    interface: SocketAddr
}

impl NetworkStack {
    // Setup our network.
    fn setup() -> NetworkStack {
        let interface = SocketAddr::from(([0,0,0,0],3000));
        let mut network_stack = NetworkStack {
            our_node: OurNodeInfo::new(),
            known_nodes: KnownNodes::new(),
            socket: UdpSocket::bind(interface)
                .expect("couldn't bind to address"),
            interface: interface
        };
        network_stack
    }
    pub fn new(bootstrap_node: NodeInfo) -> NetworkStack {
        let mut network_stack = NetworkStack::setup();
        // Install a bootstrap node.
        network_stack.known_nodes.add_node(bootstrap_node);
        // We should attempt to get a list of peers from this node.
        network_stack
    }
    /* In case we don't want to provide a bootstrap node */
    pub fn new_clean() -> NetworkStack {
        NetworkStack::setup()
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
                println!("[!] Got message!");
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
