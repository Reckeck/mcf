use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum ColorSpace {
    #[default]
    Invalid,
    Unspecified,
    Reserved,

    Rgb,
    Bt601,
    Bt709,
    Bt2020Cl,
    Bt2020Ncl,
    Fcc,
    Bt470bg,
    Smpte170m,
    Smpte240m,
    Smpte2085,
    Ycgco,
}
