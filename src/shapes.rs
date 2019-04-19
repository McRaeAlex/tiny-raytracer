use super::effects::*;
use super::rendering::Renderable;

use cgmath::*;

pub struct Sphere {
    center: Vector3<f32>,
    radius: f32,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vector3<f32>, radius: f32, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Renderable for Sphere {
    fn ray_intersect(&self, origin: &Vector3<f32>, dir: &Vector3<f32>) -> Option<f32> {
        let origin_to_center = self.center - origin;
        let result = dot(origin_to_center, *dir);

        let distance_squared = dot(origin_to_center, origin_to_center) - result * result;

        if distance_squared > self.radius * self.radius {
            return None;
        }

        let dist_to_intersection = (self.radius * self.radius - distance_squared).sqrt();
        let mut t = result - dist_to_intersection;
        let t1 = result + dist_to_intersection;
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

    fn compute_normal(&self, point: &Vector3<f32>) -> Vector3<f32> {
        (point - self.center).normalize()
    }
}

pub struct Plane {
    normal: Vector3<f32>,
    point: Vector3<f32>,
    material: Material,
}

impl Plane {
    pub fn new(normal: Vector3<f32>, point: Vector3<f32>, material: Material) -> Plane {
        Plane {
            normal,
            point,
            material,
        }
    }
}

impl Renderable for Plane {
    fn ray_intersect(&self, origin: &Vector3<f32>, dir: &Vector3<f32>) -> Option<f32> {
        let temp = dot(self.normal, *dir);
        if temp == 0.0 || temp == -0.0 {
            return None;
        }

        let t = (dot(self.normal, self.point) - dot(self.normal, *origin)) / dot(self.normal, *dir);
        //let point = add(&origin, &scalar_mult(dir, t));
        return Some(t);
    }

    fn material(&self) -> Material {
        self.material.clone()
    }

    fn compute_normal(&self, point: &Vector3<f32>) -> Vector3<f32> {
        return self.normal.normalize();
    }
}
