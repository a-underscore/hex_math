use std::{
    f32::consts::PI,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Default, PartialEq, PartialOrd, Copy, Clone)]
pub struct Vec2(pub [f32; 2]);

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self([x, y])
    }

    pub fn extend(&self, z: f32) -> [f32; 3] {
        [self.x(), self.y(), z]
    }

    pub fn trunc(p: &[f32; 3]) -> Self {
        Self::new(p[0], p[1])
    }

    pub fn x(&self) -> f32 {
        self.0[0]
    }

    pub fn y(&self) -> f32 {
        self.0[1]
    }

    pub fn x_ref(&self) -> &f32 {
        &self.0[0]
    }

    pub fn y_ref(&self) -> &f32 {
        &self.0[1]
    }

    pub fn x_mut(&mut self) -> &mut f32 {
        &mut self.0[0]
    }

    pub fn y_mut(&mut self) -> &mut f32 {
        &mut self.0[1]
    }

    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x() * rhs.x() + self.y() * rhs.y()
    }

    pub fn magnitude(&self) -> f32 {
        (self.x().powi(2) + self.y().powi(2)).sqrt()
    }

    pub fn angle(&self, other: &Self) -> f32 {
        let v = self.dot(other) / (self.magnitude() * other.magnitude());
        let angle = v.acos();

        if v >= 1.0 {
            angle - PI
        } else {
            angle
        }
    }

    pub fn normal(&self) -> Self {
        *self / self.magnitude()
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x() + rhs.x(), self.y() + rhs.y())
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self::add(*self, rhs);
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self::new(self.x() * rhs, self.y() * rhs)
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Self::mul(*self, rhs);
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.mul(-1.0)
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::add(self, rhs)
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self::sub(*self, rhs);
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self::new(self.x() / rhs, self.y() / rhs)
    }
}

impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, rhs: f32) {
        *self = Self::div(*self, rhs);
    }
}
