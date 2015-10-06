use std::collections::HashMap;
use std::ffi::CString;
use std::marker::PhantomData;
use std::mem;
use std::slice;
use std::string::String;
use std::sync::{Arc, Mutex};
use url::{UrlParser, SchemeType};
use binding::*;
use libc::{c_char, c_int, c_short, c_void, int16_t, int32_t, int64_t};

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

pub struct HdfsUtil;

/// HDFS Utility
impl HdfsUtil {

  /// Copy file from one filesystem to another.
  ///
  /// #### Params
  /// * ```srcFS``` - The handle to source filesystem.
  /// * ```src``` - The path of source file.
  /// * ```dstFS``` - The handle to destination filesystem.
  /// * ```dst``` - The path of destination file.
  pub fn copy(src_fs: &HdfsFS, src: &str, dst_fs: &HdfsFS, dst: &str)
      -> Result<bool, HdfsErr> {

    let res = unsafe {
      hdfsCopy(src_fs.fs, str_to_chars(src), dst_fs.fs, str_to_chars(dst))
    };

    if res == 0 {
      Ok(true)
    } else {
      Err(HdfsErr::UNKNOWN)
    }
  }

  /// Move file from one filesystem to another.
  ///
  /// #### Params
  /// * ```srcFS``` - The handle to source filesystem.
  /// * ```src``` - The path of source file.
  /// * ```dstFS``` - The handle to destination filesystem.
  /// * ```dst``` - The path of destination file.
  pub fn mv(src_fs: &HdfsFS, src: &str, dst_fs: &HdfsFS, dst: &str)
      -> Result<bool, HdfsErr> {

    let res = unsafe {
      hdfsMove(src_fs.fs, str_to_chars(src), dst_fs.fs, str_to_chars(dst))
    };

    if res == 0 {
      Ok(true)
    } else {
      Err(HdfsErr::UNKNOWN)
    }
  }
}

fn bool_to_c_int(val: bool) -> c_int {
  if val { 1 } else { 0 }
}

/// Options for zero-copy read
pub struct RzOptions {
  ptr: *const hadoopRzOptions
}

impl Drop for RzOptions {
  fn drop(&mut self) {
    unsafe { hadoopRzOptionsFree(self.ptr) }
  }
}

impl RzOptions {
  pub fn new() -> RzOptions {
    RzOptions { ptr: unsafe { hadoopRzOptionsAlloc() } }
  }

  pub fn skip_checksum(&self, skip: bool) -> Result<bool, HdfsErr> {
    let res = unsafe {
      hadoopRzOptionsSetSkipChecksum(self.ptr, bool_to_c_int(skip))
    };

    if res == 0 {
      Ok(true)
    } else {
      Err(HdfsErr::UNKNOWN)
    }
  }

  pub fn set_bytebuffer_pool(&self, class_name: &str)
      -> Result<bool, HdfsErr> {

    let res = unsafe {
      hadoopRzOptionsSetByteBufferPool(self.ptr, str_to_chars(class_name))
    };

    if res == 0 {
      Ok(true)
    } else {
      Err(HdfsErr::UNKNOWN)
    }
  }
}

/// A buffer returned from zero-copy read.
/// This buffer will be automatically freed when its lifetime is finished.
pub struct RzBuffer<'a> {
  file: &'a HdfsFile<'a>,
  ptr: *const hadoopRzBuffer
}

impl<'a> Drop for RzBuffer<'a> {
  fn drop(&mut self) {
    unsafe { hadoopRzBufferFree(self.file.file, self.ptr) }
  }
}

impl<'a> RzBuffer<'a> {
  /// Get the length of a raw buffer returned from zero-copy read.
  pub fn len(&self) -> i32 {
    (unsafe { hadoopRzBufferLength(self.ptr) }) as i32
  }

  /// Get a pointer to the raw buffer returned from zero-copy read.
  pub fn as_ptr(&self) -> Result<*const u8, HdfsErr> {
    let ptr = unsafe {
        hadoopRzBufferGet(self.ptr)
    };

    if !ptr.is_null() {
      Ok( ptr as *const u8 )
     } else {
      Err(HdfsErr::UNKNOWN)
     }
  }

  /// Get a Slice transformed from a raw buffer
  pub fn as_slice(&'a self) -> Result<&[u8], HdfsErr> {
     let ptr = unsafe {
        hadoopRzBufferGet(self.ptr) as *const u8
     };

     let len = unsafe {
      hadoopRzBufferLength(self.ptr) as usize
     };

     if !ptr.is_null() {
      Ok(unsafe { mem::transmute(slice::from_raw_parts(ptr, len as usize)) })
     } else {
      Err(HdfsErr::UNKNOWN)
     }
  }
}

/// Includes hostnames where a particular block of a file is stored.
pub struct BlockHosts {
  ptr: *const *const *const c_char
}

impl Drop for BlockHosts {
  fn drop(&mut self) {
    unsafe { hdfsFreeHosts(self.ptr) };
  }
}

