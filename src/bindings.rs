use libc::{c_char, c_int, c_short, c_uchar, c_void, int16_t, int32_t, 
  int64_t, uint16_t, size_t, time_t};

/// Opaque Pointer of hdfsFS
pub enum hdfsFS {}

/// Opaque Pointer of hdfsFile
pub enum hdfsFile {}

/// Opaque Pointer of hdfsBuilder
pub enum hdfsBuilder {}

/// Opaque Pointer of hadoopRzOptions
pub enum hadoopRzOptions {}

/// Opaque Pointer of hadoopRzBuffer
pub enum hadoopRzBuffer {}

/// size of data for read/write io ops
pub type tSize = int32_t;
/// time type in seconds
pub type tTime = time_t;
/// offset within the file
pub type tOffset = int64_t;
/// port
pub type tPort = uint16_t;

#[repr(C)]
pub enum tObjectKind {
  kObjectKindFile = 0x46, // 'F'
  kObjectKindDirectory = 0x44 // 'D'
}

/// Information about a file/directory.
#[repr(C)]
#[allow(non_snake_case)]
pub struct HdfsReadStatistics {
  pub totalBytesRead : u64,
  pub totalLocalBytesRead : u64,
  pub totalShortCircuitBytesRead : u64,
  pub totalZeroCopyBytesRead : u64
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct hdfsFileInfo {
  /// file or directory
  mKind: tObjectKind,
  /// the name of the file
  mName: *mut c_char,
  /// the last modification time for the file in seconds
  mLastMod: tTime,
  /// the count of replicas
  mReplication: c_short,
  /// the block size for the file
  mBlockSize: tOffset,
  /// the owner of the file
  mOwner: *mut c_char,
  /// the group associated with the file
  mGroup: *mut c_char,
  /// the permissions associated with the file
  mPermissions: c_short,
  /// the last access time for the file in seconds
  mLastAccess: tTime,
}

#[link(name="hdfs")]
extern "C" {
  
  /// Determine if a file is open for read.
  ///
  /// #### Params
  /// * file - the HDFS file
  ///
  /// #### Return
  /// Return 1 if the file is open for read; 0 otherwise
  pub fn hdfsFileIsOpenForRead(fs: *mut hdfsFile) -> c_int;
  
  /// Determine if a file is open for write.
  /// 
  /// #### Params
  /// * file - the HDFS file
  ///
  /// #### Return
  /// Return 1 if the file is open for write; 0 otherwise.
  pub fn hdfsFileIsOpenForWrite(file: *mut hdfsFile) -> c_int;
  
  /// Get read statistics about a file.  This is only applicable to files 
  /// opened for reading.
  ///
  /// #### Params
  /// * file - The HDFS file
  /// * stats - (out parameter) on a successful return, the read statistics.  
  /// Unchanged otherwise. You must free the returned statistics with 
  /// hdfsFileFreeReadStatistics.
  ///
  /// #### Return
  /// * 0 if the statistics were successfully returned,
  /// * -1 otherwise.  On a failure, please check errno against
  /// * ENOTSUP. webhdfs, LocalFilesystem, and so forth may 
  /// not support read statistics.
  pub fn hdfsFileGetReadStatistics(file: *mut hdfsFile, 
                   stats: &mut *mut HdfsReadStatistics) -> c_int;
  
  /// HDFS read statistics for a file,
  /// 
  /// #### Params
  /// * stats - HDFS read statistics for a file.
  /// 
  /// #### Return
  /// Return the number of remote bytes read.
  pub fn hdfsReadStatisticsGetRemoteBytesRead(
    stats: *const HdfsReadStatistics) -> int64_t;
  
  /// Free some HDFS read statistics.
  ///
  /// #### Params
  /// * stats - The HDFS read statistics to free.
  pub fn hdfsFileFreeReadStatistics(stats: *mut HdfsReadStatistics);
  
  /// Connect to a hdfs file system as a specific user.
  ///
  /// #### Params
  /// * nn - The NameNode.  See hdfsBuilderSetNameNode for details.
  /// * port - The port on which the server is listening.
  /// * param - user the user name (this is hadoop domain user). 
  /// Or NULL is equivelant to hhdfsConnect(host, port)
  /// 
  /// #### Return
  /// Returns a handle to the filesystem or NULL on error.  
  pub fn hdfsConnectAsUser(host: *const c_char, 
                       uint16_t: u16, user: 
                       *const c_char) -> *mut hdfsFS;
  
  /// Connect to a hdfs file system.
  ///
  /// This API is deprecated. Use hdfsBuilderConnect instead.
  ///
  /// #### Params
  /// * nn - The NameNode.  See hdfsBuilderSetNameNode for details.
  /// * port - The port on which the server is listening.
  ///
  /// #### Return
  /// Returns a handle to the filesystem or NULL on error.
  pub fn hdfsConnect(host: *const c_char, uint16_t: u16) -> *mut hdfsFS;
  
  /// Connect to an hdfs file system.
  /// 
  /// Forces a new instance to be created. This API is deprecated.
  /// Use hdfsBuilderConnect instead. 
  ///
  /// #### Params
  /// * nn - The NameNode.  See hdfsBuilderSetNameNode for details.
  /// * port - The port on which the server is listening.
  /// * user - The user name to use when connecting
  ///
  /// #### Return
  /// Returns a handle to the filesystem or NULL on error.
  pub fn hdfsConnectAsUserNewInstance(host: *const c_char, 
                    uint16_t: u16,
                    user: *const c_char) -> *mut hdfsFS;
  
  /// Connect to an hdfs file system.
  /// 
  /// Forces a new instance to be created. This API is deprecated.
  /// Use hdfsBuilderConnect instead. 
  ///
  /// #### Params
  /// * nn - The NameNode.  See hdfsBuilderSetNameNode for details.
  /// * port - The port on which the server is listening.
  ///
  /// #### Return
  /// Returns a handle to the filesystem or NULL on error.
  pub fn hdfsConnectNewInstance(host: *const c_char, 
                            uint16_t: u16) -> *mut hdfsFS;
  
  /// Connect to HDFS using the parameters defined by the builder.
  ///
  /// The HDFS builder will be freed, whether or not the connection was successful.
  ///
  /// Every successful call to hdfsBuilderConnect should be matched with a call
  /// to hdfsDisconnect, when the hdfsFS is no longer needed.
  /// 
  /// #### Params
  /// * bld - The HDFS builder
  ///
  /// #### Return
  /// Returns a handle to the filesystem, or NULL on error.
  pub fn hdfsBuilderConnect(bld : *mut hdfsBuilder) -> *mut hdfsFS;


  /// Create an HDFS builder.
  ///
  /// #### Return
  /// The HDFS builder, or NULL on error.
  pub fn hdfsNewBuilder() -> *mut hdfsBuilder;
  
  /// Force the builder to always create a new instance of the FileSystem,
  /// rather than possibly finding one in the cache.
  ///
  /// #### Params
  /// * bld - The HDFS builder
  pub fn hdfsBuilderSetForceNewInstance(bld: *mut hdfsBuilder);

  /// Set the HDFS NameNode to connect to.
  ///
  /// #### Params
  /// * bld - The HDFS builder
  /// * nn - The NameNode to use. If the string given is 'default', the default NameNode
  /// configuration will be used (from the XML configuration files).
  /// If NULL is given, a LocalFileSystem will be created.
  /// If the string starts with a protocol type such as ```file://``` or
  /// ```hdfs://```, this protocol type will be used.  If not, the
  /// ```hdfs://``` protocol type will be used.
  /// You may specify a NameNode port in the usual way by
  /// passing a string of the format ```hdfs://<hostname>:<port>```.
  /// Alternately, you may set the port with hdfsBuilderSetNameNodePort.
  /// However, you must not pass the port in two different ways.
  pub fn hdfsBuilderSetNameNode(bld: *mut hdfsBuilder, host: *const c_char);

  /// Set the port of the HDFS NameNode to connect to.
  ///
  /// #### Params
  /// * bld - The HDFS builder
  /// * port - The port.
  pub fn hdfsBuilderSetNameNodePort(bld: *mut hdfsBuilder, port : uint16_t);

  /// Set the username to use when connecting to the HDFS cluster.
  ///
  /// #### Params
  /// * bld - The HDFS builder
  /// * userName - The user name.  The string will be shallow-copied.
  pub fn hdfsBuilderSetUserName(bld: *mut hdfsBuilder, userName: *const c_char);

  /// Set the path to the Kerberos ticket cache to use when connecting to
  /// the HDFS cluster.
  ///
  /// #### Params
  /// * ```bld``` - The HDFS builder
  /// * ```kerbTicketCachePath``` - The Kerberos ticket cache path.  The string
  /// will be shallow-copied.
  pub fn hdfsBuilderSetKerbTicketCachePath(bld: *mut hdfsBuilder, 
      kerbTicketCachePath: *const c_char);

  /// Free an HDFS builder.
  ///
  /// It is normally not necessary to call this function since
  /// hdfsBuilderConnect frees the builder.
  ///
  /// #### Params
  /// * ```bld``` - The HDFS builder
  pub fn hdfsFreeBuilder(bld: *mut hdfsBuilder);
  
  /// Set a configuration string for an HdfsBuilder.
  ///
  /// #### Params
  /// * ```key``` - The key to set.
  /// * ```val``` - The value, or NULL to set no value.
  /// This will be shallow-copied.  You are responsible for
  /// ensuring that it remains valid until the builder is freed.
  ///
  /// #### Return
  /// 0 on success; nonzero error code otherwise.
  pub fn hdfsBuilderConfSetStr(bld: *mut hdfsBuilder, 
                           key: *const c_char , value: *const c_char) -> c_int;
  
  /// Get a configuration string.
  ///
  /// #### Params
  /// * ```key``` - The key to find
  /// * ```val``` - (out param) The value.  This will be set to NULL if the
  /// key isn't found.  You must free this string with
  /// ```hdfsConfStrFree```.
  ///
  /// #### Return
  /// 0 on success; nonzero error code otherwise. 
  /// Failure to find the key is not an error.
  pub fn hdfsConfGetStr(value : *const c_char, val: *mut *mut c_char) -> c_int;

  /// Get a configuration integer.
  /// 
  /// #### Params
  /// * ```key``` - The key to find
  /// * ```val``` - (out param) The value.  This will NOT be changed if the
  /// key isn't found.
  ///
  /// #### Return
  /// 0 on success; nonzero error code otherwise.
  /// Failure to find the key is not an error.
  pub fn hdfsConfGetInt(key: *const c_char, val: *mut int32_t) -> c_int;
  
  /// Free a configuration string found with hdfsConfGetStr. 
  ///
  /// #### Params
  /// * ```val``` - A configuration string obtained from hdfsConfGetStr
  pub fn hdfsConfStrFree(val: *const c_char);
   
  /// hdfsDisconnect - Disconnect from the hdfs file system.
  /// Disconnect from hdfs.
  ///
  /// #### Params
  /// * ```fs``` - The configured filesystem handle.
  /// 
  /// #### Return
  /// Returns 0 on success, -1 on error.
  /// Even if there is an error, the resources associated with the
  /// hdfsFS will be freed.
  pub fn hdfsDisconnect(fs: *mut hdfsFS) -> c_int;

  /// Open a hdfs file in given mode.
  /// 
  /// #### Params
  /// * ```fs``` - The configured filesystem handle.
  /// * ```file``` - The file handle.
  /// * ```flags``` - an ```|``` of ```bits/fcntl.h``` file flags - 
  /// supported flags are O_RDONLY, O_WRONLY (meaning create or overwrite 
  /// i.e., implies O_TRUNCAT), O_WRONLY|O_APPEND. Other flags are generally 
  /// ignored other than (O_RDWR || (O_EXCL & O_CREAT)) which return NULL and 
  /// set errno equal ENOTSUP.
  /// * ```bufferSize``` - Size of buffer for read/write - pass 0 if you want
  /// to use the default configured values.
  /// * ```replication``` Block replication - pass 0 if you want to use
  /// the default configured values.
  /// * ```blocksize``` - Size of block - pass 0 if you want to use the
  /// default configured values.
  ///
  /// #### Return
  /// Returns 0 on success, -1 on error. On error, errno will be set appropriately.
  /// If the hdfs file was valid, the memory associated with it will
  /// be freed at the end of this call, even if there was an I/O error.
  pub fn hdfsOpenFile(fs: *mut hdfsFS, path: *const c_char, flags: c_int, 
                      bufferSize: c_int, replication: c_short, 
                      blocksize: int32_t) -> *mut hdfsFile;


  /// Close an open file. 
  ///
  /// #### Params
  /// * ```fs``` - The configured filesystem handle.
  /// * ```file``` - The file handle.
  ///
  /// #### Return
  /// Returns 0 on success, -1 on error.  On error, errno will be set appropriately.
  /// If the hdfs file was valid, the memory associated with it will
  /// be freed at the end of this call, even if there was an I/O error.
  pub fn hdfsCloseFile(fs: *mut hdfsFS, file: *mut hdfsFile) -> c_int;

  /// Checks if a given path exsits on the filesystem 
  ///
  /// #### Params
  /// * ```fs``` - The configured filesystem handle.
  /// * ```path``` - The path to look for
  ///
  /// #### Return
  /// Returns 0 on success, -1 on error.  
  pub fn hdfsExists(fs: *mut hdfsFS, path: *const c_char) -> c_int;


  pub fn hdfsSeek(fs: *mut hdfsFS, file: *mut hdfsFile, 
    desiredPos: tOffset) -> c_int;

  pub fn hdfsTell(fs: *mut hdfsFS, file: *mut hdfsFile) -> tOffset;

  pub fn hdfsRead(fs: *mut hdfsFS, file: *mut hdfsFile, buffer: *mut c_void, 
    length: tSize) -> tSize;

  pub fn hdfsPread(fs: *mut hdfsFS, file: *mut hdfsFile, position: tOffset,
    buffer: *mut c_void, length: tSize) -> tSize;

  pub fn hdfsWrite(fs: *mut hdfsFS, file: *mut hdfsFile, 
    buffer: *const c_void, length: tSize) -> tSize;

  pub fn hdfsFlush(fs: *mut hdfsFS, file: *mut hdfsFile) -> c_int;

  pub fn hdfsHFlush(fs: *mut hdfsFS, file: *mut hdfsFile) -> c_int;

  pub fn hdfsHSync(fs: *mut hdfsFS, file: *mut hdfsFile) -> c_int;

  pub fn hdfsAvailable(fs: *mut hdfsFS, file: *mut hdfsFile) -> c_int;

  pub fn hdfsCopy(srcFS: *mut hdfsFS, src: *const c_char, 
    dstFS: *mut hdfsFS, dst: *const c_char) -> c_int;

  pub fn hdfsMove(srcFS: *mut hdfsFS, src: *const c_char, 
    dstFS: *mut hdfsFS, dst: *const c_char) -> c_int;

  pub fn hdfsDelete(srcFS: *mut hdfsFS, path: *const c_char, 
    recursive: c_int) -> c_int;

  pub fn hdfsRename(srcFS: *mut hdfsFS, oldPath: *const c_char, 
    newPath: *const c_char) -> c_int;

  pub fn hdfsGetWorkingDirectory(fs: *mut hdfsFS, buffer: *mut c_char, 
    bufferSize: size_t) -> *mut c_char;

  pub fn hdfsSetWorkingDirectory(fs: *mut hdfsFS, path: *const c_char) 
    -> c_int;

  pub fn hdfsCreateDirectory(fs: *mut hdfsFS, path: *const c_char) -> c_int;

  pub fn hdfsSetReplication(fs: *mut hdfsFS, path: *const c_char, 
    replication: int16_t) -> c_int;

  pub fn hdfsListDirectory(fs: *mut hdfsFS, path: *const c_char,
    numEntries: *mut c_int) -> *mut hdfsFileInfo;

  pub fn hdfsGetPathInfo(fs: *mut hdfsFS, path: *const c_char) 
    -> *mut hdfsFileInfo;

  pub fn hdfsFreeFileInfo(hdfsFileInfo: *mut hdfsFileInfo, numEntries: c_int);

  pub fn hdfsFileIsEncrypted(hdfsFileInfo: *mut hdfsFileInfo) -> c_int;

  pub fn hdfsGetHosts(fs: *mut hdfsFS, path: *const c_char,
            start: tOffset, length: tOffset) -> *mut *mut *mut c_char;

  pub fn hdfsFreeHosts(blockHosts: *mut *mut *mut c_char);

  pub fn hdfsGetDefaultBlockSize(fs: *mut hdfsFS) -> tOffset;

  pub fn hdfsGetDefaultBlockSizeAtPath(fs: *mut hdfsFS, path: *const c_char) 
    -> tOffset;

  pub fn hdfsGetCapacity(fs: *mut hdfsFS) -> tOffset;

  pub fn hdfsGetUsed(fs: *mut hdfsFS) -> tOffset;

  pub fn hdfsChown(fs: *mut hdfsFS, path: *const c_char,
    owner: *const c_char, group: *const c_char) -> c_int;

  pub fn hdfsChmod(fs: *mut hdfsFS, path: *const c_char, mode: c_short) 
    -> c_int;

  pub fn hdfsUtime(fs: *mut hdfsFS, path: *const c_char, mtime: tTime, 
    atime: tTime) -> c_int;

  pub fn hadoopRzOptionsAlloc() -> *mut hadoopRzOptions;

  pub fn hadoopRzOptionsSetSkipChecksum(
            opts: *mut hadoopRzOptions, skip: c_int) -> c_int;

  pub fn hadoopRzOptionsSetByteBufferPool(
            opts: *mut hadoopRzOptions, className: *const c_char) -> c_int;

  pub fn hadoopRzOptionsFree(opts: *mut hadoopRzOptions);

  pub fn hadoopReadZero(file: *mut hdfsFile, opts: *mut hadoopRzOptions, 
    maxLength: int32_t) -> *mut hadoopRzBuffer;

  pub fn hadoopRzBufferLength(buffer: *const hadoopRzBuffer) -> int32_t;

  pub fn hadoopRzBufferGet(buffer: *const hadoopRzBuffer) -> *const c_void;

  pub fn hadoopRzBufferFree(file: *mut hdfsFile, buffer: *mut hadoopRzBuffer);
}

pub enum NativeMiniDfsCluster {}

#[repr(C)]
#[allow(non_snake_case)]
pub struct MiniDfsConf {
  do_format: c_uchar,
  webhdfs_enabled: c_uchar,
  namenode_http_port: c_int,
  short_circuit_enabled: c_uchar
}

impl MiniDfsConf {
  pub fn new() -> MiniDfsConf {
    MiniDfsConf {
      do_format: 1,
      webhdfs_enabled: 0,
      namenode_http_port: 0,
      short_circuit_enabled: 0
    }
  }

