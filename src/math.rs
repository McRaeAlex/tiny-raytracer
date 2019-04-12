pub fn dot_prod(u: &[f32; 3], v: &[f32; 3]) -> f32 {
    u[0] * v[0] + u[1] * v[1] + u[2] * v[2]
}

pub fn normalize(u: &[f32; 3]) -> [f32; 3] {
    // create a unit vector;
    let magnitude = dot_prod(u, u).sqrt();
    [u[0] / magnitude, u[1] / magnitude, u[2] / magnitude]
}
