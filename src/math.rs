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

pub fn convert_to_u8(v: [f32; 3]) -> [u8; 3] {
    [v[0] as u8, v[1] as u8, v[2] as u8]
}

pub fn reflect(i: &[f32; 3], n: &[f32; 3]) -> [f32; 3] {
    subtract(i, &scalar_mult(n, dot_prod(i, n) * 2.0))
}