  pub fn set_do_format(&mut self, on: bool) -> &mut MiniDfsConf {
    self.do_format = if on { 1 } else { 0 };
    self
  }

  pub fn do_format(&self) -> bool {
    if self.do_format != 0 { true } else { false }
  }

  pub fn set_web_hdfs(&mut self, enable: bool) -> &mut MiniDfsConf {
    self.webhdfs_enabled = if enable { 1 } else { 0 };
    self
  }

  pub fn web_hdfs_enabled(&self) -> bool {
    if self.webhdfs_enabled != 0 { true } else { false } 
  }

  pub fn set_http_port(&mut self, port: i32) -> &mut MiniDfsConf {
    self.namenode_http_port = port as c_int;
    self
  }

  pub fn http_port(&self) -> i32 {
    self.namenode_http_port
  }

  pub fn set_short_circuit(&mut self, enable: bool) -> &mut MiniDfsConf {
    self.short_circuit_enabled = if enable { 1 } else { 0 };
    self
  }

  pub fn short_circuit_enabled(&self) -> bool {
    if self.short_circuit_enabled != 0 { true } else { false } 
  }
}

#[link(name="hdfs")]
extern "C" {
  pub fn nmdCreate(conf: *const MiniDfsConf) -> *mut NativeMiniDfsCluster;

  pub fn nmdWaitClusterUp(cl: *mut NativeMiniDfsCluster) -> c_int;

  pub fn nmdShutdown(cl: *mut NativeMiniDfsCluster) -> c_int;

  pub fn nmdFree(cl: *mut NativeMiniDfsCluster) -> c_void;

  pub fn nmdGetNameNodePort(cl: *const NativeMiniDfsCluster) -> c_int;

  pub fn nmdGetNameNodeHttpAddress(cl: *const NativeMiniDfsCluster,
                               port: *mut c_int, hostName: *mut *mut c_char) -> c_int;

  pub fn nmdConfigureHdfsBuilder(cl: *mut NativeMiniDfsCluster, bld: *mut hdfsBuilder) -> c_int;
}