impl BlockHosts {
}

/// Hdfs Filesystem
pub struct HdfsFS<'a> {
  fs_url: String,
  fs: *const hdfsFS,
  _marker: PhantomData<&'a ()>
}

impl<'a> HdfsFS<'a> {

  /// Open a file for append
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

  /// set permission
  pub fn chmod(&self, path: &str, mode: i16) -> bool {
    (unsafe {
      hdfsChmod(self.fs, str_to_chars(path), mode as c_short)}) == 0
  }

  pub fn chown(&self, path: &str, owner: &str, group: &str) -> bool {
    (unsafe {
      hdfsChown(self.fs, str_to_chars(path),
        str_to_chars(owner), str_to_chars(group))}) == 0
  }

  #[inline]
  pub fn create(&self, path: &str) -> Result<HdfsFile, HdfsErr> {
    self.create_with_params(path, false, 0, 0, 0)
  }

  #[inline]
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

  /// Get the default blocksize.
  pub fn default_blocksize(&self) -> Result<usize, HdfsErr> {
    let block_sz = unsafe { hdfsGetDefaultBlockSize(self.fs) };

    if block_sz > 0 {
      Ok(block_sz as usize)
    } else {
      Err(HdfsErr::UNKNOWN)
    }
  }

  /// Get the default blocksize at the filesystem indicated by a given path.
  pub fn block_size(&self, path: &str) -> Result<usize, HdfsErr> {
    let block_sz = unsafe {
      hdfsGetDefaultBlockSizeAtPath(self.fs, str_to_chars(path))
    };

    if block_sz > 0 {
      Ok(block_sz as usize)
    } else {
      Err(HdfsErr::UNKNOWN)
    }
  }

  /// Return the raw capacity of the filesystem.
  pub fn capacity(&self) -> Result<usize, HdfsErr> {
    let block_sz = unsafe {
      hdfsGetCapacity(self.fs)
    };

    if block_sz > 0 {
      Ok(block_sz as usize)
    } else {
      Err(HdfsErr::UNKNOWN)
    }
  }

  /// Delete file.
  pub fn delete(&self, path: &str, recursive: bool) -> Result<bool, HdfsErr> {
    let res = unsafe {
      hdfsDelete(self.fs, str_to_chars(path), recursive as c_int)
    };

    if res == 0 {
      Ok(true)
    } else {
      Err(HdfsErr::UNKNOWN)
    }
  }

  /// Checks if a given path exsits on the filesystem
  pub fn exist(&self, path: &str) -> bool {
    if unsafe {hdfsExists(self.fs, str_to_chars(path))} == 0 {
      true
    } else {
      false
    }
  }

  /// Get HDFS namenode url
  #[inline]
  pub fn fs_url(&'a self) -> &'a str {
    &self.fs_url
  }

  /// Get hostnames where a particular block (determined by
  /// pos & blocksize) of a file is stored. The last element in the array
  /// is NULL. Due to replication, a single block could be present on
  /// multiple hosts.
  pub fn get_hosts(&self, path: &str, start: usize, length: usize)
      -> Result<BlockHosts, HdfsErr> {

    let ptr = unsafe {
      hdfsGetHosts(self.fs, str_to_chars(path),
        start as int64_t, length as int64_t)
    };

    if !ptr.is_null() {
      Ok(BlockHosts {ptr: ptr})
    } else {
      Err(HdfsErr::UNKNOWN)
    }
  }

  /// create a directory
  pub fn mkdir(&self, path: &str) -> Result<bool, HdfsErr> {
    if unsafe{hdfsCreateDirectory(self.fs, str_to_chars(path))} == 0 {
      Ok(true)
    } else {
      Err(HdfsErr::UNKNOWN)
    }
  }

  /// open a file to read
  #[inline]
  pub fn open(&self, path: &str) -> Result<HdfsFile, HdfsErr> {
    self.open_with_bufsize(path, 0)
  }

  /// open a file to read with a buffer size
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

  /// Set the replication of the specified file to the supplied value
  pub fn set_replication(&self, path: &str, num: i16)
      -> Result<bool, HdfsErr> {

    let res = unsafe {
      hdfsSetReplication(self.fs, str_to_chars(path), num as int16_t)
    };

    if res == 0 {
      Ok(true)
    } else {
      Err(HdfsErr::UNKNOWN)
    }
  }

  /// Rename file.
  pub fn rename(&self, old_path: &str, new_path: &str)
      -> Result<bool, HdfsErr> {

    let res = unsafe {
      hdfsRename(self.fs, str_to_chars(old_path), str_to_chars(new_path))
    };

    if res == 0 {
      Ok(true)
    } else {
      Err(HdfsErr::UNKNOWN)
    }
  }

  /// Return the total raw size of all files in the filesystem.
  pub fn used(&self) -> Result<usize, HdfsErr> {
    let block_sz = unsafe {
      hdfsGetUsed(self.fs)
    };

    if block_sz > 0 {
      Ok(block_sz as usize)
    } else {
      Err(HdfsErr::UNKNOWN)
    }
  }
}

fn str_to_chars(s: &str) -> *const c_char {
  CString::new(s.as_bytes()).unwrap().as_ptr()
}

/// open hdfs file
pub struct HdfsFile<'a> {
  fs: &'a HdfsFS<'a>,
  path: String,
  file: *const hdfsFile
}

