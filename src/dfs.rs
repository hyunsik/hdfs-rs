use std::collections::HashMap;
use std::marker::PhantomData;
use std::mem;
use std::rc::Rc;
use std::slice;
use std::string::String;
use std::sync::Mutex;

use url::{UrlParser,SchemeType};
use libc::{c_char, c_int, c_short, c_void, int16_t, int32_t, int64_t, time_t};

use err::HdfsErr;
use native::*;
use util::{chars_to_str, str_to_chars, bool_to_c_int};

const O_RDONLY: c_int = 0;
const O_WRONLY: c_int = 1;
const O_APPEND: c_int = 1024;

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
      Err(HdfsErr::Unknown)
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
      Err(HdfsErr::Unknown)
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
      Err(HdfsErr::Unknown)
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
      Err(HdfsErr::Unknown)
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

/// Safely deallocable hdfsFileInfo pointer
struct HdfsFileInfoPtr {
  pub ptr: *const hdfsFileInfo,
  pub len: i32
}

/// for safe deallocation
impl<'a> Drop for HdfsFileInfoPtr {
  fn drop(&mut self) {
    unsafe { hdfsFreeFileInfo(self.ptr, self.len) };
  }
}

impl HdfsFileInfoPtr {
  fn new(ptr: *const hdfsFileInfo) -> HdfsFileInfoPtr {
    HdfsFileInfoPtr {
      ptr: ptr,
      len: 1
    }
  }
  
  pub fn new_array(ptr: *const hdfsFileInfo, len: i32) -> HdfsFileInfoPtr {
    HdfsFileInfoPtr {
      ptr: ptr,
      len: len
    }
  }
}

/// Interface that represents the client side information for a file or directory.
pub struct FileStatus<'a> {
  raw: Rc<HdfsFileInfoPtr>,
  idx: u32,
  _marker: PhantomData<&'a ()>
}

impl<'a> FileStatus<'a> {
  #[inline]
  /// create FileStatus from *const hdfsFileInfo
  fn new(ptr: *const hdfsFileInfo) -> FileStatus<'a> {
    FileStatus {
      raw: Rc::new(HdfsFileInfoPtr::new(ptr)),
      idx: 0,
      _marker: PhantomData
    }
  }
  
  /// create FileStatus from *const hdfsFileInfo which points 
  /// to dynamically allocated array.
  #[inline]
  fn from_array(raw: Rc<HdfsFileInfoPtr>, idx: u32) -> FileStatus<'a> {
    FileStatus {
      raw: raw,
      idx: idx,
      _marker: PhantomData
    }
  }
  
  #[inline]
  fn ptr(&self) -> *const hdfsFileInfo {
    unsafe {self.raw.ptr.offset(self.idx as isize)}
  }
  
  /// Get the name of the file
  #[inline]
  pub fn name(&self) -> &'a str 
  { 
    chars_to_str(unsafe {&*self.ptr()}.mName) 
  }
  
  /// Is this a file?
  #[inline]
  pub fn is_file(&self) -> bool {
    match unsafe {&*self.ptr()}.mKind {
      tObjectKind::kObjectKindFile => true,
      tObjectKind::kObjectKindDirectory => false,
    }
  }
  
  /// Is this a directory?
  #[inline]
  pub fn is_directory(&self) -> bool {
    match unsafe {&*self.ptr()}.mKind {
      tObjectKind::kObjectKindFile => false,
      tObjectKind::kObjectKindDirectory => true,
    }
  }
  
  /// Get the owner of the file
  #[inline]
  pub fn owner(&self) -> &'a str
  {
    chars_to_str(unsafe {&*self.ptr()}.mOwner)
  }
  
  /// Get the group associated with the file
  #[inline]
  pub fn group(&self) -> &'a str
  {
    chars_to_str(unsafe {&*self.ptr()}.mGroup)
  }
  
  /// Get the permissions associated with the file
  #[inline]
  pub fn permission(&self) -> i16
  {
    unsafe {&*self.ptr()}.mPermissions as i16
  }
  
  /// Get the length of this file, in bytes.
  #[inline]
  pub fn len(&self) -> usize 
  {
    unsafe {&*self.ptr()}.mSize as usize
  }
  
  /// Get the block size of the file.
  #[inline]
  pub fn block_size(&self) -> usize
  {
    unsafe {&*self.ptr()}.mBlockSize as usize
  }
  
  /// Get the replication factor of a file.
  #[inline]
  pub fn replica_count(&self) -> i16
  {
    unsafe {&*self.ptr()}.mReplication as i16
  }
  
  /// Get the last modification time for the file in seconds
  #[inline]
  pub fn last_modified(&self) -> time_t
  {
    unsafe {&*self.ptr()}.mLastMod
  }
  
  /// Get the last access time for the file in seconds
  #[inline]
  pub fn last_accced(&self) -> time_t
  {
    unsafe {&*self.ptr()}.mLastAccess
  }
}

