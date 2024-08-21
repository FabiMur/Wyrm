use std::sync::Arc;
use crate::primitives::*;
use crate::scattering_function::*;

#[derive(Clone)]
pub struct Material {
    pub diffuse: Arc<dyn ScatteringFunction>,
    pub specular: Arc<dyn ScatteringFunction>,
    pub refractive: Arc<dyn ScatteringFunction>,
    pub emit: Option<Color>,  // Optional emission color for emissive materials
    pub kd: f64,  // Diffuse coefficient
    pub ks: f64,  // Specular coefficient
    pub kt: f64,  // Transmission/refractive coefficient
    pub absorption: f64,  // Absorption coefficient
}

impl Material {
    pub fn new(
        diffuse: Arc<dyn ScatteringFunction>,
        specular: Arc<dyn ScatteringFunction>,
        refractive: Arc<dyn ScatteringFunction>,
        emit: Option<Color>,
        mut kd: f64,
        mut ks: f64,
        mut kt: f64,
        mut absorption: f64,
    ) -> Self {
        let sum = kd + ks + kt + absorption;
        if sum > 0.0 {
            kd /= sum;
            ks /= sum;
            kt /= sum;
            absorption /= sum;
        }

        Material {
            diffuse,
            specular,
            refractive,
            emit,
            kd,
            ks,
            kt,
            absorption,
        }
    }
}
