use std::collections::HashMap;
use std::ffi::CString;
use std::string::String;
use std::sync::{Arc, Mutex};

use url::{UrlParser,SchemeType};

use dfs::HdfsFS;
use native::*;

pub static LOCALFS_SCHEME: &'static str = "file";


/// for HDFS URL scheme (i.e., hdfs://)
pub fn hdfs_scheme_handler(scheme: &str) -> SchemeType 
{
  match scheme {
    "file" => SchemeType::FileLike,
    "hdfs" => SchemeType::Relative(50070),
    _ => panic!("Unsupported scheme: {}", scheme)
  }
}

/// HdfsFs Cache. Basically, It is a cache as well as a way to guarantees 
/// thread-safe when you get HdfsFs.
pub struct HdfsFsCache<'a> {
  fs_map: HashMap<String, HdfsFS<'a>>,
  lock: Arc<Mutex<i32>>,
  url_parser: UrlParser<'a>
}

impl<'a> HdfsFsCache<'a> {

  pub fn new() -> HdfsFsCache<'a> {
    let mut url_parser = UrlParser::new();
    url_parser.scheme_type_mapper(hdfs_scheme_handler);

    HdfsFsCache {
      fs_map: HashMap::new(),
      lock: Arc::new(Mutex::new(0)),
      url_parser: url_parser
    }
  }

  pub fn get(&mut self, path: &str) -> Option<&HdfsFS> {
    let mut namenode_uri = String::new();

    match self.url_parser.parse(path) {
      Ok(url) => {

        if &url.scheme == LOCALFS_SCHEME {
          namenode_uri.push_str("file:///");
        } else {

          if url.host().is_some() {
            namenode_uri.push_str(&(
              format!("{}://{}", &url.scheme, url.host().unwrap())));

            if url.port().is_some() {
              namenode_uri.push_str(&(format!(":{}", url.port().unwrap())));
            }

          } else {
            panic!("No hostname");
          }
        }
      },
      Err(_) => {
        error!("Path parsing failed: {}", path);
      }
    };

    self.lock.lock();

    info!("Connect to Namenode ({})", &namenode_uri);

    if !self.fs_map.contains_key(&namenode_uri) {
      unsafe {
        let hdfs_builder = hdfsNewBuilder();
        let namenode_uri_bytes: Vec<u8> = namenode_uri.bytes().collect();
        let namenode_cstr = CString::new(namenode_uri_bytes).unwrap();
        hdfsBuilderSetNameNode(hdfs_builder, namenode_cstr.as_ptr());
        let hdfs_fs = hdfsBuilderConnect(hdfs_builder);

        if hdfs_fs.is_null() {
          return None
        }

        self.fs_map.insert(
          namenode_uri.clone(),
          HdfsFS::new(namenode_uri.clone(), hdfs_fs));
      }
    }

    Some(self.fs_map.get(&namenode_uri).unwrap())
  }
}