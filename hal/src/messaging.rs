use anpp::Packet;
use scroll_derive::*;

/// Something which can handle a request and generate a response.
pub trait Handler<M> {
    /// The type of response.
    type Response;

    fn handle(&mut self, msg: M) -> Self::Response;
}

/// Acknowledge a request without returning any extra information.
#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Pread,
    Pwrite,
    IOread,
    IOwrite,
    SizeWith,
)]
pub struct Ack {}

impl Ack {
    /// The ID used when encoded as a [`Packet`].
    pub const ID: u8 = 0;

    pub fn new() -> Ack { Ack::default() }
}

impl From<Ack> for Packet {
    fn from(_: Ack) -> Packet { Packet::new(Ack::ID) }
}
