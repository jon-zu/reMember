use bytes::BytesMut;
use shroom_pkt::{pkt::{EncodeMessage, Message}, Packet};

#[derive(Debug)]
pub struct EncodeBuf(BytesMut);

impl Default for EncodeBuf {
    fn default() -> Self {
        Self::new()
    }
}

impl EncodeBuf {
    #[must_use]
    pub fn new() -> Self {
        Self(BytesMut::with_capacity(2048))
    }

    pub fn encode_onto(&mut self, data: impl EncodeMessage) -> Result<Message, shroom_pkt::Error> {
        self.0.reserve(4096);
        data.encode_message(&mut self.0)?;
        Ok(Packet::from(self.0.split().freeze()).try_into().unwrap())
    }
}
