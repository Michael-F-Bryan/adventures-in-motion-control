#![no_std]

use aimc_hal::System;
use anpp::{errors::DecodeError, Decoder, Packet};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Communications {
    decoder: Decoder,
}

impl Communications {
    pub fn new() -> Communications {
        Communications {
            decoder: Decoder::new(),
        }
    }
}

impl<I, T, M> System<I, Outputs<T, M>> for Communications
where
    I: Rx,
    T: Tx,
    M: MessageHandler,
{
    fn poll(&mut self, inputs: &I, outputs: &mut Outputs<T, M>) {
        // A: how do we want to handle overflows?
        let _ = self.decoder.push_data(inputs.receive());

        loop {
            match self.decoder.decode() {
                Ok(request) => {
                    let response = outputs
                        .message_handler
                        .handle_message(&request)
                        .expect("Unhandled message");
                    outputs.tx.send(response);
                },
                Err(DecodeError::InvalidCRC) => {
                    unimplemented!("C: How do we handle corrupted packets?")
                },
                Err(DecodeError::RequiresMoreData) => break,
            }
        }
    }
}

/// The receiving end of a *Serial Connection*.
pub trait Rx {
    /// Get all bytes received by the simulator since the last tick.
    ///
    /// # Note to Implementors
    ///
    /// To prevent reading data twice, this buffer should be cleared after every
    /// tick.
    fn receive(&self) -> &[u8];
}

#[derive(Debug, Clone, PartialEq)]
pub struct Outputs<T, M> {
    message_handler: M,
    tx: T,
}

impl<T, M> Outputs<T, M> {
    pub fn new(tx: T, message_handler: M) -> Outputs<T, M> {
        Outputs {
            tx,
            message_handler,
        }
    }
}

/// The transmitting end of a *Serial Connection*.
pub trait Tx {
    /// Queue a [`Packet`] to be sent to the frontend.
    ///
    /// There is no guarantee that the data will all be sent. This may happen if
    /// the receiver isn't listening or they aren't able to receive at this
    /// time.
    fn send(&mut self, message: Packet);
}

impl<'a, T: Tx> Tx for &'a mut T {
    fn send(&mut self, message: Packet) { (*self).send(message); }
}

pub trait MessageHandler {
    fn handle_message(&mut self, msg: &Packet) -> Result<Packet, CommsError>;
}

impl<'a, M: MessageHandler> MessageHandler for &'a mut M {
    fn handle_message(&mut self, msg: &Packet) -> Result<Packet, CommsError> {
        (*self).handle_message(msg)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CommsError {
    /// The [`MessageHandler`] doesn't know how to handle the message.
    UnknownMessageType,
}
