use std::collections::HashMap;
use std::string::String;
use std::ffi::CString;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use url::{Url, ParseError};
use bindings::*;

pub static LOCALFS_SCHEME: &'static str = "file";

pub struct HdfsCache {
  fs_map: HashMap<String, HdfsConnection>,
  lock: Arc<Mutex<i32>>,
  //_marker: PhantomData<&'a ()>
}

impl HdfsCache {
  pub fn get_default_fs() -> Option<HdfsConnection> { 
    None   
  }

  pub fn get(&mut self, path: &str) -> Option<&HdfsConnection> {   

    let mut namenode_uri = String::new();

    match Url::parse(path) {
      Ok(url) => {        

          if url.scheme.as_str() == LOCALFS_SCHEME { 
            namenode_uri.push_str("file:///");
          } else {

            if url.domain().is_some() {
              namenode_uri.push_str(url.scheme.as_str());
              namenode_uri.push_str("://");
              namenode_uri.push_str(url.domain().unwrap());

              // if url.port().is_some() {                
              //   namenode_uri.push_str(url.port().unwrap());
              // }
            } else {
              error!("No hostname is given: {}", path);              
            }
          }
      },
			Err(_) => {
        error!("Path parsing failed: {}", path);
      }
    };

    let lock_guard = self.lock.lock();        

    if !self.fs_map.contains_key(&namenode_uri) {
      unsafe {
        let mut hdfs_builder = hdfsNewBuilder();
        let namenode_uri_bytes = namenode_uri.into_bytes();
        let namenode_cstr = CString::new(namenode_uri_bytes).unwrap();
        hdfsBuilderSetNameNode(hdfs_builder, namenode_cstr.as_ptr());
        let hdfs_fs = hdfsBuilderConnect(hdfs_builder);
        self.fs_map.insert(namenode_uri, unsafe { HdfsConnection {fs_url: namenode_uri, fs: hdfs_fs} });
      }
    }

    Some(self.fs_map.get(&namenode_uri).unwrap())    
  }
}

pub struct HdfsConnection {
  fs_url: String,
  fs: *mut HdfsFS
}

/// FileSystem
// lazy_static! {
//   static ref HDFS: HdfsCache = HdfsCache {
//     fs_map: HashMap::new(),
//     lock: Arc::new(Mutex::new(0))
//   };
// }

#[test]
fn test_singleton() {
  let mut cache = HdfsCache {
    fs_map: HashMap::new(),
    lock: Arc::new(Mutex::new(0))
  };
  assert_eq!("file:///".to_string(), cache.get("file:/blah").unwrap().fs_url);
}