/// Hdfs Filesystem
///
/// It is basically thread safe because the native API for hdfsFs is thread-safe. 
#[derive(Clone)]
#[allow(raw_pointer_derive)]
pub struct HdfsFs<'a> {
  url: String,
  raw: *const hdfsFS,
  _marker: PhantomData<&'a ()>
}

impl<'a> HdfsFs<'a> {
  /// create HdfsFs instance. Please use HdfsFsCache rather than using this API directly. 
  #[inline]
  fn new(url: String, raw: *const hdfsFS) -> HdfsFs<'a>
  {
    HdfsFs {
      url: url,
      raw: raw,
      _marker: PhantomData
    }
  }
  
  /// Get HDFS namenode url
  #[inline]
  pub fn url(&self) -> &str
  {
    &self.url
  }
  
  /// Get a raw pointer of JNI API's HdfsFs
  #[inline]
  pub fn raw(&self) -> *const hdfsFS
  {
    self.raw
  }

  /// Open a file for append
  pub fn append(&self, path: &str) -> Result<HdfsFile, HdfsErr> {
    if !self.exist(path) {
      return Err(HdfsErr::FileNotFound(path.to_owned()));
    }

    let file = unsafe {
      hdfsOpenFile(self.raw, str_to_chars(path), O_APPEND, 0,0,0)
    };

    if file.is_null() {
      Err(HdfsErr::Unknown)
    } else {
      Ok(HdfsFile {fs: self, path: path.to_owned(), file: file})
    }
  }

  /// set permission
  pub fn chmod(&self, path: &str, mode: i16) -> bool {
    (unsafe {
      hdfsChmod(self.raw, str_to_chars(path), mode as c_short)}) == 0
  }

