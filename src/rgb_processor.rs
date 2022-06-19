use std::cmp::min;

use image::ImageBuffer;

use crate::rawconfig::{RawConfig, RawImage2D};

mod demosaic;

#[derive(Debug)]
pub enum Rgb {
    Integer([u8; 3]),
    Float([f32; 3]),
}

type RgbData = Vec<Vec<Rgb>>;
#[derive(Debug)]
pub struct RgbImage {
    pub rgbdata: RgbData,
    pub config: RawConfig,
}

impl RgbImage {
    pub fn new(raw: RawImage2D) -> Self {
        RgbImage {
            rgbdata: demosaic::linear_correction(&raw),
            config: raw.config,
        }
    }
    pub fn use_custum_demosaic_fn(raw: RawImage2D, f: fn(&RawImage2D) -> RgbData) -> Self {
        RgbImage {
            rgbdata: f(&raw),
            config: raw.config,
        }
    }
    pub fn gamma_correction(&mut self) {
        for i in &mut self.rgbdata {
            for rgb in i {
                match rgb {
                    Rgb::Float(x) => {
                        for i in 0..3 {
                            x[i] = (x[i] / 255.0).powf(1.0 / 2.2) * 255.0;
                        }
                    }
                    Rgb::Integer(x) => {
                        for i in 0..3 {
                            x[i] = ((x[i] as f32 / 255.0).powf(1.0 / 2.2) * 255.0) as u8;
                        }
                    }
                };
            }
        }
    }
    pub fn whiteblance(&mut self) {
        let whiteblance = self.config.whiteblance;
        let min = 1024.0;
        // let min = whiteblance
        //     .into_iter()
        //     .min_by(|a, b| a.partial_cmp(b).expect(&format!("{:?}", whiteblance)))
        //     .unwrap();
        println!("whiteblance: {:?}", whiteblance);
        for d in &mut self.rgbdata {
            for rgb in d {
                match rgb {
                    Rgb::Integer(x) => {
                        x[0] = (x[0] as f32 * whiteblance[0] as f32 / min) as u8;
                        x[1] = (x[1] as f32 * whiteblance[1] as f32 / min) as u8;
                        x[2] = (x[2] as f32 * whiteblance[2] as f32 / min) as u8;
                    }
                    Rgb::Float(x) => {
                        x[0] *= whiteblance[0] / min;
                        x[1] *= whiteblance[1] / min;
                        x[2] *= whiteblance[2] / min;
                    }
                }
            }
        }
    }
    pub fn histgram_equalization(&mut self) {
        let mut rhist = vec![0.0; 256];
        let mut ghist = vec![0.0; 256];
        let mut bhist = vec![0.0; 256];
        for v in &self.rgbdata {
            for rgb in v {
                match rgb {
                    Rgb::Float([r, g, b]) => {
                        rhist[min(*r as usize, 255 as usize)] += 1.0;
                        ghist[min(*g as usize, 255 as usize)] += 1.0;
                        bhist[min(*b as usize, 255 as usize)] += 1.0;
                    }
                    Rgb::Integer([r, g, b]) => {
                        rhist[min(*r as usize, 255 as usize)] += 1.0;
                        ghist[min(*g as usize, 255 as usize)] += 1.0;
                        bhist[min(*b as usize, 255 as usize)] += 1.0;
                    }
                }
            }
        }
        // cumsum
        let mut rhist_cum = Self::cumsum(rhist);
        let mut ghist_cum = Self::cumsum(ghist);
        let mut bhist_cum = Self::cumsum(bhist);

        let rhist_max = *rhist_cum.last().unwrap();
        let ghist_max = *ghist_cum.last().unwrap();
        let bhist_max = *bhist_cum.last().unwrap();

        rhist_cum = rhist_cum
            .into_iter()
            .map(|x| x / rhist_max * 255.0)
            .collect();
        rhist_cum.push(*rhist_cum.last().unwrap());
        ghist_cum = ghist_cum
            .into_iter()
            .map(|x| x / ghist_max * 255.0)
            .collect();
        ghist_cum.push(*ghist_cum.last().unwrap());
        bhist_cum = bhist_cum
            .into_iter()
            .map(|x| x / bhist_max * 255.0)
            .collect();
        bhist_cum.push(*bhist_cum.last().unwrap());

        for v in &mut self.rgbdata {
            for rgb in v {
                match rgb {
                    Rgb::Float([r, g, b]) => {
                        *r = Self::currection_cumsum_f32(*r, &rhist_cum);
                        *g = Self::currection_cumsum_f32(*g, &ghist_cum);
                        *b = Self::currection_cumsum_f32(*b, &bhist_cum);
                    }
                    Rgb::Integer([r, g, b]) => {
                        *r = Self::currection_cumsum_u8(*r, &rhist_cum);
                        *g = Self::currection_cumsum_u8(*g, &ghist_cum);
                        *b = Self::currection_cumsum_u8(*b, &bhist_cum);
                    }
                }
            }
        }
    }

    #[inline]
    fn currection_cumsum_u8(val: u8, v_cumsum: &Vec<f32>) -> u8 {
        v_cumsum[val as usize] as u8
    }
    #[inline]
    fn currection_cumsum_f32(val: f32, v_cumsum: &Vec<f32>) -> f32 {
        let u_val = val as usize;
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

    pub fn u8_rgb(&self) -> RgbData {
        let mut u8rgb = Vec::with_capacity(self.rgbdata.len());
        for d in &self.rgbdata {
            u8rgb.push(
                d.iter()
                    .map(|x| match x {
                        Rgb::Float([a, b, c]) => Rgb::Integer([*a as u8, *b as u8, *c as u8]),
                        Rgb::Integer(x) => Rgb::Integer(*x),
                    })
                    .collect(),
            );
        }

        u8rgb
    }
    pub fn to_imagebuffer(self) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        let u8data = self.u8_rgb();
        let height = u8data.len();
        let width = u8data[0].len();
        let mut img = image::RgbImage::new(width as u32, height as u32);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            *pixel = image::Rgb(match u8data[y as usize][x as usize] {
                Rgb::Integer(u) => u,
                _ => panic!("why..?"),
            });
        }
        img
    }
}
