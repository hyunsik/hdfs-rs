use bindings::*;

use libc::{c_char, c_int};
use std::ffi;
use std::mem;
use std::str;

pub struct MiniDFS {
  cluster: *mut NativeMiniDfsCluster
}

impl MiniDFS {
  pub fn start(conf: &mut MiniDfsConf) -> Option<MiniDFS> {

    match unsafe { nmdCreate(conf) } {
      val if !val.is_null() => Some(MiniDFS { cluster: val }),
      _ => None      
    }
  }

  pub fn stop(&self) {
    unsafe {
      nmdShutdown(self.cluster);
      nmdFree(self.cluster);
    }
  }

  pub fn wait_for_clusterup(&self) -> bool {
    if unsafe { nmdWaitClusterUp(self.cluster) } == 0 { true } else { false }
  }

  pub fn namenode_port(&self) -> Option<i32> {
    match unsafe { nmdGetNameNodePort(self.cluster) as i32 } {
      val if val > 0 => Some(val),
      _ => None
    }
  }

  pub fn namenode_http_addr(&self) -> Option<(&str, i32)> {
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

  pub fn set_hdfs_builder(&self, builder: *mut HdfsBuilder) -> bool {
    if unsafe { nmdConfigureHdfsBuilder(self.cluster, builder) } == 0 
    { true } else { false }
  }
}

#[test]
fn test_minidfs() {
  let mut conf = MiniDfsConf::new();
  let dfs = MiniDFS::start(&mut conf).unwrap();

  let port = dfs.namenode_port();
  assert!(port.unwrap() > 0);

  let http_addr = dfs.namenode_http_addr();
  assert_eq!("localhost", http_addr.unwrap().0);
  assert!(http_addr.unwrap().1 > 0);

  let mut builder = unsafe { hdfsNewBuilder() };
  let fs = unsafe { hdfsBuilderConnect(builder) };
  assert!(!fs.is_null());
  unsafe { hdfsDisconnect(fs); }

  dfs.stop();
}