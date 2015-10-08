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
//! ## Important Note
//! The original ``libhdfs`` implementation allows only one ``HdfsFs`` instance for the 
//! same namenode because ``libhdfs`` only keeps a single ``hdfsFs`` entry for each namenode.
//! As a result, you need to keep a singleton ``HdfsFsCache`` in an entire program, and
//! you must get ``HdfsFs`` through only ``HdfsFsCache``. For it, you need to share 
//! ``HdfsFsCache`` instance across all threads in the program. 
//! Contrast, ``HdfsFs`` instance itself is thread-safe. 
//!
//! ## Usage
//! in Cargo.toml:
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
//! and this to your crate root:
//! 
//! ```ignore
//! extern crate hdfs;
//! ```
//!
//! hdfs-rs uses libhdfs, which is JNI native implementation. JNI native implementation 
//! requires the proper ``CLASSPATH``. exec.sh included in the source code root plays a role to 
//! execute your program with the proper ``CLASSPATH``. ``exec.sh`` requires ``HADOOP_HOME``. 
//! So, you firstly set ``HADOOP_HOME`` shell environment variable as follows:
//! 
//! ```ignore
//! export HADOOP_HOME=<hadoop install dir>
//! ```
//! 
//! Then, you can execute your program as follows:
//! 
//! ```ignore
//! ./exec.sh your_program arg1 arg2
//! ```
//!
//! ## Testing
//! The test also requires the ``CLASSPATH``. So, you should run ``cargo test`` 
//! through ``exec.sh``.
//!
//! ```ignore
//! ./exec.sh cargo test
//! ```
//!
//! ## Example
//!
//! ```ignore
//! use std::rc::Rc;
//! use std::cell::RefCell;
//! use hdfs::HdfsFsCache;
//! 
//! // You must get HdfsFs instance through HdfsFsCache. Also, HdfsFsCache 
//! // must be shared across all threads in the entire program in order to
//! // avoid the thread-safe problem of the original libhdfs.
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
