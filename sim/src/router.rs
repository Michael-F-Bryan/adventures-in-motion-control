use aimc_comms::{CommsError, MessageHandler};
use aimc_fps_counter::{Clear, FpsCounter};
use aimc_hal::messaging::{Ack, Handler, Nack};
use aimc_motion::{GcodeProgram, Motion, StartHomingSequence};
use anpp::Packet;
use scroll::{ctx::TryFromCtx, Pread};

/// A middleman used for dispatching messages to the various systems across the
/// application.
pub(crate) struct Router<'a> {
    pub(crate) fps: &'a mut FpsCounter,
    pub(crate) motion: &'a mut Motion,
}

impl<'a> MessageHandler for Router<'a> {
    fn handle_message(&mut self, msg: &Packet) -> Result<Packet, CommsError> {
        match msg.id() {
            Ack::ID | Nack::ID => Ok(msg.clone()),
            Clear::ID => {
                dispatch::<_, Clear, _>(self.fps, msg.contents(), Into::into)
            },
            StartHomingSequence::ID => dispatch::<_, StartHomingSequence, _>(
                self.motion,
                msg.contents(),
                map_result,
            ),
            GcodeProgram::ID => unimplemented!(),
            // echo
            42 => Ok(msg.clone()),
            _ => Err(CommsError::UnknownMessageType),
        }
    }
}

/// Parse a message using [`scroll`] and send it to some [`Handler`], turning
/// the response back into an ANPP [`Packet`].
fn dispatch<'a, H, M, F>(
    handler: &mut H,
    raw_msg: &'a [u8],
    to_packet: F,
) -> Result<Packet, CommsError>
where
    H: Handler<M>,
    M: TryFromCtx<'a, scroll::Endian, Size = usize>,
    M::Error: From<scroll::Error>,
    F: Fn(H::Response) -> Packet,
{
    let msg: M = raw_msg
        .pread_with(0, scroll::NETWORK)
        .map_err(|_| CommsError::ParseFailed)?;

    let response = handler.handle(msg);

    Ok(to_packet(response))
}

fn map_result<A, B>(result: Result<A, B>) -> Packet
where
    A: Into<Packet>,
    B: Into<Packet>,
{
    match result {
        Result::Ok(a) => a.into(),
        Result::Err(b) => b.into(),
    }
}
