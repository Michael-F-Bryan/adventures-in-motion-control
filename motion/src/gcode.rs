use core::mem;
use scroll::{
    ctx::{StrCtx, TryFromCtx},
    Endian,
};

/// A message containing part of a g-code program.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct GcodeProgram<'a> {
    chunk_number: u16,
    first_line: u32,
    text: &'a str,
}

impl<'a> GcodeProgram<'a> {
    /// The message ID used with [`anpp::Packet::id()`].
    pub const ID: u8 = 5;
    /// The maximum amount of text a [`GcodeProgram`] message can contain.
    pub const MAX_TEXT_SIZE: usize = anpp::Packet::MAX_PACKET_SIZE
        - mem::size_of::<u16>()
        - mem::size_of::<u32>();

    /// Create a new [`GcodeProgram`] message.
    ///
    /// # Panics
    ///
    /// The `text` must be smaller than [`GcodeProgram::MAX_TEXT_SIZE`] bytes
    /// long.
    pub fn new(
        chunk_number: u16,
        first_line: u32,
        text: &'a str,
    ) -> GcodeProgram<'a> {
        assert!(text.len() < Self::MAX_TEXT_SIZE);

        GcodeProgram {
            chunk_number,
            first_line,
            text,
        }
    }

    /// A number used to indicate which chunk of the program this is.
    ///
    /// The `chunk_number` should be reset to `0` when sending a new program
    /// and incremented for every chunk thereafter, wrapping back to `0` on
    /// overflow.
    pub fn chunk_number(&self) -> u16 { self.chunk_number }

    /// The (zero-based) line number this chunk starts on.
    ///
    /// Primarily used for error messages and progress reporting.
    pub fn first_line(&self) -> u32 { self.first_line }

    /// The g-code program itself.
    pub fn text(&self) -> &'a str { self.text }
}

impl<'a> TryFromCtx<'a, Endian> for GcodeProgram<'a> {
    type Error = scroll::Error;
    type Size = usize;

    fn try_from_ctx(
        from: &'a [u8],
        ctx: Endian,
    ) -> Result<(Self, Self::Size), Self::Error> {
        let total_length = from.len();

        let (chunk_number, bytes_read) = TryFromCtx::try_from_ctx(from, ctx)?;
        let from = &from[bytes_read..];
        let (first_line, bytes_read) = TryFromCtx::try_from_ctx(from, ctx)?;
        let text = decode_rest_as_str(&from[bytes_read..])?;

        let msg = GcodeProgram {
            chunk_number,
            first_line,
            text,
        };
        Ok((msg, total_length))
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
    use std::prelude::v1::*;

    #[test]
    fn decode_gcode_program_message() {
        let mut buffer = vec![0, 42, 0, 0, 2, 0];
        let expected = GcodeProgram {
            chunk_number: 42,
            first_line: 2 * 256,
            text: "Hello, World!",
        };
        buffer.extend(expected.text.as_bytes());

        let (got, bytes_read) =
            GcodeProgram::try_from_ctx(&buffer, Endian::network()).unwrap();

        assert_eq!(got, expected);
        assert_eq!(bytes_read, buffer.len());
    }
}
