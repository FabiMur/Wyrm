
use crate::primitives::*;
pub trait Texture: Send + Sync {
    fn value(&self, u:f64, v:f64, p: &Point3) -> Color;
}
