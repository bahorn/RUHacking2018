use serde_json;
use sodiumoxide::crypto::box_;

/* How we describe messages sent over the wire */

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    AnnouncePeer,
    HelloPeer
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PeerItem {
    public_key: String,
    host: String,
    port: u16
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnnouncePeerItem {
    peers: Vec<PeerItem>
}

impl AnnouncePeerItem {
    pub fn to_announce_peer(value: serde_json::Value) -> AnnouncePeerItem {
        serde_json::from_value(value).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageFormat {
    pub message_type: MessageType,
    pub data: serde_json::Value
}

impl MessageFormat {
    pub fn parse(self) {
        match &self.message_type {
            AnnouncePeer => {
                let details: AnnouncePeerItem;
                details = AnnouncePeerItem::to_announce_peer(self.data);
                println!("Peer Announcement! {:?}", details);
            },
            HelloPeer => {
                println!("Greetings!");
            }
        }
    }
}
