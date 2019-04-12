extern crate image;

mod math;

use std::f32;

use math::*;

struct Sphere {
    center: [f32; 3],
    radius: f32,
}

impl Sphere {
    fn new(center: [f32; 3], radius: f32) -> Sphere {
        Sphere { center, radius }
    }

    fn ray_intersect(&self, origin: &[f32; 3], dir: &[f32; 3], _t: f32) -> bool {
        // Note it is assumed that dir is a unit vector and that is why we
        // do not have to do the entire projection just the dot product
        let origin_to_center = [
            self.center[0] - origin[0],
            self.center[1] - origin[1],
            self.center[2] - origin[2],
        ];
        let dot = dot_prod(&origin_to_center, dir);

        let distance_squared = dot_prod(&origin_to_center, &origin_to_center) - dot * dot;

        if distance_squared > self.radius * self.radius {
            return false;
        }

        let dist_to_intersection = (self.radius * self.radius - distance_squared).sqrt();
        let mut t0 = dot - dist_to_intersection;
        let t1 = dot + dist_to_intersection;
        if t0 < 0.0 {
            t0 = t1;
        }
        if t0 < 0.0 {
            return false;
        }
        return true;
    }
}

fn cast_ray(origin: &[f32; 3], dir: &[f32; 3], sphere: &Sphere) -> [u8; 3] {
    if !sphere.ray_intersect(origin, dir, f32::MAX) {
        return [0, 51, 0]; // background color
    }
    return [204, 51, 153]; // color of sphere
}

fn render(sphere: &Sphere) {
    let width = 1024;
    let height = 768;
    let fov = f32::consts::PI / 2.0;

    let width_f = width as f32;
    let height_f = height as f32;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let x_f = x as f32;
        let y_f = y as f32;
        let screen_width = (fov / 2.0).tan();
        let i = (2.0 * (x_f + 0.5) / width_f - 1.0) * screen_width * width_f / height_f;
        let j = -(2.0 * (y_f + 0.5) / height_f - 1.0) * screen_width;
        let dir = normalize(&[i, j, -1.0]);
        *pixel = image::Rgb(cast_ray(&[0.0, 0.0, 0.0], &dir, sphere));
    }

    imgbuf.save("step2.png").expect("Could not write image");
}

fn main() {
    let s = Sphere::new([-3.0, 0.0, -16.0], 2.0);
    render(&s);
}
