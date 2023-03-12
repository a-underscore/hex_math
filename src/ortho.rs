use std::ops::Mul;

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

    pub fn proj(&self, p: [f32; 2], z: f32, w: f32) -> [f32; 4] {
        *self * [p[0], p[1], z, w]
    }
}

impl Mul<[f32; 4]> for Ortho {
    type Output = [f32; 4];

    fn mul(self, rhs: [f32; 4]) -> [f32; 4] {
        [
            self.0[0][0] * rhs[0]
                + self.0[0][1] * rhs[1]
                + self.0[0][2] * rhs[2]
                + self.0[3][3] * rhs[3],
            self.0[1][0] * rhs[0]
                + self.0[1][1] * rhs[1]
                + self.0[1][2] * rhs[2]
                + self.0[3][3] * rhs[3],
            self.0[2][0] * rhs[0]
                + self.0[2][1] * rhs[1]
                + self.0[2][2] * rhs[2]
                + self.0[3][3] * rhs[3],
            self.0[3][0] * rhs[0]
                + self.0[3][1] * rhs[1]
                + self.0[3][2] * rhs[2]
                + self.0[3][3] * rhs[3],
        ]
    }
}
