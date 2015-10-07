//! MiniDfs Cluster
//!
//! MiniDFS provides a embedded HDFS cluster. It is usually for testing.
//! 
//! ## Example
//!
//! ```ignore
//!  let mut conf = MiniDfsConf::new();
//!  let dfs = MiniDFS::start(&mut conf).unwrap();
//!  let port = dfs.namenode_port();
//!  ...
//!  dfs.stop()
//! ```

use libc::{c_char, c_int};
use std::ffi;
use std::mem;
use std::str;

use native::*;

pub struct MiniDFS 
{
  cluster: *const NativeMiniDfsCluster
}

impl MiniDFS 
{
  pub fn start(conf: &MiniDfsConf) -> Option<MiniDFS> 
  {
    match unsafe { nmdCreate(conf) } {
      val if !val.is_null() => Some(MiniDFS { cluster: val }),
      _ => None      
    }
  }

  pub fn stop(&self) 
  {
    unsafe {
      nmdShutdown(self.cluster);
      nmdFree(self.cluster);
    }
  }

  pub fn wait_for_clusterup(&self) -> bool 
  {
    if unsafe { nmdWaitClusterUp(self.cluster) } == 0 { true } else { false }
  }

  pub fn namenode_port(&self) -> Option<i32> 
  {
    match unsafe { nmdGetNameNodePort(self.cluster) as i32 } {
      val if val > 0 => Some(val),
      _ => None
    }
  }

  pub fn namenode_http_addr(&self) -> Option<(&str, i32)> 
  {
    let mut hostname: *mut c_char = unsafe {mem::zeroed()};
    let mut port: c_int = 0;

    match unsafe { 
      nmdGetNameNodeHttpAddress(self.cluster, &mut port, &mut hostname) 
    } {
      0 => {
        let slice = unsafe { ffi::CStr::from_ptr(hostname) }.to_bytes();
        let str = str::from_utf8(slice).unwrap();
        
        Some((str, port as i32))
      },
      _ => None
    }
  }

  pub fn set_hdfs_builder(&self, builder: *mut hdfsBuilder) -> bool 
  {
    if unsafe { nmdConfigureHdfsBuilder(self.cluster, builder) } == 0 
    { true } else { false }
  }
}