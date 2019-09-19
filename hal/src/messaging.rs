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

    pub const fn new() -> Self { Ack {} }
}

impl From<Ack> for Packet {
    fn from(_: Ack) -> Packet { Packet::new(Ack::ID) }
}

/// The message was not acknowledged.
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
pub struct Nack {}

impl Nack {
    /// The ID used when encoded as a [`Packet`].
    pub const ID: u8 = 1;

    pub const fn new() -> Self { Nack {} }
}

impl From<Nack> for Packet {
    fn from(_: Nack) -> Packet { Packet::new(Nack::ID) }
}