  pub fn chown(&self, path: &str, owner: &str, group: &str) -> bool {
    (unsafe {
      hdfsChown(self.raw, str_to_chars(path),
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
      hdfsOpenFile(self.raw, str_to_chars(path), O_WRONLY,
        buf_size as c_int, replica_num as c_short, block_size as int32_t)
    };

    if file.is_null() {
      Err(HdfsErr::Unknown)
    } else {
      Ok(HdfsFile {fs: self, path: path.to_owned(), file: file})
    }
  }

  /// Get the default blocksize.
  pub fn default_blocksize(&self) -> Result<usize, HdfsErr> {
    let block_sz = unsafe { hdfsGetDefaultBlockSize(self.raw) };

    if block_sz > 0 {
      Ok(block_sz as usize)
    } else {
      Err(HdfsErr::Unknown)
    }
  }

  /// Get the default blocksize at the filesystem indicated by a given path.
  pub fn block_size(&self, path: &str) -> Result<usize, HdfsErr> {
    let block_sz = unsafe {
      hdfsGetDefaultBlockSizeAtPath(self.raw, str_to_chars(path))
    };

    if block_sz > 0 {
      Ok(block_sz as usize)
    } else {
      Err(HdfsErr::Unknown)
    }
  }

  /// Return the raw capacity of the filesystem.
  pub fn capacity(&self) -> Result<usize, HdfsErr> {
    let block_sz = unsafe {
      hdfsGetCapacity(self.raw)
    };

    if block_sz > 0 {
      Ok(block_sz as usize)
    } else {
      Err(HdfsErr::Unknown)
    }
  }

  /// Delete file.
  pub fn delete(&self, path: &str, recursive: bool) -> Result<bool, HdfsErr> {
    let res = unsafe {
      hdfsDelete(self.raw, str_to_chars(path), recursive as c_int)
    };

    if res == 0 {
      Ok(true)
    } else {
      Err(HdfsErr::Unknown)
    }
  }

  /// Checks if a given path exsits on the filesystem
  pub fn exist(&self, path: &str) -> bool {
    if unsafe {hdfsExists(self.raw, str_to_chars(path))} == 0 {
      true
    } else {
      false
    }
  }

  /// Get hostnames where a particular block (determined by
  /// pos & blocksize) of a file is stored. The last element in the array
  /// is NULL. Due to replication, a single block could be present on
  /// multiple hosts.
  pub fn get_hosts(&self, path: &str, start: usize, length: usize)
      -> Result<BlockHosts, HdfsErr> {

    let ptr = unsafe {
      hdfsGetHosts(self.raw, str_to_chars(path),
        start as int64_t, length as int64_t)
    };

    if !ptr.is_null() {
      Ok(BlockHosts {ptr: ptr})
    } else {
      Err(HdfsErr::Unknown)
    }
  }

  /// create a directory
  pub fn mkdir(&self, path: &str) -> Result<bool, HdfsErr> {
    if unsafe{hdfsCreateDirectory(self.raw, str_to_chars(path))} == 0 {
      Ok(true)
    } else {
      Err(HdfsErr::Unknown)
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
      hdfsOpenFile(self.raw, str_to_chars(path), O_RDONLY,
        buf_size as c_int, 0, 0)
    };

    if file.is_null() {
      Err(HdfsErr::Unknown)
    } else {
      Ok(HdfsFile {fs: self, path: path.to_owned(), file: file})
    }
  }

  /// Set the replication of the specified file to the supplied value
  pub fn set_replication(&self, path: &str, num: i16)
      -> Result<bool, HdfsErr> {

    let res = unsafe {
      hdfsSetReplication(self.raw, str_to_chars(path), num as int16_t)
    };

    if res == 0 {
      Ok(true)
    } else {
      Err(HdfsErr::Unknown)
    }
  }

  /// Rename file.
  pub fn rename(&self, old_path: &str, new_path: &str)
      -> Result<bool, HdfsErr> {

    let res = unsafe {
      hdfsRename(self.raw, str_to_chars(old_path), str_to_chars(new_path))
    };

    if res == 0 {
      Ok(true)
    } else {
      Err(HdfsErr::Unknown)
    }
  }

  /// Return the total raw size of all files in the filesystem.
  pub fn used(&self) -> Result<usize, HdfsErr> {
    let block_sz = unsafe {
      hdfsGetUsed(self.raw)
    };

    if block_sz > 0 {
      Ok(block_sz as usize)
    } else {
      Err(HdfsErr::Unknown)
    }
  }
  
  pub fn list_status(&self, path: &str) -> Result<Vec<FileStatus>, HdfsErr> {
    let mut entry_num: c_int = 0;
    
    let ptr = unsafe {
      hdfsListDirectory(self.raw, str_to_chars(path), &mut entry_num)
    };
    
    if ptr.is_null() {
      return Err(HdfsErr::Unknown)
    }
    
    let shared_ptr = Rc::new(HdfsFileInfoPtr::new_array(ptr, entry_num));
    
    let mut list = Vec::new(); 
    for idx in 0 .. entry_num {
      list.push(FileStatus::from_array(shared_ptr.clone(), idx as u32));
    }
    
    Ok(list)
  }    
  
  pub fn get_file_status(&self, path: &str) -> Result<FileStatus, HdfsErr> {
    let ptr = unsafe {
      hdfsGetPathInfo(self.raw, str_to_chars(path))
    };
    
    if ptr.is_null() {
      Err(HdfsErr::Unknown)
    } else {
      Ok(FileStatus::new(ptr))
    }
  }
}

/// open hdfs file
pub struct HdfsFile<'a> {
  fs: &'a HdfsFs<'a>,
  path: String,
  file: *const hdfsFile
}

impl<'a> HdfsFile<'a> {

  pub fn available(&self) -> Result<bool, HdfsErr> {
    if unsafe { hdfsAvailable(self.fs.raw, self.file) } == 0 {
      Ok(true)
    } else {
      Err(HdfsErr::Unknown)
    }
  }

