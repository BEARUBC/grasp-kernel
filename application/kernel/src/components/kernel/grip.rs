#[derive(PartialEq, Eq)]
#[cfg_attr(not(release), derive(Debug))]
pub enum Grip {
    Hammer,
    Cup,
    Flat,
}

impl Default for Grip {
    fn default() -> Self {
        Self::Flat
    }
}

#[cfg(feature = "pseudo_analytics")]
impl From<f64> for Grip {
    fn from(data: f64) -> Self {
        const MODULO_BASE: u64 = 3;
        let data = data.floor() as u64;
        let data = data % MODULO_BASE;
        match data {
            0 => Self::Hammer,
            1 => Self::Cup,
            _ => Self::Flat,
        }
    }
}
