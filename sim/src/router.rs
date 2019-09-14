use aimc_comms::{CommsError, MessageHandler};
use aimc_fps_counter::{Clear, FpsCounter};
use aimc_hal::Handler;
use anpp::Packet;
use scroll::{ctx::TryFromCtx, Pread};

/// A middleman used for dispatching messages to the various systems across the
/// application.
pub(crate) struct Router<'a> {
    pub(crate) fps: &'a mut FpsCounter,
}

impl<'a> MessageHandler for Router<'a> {
    fn handle_message(&mut self, msg: &Packet) -> Result<Packet, CommsError> {
        match msg.id() {
            1 => dispatch::<_, Clear>(self.fps, msg.contents()),
            // echo
            42 => Ok(msg.clone()),
            _ => Err(CommsError::UnknownMessageType),
        }
    }
}

/// Parse a message using [`scroll`] and send it to some [`Handler`], turning
/// the response back into an ANPP [`Packet`].
fn dispatch<'a, H, M>(
    handler: &mut H,
    raw_msg: &'a [u8],
) -> Result<Packet, CommsError>
where
    H: Handler<M>,
    H::Response: Into<Packet>,
    M: TryFromCtx<'a, scroll::Endian, Size = usize>,
    M::Error: From<scroll::Error>,
{
    let msg: M = raw_msg
        .pread_with(0, scroll::NETWORK)
        .map_err(|_| CommsError::ParseFailed)?;

    let response = handler.handle(msg);

    Ok(response.into())
}
