use cgmath::prelude::*;
use cgmath::*;

pub fn max(v1: f32, v2: f32) -> f32 {
    if v2 > v1 {
        return v2;
    }
    return v1;
}

pub fn min(v1: f32, v2: f32) -> f32 {
    if v2 > v1 {
        return v1;
    }
    return v2;
}

pub fn reflect(i: &Vector3<f32>, n: &Vector3<f32>) -> Vector3<f32> {
    i - (n * dot(*i, *n) * 2.0)
}

pub fn refract(i: &Vector3<f32>, n: &Vector3<f32>, refractive_index: f32) -> Vector3<f32> {
    let mut cosi = -1.0 * max(-1.0, min(1.0, dot(*i, *n)));
    let mut etai = 1.0;
    let mut etat = refractive_index;
    let normal = match cosi < 0.0 {
        true => {
            cosi = -1.0 * cosi;
            let temp = etat;
            etat = etai;
            etai = temp;
            n * -1.0
        }
        false => n * 1.0,
    };
    let eta = etai / etat;
    let k = 1.0 - eta * eta * (1.0 - cosi * cosi);
    match k < 0.0 {
        true => Vector3::new(0.0, 0.0, 0.0),
        false => i * eta + normal * (eta * cosi - k.sqrt()),
    }
}
