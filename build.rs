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

use bindgen::builder;
use std::{env, path::PathBuf, process::Command};

fn main() {
    Command::new("cmake")
        .current_dir("./c")
        .args(["-S.", "-Bbuild"])
        .output()
        .expect("failed to execute process");

    Command::new("cmake")
        .current_dir("./c")
        .args(["--build", "build"])
        .output()
        .expect("failed to execute process");

    println!("cargo:rustc-link-lib=static=apsi");
    println!("cargo:rustc-link-search=native=./c/build");

    let bindings = bindgen::Builder::default()
        .header("./c/fourq/FourQ.h")
        .header("./c/fourq/FourQ_api.h")
        .header("./c/fourq/Fourq_define.h")
        .header("./c/fourq/Fourq_internal.h")
        .clang_arg("-I./c")
        .derive_default(true)
        .generate()
        .expect("failed to generate");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