  /// Close the opened file
  pub fn close(&self) -> Result<bool, HdfsErr> {
    if unsafe {hdfsCloseFile(self.fs.raw, self.file)} == 0 {
      Ok(true)
    } else {
      Err(HdfsErr::Unknown)
    }
  }

  /// Flush the data.
  pub fn flush(&self) -> bool {
    (unsafe { hdfsFlush(self.fs.raw, self.file) }) == 0
  }

  /// Flush out the data in client's user buffer. After the return of this
  /// call, new readers will see the data.
  pub fn hflush(&self) -> bool {
    (unsafe { hdfsHFlush(self.fs.raw, self.file) }) == 0
  }

  /// Similar to posix fsync, Flush out the data in client's
  /// user buffer. all the way to the disk device (but the disk may have
  /// it in its cache).
  pub fn hsync(&self) -> bool {
    (unsafe { hdfsHSync(self.fs.raw, self.file) }) == 0
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
    let pos = unsafe {hdfsTell(self.fs.raw, self.file)};

    if pos > 0 {
      Ok(pos as u64)
    } else {
      Err(HdfsErr::Unknown)
    }
  }

  /// Read data from an open file.
  pub fn read(&self, buf: &mut [u8]) -> Result<i32, HdfsErr> {
    let read_len = unsafe {
      hdfsRead(self.fs.raw, self.file, buf.as_ptr() as *mut c_void,
        buf.len() as tSize)
    };

    if read_len > 0 {
      Ok(read_len as i32)
    } else {
      Err(HdfsErr::Unknown)
    }
  }

  /// Positional read of data from an open file.
  pub fn read_with_pos(&self, pos: i64, buf: &mut [u8]) -> Result<i32, HdfsErr> {
    let read_len = unsafe {
      hdfsPread(self.fs.raw, self.file, pos as tOffset,
        buf.as_ptr() as *mut c_void, buf.len() as tSize)
    };

    if read_len > 0 {
      Ok(read_len as i32)
    } else {
      Err(HdfsErr::Unknown)
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
      Err(HdfsErr::Unknown)
    }
  }

  /// Seek to given offset in file.
  pub fn seek(&self, offset: u64) -> bool {
    (unsafe { hdfsSeek(self.fs.raw, self.file, offset as tOffset) }) == 0
  }

  /// Write data into an open file.
  pub fn write(&self, buf: &[u8]) -> Result<i32, HdfsErr> {
    let written_len = unsafe {
      hdfsWrite(self.fs.raw, self.file,
        buf.as_ptr() as *mut c_void, buf.len() as tSize)
    };

    if written_len > 0 {
      Ok(written_len)
    } else {
      Err(HdfsErr::Unknown)
    }
  }
}


static LOCAL_FS_SCHEME: &'static str = "file";

/// for HDFS URL scheme (i.e., hdfs://)
fn hdfs_scheme_handler(scheme: &str) -> SchemeType 
{
  match scheme {
    "file" => SchemeType::FileLike,
    "hdfs" => SchemeType::Relative(50070),
    _ => panic!("Unsupported scheme: {}", scheme)
  }
}

/// HdfsFsCache which caches HdfsFs instances.  
///
/// The original libhdfs allows only one HdfsFs instance for the same namenode. In otherwords,
/// Some APIs of libhdfs are not thread-safe. So, You must get HdfsFs instance through HdfsFsCache, 
/// caching initialized HdfsFs instances and returning them.  
pub struct HdfsFsCache<'a> 
{
  fs_map: Mutex<HashMap<String, HdfsFs<'a>>>,
  url_parser: UrlParser<'a>
}

impl<'a> HdfsFsCache<'a> 
{
  pub fn new() -> HdfsFsCache<'a> 
  {
    let mut url_parser = UrlParser::new();
    url_parser.scheme_type_mapper(hdfs_scheme_handler);

    HdfsFsCache {
      fs_map: Mutex::new(HashMap::new()),
      url_parser: url_parser
    }
  }

