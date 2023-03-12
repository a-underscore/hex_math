#[derive(Default, Copy, Clone)]
pub struct Ortho(pub [[f32; 4]; 4]);

impl Ortho {
    pub fn new(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        Self([
            [2.0 / (right - left), 0.0, 0.0, 0.0],
            [0.0, 2.0 / (top - bottom), 0.0, 0.0],
            [0.0, 0.0, -2.0 / (far - near), 0.0],
            [
                -(right + left) / (right - left),
                -(top + bottom) / (top - bottom),
                -(far + near) / (far - near),
                1.0,
            ],
        ])
    }
}
