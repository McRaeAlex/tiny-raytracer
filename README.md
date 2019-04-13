# Tiny Raytacer
This is a implementation of a ray tracer in rust based off of ssloy's.
Each step will contain my code written for that step.

## Step 1:

This step just gets writing a image to work. I found the rust version to be much
more readable than the c++ version.

```rust
extern crate image;

fn render() {
    let width = 1024;
    let height = 768;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let red = ((x as f32 / width as f32) * 255.0) as u8;
        let green = ((y as f32 / height as f32) * 255.0) as u8;
        *pixel = image::Rgb([red, green, 0]);
    }

    imgbuf.save("oneframe.png").expect("Could not write image");
}

fn main() {
    render();
}
```

### Result
![Result of step 1](step1.png)

## Step 2:

This part was really fun and frusterating for a bit. Once I understood the math
I felt very comfortable with it and it just made sense. What I think is really 
cool is I felt I could implement a rendering engine from this using traits in 
rust.

### Result
![Result from step 2](step2.png)

## Step 3:

This part was easy, one thing I noticed is that the tutorial checks each object 
to see if there is a collision and then after checks which one should be shown.
I think, however am not sure, that we could just sort the items by distance from
camera and then break as soon as we see the first one. I'm sure this has already
been though of and I might find out later its dumb but I wonder how big of a 
difference, if any it would make.

extern crate image;

mod math;

use std::f32;

use math::*;

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

fn cast_ray(origin: &[f32; 3], dir: &[f32; 3], spheres: &Vec<Sphere>) -> [u8; 3] {
    let mut point = [0.0, 0.0, 0.0];
    let mut n = [0.0, 0.0, 0.0];
    let mut m = Material::new([0, 0, 0]);

    if !scene_intersect(origin, dir, spheres, &mut point, &mut n, &mut m) {
        return [0, 51, 0]; // background color
    }
    return m.diffuse_color; // color of sphere
}

fn render(spheres: &Vec<Sphere>) {
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
        *pixel = image::Rgb(cast_ray(&[0.0, 0.0, 0.0], &dir, spheres));
    }

    imgbuf.save("step3.png").expect("Could not write image");
}

fn main() {
    let s = vec![
        Sphere::new([-3.0, 0.0, -16.0], 2.0, Material::new([255, 255, 0])),
        Sphere::new([-1.0, -1.5, -12.0], 2.0, Material::new([76, 25, 25])),
        Sphere::new([1.5, -0.5, -18.0], 3.0, Material::new([76, 25, 25])),
        Sphere::new([7.0, 5.0, -18.0], 4.0, Material::new([255, 255, 0])),
    ];
    render(&s);
}
```

### Result:
![Result from step 3](step3.png)

## Step 4:

This was a big step for me and made me realize why a game might not use this.
Still its pretty cool and fun to write. One issue is that if the color is 
already white then adding light to make it more light makes it black. I know 
how I could fix it to not round but I feel like there is a better way.



### Result:
![Result from step 4](step4.png)

## Step 5:
