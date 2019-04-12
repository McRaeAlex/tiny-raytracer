extern crate image;

mod math;

use std::f32;

use math::*;

struct Light {
    position: [f32; 3],
    intensity: f32,
}

impl Light {
    fn new(position: [f32; 3], intensity: f32) -> Light {
        Light {
            position,
            intensity,
        }
    }
}

#[derive(Copy, Clone)]
struct Material {
    diffuse_color: [u8; 3],
}

impl Material {
    fn new(diffuse_color: [u8; 3]) -> Material {
        Material { diffuse_color }
    }
}

struct Sphere {
    center: [f32; 3],
    radius: f32,
    material: Material,
}

impl Sphere {
    fn new(center: [f32; 3], radius: f32, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }

    fn ray_intersect(&self, origin: &[f32; 3], dir: &[f32; 3], t: &mut f32) -> bool {
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
        *t = dot - dist_to_intersection;
        let t1 = dot + dist_to_intersection;
        if *t < 0.0 {
            *t = t1;
        }
        if *t < 0.0 {
            return false;
        }
        return true;
    }
}

fn scene_intersect(
    origin: &[f32; 3],
    dir: &[f32; 3],
    spheres: &Vec<Sphere>,
    hit: &mut [f32; 3],
    normal: &mut [f32; 3],
    material: &mut Material,
) -> bool {
    let mut closest_sphere_dist = f32::MAX;
    for sphere in spheres {
        let mut dist_i = f32::MAX;
        if sphere.ray_intersect(origin, dir, &mut dist_i) && dist_i < closest_sphere_dist {
            closest_sphere_dist = dist_i;
            *hit = add(origin, &scalar_mult(&dir, dist_i));
            *normal = normalize(&subtract(&hit, &sphere.center));
            *material = sphere.material.clone();
        }
    }
    closest_sphere_dist < 1000.0
}

fn cast_ray(
    origin: &[f32; 3],
    dir: &[f32; 3],
    spheres: &Vec<Sphere>,
    lights: &Vec<Light>,
) -> [u8; 3] {
    let mut point = [0.0, 0.0, 0.0];
    let mut n = [0.0, 0.0, 0.0];
    let mut m = Material::new([0, 0, 0]);

    if !scene_intersect(origin, dir, spheres, &mut point, &mut n, &mut m) {
        return [0, 51, 0]; // background color
    }

    let mut diffuse_light_intesity = 0.0;
    for light in lights {
        let light_direction = normalize(&subtract(&light.position, &point));
        diffuse_light_intesity += light.intensity * max(0.0, dot_prod(&light_direction, &n));
    }

    let mut f32_color = scalar_mult(
        &[
            m.diffuse_color[0] as f32,
            m.diffuse_color[1] as f32,
            m.diffuse_color[2] as f32,
        ],
        1.0 / 255.0,
    );
    f32_color = scalar_mult(&f32_color, diffuse_light_intesity);
    f32_color = scalar_mult(&f32_color, 255.0);
    let u8_version = convert_to_u8(f32_color);
    return u8_version; // color of sphere
}

fn render(spheres: &Vec<Sphere>, lights: &Vec<Light>) {
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
        *pixel = image::Rgb(cast_ray(&[0.0, 0.0, 0.0], &dir, spheres, lights));
    }

    imgbuf.save("step4.png").expect("Could not write image");
}

fn main() {
    let s = vec![
        Sphere::new([-3.0, 0.0, -16.0], 2.0, Material::new([102, 102, 76])),
        Sphere::new([-1.0, -1.5, -12.0], 2.0, Material::new([76, 25, 25])),
        Sphere::new([1.5, -0.5, -18.0], 3.0, Material::new([76, 25, 25])),
        Sphere::new([7.0, 5.0, -18.0], 4.0, Material::new([102, 102, 76])),
    ];

    let l = vec![Light::new([20.0, 20.0, 20.0], 1.5)];
    render(&s, &l);
}
