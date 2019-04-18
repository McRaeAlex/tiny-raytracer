extern crate image;
extern crate tiny_raytracer_lib;

use tiny_raytracer_lib::{rendering::*, *};

fn main() {
    let s: Vec<Box<Renderable>> = vec![
        Box::new(shapes::Sphere::from(
            [-3.0, 0.0, -16.0],
            2.0,
            effects::Material::from([102, 102, 76], 50.0, [0.6, 0.3, 0.1, 0.0], 1.0),
        )),
        Box::new(shapes::Sphere::from(
            [-1.0, -1.5, -12.0],
            2.0,
            effects::Material::from([153, 179, 204], 125.0, [0.0, 0.5, 0.1, 0.8], 1.5),
        )),
        Box::new(shapes::Sphere::from(
            [1.5, -0.5, -18.0],
            3.0,
            effects::Material::from([76, 25, 25], 10.0, [0.9, 0.1, 0.0, 0.0], 1.0),
        )),
        Box::new(shapes::Sphere::from(
            [7.0, 5.0, -18.0],
            4.0,
            effects::Material::from([255, 255, 255], 1425.0, [0.0, 10.0, 0.8, 0.0], 1.0),
        )),
        /*Box::new(shapes::Plane::new(
            [0.0, -4.0, 0.0],
            [0.0, -0.1, 0.0],
            effects::Material::from([255, 255, 10], 10.0, [0.6, 0.3, 0.1, 0.0], 1.0),
        )),*/
    ];

    let l = vec![
        effects::Light::new([20.0, 20.0, 20.0], 1.5),
        effects::Light::new([30.0, 50.0, -25.0], 1.8),
        effects::Light::new([30.0, 20.0, 30.0], 1.7),
    ];
    render(&s, &l);
}
