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

//! [![OpenCL 3.0](https://img.shields.io/badge/OpenCL-3.0-blue.svg)](https://www.khronos.org/registry/OpenCL/)
//! [![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
//! ![Rust](https://github.com/kenba/cl3/workflows/Rust/badge.svg)
//!
//! OpenCL C FFI Bindings.
//!
//! # Description
//!
//! Rust [FFI](https://doc.rust-lang.org/nomicon/ffi.html) Bindings to the
//! Khronos OpenCL 3.0 C language headers located at:
//! [https://github.com/KhronosGroup/OpenCL-Headers](https://github.com/KhronosGroup/OpenCL-Headers)
//!
//! The Rust FFI files attempt to match the layout and format of the original
//! C source files instead of the `bindgen` output files to ease maintenance.
//!
//! [OpenCL 3.0](https://www.khronos.org/registry/OpenCL/specs/3.0-unified/html/OpenCL_API.html)
//! is a unified specification that adds little new functionality to previous OpenCL versions.  
//! It specifies that all **OpenCL 1.2** features are **mandatory**, while all
//! OpenCL 2.x and 3.0 features are now optional.
//!
//! ## License
//!
//! Licensed under the Apache License, Version 2.0, as per Khronos Group OpenCL.  
//! You may obtain a copy of the License at: <http://www.apache.org/licenses/LICENSE-2.0>
//!
//! OpenCL and the OpenCL logo are trademarks of Apple Inc. used under license by Khronos.

extern crate libc;

mod cl;
mod cl_ext;
mod cl_platform;

pub use self::cl::*;
pub use self::cl_ext::*;
pub use self::cl_platform::*;

pub mod cl_d3d10;
pub mod cl_d3d11;
pub mod cl_dx9_media_sharing;
pub mod cl_egl;
pub mod cl_gl;
