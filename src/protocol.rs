use serde_json;

/* How we describe messages sent over the wire */

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    AnnouncePeer,
    HelloPeer
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageFormat {
    pub message_type: MessageType,
    pub data: serde_json::Value
}

impl MessageFormat {
    pub fn parse(&self) {
        match &self.message_type {
            AnnouncePeer => {
                println!("Peer Announcement!");
            },
            HelloPeer => {
                println!("Greetings!");
            }
        }
    }
}
