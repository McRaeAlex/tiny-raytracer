pub struct Light {
    pub position: [f32; 3],
    pub intensity: f32,
}

impl Light {
    pub fn new(position: [f32; 3], intensity: f32) -> Light {
        Light {
            position,
            intensity,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Material {
    pub diffuse_color: [u8; 3],
    pub specular_exponent: f32,
    pub albedo: [f32; 3],
}

impl Material {
    pub fn from(diffuse_color: [u8; 3], specular_exponent: f32, albedo: [f32; 3]) -> Material {
        Material {
            diffuse_color,
            specular_exponent,
            albedo,
        }
    }
}
