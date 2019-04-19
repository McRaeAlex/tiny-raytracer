use cgmath::*;

pub struct Light {
    pub position: Vector3<f32>,
    pub intensity: f32,
}

impl Light {
    pub fn new(position: Vector3<f32>, intensity: f32) -> Light {
        Light {
            position,
            intensity,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Material {
    pub diffuse_color: Vector3<f32>,
    pub specular_exponent: f32,
    pub albedo: Vector4<f32>,
    pub refractive_index: f32,
}

impl Material {
    pub fn new(
        diffuse_color: Vector3<f32>,
        specular_exponent: f32,
        albedo: Vector4<f32>,
        refractive_index: f32,
    ) -> Material {
        Material {
            diffuse_color,
            specular_exponent,
            albedo,
            refractive_index,
        }
    }
}
