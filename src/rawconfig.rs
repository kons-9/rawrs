#[derive(Debug)]
pub struct RawConfig {
    pub height: usize,
    pub width: usize,
    pub cpp: usize,
    pub max: u32,
    pub blacklevel: [u16; 4], // RGGB
    pub whiteblance: [f32; 4],
    pub beyer_pattern: [usize; 4], // R0, G1, G2, B2
}

#[derive(Debug)]
pub struct RawImage {
    pub data: Vec<u16>,
    pub config: RawConfig,
}
#[derive(Debug)]
pub struct RawImage2D {
    pub data: Vec<Vec<u32>>,
    pub config: RawConfig,
}

impl RawImage2D {
    pub fn from1d(rawimage: RawImage) -> Self {
        let height = rawimage.config.height;
        let width = rawimage.config.width;

        let mut data = Vec::with_capacity(height);
        for h in 0..height {
            data.push(Vec::with_capacity(width));
            for w in 0..width {
                data[h].push(rawimage.data[h * width + w] as u32);
            }
        }
        RawImage2D {
            data: data,
            config: rawimage.config,
        }
    }
}
