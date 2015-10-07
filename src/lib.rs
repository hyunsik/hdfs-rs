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
//! Basically, this library provides libhdfs APIs bindings.
//! It also provides more idiomatic and abstract Rust APIs, 
//! hiding manual memory management and some thread-safety problem of libhdfs.
//! Rust APIs are highly recommended for most users.

extern crate libc;
#[macro_use]
extern crate log;
extern crate url;

/// libhdfs binding APIs
pub mod binding;

/// Rust APIs wrapping libhdfs API, providing better semantic and abstraction
pub mod dfs;

/// Mini HDFS Cluster for easily building unit tests
pub mod minidfs;

pub mod util;
