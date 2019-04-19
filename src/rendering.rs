extern crate image;

use std::f32;

use super::effects::*;
use crate::math::*;
use cgmath::prelude::*;
use cgmath::*;

/// This is a trait that defines something that can be rendered.
pub trait Renderable {
    /// This returns Some(dist) if the ray intersects
    /// return none is the ray misses.
    /// Assumes the direction passed in is a unit vector.
    fn ray_intersect(&self, origin: &Vector3<f32>, direction: &Vector3<f32>) -> Option<f32>;

    /// This returns a reference to the material of the item casted too.
    fn material(&self) -> Material;

    /// Given the location that the ray hit the renderable, give the normal
    fn compute_normal(&self, hit: &Vector3<f32>) -> Vector3<f32>;
}

/// Render takes in a set of object that can be displayed on the screen and a
/// set of lights and creates a image that displays the scene.
pub fn render(objects: &Vec<Box<Renderable>>, lights: &Vec<Light>) {
    let height = 768;
    let width = 1024;
    let fov = f32::consts::PI / 2.0;

    let width_f = width as f32;
    let height_f = height as f32;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    // compute each pixel
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // compute the direction vector
        let x_f = x as f32;
        let y_f = y as f32;
        let screen_width = (fov / 2.0).tan();
        let i = (2.0 * (x_f + 0.5) / width_f - 1.0) * screen_width * width_f / height_f;
        let j = -(2.0 * (y_f + 0.5) / height_f - 1.0) * screen_width;

        let direction = Vector3::new(i, j, 0.0).normalize();
        let origin = Vector3::new(0.0, 0.0, 0.0);

        // send the ray out and get the color back to draw the pixel
        let color_f32 = cast_ray(&origin, &direction, objects, lights, 0);
        let color_u8: Vector3<u8> = (color_f32 * 255.0).cast().expect("Error: failed on cast");
        *pixel = image::Rgb([color_u8.x, color_u8.y, color_u8.z]);
    }

    // write to disk
    imgbuf
        .save("rewrite_cgmath.png")
        .expect("failed to write image to disk");
}

pub fn cast_ray(
    origin: &Vector3<f32>,
    direction: &Vector3<f32>,
    objects: &Vec<Box<Renderable>>,
    lights: &Vec<Light>,
    depth: u8,
) -> Vector3<f32> {
    // if we have reflected alot
    if depth > 4 {
        return Vector3::new(0.0, 0.6, 0.3);
    }

    let (hit, normal, material) = match scene_intersect(origin, direction, objects) {
        Some(val) => val,                           // something was hit
        None => return Vector3::new(0.0, 0.6, 0.3), // not object was hit so background color
    };

    // Compute the direction of the refraction
    let refract_dir = refract(direction, &normal, material.refractive_index).normalize();
    let refract_origin = match dot(refract_dir, normal) < 0.0 {
        true => hit - normal * 0.001,
        false => hit + normal * 0.001,
    };
    // Send the ray out
    let refract_color = cast_ray(&refract_origin, &refract_dir, objects, lights, depth + 1);

    // Compute the direction of the reflection
    let reflect_dir = reflect(direction, &normal).normalize();
    let reflect_origin = match dot(reflect_dir, normal) < 0.0 {
        true => hit - normal * 0.001,
        false => hit + normal * 0.001,
    };
    // Send the ray
    let reflect_color = cast_ray(&reflect_origin, &reflect_dir, objects, lights, depth + 1);

    let mut diffuse_light_intensity = 0.0;
    let mut specular_light_intensity = 0.0;

    for light in lights {
        // get a vector from the point we hit the object to the light source
        let light_direction = (light.position - hit).normalize();

        // get the light distance
        let light_distance = (light.position - hit).magnitude();

        // we move the origin from the hit slightly to make sure we dont colide
        // with where we hit again.
        let shadow_origin = match dot(light_direction, normal) < 0.0 {
            true => hit - normal * 0.001,
            false => hit + normal * 0.001,
        };

        // check if the ray hits anything and then check if it is closer than
        // the light
        // this is a bit messy
        if let Some((shadow_hit, _shadow_norm, _)) =
            scene_intersect(&shadow_origin, &light_direction, objects)
        {
            if (shadow_hit - shadow_origin).magnitude() < light_distance {
                continue;
            }
        };

        // add to the brightness
        diffuse_light_intensity += light.intensity * max(0.0, dot(light_direction, normal));

        // add to the shinyness
        specular_light_intensity += max(
            0.0,
            dot(
                reflect(&(light_direction * -1.0), &normal) * -1.0,
                *direction,
            ),
        )
        .powf(material.specular_exponent)
            * light.intensity;
    }

    let mut color = material.diffuse_color * diffuse_light_intensity * material.albedo[0]
        + Vector3::new(1.0, 1.0, 1.0) * specular_light_intensity * material.albedo[1]
        + reflect_color * material.albedo[2]
        + refract_color * material.albedo[4];

    // get the max color and if its over the value 255 make it 255
    let max_color = max(color.x, max(color.y, color.z));
    if max_color > 1.0 {
        color = color * 1.0 / max_color;
    }
    return color; // color of sphere
}

/// Scene intersect returns the None if the ray casted does not hit anything.
/// It returns Some((hit: [f32;3], normal: Vector3<f32>, material: Material))
/// if it hits anything. The distance is the distance from the origin passed in
/// to the point, the normal is the vector directly out of the surface where the
/// ray hit. And the material is the material of the renderable hit.
pub fn scene_intersect(
    origin: &Vector3<f32>,
    dir: &Vector3<f32>,
    renders: &Vec<Box<Renderable>>,
) -> Option<(Vector3<f32>, Vector3<f32>, Material)> {
    let mut closest_model_dist = f32::MAX;
    let mut hit = Vector3::new(0.0, 0.0, 0.0);
    let mut normal = Vector3::new(0.0, 0.0, 0.0);
    let mut material = Material::new(
        Vector3::new(0.4, 0.9, 0.4),
        10.0,
        Vector4::new(0.6, 0.3, 0.0, 0.0),
        0.0,
    );
    for model in renders {
        let dist_i = match model.ray_intersect(origin, dir) {
            Some(dist) => dist, // the object was hit
            None => continue,   // this means we missed this object so we can move onto the next
        };
        if dist_i < closest_model_dist {
            closest_model_dist = dist_i;
            hit = origin + dir * dist_i;
            normal = model.compute_normal(&hit);
            material = model.material();
        }
    }

    if closest_model_dist < 1000.0 {
        return Some((hit, normal, material));
    } else {
        return None;
    }
}
