/* How we describe messages sent over the wire */

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    AnnouncePeer
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageFormat {
    pub message_type: MessageType,
    pub data: Vec<u8>
}

impl MessageFormat {
    pub fn parse(&self) {
        match &self.message_type {
            AnnouncePeer => {
                println!("Peer Announcement!");
            }
        }
    }
}
