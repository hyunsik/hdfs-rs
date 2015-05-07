use std::collections::HashMap;
use std::ffi::CString;
use std::marker::PhantomData;
use std::string::String;
use std::sync::{Arc, Mutex};
use url::{UrlParser, SchemeType};
use binding::*;
use libc::{c_char, c_int, c_short, int32_t};

pub static LOCALFS_SCHEME: &'static str = "file";

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

          if url.scheme.as_str() == LOCALFS_SCHEME { 
            namenode_uri.push_str("file:///");
          } else {

            if url.host().is_some() {              
              namenode_uri.push_str(
                format!("{}://{}", url.scheme.as_str(), url.host().unwrap()).as_str());

              if url.port().is_some() {
                namenode_uri.push_str(format!(":{}", url.port().unwrap()).as_str());
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

    info!("Connect to Namenode ({})", namenode_uri.as_str());

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
          HdfsFS {fs_url: namenode_uri.clone(), fs: hdfs_fs, _marker: PhantomData});
      }
    }

    Some(self.fs_map.get(&namenode_uri).unwrap())    
  }
}

const O_RDONLY: c_int = 0;
const O_WRONLY: c_int = 1;
const O_APPEND: c_int = 1024;

pub enum HdfsErr {
  FileNotFound(String),
  FileAlreadyExists(String),
  UNKNOWN 
}


pub struct HdfsFS<'a> {
  fs_url: String,
  fs: *mut hdfsFS,
  _marker: PhantomData<&'a ()>
}

impl<'a> HdfsFS<'a> {

  #[inline]
  pub fn fs_url(&'a self) -> &'a str {
    &self.fs_url.as_str()
  }

  // pub fn mkdir(&self, path: &str) -> Result<bool, HdfsErr> {

  // }

  #[inline]
  pub fn exist(&self, path: &str) -> bool {
    match unsafe {hdfsExists(self.fs, str_to_chars(path))} {
      0 => true,
      _ => false
    }
  }

  pub fn open(&self, path: &str) -> Result<HdfsFile, HdfsErr> {
    self.open_with_bufsize(path, 0)
  }

  pub fn open_with_bufsize(&self, path: &str, buf_size: i32) 
    -> Result<HdfsFile, HdfsErr> {    

    let file = unsafe { 
      hdfsOpenFile(self.fs, str_to_chars(path), O_RDONLY, 
        buf_size as c_int, 0, 0) 
    };

    if file.is_null() {
      Err(HdfsErr::UNKNOWN)
    } else {
      Ok(HdfsFile {fs: self, path: path.to_owned(), file: file})
    }
  }

  pub fn create(&self, path: &str) -> Result<HdfsFile, HdfsErr> {
    self.create_with_params(path, false, 0, 0, 0)
  }

  pub fn create_with_overwrite(&self, path: &str, 
      overwrite: bool) -> Result<HdfsFile, HdfsErr> {

    self.create_with_params(path, overwrite, 0, 0, 0)
  }

  pub fn create_with_params(
    &'a self,
    path: &str, 
    overwrite: bool,
    buf_size: i32,
    replica_num: i16,
    block_size: i32) -> Result<HdfsFile, HdfsErr> {

    if !overwrite && self.exist(path) {
      return Err(HdfsErr::FileAlreadyExists(path.to_owned()));
    }

    let file = unsafe { 
      hdfsOpenFile(self.fs, str_to_chars(path), O_WRONLY, 
        buf_size as c_int, replica_num as c_short, block_size as int32_t) 
    };

    if file.is_null() {
      Err(HdfsErr::UNKNOWN)
    } else {
      Ok(HdfsFile {fs: self, path: path.to_owned(), file: file})
    }
  }

  pub fn append(&self, path: &str) -> Result<HdfsFile, HdfsErr> {    
    if !self.exist(path) {
      return Err(HdfsErr::FileNotFound(path.to_owned()));
    }

    let file = unsafe { 
      hdfsOpenFile(self.fs, str_to_chars(path), O_APPEND, 0,0,0) 
    };

    if file.is_null() {
      Err(HdfsErr::UNKNOWN)
    } else {
      Ok(HdfsFile {fs: self, path: path.to_owned(), file: file})
    }
  }

  pub fn close(&self, file: &HdfsFile) -> Result<bool, HdfsErr> {
    file.close()
  }

}

fn str_to_chars(s: &str) -> *const c_char {
  CString::new(s.as_bytes()).unwrap().as_ptr()
}

pub struct HdfsFile<'a> {
  fs: &'a HdfsFS<'a>,
  path: String,
  file: *mut hdfsFile
}

impl<'a> HdfsFile<'a> {

  pub fn path(&'a self) -> &'a str {
    self.path.as_str()
  }

  pub fn close(&self) -> Result<bool, HdfsErr> {
    match unsafe {hdfsCloseFile(self.fs.fs, self.file)} {
      0 => Ok(true),
      _ => Err(HdfsErr::UNKNOWN)
    }
  }
}

//struct HdfsFile

fn hdfs_scheme_handler(scheme: &str) -> SchemeType {
  match scheme {
    "file" => SchemeType::FileLike,
    "hdfs" => SchemeType::Relative(50070),
    _ => panic!("Unsupported scheme: {}", scheme)
  }
}


#[test]
fn test_hdfs_connection() {
  use minidfs::*;

  let mut conf = MiniDfsConf::new();
  let dfs = MiniDFS::start(&mut conf).unwrap();
  let port = dfs.namenode_port().unwrap();

  let minidfs_addr = format!("hdfs://localhost:{}", port);  

  let mut cache: HdfsFsCache = HdfsFsCache::new();


  // Parse namenode uris
  assert_eq!("file:///".to_string(), cache.get("file:/blah").unwrap().fs_url);
  let test_path = format!("hdfs://localhost:{}/users/test", port);
  println!("Trying to get {}", test_path.as_str());
  assert_eq!(minidfs_addr, cache.get(test_path.as_str()).unwrap().fs_url);


  // create a file, check existence, and close
  let fs = cache.get(test_path.as_str()).unwrap();
  let test_file = "/test_file";
  let created_file = match fs.create(test_file) {
    Ok(f) => f,
    Err(e) => panic!("Couldn't create a file")
  };
  assert!(created_file.close().is_ok());
  assert!(fs.exist(test_file));


  // open a file and close
  let opened_file = match fs.open(test_file) {
    Ok(f) => f,
    Err(e) => panic!("Couldn't open a file")
  };
  assert!(opened_file.close().is_ok());


  dfs.stop();
}