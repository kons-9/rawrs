use super::Rgb;
use crate::rawconfig::RawImage2D;

pub fn linear_correction(raw: &RawImage2D) -> Vec<Vec<Rgb>> {
    let maxval = raw.config.max;
    let normalize =
        |x: u32, n: u32| -> f32 { (x as f32 / n as f32 * 255.0 / maxval as f32).min(255.0) };
    let data = &raw.data;
    let normal_beyer_to_rgb: [Box<dyn Fn(usize, usize) -> Rgb>; 4] = [
        Box::new(|w, h| {
            let r = data[w][h];
            let g = data[w][h - 1] + data[w - 1][h] + data[w + 1][h] + data[w][h + 1];
            let b =
                data[w - 1][h - 1] + data[w + 1][h - 1] + data[w - 1][h + 1] + data[w + 1][h + 1];

            Rgb::Float([normalize(r, 1), normalize(g, 4), normalize(b, 4)])
        }),
        Box::new(|w, h| {
            let r = data[w - 1][h] + data[w + 1][h];
            let g = data[w][h];
            let b = data[w][h - 1] + data[w][h + 1];
            Rgb::Float([normalize(r, 2), normalize(g, 1), normalize(b, 2)])
        }),
        Box::new(|w, h| {
            let r = data[w][h - 1] + data[w][h + 1];
            let g = data[w][h];
            let b = data[w - 1][h] + data[w + 1][h];
            Rgb::Float([normalize(r, 2), normalize(g, 1), normalize(b, 2)])
        }),
        Box::new(|w, h| {
            let r =
                data[w - 1][h - 1] + data[w + 1][h - 1] + data[w - 1][h + 1] + data[w + 1][h + 1];
            let g = data[w][h - 1] + data[w - 1][h] + data[w + 1][h] + data[w][h + 1];
            let b = data[w][h];
            Rgb::Float([normalize(r, 4), normalize(g, 4), normalize(b, 1)])
        }),
    ];
    let mut rgbdata = Vec::with_capacity(raw.config.height - 2);

    println!(
        "w:{} h:{} cw: {} ch: {}",
        data.len(),
        data[0].len(),
        raw.config.width,
        raw.config.height
    );
    for height in 1..(raw.config.height - 1) {
        rgbdata.push(Vec::with_capacity(raw.config.width - 2));
        for width in 1..(raw.config.width - 1) {
            let idx = (height & 1) * 2 + (width & 1);
            let rgb = { normal_beyer_to_rgb[idx](height, width) };
            rgbdata[height - 1].push(rgb)
        }
    }
    rgbdata
}
