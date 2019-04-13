use super::rendering::Renderable;
use super::math::*;
use super::effects::*;

pub struct Sphere {
    center: [f32; 3], 
    radius: f32, 
    material: Material,
}

impl Sphere {
    pub fn from(center: [f32; 3], radius: f32, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material
        }
    }
}

impl Renderable for Sphere {
    fn ray_intersect(&self, origin: &[f32; 3], dir: &[f32; 3]) -> Option<f32> {
        let origin_to_center = [
            self.center[0] - origin[0],
            self.center[1] - origin[1],
            self.center[2] - origin[2],
        ];
        let dot = dot_prod(&origin_to_center, dir);

        let distance_squared = dot_prod(&origin_to_center, &origin_to_center) - dot * dot;

        if distance_squared > self.radius * self.radius {
            return None;
        }

        let dist_to_intersection = (self.radius * self.radius - distance_squared).sqrt();
        let mut t = dot - dist_to_intersection;
        let t1 = dot + dist_to_intersection;
        if t < 0.0 {
            t = t1;
        }
        if t < 0.0 {
            return None;
        }
        return Some(t);
    }

    fn material(&self) -> Material {
        self.material.clone()
    }

    fn compute_normal(&self, point: &[f32; 3]) -> [f32; 3] {
        normalize(&subtract(point, &self.center))
    }
}