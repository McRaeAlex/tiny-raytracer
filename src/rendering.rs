extern crate image;

use std::f32;

use super::effects::*;
use crate::math::*;

/// This is a trait that defines something that can be rendered.
pub trait Renderable {
    /// This returns Some(dist) if the ray intersects
    /// return none is the ray misses.
    /// Assumes the direction passed in is a unit vector.
    fn ray_intersect(&self, origin: &[f32; 3], direction: &[f32; 3]) -> Option<f32>;

    /// This returns a reference to the material of the item casted too.
    fn material(&self) -> Material;

    /// Given the location that the ray hit the renderable, give the normal
    fn compute_normal(&self, hit: &[f32; 3]) -> [f32; 3];
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
        let direction = normalize(&[i, j, -1.0]);

        // send the ray out and get the color back to draw the pixel
        let color = cast_ray(&[0.0, 0.0, 0.0], &direction, objects, lights);
        *pixel = image::Rgb(color);
    }

    // write to disk
    imgbuf
        .save("step6.png")
        .expect("failed to write image to disk");
}

pub fn cast_ray(
    origin: &[f32; 3],
    direction: &[f32; 3],
    objects: &Vec<Box<Renderable>>,
    lights: &Vec<Light>,
) -> [u8; 3] {
    let (hit, normal, material) = match scene_intersect(origin, direction, objects) {
        Some(val) => val,           // something was hit
        None => return [0, 53, 53], // not object was hit so background color
    };

    let mut diffuse_light_intensity = 0.0;
    let mut specular_light_intensity = 0.0;

    for light in lights {
        // get a vector from the point we hit the object to the light source
        let light_direction = normalize(&subtract(&light.position, &hit));

        // get the light distance
        let light_distance = length(&subtract(&light.position, &hit));

        // we move the origin from the hit slightly to make sure we dont colide
        // with where we hit again.
        let shadow_origin = match dot_prod(&light_direction, &normal) < 0.0 {
            true => subtract(&hit, &scalar_mult(&normal, 0.001)),
            false => add(&hit, &scalar_mult(&normal, 0.001)),
        };

        // check if the ray hits anything and then check if it is closer than
        // the light
        // this is a bit messy
        if let Some((shadow_hit, shadow_norm, _)) =
            scene_intersect(&shadow_origin, &light_direction, objects)
        {
            if length(&subtract(&shadow_hit, &shadow_origin)) < light_distance {
                continue;
            }
        };

        // add to the brightness
        diffuse_light_intensity += light.intensity * max(0.0, dot_prod(&light_direction, &normal));

        // add to the shinyness
        specular_light_intensity += max(
            0.0,
            dot_prod(
                &scalar_mult(
                    &reflect(&scalar_mult(&light_direction, -1.0), &normal),
                    -1.0,
                ),
                direction,
            ),
        )
        .powf(material.specular_exponent)
            * light.intensity;
    }

    let mut f32_color = scalar_mult(
        &[
            material.diffuse_color[0] as f32,
            material.diffuse_color[1] as f32,
            material.diffuse_color[2] as f32,
        ],
        diffuse_light_intensity,
    );
    f32_color = scalar_mult(&f32_color, material.albedo[0]);
    f32_color = add(
        &f32_color,
        &scalar_mult(
            &[255.0, 255.0, 255.0],
            specular_light_intensity * material.albedo[1],
        ),
    );

    // get the max color and if its over the value 255 make it 255
    let max_color = max(f32_color[0], max(f32_color[1], f32_color[2]));
    if max_color > 255.0 {
        f32_color = scalar_mult(&f32_color, 255.0 / max_color);
    }
    let u8_version = convert_to_u8(f32_color);
    return u8_version; // color of sphere
}

/// Scene intersect returns the None if the ray casted does not hit anything.
/// It returns Some((hit: [f32;3], normal: [f32; 3], material: Material))
/// if it hits anything. The distance is the distance from the origin passed in
/// to the point, the normal is the vector directly out of the surface where the
/// ray hit. And the material is the material of the renderable hit.
pub fn scene_intersect(
    origin: &[f32; 3],
    dir: &[f32; 3],
    renders: &Vec<Box<Renderable>>,
) -> Option<([f32; 3], [f32; 3], Material)> {
    let mut closest_model_dist = f32::MAX;
    let mut hit = [0.0, 0.0, 0.0];
    let mut normal = [0.0, 0.0, 0.0];
    let mut material = Material::from([67, 249, 85], 10.0, [0.6, 0.3]);
    for model in renders {
        let dist_i = match model.ray_intersect(origin, dir) {
            Some(dist) => dist, // the object was hit
            None => continue,   // this means we missed this object so we can move onto the next
        };
        if dist_i < closest_model_dist {
            closest_model_dist = dist_i;
            hit = add(origin, &scalar_mult(&dir, dist_i));
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
