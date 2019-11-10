use core::mem;
use scroll::{
    ctx::{StrCtx, TryFromCtx, TryIntoCtx},
    Endian,
};

/// A message containing part of a g-code program.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct GcodeProgram<'a> {
    first_line: u32,
    text: &'a str,
}

impl<'a> GcodeProgram<'a> {
    /// The message ID used with [`anpp::Packet::id()`].
    pub const ID: u8 = 5;
    /// The maximum amount of text a [`GcodeProgram`] message can contain.
    pub const MAX_TEXT_SIZE: usize =
        anpp::Packet::MAX_PACKET_SIZE - mem::size_of::<u32>();

    /// Create a new [`GcodeProgram`] message.
    ///
    /// # Panics
    ///
    /// The `text` must be smaller than [`GcodeProgram::MAX_TEXT_SIZE`] bytes
    /// long.
    pub fn new(first_line: u32, text: &'a str) -> GcodeProgram<'a> {
        assert!(text.len() < Self::MAX_TEXT_SIZE);

        GcodeProgram { first_line, text }
    }

    /// The (zero-based) line number this chunk starts on.
    ///
    /// Primarily used for error messages and progress reporting.
    pub fn first_line(&self) -> u32 { self.first_line }

    /// The g-code program itself.
    pub fn text(&self) -> &'a str { self.text }
}

impl<'a> TryFromCtx<'a, Endian> for GcodeProgram<'a> {
    type Error = scroll::Error;

    fn try_from_ctx(
        from: &'a [u8],
        ctx: Endian,
    ) -> Result<(Self, usize), Self::Error> {
        let total_length = from.len();

        let (first_line, bytes_read) = TryFromCtx::try_from_ctx(from, ctx)?;
        let text = decode_rest_as_str(&from[bytes_read..])?;

        let msg = GcodeProgram { first_line, text };
        Ok((msg, total_length))
    }
}

impl<'a> TryIntoCtx<Endian> for GcodeProgram<'a> {
    type Error = scroll::Error;

    fn try_into_ctx(
        self,
        buffer: &mut [u8],
        ctx: Endian,
    ) -> Result<usize, Self::Error> {
        let GcodeProgram { first_line, text } = self;
        let mut bytes_written = 0;

        bytes_written +=
            first_line.try_into_ctx(&mut buffer[bytes_written..], ctx)?;
        bytes_written += text
            .as_bytes()
            .try_into_ctx(&mut buffer[bytes_written..], ())?;

        Ok(bytes_written)
    }
}

fn decode_rest_as_str(from: &[u8]) -> Result<&str, scroll::Error> {
    let (text, _) = <&str as TryFromCtx<StrCtx>>::try_from_ctx(
        from,
        StrCtx::Length(from.len()),
    )?;

    Ok(text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use scroll::{Pread, Pwrite};
    use std::prelude::v1::*;

    #[test]
    fn decode_gcode_program_message() {
        let mut buffer = vec![0, 0, 2, 0];
        let expected = GcodeProgram {
            first_line: 2 * 256,
            text: "Hello, World!",
        };
        buffer.extend(expected.text.as_bytes());

        let (got, bytes_read) =
            GcodeProgram::try_from_ctx(&buffer, Endian::network()).unwrap();

        assert_eq!(got, expected);
        assert_eq!(bytes_read, buffer.len());
    }

    #[test]
    fn round_trip_through_scroll() {
        let msg = GcodeProgram {
            first_line: 2 * 256,
            text: "Hello, World!",
        };
        let mut buffer = [0; anpp::Packet::MAX_PACKET_SIZE];

        let bytes_written =
            buffer.pwrite_with(msg, 0, Endian::network()).unwrap();
        let buffer = &buffer[..bytes_written];
        let round_tripped: GcodeProgram =
            buffer.pread_with(0, Endian::network()).unwrap();

        assert_eq!(round_tripped, msg);
    }
}
