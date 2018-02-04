use nodes::{OurNodeInfo, NodeInfo, KnownNodes};
use protocol::{MessageFormat};
use config::Config;

use std::io;
use chrono::Utc;
use std::net::{SocketAddr};
use tokio_core::net::UdpSocket;
use tokio_core::reactor::{Core, Handle};
use futures::{Future, Poll};
use sodiumoxide::crypto::box_;
use std::collections::{HashMap, VecDeque};
use futures::Async::Ready;

#[derive(Serialize, Deserialize, Debug)]
pub struct CachedKeys {
    cached_keys: HashMap<SocketAddr, box_::PublicKey>
}

impl CachedKeys {
    pub fn new() -> CachedKeys {
        CachedKeys {cached_keys: HashMap::new()}
    }
    pub fn cache_key(&mut self, host: SocketAddr, key: box_::PublicKey) {
        &self.cached_keys.insert(host, key);
    }
}

/* Used to express what we need to do on the network */
#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkAction {
    message: MessageFormat,
    to: box_::PublicKey
}

/* Implement the main network state. This is meant to be stuff we can serialize.
 * */
#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkCore {
    our_node: OurNodeInfo,
    known_nodes: KnownNodes,
    cached_keys: CachedKeys,
}

impl NetworkCore {
    pub fn new() -> NetworkCore {
        NetworkCore {
            cached_keys: CachedKeys::new(),
            our_node: OurNodeInfo::new(),
            known_nodes: KnownNodes::new(),
        }
    }
}

pub struct NetworkStack {
    core: NetworkCore,
    socket: UdpSocket,
    interface: SocketAddr,
    message_queue: VecDeque<NetworkAction>,
    to_send: Option<(usize, SocketAddr)>,
    buf: Vec<u8>
}

impl NetworkStack {

    // Setup our network.
    fn setup(handle: Handle, port: u16, node_config: NetworkCore) -> NetworkStack {
        let interface = SocketAddr::from(([127, 0, 0, 1], port));
        let mut network_stack = NetworkStack {
            core: node_config,
            message_queue: VecDeque::new(),
            socket: UdpSocket::bind(&interface, &handle)
                .expect("couldn't bind to address"),
            interface: interface,
            buf: vec![0; 2048], // Make it greater than the MTU.
            to_send: None
        };
        network_stack
    }

    pub fn new(handle: Handle, port: u16, node_config: NetworkCore) -> NetworkStack {
        let mut network_stack = NetworkStack::setup(handle, port, node_config);
        // We should attempt to get a list of peers from this node.
        network_stack
    }

    /* Talk to a known node */
    pub fn send_message(&self, public_key: box_::PublicKey,
                        message: MessageFormat) -> Result<(), &'static str> {
        let node = self.core.known_nodes.get(public_key);
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

        Ok(())
    }

    pub fn read_message(&self, buf: &[u8]) -> Result<MessageFormat, ()> {
        let message = self.core.our_node.decrypt(&buf);
        match message {
            Ok(message_format) => {
                return Ok(message_format);
            },
            Err(_) => {
                return Err(());
            }
        }
    }

    pub fn add_node(&mut self, node: NodeInfo) {
        self.core.known_nodes.add_node(node);
    }
    //
    pub fn add_to_queue(&mut self, action: NetworkAction) {
    }
}


/* stolen from the tokio-core udp example */
impl Future for NetworkStack {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<(), io::Error> {
        loop {
            if let Some((size, peer)) = self.to_send {
                let amt = try_nb!(self.socket.send_to(&self.buf[..size], &peer));
                match self.read_message(&self.buf[..size]) {
                    Ok(message) => {
                        // Now preform what we need to do.
                        message.parse();
                    },
                    Err(_) => {
                        println!("[!] Couldn't parse it.");
                    }
                }
                self.to_send = None;
            }
            self.to_send = Some(try_nb!(self.socket.recv_from(&mut self.buf)));
        }
        /*loop {

            //self.to_send = Some(try_nb!(self.socket.recv_from(&mut self.buf)));

            /* Check if we can read anything */
            match self.socket.poll_read() {
                Ready(x) => {
                    // Attempt to read.
                    println!("hit");
                    self.to_send = Some(try_nb!(self.socket.recv_from(&mut self.buf)));
                },
                NotReady => {
                    // Nothing to do.
                }
            }
        }*/
    }
}

