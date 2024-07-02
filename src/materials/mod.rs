
pub mod material;
pub use self::material::MaterialArcWrapper;
pub use self::material::Material;


pub mod metal;
pub use self::metal::Metal;

pub mod lambertian;
pub use self::lambertian::Lambertian;


pub mod dielectric;
pub use self::dielectric::Dielectric;