// TODO: Redo math library

pub fn dot_prod(u: &[f32; 3], v: &[f32; 3]) -> f32 {
    u[0] * v[0] + u[1] * v[1] + u[2] * v[2]
}

pub fn normalize(u: &[f32; 3]) -> [f32; 3] {
    // create a unit vector;
    let magnitude = dot_prod(u, u).sqrt();
    [u[0] / magnitude, u[1] / magnitude, u[2] / magnitude]
}

pub fn scalar_mult(u: &[f32; 3], t: f32) -> [f32; 3] {
    [u[0] * t, u[1] * t, u[2] * t]
}

pub fn scalar_mult_u8(u: &[u8; 3], t: f32) -> [f32; 3] {
    [u[0] as f32 * t, u[1] as f32 * t, u[2] as f32 * t]
}

pub fn add(u: &[f32; 3], v: &[f32; 3]) -> [f32; 3] {
    [u[0] + v[0], u[1] + v[1], u[2] + v[2]]
}

pub fn subtract(u: &[f32; 3], v: &[f32; 3]) -> [f32; 3] {
    [u[0] - v[0], u[1] - v[1], u[2] - v[2]]
}

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

pub fn convert_to_u8(v: [f32; 3]) -> [u8; 3] {
    [v[0] as u8, v[1] as u8, v[2] as u8]
}

pub fn reflect(i: &[f32; 3], n: &[f32; 3]) -> [f32; 3] {
    subtract(i, &scalar_mult(n, dot_prod(i, n) * 2.0))
}

pub fn length(u: &[f32; 3]) -> f32 {
    let squared = u[0].powi(2) + u[1].powi(2) + u[2].powi(2);
    squared.sqrt()
}

pub fn refract(i: &[f32;3], n: &[f32; 3], refractive_index: f32) -> [f32; 3] {
    let mut cosi = -1.0 * max(-1.0, min(1.0, dot_prod(i, n)));
    let mut etai = 1.0;
    let mut etat = refractive_index;
    let normal = match cosi < 0.0 {
        true => {
            cosi = -1.0 * cosi;
            let temp = etat;
            etat = etai;
            etai = temp;
            scalar_mult(n, -1.0)
        },
        false => [n[0], n[1], n[2]],
    };
    let eta = etai/etat;
    let k = 1.0 - eta *eta * (1.0 - cosi*cosi);
    match k < 0.0 {
        true => [0.0,0.0,0.0],
        false => {
            add(&scalar_mult(i, eta), &scalar_mult(&normal, eta * cosi - k.sqrt()))
        },
    }
}
