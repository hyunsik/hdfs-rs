// Copyright 2015 Hyunsik Choi
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! hdfs-rs is a library for accessing to HDFS cluster. 
//! Basically, it provides libhdfs FFI APIs.
//! It also provides more idiomatic and abstract Rust APIs, 
//! hiding manual memory management and some thread-safety problem of libhdfs.
//! Rust APIs are highly recommended for most users.

#[macro_use] extern crate itertools;
extern crate libc;
#[macro_use] extern crate log;
extern crate url;

mod err;
pub use err::HdfsErr;

/// libhdfs native binding APIs
pub mod native;

/// Rust APIs wrapping libhdfs API, providing better semantic and abstraction
mod dfs;
pub use dfs::*;

/// Mini HDFS Cluster for easily building unit tests
pub mod minidfs;

mod util;
pub use util::HdfsUtil;
