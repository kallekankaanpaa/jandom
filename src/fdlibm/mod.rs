mod parts;
mod halves;
mod log;
mod sqrt;

pub use log::log;
pub use sqrt::sqrt;

const TINY: f64 = 1.0e-300;
const ONE: f64 = 1.0;
