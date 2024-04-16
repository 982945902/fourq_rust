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

#[derive(Debug, Clone, Copy, Default)]
pub struct Scalar {
    pub(crate) bytes: [u8; 32],
}

impl Scalar {
    pub fn new(bytes: [u8; 32]) -> Scalar {
        Scalar { bytes }
    }
}

impl From<[u8; 32]> for Scalar {
    fn from(bytes: [u8; 32]) -> Scalar {
        Scalar { bytes }
    }
}

impl From<Scalar> for [u8; 32] {
    fn from(scalar: Scalar) -> [u8; 32] {
        scalar.bytes
    }
}
