use crate::rawconfig::RawImage2D;

mod blacklevel;

pub struct RawProcessor {}
impl RawProcessor {
    pub fn blacklevel_correction(raw: &mut RawImage2D) {
        blacklevel::blacklevel_correction(raw)
    }
}
