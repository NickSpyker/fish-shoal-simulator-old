/*
 * Copyright 2025 Nicolas Spijkerman
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use rand::Rng;
use std::{
    f32::consts::TAU,
    fmt::{self, Display, Formatter},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Range, Sub, SubAssign},
};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub const ONE: Self = Self { x: 1.0, y: 1.0 };

    #[inline]
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn new_random(rng: &mut impl Rng, range_x: Range<f32>, range_y: Range<f32>) -> Self {
        Self {
            x: rng.random_range(range_x),
            y: rng.random_range(range_y),
        }
    }

    #[inline]
    pub fn random_dir(rng: &mut impl Rng) -> Self {
        let angle: f32 = rng.random_range(0.0..TAU);
        let (sin, cos): (f32, f32) = angle.sin_cos();
        Self { x: cos, y: sin }
    }

    #[inline]
    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    #[inline]
    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    #[inline]
    pub fn distance(self, other: Self) -> f32 {
        (self - other).length()
    }

    #[inline]
    pub fn distance_squared(self, other: Self) -> f32 {
        (self - other).length_squared()
    }

    #[inline]
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    #[inline]
    pub fn lerp(self, to: Self, t: f32) -> Self {
        let t: f32 = t.clamp(0.0, 1.0);
        self + (to - self) * t
    }

    #[inline]
    pub fn normalized(self) -> Self {
        let len: f32 = self.length();
        if len > 0.0 { self / len } else { Self::ZERO }
    }

    #[inline]
    pub fn normalize(&mut self) {
        let len: f32 = self.length();
        if len > 0.0 {
            *self /= len;
        }
    }

    #[inline]
    pub fn clamp(&mut self, min: Self, max: Self) {
        self.x = self.x.clamp(min.x, max.x);
        self.y = self.y.clamp(min.y, max.y);
    }
}

impl Display for Vec2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{{{}, {}}}", self.x, self.y)
    }
}

impl From<[f32; 2]> for Vec2 {
    #[inline]
    fn from(arr: [f32; 2]) -> Self {
        Self {
            x: arr[0],
            y: arr[1],
        }
    }
}

impl From<(f32, f32)> for Vec2 {
    #[inline]
    fn from(tuple: (f32, f32)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl From<Vec2> for [f32; 2] {
    #[inline]
    fn from(v: Vec2) -> Self {
        [v.x, v.y]
    }
}

impl From<Vec2> for (f32, f32) {
    #[inline]
    fn from(v: Vec2) -> Self {
        (v.x, v.y)
    }
}

impl Add for Vec2 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vec2 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vec2 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Vec2 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    #[inline]
    fn mul(self, scalar: f32) -> Self::Output {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl MulAssign<f32> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    #[inline]
    fn div(self, scalar: f32) -> Self::Output {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl DivAssign<f32> for Vec2 {
    #[inline]
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
    }
}

impl Neg for Vec2 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Vec2;
    use rand::{rngs::StdRng, SeedableRng};

    #[test]
    fn vec2_ops() {
        let mut vec: Vec2 = Vec2::new(1.0, 2.0);
        assert_eq!(vec + Vec2::new(3.0, 4.0), Vec2::new(4.0, 6.0));
        assert_eq!(vec * 2.0, Vec2::new(2.0, 4.0));
        vec += Vec2::new(1.0, 1.0);
        assert_eq!(vec, Vec2::new(2.0, 3.0));
    }

    #[test]
    fn vec2_math() {
        let vec_a: Vec2 = Vec2::new(3.0, 0.0);
        let vec_b: Vec2 = Vec2::new(0.0, 4.0);
        assert_eq!(vec_a.length(), 3.0);
        assert_eq!(vec_a.distance(vec_b), 5.0);
        assert_eq!(vec_a.dot(vec_b), 0.0);
    }

    #[test]
    fn vec2_random() {
        let mut rng: StdRng = StdRng::seed_from_u64(42);

        let vec: Vec2 = Vec2::new_random(&mut rng, 0.0f32..10.0f32, 0.0f32..10.0f32);

        assert!(vec.x >= 0.0 && vec.x < 10.0);
        assert!(vec.y >= 0.0 && vec.y < 10.0);
    }
}