  #[inline]
  fn get_namenode_uri(&self, path: &str) -> Result<String, HdfsErr> 
  {
    match self.url_parser.parse(path) {
      Ok(url) => {
        
        if &url.scheme == LOCAL_FS_SCHEME {
          return Ok("file:///".to_string());
          
        } else {
          let mut uri_builder = String::new();
          if url.host().is_some() {
            uri_builder.push_str(&(
              format!("{}://{}", &url.scheme, url.host().unwrap())));

            if url.port().is_some() {
              uri_builder.push_str(&(format!(":{}", url.port().unwrap())));
            }

            return Ok(uri_builder);
          } else {
            Err(HdfsErr::InvalidUrl(path.to_string()))
          }
        }
      },
      Err(_) => Err(HdfsErr::InvalidUrl(path.to_string()))
    }
  }

  pub fn get(&mut self, path: &str) -> Result<HdfsFs<'a>, HdfsErr> 
  {
    let namenode_uri = try!(self.get_namenode_uri(path));
 
    let mut map = self.fs_map.lock().unwrap();
      
    if !map.contains_key(&namenode_uri) {  
      let hdfs_fs = unsafe {
        let hdfs_builder = hdfsNewBuilder();
        hdfsBuilderSetNameNode(hdfs_builder, str_to_chars(&namenode_uri));
        info!("Connecting to Namenode ({})", &namenode_uri);
        hdfsBuilderConnect(hdfs_builder)
      };
        
      if hdfs_fs.is_null() {
        return Err(HdfsErr::CannotConnectToNameNode(namenode_uri.clone()))
      }
          
      map.insert(
        namenode_uri.clone(),
        HdfsFs::new(namenode_uri.clone(), hdfs_fs));
    }
      
    Ok(map.get(&namenode_uri).unwrap().clone())
  }
}

#[cfg(test)]
mod test {
  use std::rc::Rc;
  use std::cell::RefCell;
  
  use itertools::Itertools;
  
  use native::MiniDfsConf;
  use minidfs::*;
  use super::HdfsFsCache;
  
  #[test]
  fn test_hdfs_connection() {
  
    let mut conf = MiniDfsConf::new();
    let dfs = MiniDFS::start(&mut conf).unwrap();
    let port = dfs.namenode_port().unwrap();
  
    let minidfs_addr = format!("hdfs://localhost:{}", port);
    let cache = Rc::new(RefCell::new(HdfsFsCache::new()));
  
  
    // Parse namenode uris
    assert_eq!("file:///".to_string(), cache.borrow_mut().get("file:/blah").ok().unwrap().url);
    let test_path = format!("hdfs://localhost:{}/users/test", port);
    println!("Trying to get {}", &test_path);
    assert_eq!(minidfs_addr, cache.borrow_mut().get(&test_path).ok().unwrap().url);
  
  
  
    // create a file, check existence, and close
    let fs = cache.borrow_mut().get(&test_path).ok().unwrap();
    let test_file = "/test_file";
    let created_file = match fs.create(test_file) {
      Ok(f) => f,
      Err(_) => panic!("Couldn't create a file")
    };
    assert!(created_file.close().is_ok());
    assert!(fs.exist(test_file));
  
  
    // open a file and close
    let opened_file = fs.open(test_file).ok().unwrap();
    assert!(opened_file.close().is_ok());
  
    match fs.mkdir("/dir1") {
      Ok(_) => println!("/dir1 created"),
      Err(_) => panic!("Couldn't create /dir1 directory")
    };
    
    let file_info = fs.get_file_status("/dir1").ok().unwrap();
    
    let expected_path = format!("hdfs://localhost:{}/dir1", port);
    assert_eq!(&expected_path, file_info.name());
    assert!(!file_info.is_file());
    assert!(file_info.is_directory());
    
    
    let sub_dir_num = 3;
    let mut expected_list = Vec::new();
    for x in 0..sub_dir_num {
      let filename = format!("/dir1/{}", x);
      expected_list.push(format!("hdfs://localhost:{}/dir1/{}", port, x));
      
      match fs.mkdir(&filename) {
        Ok(_) => println!("/dir1.x created"),
        Err(_) => panic!("Couldn't create /dir1 directory")
      };
    }
    
    let mut list = fs.list_status("/dir1").ok().unwrap();
    assert_eq!(sub_dir_num, list.len());
    
    list.sort_by(|a, b| Ord::cmp(a.name(), b.name()));
    for (expected, name) in izip!(expected_list, list.iter().map(|status| status.name())) {
      assert_eq!(expected, name);
    }
  
    dfs.stop();
  }
}