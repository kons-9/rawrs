use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use byteorder::{BigEndian, ByteOrder, LittleEndian};
// use rawloader::Buffer;

// pub struct Buffer<'a> {
//     pub buf: &'a [u8],
// }

pub mod arw;

use arw::ArwDecoder;

pub type DecodeResult<T> = Result<T, DecodingError>;
pub enum Endian {
    LittleEndian,
    BigEndian,
}

impl Endian {
    pub fn new(big: bool) -> Self {
        if big {
            Endian::BigEndian
        } else {
            Endian::LittleEndian
        }
    }
    pub fn read_u16(&self, buf: &[u8], pos: usize) -> u16 {
        match self {
            Endian::BigEndian => BigEndian::read_u16(&buf[pos..pos + 2]),
            Endian::LittleEndian => LittleEndian::read_u16(&buf[pos..pos + 2]),
        }
    }
}

trait Decoder {
    fn to_image<'a, 'b>(&'a self) -> DecodeResult<Buffer>;
}

#[derive(Debug)]
pub struct DecodingError {
    error: String,
}

impl DecodingError {
    pub fn new(s: impl Into<String>) -> Self {
        DecodingError { error: s.into() }
    }
}
pub struct Buffer {
    buf: Vec<u8>,
    size: usize,
}

impl Buffer {
    pub fn new(reader: &mut dyn Read) -> DecodeResult<Self> {
        let mut buf = Vec::new();
        match reader.read_to_end(&mut buf) {
            Ok(_) => {
                let size = buf.len();
                Ok(Buffer { buf, size })
            }
            Err(e) => Err(DecodingError::new(e.to_string())),
        }
    }
}

pub struct RawLoader {}
impl RawLoader {
    pub fn new() -> Self {
        // pass
        RawLoader {}
    }
    pub fn get_decoder<'a>(&'a self, buf: &'a Buffer) -> DecodeResult<Box<dyn Decoder>> {
        Ok(Box::new(ArwDecoder {}))
    }

    pub fn decode(&self, reader: &mut BufReader<File>) -> DecodeResult<Buffer> {
        let buf = Buffer::new(reader)?;
        let decoder = self.get_decoder(&buf)?;
        decoder.to_image()
    }

    pub fn open_file(&self, path: &Path) -> DecodeResult<Buffer> {
        let file = match File::open(path) {
            Ok(x) => x,
            Err(e) => return Err(DecodingError::new(e.to_string())),
        };
        let mut bufreader = BufReader::new(file);
        self.decode(&mut bufreader)
    }
}
