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
//!
//! ## Usage
//! in Cargo.toml
//!
//! ```ignore
//! [dependencies]
//! hdfs = "0.0.4"
//! ```
//! or
//!
//! ```ignore
//! [dependencies.hdfs]
//! git = "https://github.com/hyunsik/hdfs-rs.git"
//! ```
//! 
//! Then, in your source code.
//! 
//! ```ignore
//! extern crate hdfs;
//! ```
//!
//! ## Example
//!
//! ```ignore
//! use std::rc::Rc;
//! use std::cell::RefCell;
//! use hdfs::HdfsFsCache;
//!
//! // You should get HdfsFs through HdfsFsCache in order to guarantee the thread safety.
//! // HdfsFs itself is thread-safe, but the original libhdfs allows only one HdfsFs instance 
//! // for the same namenode.
//! let cache = Rc::new(RefCell::new(HdfsFsCache::new()));  
//! let fs: HdfsFs = cache.borrow_mut().get("hdfs://localhost:8020/").ok().unwrap();
//! match fs.mkdir("/data") {
//!   Ok(_) => { println!("/data has been created") },
//!   Err(_)  => { panic!("/data creation has failed") }
//! }; 
//! ```

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
