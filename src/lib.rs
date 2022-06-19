// mod converter;
mod decoder;
mod raw_processor;
mod rawconfig;
mod rgb_processor;
mod using_rawloader;
mod yuv_processor;
use raw_processor::RawProcessor;
use rawconfig::RawImage2D;
use rgb_processor::RgbImage;
use std::{env, path::Path};

pub fn sample() {
    let args: Vec<_> = env::args().collect();
    let default = "sample/tmp.arw".to_string();
    let file = if args.len() != 2 {
        // println!("Usage: {} [file]", args[0]);
        // std::process::exit(2);
        &default
    } else {
        &args[1]
    };
    let rawimage = using_rawloader::decode(Path::new(file)).unwrap();
    let mut raw2d = RawImage2D::from1d(rawimage);
    RawProcessor::blacklevel_correction(&mut raw2d);
    let mut rgb_processor = RgbImage::new(raw2d);
    // rgb_processor.histgram_equalization();
    rgb_processor.whiteblance();
    rgb_processor.gamma_correction();
    let mut yuv_processor = yuv_processor::YuvImage::from_rgbimage(rgb_processor);
    yuv_processor.y_histgram_equalization();
    rgb_processor = yuv_processor.to_rgbimage();
    rgb_processor
        .to_imagebuffer()
        .save("output/outyuv.jpeg")
        .unwrap();
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn sample() {
        let args: Vec<_> = env::args().collect();
        let default = "sample/tmp.arw".to_string();
        let file = if args.len() != 2 {
            // println!("Usage: {} [file]", args[0]);
            // std::process::exit(2);
            &default
        } else {
            &args[1]
        };
        let rawimage = using_rawloader::decode(Path::new(file)).unwrap();
        let mut raw2d = RawImage2D::from1d(rawimage);
        RawProcessor::blacklevel_correction(&mut raw2d);
        let mut rgb_processor = RgbImage::new(raw2d);
        rgb_processor.whiteblance();
        rgb_processor.gamma_correction();
        rgb_processor
            .to_imagebuffer()
            .save("output/out.jpeg")
            .unwrap();
    }
    #[test]
    fn easytask() {
        let args: Vec<_> = env::args().collect();
        let default = "sample/tmp.arw".to_string();
        let file = if args.len() != 2 {
            // println!("Usage: {} [file]", args[0]);
            // std::process::exit(2);
            &default
        } else {
            &args[1]
        };
        let rawimage = using_rawloader::decode(Path::new(file)).unwrap();
        let raw2d = RawImage2D::from1d(rawimage);
        let rgb_processor = RgbImage::new(raw2d);
        // println!("{:?}", rgb_processor.rgbdata);
        rgb_processor
            .to_imagebuffer()
            .save("output/out.jpeg")
            .unwrap();
    }
}
