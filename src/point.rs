// Copyright 982945902@qq.com.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::scalar::Scalar;

use core::ops::Mul;

pub(crate) mod C {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Point {
    point: C::point_t,
}

macro_rules! point_to_ptr {
    ($point:expr) => {
        &$point as *const [C::point_affine; 1] as *mut C::point_affine
    };
}

impl Point {
    pub fn from_hash(bytes: &[u8]) -> Point {
        let point = [C::point_affine::default()];
        unsafe {
            C::HashToCurve(bytes.as_ptr() as *mut [u64; 2], point_to_ptr!(point));
        }
        Point { point }
    }

    pub fn encode(&self, bytes: &mut [u8]) {
        unsafe { C::encode(point_to_ptr!(self.point), bytes.as_mut_ptr()) }
    }

    pub fn decode(bytes: &mut [u8]) -> Self {
        let point = [C::point_affine::default()];

        unsafe {
            C::decode(bytes.as_mut_ptr(), point_to_ptr!(point));
        }

        Point { point }
    }
}

impl Mul<Scalar> for Point {
    type Output = Point;

    fn mul(self, scalar: Scalar) -> Point {
        let result = [C::point_affine::default()];

        unsafe {
            C::ecc_mul(
                point_to_ptr!(self.point),
                scalar.bytes.as_ptr() as *mut u64,
                point_to_ptr!(result),
                false,
            );
        }

        Point { point: result }
    }
}

impl Mul<Point> for Scalar {
    type Output = Point;

    fn mul(self, point: Point) -> Point {
        point * self
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        let mut self_bytes = [0u8; 32];
        let mut other_bytes = [0u8; 32];

        self.encode(&mut self_bytes);
        other.encode(&mut other_bytes);

        self_bytes == other_bytes
    }
}

impl Eq for Point {}
