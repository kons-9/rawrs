use byteorder::ByteOrder;
use byteorder::LittleEndian;

use crate::decoder::DecodeResult;
use crate::decoder::Decoder;
use crate::decoder::DecodingError;
use crate::decoder::Endian;

use super::Buffer;
pub struct ArwDecoder {}

impl Decoder for ArwDecoder {
    fn to_image<'a, 'b>(&'a self) -> Result<Buffer, DecodingError> {
        Err(DecodingError {
            error: "error".to_string(),
        })
    }
}

impl ArwDecoder {
    pub fn new(buf: &[u8]) -> DecodeResult<Buffer> {
        let endian = match LittleEndian::read_u16(&buf[0..2]) {
            0x4949 => Endian::LittleEndian,
            0x4d4d => Endian::BigEndian,
            x => return Err(DecodingError::new(format!("invalid format 0x{:x}", x))),
        };

        return Err(DecodingError::new(format!("TIFF: don't know marker",)));
    }
}
