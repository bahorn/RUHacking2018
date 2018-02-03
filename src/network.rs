use nodes::{OurNodeInfo, NodeInfo, KnownNodes};
use protocol::{MessageFormat};

use std::io;
use std::net::{SocketAddr};
use tokio_core::net::UdpSocket;
use tokio_core::reactor::{Core, Handle};
use futures::{Future, Poll};
use sodiumoxide::crypto::box_;

/* Implement the main network state */
pub struct NetworkStack {
    our_node: OurNodeInfo,
    known_nodes: KnownNodes,
    socket: UdpSocket,
    interface: SocketAddr,
    to_send: Option<(usize, SocketAddr)>,
    buf: Vec<u8>
}

impl NetworkStack {
    // Setup our network.
    fn setup(handle: Handle) -> NetworkStack {
        let interface = SocketAddr::from(([0,0,0,0],3000));
        let mut network_stack = NetworkStack {
            our_node: OurNodeInfo::new(),
            known_nodes: KnownNodes::new(),
            socket: UdpSocket::bind(&interface, &handle)
                .expect("couldn't bind to address"),
            interface: interface,
            buf: vec![0; 1024],
            to_send: None
        };
        network_stack
    }
    pub fn new(bootstrap_node: NodeInfo, handle: Handle) -> NetworkStack {
        let mut network_stack = NetworkStack::setup(handle);
        // Install a bootstrap node.
        network_stack.known_nodes.add_node(bootstrap_node);
        // We should attempt to get a list of peers from this node.
        network_stack
    }
    /* In case we don't want to provide a bootstrap node */
    pub fn new_clean(handle: Handle) -> NetworkStack {
        NetworkStack::setup(handle)
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
        //self.socket.connect(user_node.address)
        //    .expect("connect function failed");
        //self.socket.send(&encrypted_message)
        //    .expect("couldn't send message");
        Ok(())
    }
    /* Read a message from the UDP socket. */
    /*
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
    }*/
    pub fn read_real_message(&self, buf: &[u8]) -> Result<MessageFormat, ()> {
        let message = self.our_node.decrypt(&buf);
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
        self.known_nodes.add_node(node);
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
                match self.read_real_message(&self.buf[..size]) {
                    Ok(message) => {
                        // Now preform what we need to do.
                        println!("{:?}",message);
                    },
                    Err(_) => {
                        println!("[!] Couldn't parse it.");
                    }
                }
                self.to_send = None;
            }

            // If we're here then `to_send` is `None`, so we take a look for the
            // next message we're going to echo back.
            self.to_send = Some(try_nb!(self.socket.recv_from(&mut self.buf)));
        }
    }
}