impl<'a> HdfsFile<'a> {

  pub fn available(&self) -> Result<bool, HdfsErr> {
    if unsafe { hdfsAvailable(self.fs.fs, self.file) } == 0 {
      Ok(true)
    } else {
      Err(HdfsErr::UNKNOWN)
    }
  }

  /// Close the opened file
  pub fn close(&self) -> Result<bool, HdfsErr> {
    if unsafe {hdfsCloseFile(self.fs.fs, self.file)} == 0 {
      Ok(true)
    } else {
      Err(HdfsErr::UNKNOWN)
    }
  }

  /// Flush the data.
  pub fn flush(&self) -> bool {
    (unsafe { hdfsFlush(self.fs.fs, self.file) }) == 0
  }

  /// Flush out the data in client's user buffer. After the return of this
  /// call, new readers will see the data.
  pub fn hflush(&self) -> bool {
    (unsafe { hdfsHFlush(self.fs.fs, self.file) }) == 0
  }

  /// Similar to posix fsync, Flush out the data in client's
  /// user buffer. all the way to the disk device (but the disk may have
  /// it in its cache).
  pub fn hsync(&self) -> bool {
    (unsafe { hdfsHSync(self.fs.fs, self.file) }) == 0
  }

  /// Determine if a file is open for read.
  pub fn is_readable(&self) -> bool {
    (unsafe { hdfsFileIsOpenForRead(self.file) }) == 1
  }

  /// Determine if a file is open for write.
  pub fn is_writable(&self) -> bool {
    (unsafe { hdfsFileIsOpenForWrite(self.file) }) == 1
  }

  /// Return a file path
  pub fn path(&'a self) -> &'a str {
    &self.path
  }

  /// Get the current offset in the file, in bytes.
  pub fn pos(&self) -> Result<u64, HdfsErr> {
    let pos = unsafe {hdfsTell(self.fs.fs, self.file)};

    if pos > 0 {
      Ok(pos as u64)
    } else {
      Err(HdfsErr::UNKNOWN)
    }
  }

  /// Read data from an open file.
  pub fn read(&self, buf: &mut [u8]) -> Result<i32, HdfsErr> {
    let read_len = unsafe {
      hdfsRead(self.fs.fs, self.file, buf.as_ptr() as *mut c_void,
        buf.len() as tSize)
    };

    if read_len > 0 {
      Ok(read_len as i32)
    } else {
      Err(HdfsErr::UNKNOWN)
    }
  }

  /// Positional read of data from an open file.
  pub fn read_with_pos(&self, pos: i64, buf: &mut [u8]) -> Result<i32, HdfsErr> {
    let read_len = unsafe {
      hdfsPread(self.fs.fs, self.file, pos as tOffset,
        buf.as_ptr() as *mut c_void, buf.len() as tSize)
    };

    if read_len > 0 {
      Ok(read_len as i32)
    } else {
      Err(HdfsErr::UNKNOWN)
    }
  }

  /// Perform a byte buffer read. If possible, this will be a zero-copy
  /// (mmap) read.
  pub fn read_zc(&'a self, opts: &RzOptions, max_len: i32) -> Result<RzBuffer<'a>, HdfsErr> {
    let buf : *const hadoopRzBuffer = unsafe {
      hadoopReadZero(self.file, opts.ptr, max_len as int32_t)
    };

    if !buf.is_null() {
      Ok(RzBuffer {file: self, ptr: buf})
    } else {
      Err(HdfsErr::UNKNOWN)
    }
  }

  /// Seek to given offset in file.
  pub fn seek(&self, offset: u64) -> bool {
    (unsafe { hdfsSeek(self.fs.fs, self.file, offset as tOffset) }) == 0
  }

  /// Write data into an open file.
  pub fn write(&self, buf: &[u8]) -> Result<i32, HdfsErr> {
    let written_len = unsafe {
      hdfsWrite(self.fs.fs, self.file,
        buf.as_ptr() as *mut c_void, buf.len() as tSize)
    };

    if written_len > 0 {
      Ok(written_len)
    } else {
      Err(HdfsErr::UNKNOWN)
    }
  }
}

/// for HDFS URL scheme (i.e., hdfs://)
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
  println!("Trying to get {}", &test_path);
  assert_eq!(minidfs_addr, cache.get(&test_path).unwrap().fs_url);


  // create a file, check existence, and close
  let fs = cache.get(&test_path).unwrap();
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
