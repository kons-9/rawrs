use crate::decoder::DecodeResult;
use crate::decoder::DecodingError;
use crate::rawconfig::RawConfig;
use crate::rawconfig::RawImage;
use std::path::Path;

use rawloader;

pub fn decode(file: &Path) -> DecodeResult<RawImage> {
    let image = rawloader::decode_file(file).unwrap();
    if let rawloader::RawImageData::Integer(data) = image.data {
        Ok(RawImage {
            data: data,
            config: RawConfig {
                height: image.height,
                width: image.width,
                cpp: image.cpp,
                max: 16300,
                // max: 5000,
                blacklevel: image.blacklevels,
                whiteblance: image.wb_coeffs,
                beyer_pattern: [0, 1, 1, 2],
            },
        })
    } else {
        Err(DecodingError::new("decode error"))
    }
}
