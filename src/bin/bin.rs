extern crate cgmath;
extern crate image;
extern crate tiny_raytracer_lib;

use cgmath::*;

use tiny_raytracer_lib::{rendering::*, *};

fn main() {
    let ivory = effects::Material::new(
        Vector3::new(0.4, 0.4, 0.3),
        50.0,
        Vector4::new(0.6, 0.3, 0.1, 0.0),
        1.0,
    );
    let glass = effects::Material::new(
        Vector3::new(0.6, 0.7, 0.8),
        125.0,
        Vector4::new(0.0, 0.5, 0.1, 0.8),
        1.5,
    );
    let red_rubber = effects::Material::new(
        Vector3::new(0.3, 0.1, 0.1),
        10.0,
        Vector4::new(0.9, 0.1, 0.0, 0.0),
        1.0,
    );
    let mirror = effects::Material::new(
        Vector3::new(1.0, 1.0, 1.0),
        1425.0,
        Vector4::new(0.0, 10.0, 0.8, 0.0),
        1.0,
    );

    let s: Vec<Box<Renderable>> = vec![
        Box::new(shapes::Sphere::new(
            Vector3::new(-3.0, 0.0, -16.0),
            2.0,
            ivory,
        )),
        Box::new(shapes::Sphere::new(
            Vector3::new(-1.0, -1.5, -12.0),
            2.0,
            glass,
        )),
        Box::new(shapes::Sphere::new(
            Vector3::new(1.5, -0.5, -18.0),
            3.0,
            red_rubber,
        )),
        Box::new(shapes::Sphere::new(
            Vector3::new(7.0, 5.0, -18.0),
            4.0,
            mirror,
        )),
        /*Box::new(shapes::Plane::new(
            [0.0, -4.0, 0.0],
            [0.0, -0.1, 0.0],
            effects::Material::from([255, 255, 10], 10.0, [0.6, 0.3, 0.1, 0.0], 1.0),
        )),*/
    ];

    let l = vec![
        effects::Light::new(Vector3::new(20.0, 20.0, 20.0), 1.5),
        effects::Light::new(Vector3::new(30.0, 50.0, -25.0), 1.8),
        effects::Light::new(Vector3::new(30.0, 20.0, 30.0), 1.7),
    ];
    render(&s, &l);
}
