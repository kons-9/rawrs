use std::cmp::{max, min};

use crate::{
    rawconfig::RawConfig,
    rgb_processor::{Rgb, RgbImage},
};

pub struct Yuv([f32; 3]);

impl Yuv {
    pub fn from_rgb(rgb: &Rgb) -> Yuv {
        match rgb {
            Rgb::Float([r, g, b]) => {
                let y = 0.299 * r + 0.587 * g + 0.114 * b;
                let u = -0.169 * r - 0.331 * g + 0.500 * b;
                let v = 0.500 * r - 0.419 * g - 0.081 * b;
                Yuv([y, u, v])
            }
            Rgb::Integer([r, g, b]) => {
                let r = *r as f32;
                let g = *g as f32;
                let b = *b as f32;

                let y = 0.299 * r + 0.587 * g + 0.114 * b;
                let u = -0.169 * r - 0.331 * g + 0.500 * b;
                let v = 0.500 * r - 0.419 * g - 0.081 * b;
                Yuv([y, u, v])
            }
        }
    }
    pub fn to_rgb(&self) -> Rgb {
        let [mut y, mut u, mut v] = self.0;
        let r = 1.000 * y + 1.402 * v;
        let g = 1.000 * y - 0.344 * u - 0.714 * v;
        let b = 1.000 * y + 1.772 * u;
        Rgb::Float([r, g, b])
    }
}

pub struct YuvImage {
    yuvdata: Vec<Vec<Yuv>>,
    config: RawConfig,
}

impl YuvImage {
    pub fn from_rgbimage(rgbimage: RgbImage) -> Self {
        let config = rgbimage.config;
        let data = rgbimage.rgbdata;

        let mut yuvdata = Vec::with_capacity(data.len());
        let width = data[0].len();

        for (idx, v) in data.iter().enumerate() {
            yuvdata.push(Vec::with_capacity(width));
            for rgb in v {
                yuvdata[idx].push(Yuv::from_rgb(rgb));
            }
        }
        YuvImage { yuvdata, config }
    }
    pub fn to_rgbimage(self) -> RgbImage {
        let config = self.config;
        let data = self.yuvdata;

        let mut rgbdata = Vec::with_capacity(data.len());
        let width = data[0].len();

        for (idx, v) in data.iter().enumerate() {
            rgbdata.push(Vec::with_capacity(width));
            for yuv in v {
                rgbdata[idx].push(yuv.to_rgb());
            }
        }
        RgbImage { rgbdata, config }
    }
    pub fn y_histgram_equalization(&mut self) {
        // only y(luna)
        let mut yhist = vec![0.0; 256];
        for v in &self.yuvdata {
            for yuv in v {
                match yuv {
                    Yuv([y, _, _]) => {
                        yhist[min(*y as usize, 255 as usize)] += 1.0;
                    }
                }
            }
        }
        // cumsum
        let mut yhist_cum = Self::cumsum(yhist);

        let yhist_max = *yhist_cum.last().unwrap();

        yhist_cum = yhist_cum
            .into_iter()
            .map(|x| x / yhist_max * 255.0)
            .collect();
        yhist_cum.push(*yhist_cum.last().unwrap());

        for v in &mut self.yuvdata {
            for yuv in v {
                match yuv {
                    Yuv([y, _, _]) => {
                        *y = Self::currection_cumsum_f32(*y, &yhist_cum);
                    }
                }
            }
        }
    }
    #[inline]
    fn currection_cumsum_f32(val: f32, v_cumsum: &Vec<f32>) -> f32 {
        let u_val = min(val as usize, 255);
        let v_before = v_cumsum[u_val];
        let v_after = v_cumsum[u_val + 1];
        let sub_val = val - u_val as f32;
        sub_val * v_after + (1.0 - sub_val) * v_before
    }

    #[inline]
    fn cumsum(ve: Vec<f32>) -> Vec<f32> {
        ve.into_iter().fold(Vec::new(), |mut acc, v| {
            acc.push(acc.last().unwrap_or(&0.0) + v);
            acc
        })
    }
}
