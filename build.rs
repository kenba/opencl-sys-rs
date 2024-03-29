// Copyright (c) 2022 Via Technology Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Add an OpenCL ICD library path to the rust library search path on Windows.
//
// Searches for Intel, AMD and Nvidia OpenCL vendor SDK environment variables
// or OPENCL_ROOT environment variable, if an OpenCL vendor SDK environment variable
// was not found.
fn main() {
    if cfg!(windows) {
        let known_sdk = [
            // E.g. "C:\Intel\OpenCL\sdk\lib\x64\"
            ("INTELOCLSDKROOT", "x64", "x86"),
            // E.g. "C:\Program Files (x86)\AMD APP SDK\3.0\lib\x86_64\"
            ("AMDAPPSDKROOT", "x86_64", "x86"),
            // E.g. "c:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v8.0\lib\Win32\"
            ("CUDA_PATH", "x64", "Win32"),
            // E.g. "$OPENCL_ROOT\lib\x64\"
            ("OPENCL_ROOT", "x64", "x86"),
        ];

        for info in &known_sdk {
            if let Ok(sdk) = std::env::var(info.0) {
                let mut path = std::path::PathBuf::from(sdk);
                path.push("lib");
                path.push(if cfg!(target_arch = "x86_64") {
                    info.1
                } else {
                    info.2
                });
                println!("cargo:rustc-link-search=native={}", path.display());
                break;
            }
        }
    }
}
