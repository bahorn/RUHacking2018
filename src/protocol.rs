

/* How we describe messages sent over the wire */
#[derive(Serialize, Deserialize, Debug)]
pub struct MessageFormat {
    pub message_type: i32,
    pub data: Vec<u8>
}

