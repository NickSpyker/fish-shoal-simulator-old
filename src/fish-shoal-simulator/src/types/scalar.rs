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
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Range, Sub, SubAssign};

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct Scalar {
    pub value: f32,
}

impl Scalar {
    pub const ZERO: Self = Self { value: 0.0 };
    pub const ONE: Self = Self { value: 1.0 };

    #[inline]
    pub fn new(value: f32) -> Self {
        Self { value }
    }

    pub fn new_random(rng: &mut impl Rng, range: Range<f32>) -> Self {
        Self {
            value: rng.random_range(range),
        }
    }

    #[inline]
    pub fn abs(self) -> Self {
        Self {
            value: self.value.abs(),
        }
    }

    #[inline]
    pub fn min(self, other: Self) -> Self {
        Self {
            value: self.value.min(other.value),
        }
    }

    #[inline]
    pub fn max(self, other: Self) -> Self {
        Self {
            value: self.value.max(other.value),
        }
    }

    #[inline]
    pub fn clamp(&mut self, min: Self, max: Self) {
        self.value = self.value.clamp(min.value, max.value);
    }

    #[inline]
    pub fn lerp(self, to: Self, t: f32) -> Self {
        let t: f32 = t.clamp(0.0, 1.0);
        Self {
            value: self.value + (to.value - self.value) * t,
        }
    }

    #[inline]
    pub fn squared(self) -> Self {
        Self {
            value: self.value * self.value,
        }
    }

    #[inline]
    pub fn sqrt(self) -> Self {
        Self {
            value: self.value.sqrt(),
        }
    }
}

impl From<f32> for Scalar {
    #[inline]
    fn from(value: f32) -> Self {
        Self { value }
    }
}

impl From<Scalar> for f32 {
    #[inline]
    fn from(scalar: Scalar) -> Self {
        scalar.value
    }
}

impl Add for Scalar {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value + rhs.value,
        }
    }
}

impl AddAssign for Scalar {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}

impl Sub for Scalar {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value - rhs.value,
        }
    }
}

impl SubAssign for Scalar {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value;
    }
}

impl Mul<f32> for Scalar {
    type Output = Self;

    #[inline]
    fn mul(self, scalar: f32) -> Self::Output {
        Self {
            value: self.value * scalar,
        }
    }
}

impl MulAssign<f32> for Scalar {
    #[inline]
    fn mul_assign(&mut self, scalar: f32) {
        self.value *= scalar;
    }
}

impl Div<f32> for Scalar {
    type Output = Self;

    #[inline]
    fn div(self, scalar: f32) -> Self::Output {
        Self {
            value: self.value / scalar,
        }
    }
}

impl DivAssign<f32> for Scalar {
    #[inline]
    fn div_assign(&mut self, scalar: f32) {
        self.value /= scalar;
    }
}

impl Neg for Scalar {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self { value: -self.value }
    }
}

#[cfg(test)]
mod tests {
    use super::Scalar;
    use rand::{rngs::StdRng, SeedableRng};

    #[test]
    fn scalar_ops() {
        let mut scalar: Scalar = Scalar::new(10.0);

        assert_eq!(scalar + Scalar::new(5.0), Scalar::new(15.0));

        assert_eq!(scalar - Scalar::new(5.0), Scalar::new(5.0));

        assert_eq!(scalar * 2.0, Scalar::new(20.0));

        scalar += Scalar::new(2.0);
        assert_eq!(scalar, Scalar::new(12.0));
    }

    #[test]
    fn scalar_math() {
        let scalar_a: Scalar = Scalar::new(-5.0);
        let scalar_b: Scalar = Scalar::new(10.0);

        assert_eq!(scalar_a.abs(), Scalar::new(5.0));
        assert_eq!(scalar_a.max(scalar_b), scalar_b);
        assert_eq!(scalar_a.min(scalar_b), scalar_a);

        let squared: Scalar = Scalar::new(4.0).squared();
        assert_eq!(squared, Scalar::new(16.0));

        let root: Scalar = squared.sqrt();
        assert_eq!(root, Scalar::new(4.0));
    }

    #[test]
    fn scalar_lerp() {
        let start: Scalar = Scalar::new(0.0);
        let end: Scalar = Scalar::new(100.0);

        let mid: Scalar = start.lerp(end, 0.5);
        assert_eq!(mid, Scalar::new(50.0));

        let clamped_low: Scalar = start.lerp(end, -0.5);
        assert_eq!(clamped_low, start);
    }

    #[test]
    fn scalar_random() {
        let mut rng: StdRng = StdRng::seed_from_u64(42);

        let scalar: Scalar = Scalar::new_random(&mut rng, 0.0f32..1.0f32);

        assert!(scalar.value >= 0.0 && scalar.value < 1.0);
    }
}
