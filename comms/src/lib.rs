#![no_std]

use aimc_hal::System;
use anpp::{errors::DecodeError, Decoder, Packet};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Communications {
    decoder: Decoder,
}

impl Communications {
    pub fn new() -> Self {
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
        let received = inputs.receive();

        if self.decoder.push_data(received).is_err() {
            // we've run out of space in the decoder buffer, clear out leftovers
            // from previous runs and copy in as much new data as possible
            self.decoder.clear();
            let len = core::cmp::min(
                received.len(),
                self.decoder.remaining_capacity(),
            );
            let _ = self.decoder.push_data(&received[..len]);
        }

        loop {
            match self.decoder.decode() {
                Ok(request) => {
                    let response = outputs
                        .message_handler
                        .handle_message(&request)
                        .expect("Unhandled message");
                    outputs.send(&response);
                },
                Err(DecodeError::InvalidCRC) => {
                    outputs.message_handler.on_crc_error()
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
    pub fn new(tx: T, message_handler: M) -> Self {
        Outputs {
            tx,
            message_handler,
        }
    }
}

impl<T: Tx, M> Outputs<T, M> {
    fn send(&mut self, packet: &Packet) {
        let mut buffer = [0; Packet::MAX_PACKET_SIZE + 5];
        debug_assert!(buffer.len() >= packet.total_length());

        let bytes_written = packet
            .write_to_buffer(&mut buffer)
            .expect("our buffer should have been big enough");

        self.tx.send(&buffer[..bytes_written]);
    }
}

/// The transmitting end of a *Serial Connection*.
pub trait Tx {
    /// Queue a [`Packet`] to be sent to the frontend.
    ///
    /// There is no guarantee that the data will all be sent. This may happen if
    /// the receiver isn't listening or they aren't able to receive at this
    /// time.
    fn send(&mut self, data: &[u8]);
}

impl<'a, T: Tx> Tx for &'a mut T {
    fn send(&mut self, data: &[u8]) { (*self).send(data); }
}

pub trait MessageHandler {
    fn handle_message(&mut self, msg: &Packet) -> Result<Packet, CommsError>;
    /// Callback used to notify the application whenever a CRC error occurs.
    fn on_crc_error(&mut self) {}
}

impl<'a, M: MessageHandler> MessageHandler for &'a mut M {
    fn handle_message(&mut self, msg: &Packet) -> Result<Packet, CommsError> {
        (*self).handle_message(msg)
    }

    fn on_crc_error(&mut self) { (*self).on_crc_error(); }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CommsError {
    /// The [`MessageHandler`] doesn't know how to handle the message.
    UnknownMessageType,
    ParseFailed,
}
