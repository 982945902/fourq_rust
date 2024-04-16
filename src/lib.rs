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

pub mod scalar;

pub mod point;

#[cfg(test)]
mod tests {
    use crate::point::Point;
    use crate::scalar::Scalar;
    use rand::{thread_rng, Rng};
    #[test]
    fn it_works() {
        let scalar = Scalar::new(thread_rng().gen());
        let point = Point::from_hash(&scalar.bytes);

        let a = scalar * point;
        let b = point * scalar;
        assert!(a == b);
    }

    #[test]
    fn ecdh_test() {
        let p1_key = Scalar::new(thread_rng().gen());
        let p2_key = Scalar::new(thread_rng().gen());

        let example_id: [u8; 32] = thread_rng().gen();

        let p1_example = example_id;
        let p2_example = example_id;

        let p1_example_point = Point::from_hash(&p1_example);
        let p2_example_point = Point::from_hash(&p2_example);

        let p1_ecc_k = p1_key * p1_example_point * p2_key;
        let p2_ecc_k = p2_key * p2_example_point * p1_key;

        assert!(p1_ecc_k == p2_ecc_k);
    }
}
