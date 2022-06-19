use crate::rawconfig::RawImage2D;
pub fn blacklevel_correction(raw: &mut RawImage2D) {
    let config = &raw.config;
    let data = &mut raw.data;
    let height = config.height;
    let width = config.width;
    let blacklevel = raw.config.blacklevel;
    let beyer_pattern = raw.config.beyer_pattern;

    let dummy_data = 0;
    let dummy_vec = vec![0; width];
    data.push(dummy_vec);
    for h in (0..height).step_by(2) {
        data[h].push(dummy_data);
        for w in (0..width).step_by(2) {
            data[h][w] = data[h][w].saturating_sub(blacklevel[beyer_pattern[0]] as u32);
            data[h][w + 1] = data[h][w + 1].saturating_sub(blacklevel[beyer_pattern[1]] as u32);
            data[h + 1][w] = data[h + 1][w].saturating_sub(blacklevel[beyer_pattern[2]] as u32);
            data[h + 1][w + 1] =
                data[h + 1][w + 1].saturating_sub(blacklevel[beyer_pattern[3]] as u32);
        }
        data[h].pop();
    }
    data.pop();
}
