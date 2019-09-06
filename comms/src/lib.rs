#![no_std]

#[derive(Debug, Clone, PartialEq)]
pub struct Communications {}

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

/// The transmitting end of a *Serial Connection*.
pub trait Tx {
    /// Queue some data to be sent to the frontend.
    ///
    /// There is no guarantee that the data will all be sent. This may happen if
    /// the receiver isn't listening or they aren't able to receive at this
    /// time.
    fn send(&mut self, data: &[u8]);
}
