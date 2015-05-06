use libc::{c_char, c_int, c_short, c_uchar, c_void, int16_t, int32_t, 
  int64_t, uint16_t, size_t, time_t};

/// Opaque Pointer of hdfsFS
#[allow(non_camel_case_types)]
pub enum hdfsFS {}

/// Opaque Pointer of hdfsFile
#[allow(non_camel_case_types)]
pub enum hdfsFile {}

/// Opaque Pointer of hdfsBuilder
#[allow(non_camel_case_types)]
pub enum hdfsBuilder {}

/// Opaque Pointer of hadoopRzOptions
 #[allow(non_camel_case_types)]
pub enum hadoopRzOptions {}

/// Opaque Pointer of hadoopRzBuffer
#[allow(non_camel_case_types)]
pub enum hadoopRzBuffer {}

/// size of data for read/write io ops
#[allow(non_camel_case_types)]
pub type tSize = int32_t;
/// time type in seconds
#[allow(non_camel_case_types)]
pub type tTime = time_t;
/// offset within the file
#[allow(non_camel_case_types)]
pub type tOffset = int64_t;
/// port
#[allow(non_camel_case_types)]
pub type tPort = uint16_t;

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum tObjectKind {
  kObjectKindFile = 0x46, // 'F'
  kObjectKindDirectory = 0x44 // 'D'
}

/// Information about a file/directory.
#[repr(C)]
#[allow(non_snake_case)]
pub struct hdfsReadStatistics {
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
                   stats: &mut *mut hdfsReadStatistics) -> c_int;
  
  /// HDFS read statistics for a file,
  /// 
  /// #### Params
  /// * stats - HDFS read statistics for a file.
  /// 
  /// #### Return
  /// Return the number of remote bytes read.
  pub fn hdfsReadStatisticsGetRemoteBytesRead(
    stats: *const hdfsReadStatistics) -> int64_t;
  
  /// Free some HDFS read statistics.
  ///
  /// #### Params
  /// * stats - The HDFS read statistics to free.
  pub fn hdfsFileFreeReadStatistics(stats: *mut hdfsReadStatistics);
  
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
  pub fn hdfsConnect(host: *const c_char, uint16_t: tPort) -> *mut hdfsFS;
  
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
                    uint16_t: tPort,
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
                            uint16_t: tPort) -> *mut hdfsFS;
  
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

   
  /// Seek to given offset in file.
  ///
  /// This works only for files opened in read-only mode. 
  ///
  /// #### Params
  /// ```fs``` The configured filesystem handle.
  /// ```file``` The file handle.
  /// ```desiredPos``` Offset into the file to seek into.
  ///
  /// #### Return
  /// @return Returns 0 on success, -1 on error.
  pub fn hdfsSeek(fs: *mut hdfsFS, file: *mut hdfsFile, 
    desiredPos: tOffset) -> c_int;

  /// Get the current offset in the file, in bytes.
  ///
  /// #### Params
  ///
  /// ```fs``` - The configured filesystem handle.
  /// ```file``` - The file handle.
  ///
  /// #### Return
  /// Current offset, -1 on error.
  pub fn hdfsTell(fs: *mut hdfsFS, file: *mut hdfsFile) -> tOffset;

  /// Read data from an open file.
  ///
  /// #### Params
  /// * ```fs``` - The configured filesystem handle.
  /// * ```file``` - The file handle.
  /// * ```buffer``` - The buffer to copy read bytes into.
  /// * ```length``` - The length of the buffer.
  ///
  /// #### Return
  /// On success, a positive number indicating how many bytes were read.
  /// On end-of-file, 0. On error, -1.  Errno will be set to the error code.
  /// Just like the POSIX read function, hdfsRead will return -1
  /// and set errno to EINTR if data is temporarily unavailable,
  /// but we are not yet at the end of the file.
  pub fn hdfsRead(fs: *mut hdfsFS, file: *mut hdfsFile, buffer: *mut c_void, 
    length: tSize) -> tSize;

  /// Positional read of data from an open file.
  ///
  /// #### Params
  /// * ```fs``` - The configured filesystem handle.
  /// * ```file``` - The file handle.
  /// * ```position``` - Position from which to read
  /// * ```buffer``` - The buffer to copy read bytes into.
  /// * ```length``` - The length of the buffer.
  ///
  /// #### Return
  /// See hdfsRead
  pub fn hdfsPread(fs: *mut hdfsFS, file: *mut hdfsFile, position: tOffset,
    buffer: *mut c_void, length: tSize) -> tSize;

  /// Write data into an open file.
  ///
  /// #### Params
  /// * ```fs``` - The configured filesystem handle.
  /// * ```file``` - The file handle.
  /// * ```buffer``` - The data.
  /// * ```length``` - The no. of bytes to write. 
  ///
  /// #### Return
  /// the number of bytes written, -1 on error.
  pub fn hdfsWrite(fs: *mut hdfsFS, file: *mut hdfsFile, 
    buffer: *const c_void, length: tSize) -> tSize;

  /// Flush the data. 
  ///
  /// #### Params
  /// * ```fs``` - The configured filesystem handle.
  /// * ```file``` - The file handle.
  ///
  /// #### Return
  /// Returns 0 on success, -1 on error. 
  pub fn hdfsFlush(fs: *mut hdfsFS, file: *mut hdfsFile) -> c_int;

  /// Flush out the data in client's user buffer. After the return of this 
  /// call, new readers will see the data.
  ///
  /// #### Params
  /// * ```fs``` - The configured filesystem handle.
  /// * ```file``` - The file handle.
  ///
  /// #### Return
  /// 0 on success, -1 on error and sets errno
  pub fn hdfsHFlush(fs: *mut hdfsFS, file: *mut hdfsFile) -> c_int;

  /// Similar to posix fsync, Flush out the data in client's 
  /// user buffer. all the way to the disk device (but the disk may have
  /// it in its cache).
  ///
  /// #### Params
  /// * ```fs``` - The configured filesystem handle.
  /// * ```file``` - The file handle.
  ///
  /// #### Return
  /// 0 on success, -1 on error and sets errno
  pub fn hdfsHSync(fs: *mut hdfsFS, file: *mut hdfsFile) -> c_int;

  /// Number of bytes that can be read from this input stream without 
  /// blocking.
  ///
  /// #### Params
  /// * ```fs``` - The configured filesystem handle.
  /// * ```file``` - The file handle.
  ///
  /// #### Return
  /// 0 on success, -1 on error and sets errno
  pub fn hdfsAvailable(fs: *mut hdfsFS, file: *mut hdfsFile) -> c_int;

  /// Copy file from one filesystem to another.
  ///
  /// #### Params
  /// * ```srcFS``` - The handle to source filesystem.
  /// * ```src``` - The path of source file. 
  /// * ```dstFS``` - The handle to destination filesystem.
  /// * ```dst``` - The path of destination file.
  ///
  /// #### Return
  /// Returns 0 on success, -1 on error.
  pub fn hdfsCopy(srcFS: *mut hdfsFS, src: *const c_char, 
    dstFS: *mut hdfsFS, dst: *const c_char) -> c_int;

  /// Move file from one filesystem to another.
  ///
  /// #### Params
  /// * ```srcFS``` - The handle to source filesystem.
  /// * ```src``` - The path of source file. 
  /// * ```dstFS``` - The handle to destination filesystem.
  /// * ```dst``` - The path of destination file. 
  ///
  /// #### Return
  /// Returns 0 on success, -1 on error.
  pub fn hdfsMove(srcFS: *mut hdfsFS, src: *const c_char, 
    dstFS: *mut hdfsFS, dst: *const c_char) -> c_int;

  /// Delete file. 
  ///
  /// #### Params
  /// * ```fs``` - The configured filesystem handle.
  /// * ```path``` - The path of the file. 
  /// * ```recursive``` - if path is a directory and set to 
  /// non-zero, the directory is deleted else throws an exception. In
  /// case of a file the recursive argument is irrelevant.
  ///
  /// #### Return
  /// Returns 0 on success, -1 on error. 
  pub fn hdfsDelete(srcFS: *mut hdfsFS, path: *const c_char, 
    recursive: c_int) -> c_int;

  /// hdfsRename - Rename file. 
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


/// Opaque Pointer for NativeMiniDfsCluster
pub enum NativeMiniDfsCluster {}

/// Represents a configuration to use for creating a Native MiniDFSCluster
#[repr(C)]
#[allow(non_snake_case)]
pub struct MiniDfsConf {
  /// Nonzero if the cluster should be formatted prior to startup.
  do_format: c_uchar,
  /// Whether or not to enable webhdfs in MiniDfsCluster
  webhdfs_enabled: c_uchar,
  /// The http port of the namenode in MiniDfsCluster
  namenode_http_port: c_int,
  /// Nonzero if we should configure short circuit.
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

  /// Set TRUE if the cluster should be formatted prior to startup
  pub fn set_do_format(&mut self, on: bool) -> &mut MiniDfsConf {
    self.do_format = if on { 1 } else { 0 };
    self
  }

  /// The cluster will be formatted prior to startup if TRUE
  pub fn do_format(&self) -> bool {
    if self.do_format != 0 { true } else { false }
  }

  /// Set TRUE in order to enable webhdfs in MiniDfsCluster
  pub fn set_web_hdfs(&mut self, enable: bool) -> &mut MiniDfsConf {
    self.webhdfs_enabled = if enable { 1 } else { 0 };
    self
  }

  /// webhdfs in MiniDfsCluster will be available if TRUE
  pub fn web_hdfs_enabled(&self) -> bool {
    if self.webhdfs_enabled != 0 { true } else { false } 
  }

  /// Set http port of the namenode in MiniDfsCluster
  pub fn set_http_port(&mut self, port: i32) -> &mut MiniDfsConf {
    self.namenode_http_port = port as c_int;
    self
  }

  /// The http port of the namenode in MiniDfsCluster
  pub fn http_port(&self) -> i32 {
    self.namenode_http_port
  }

  /// Set TRUE if we should configure short circuit.
  pub fn set_short_circuit(&mut self, enable: bool) -> &mut MiniDfsConf {
    self.short_circuit_enabled = if enable { 1 } else { 0 };
    self
  }

  /// short circuit will be available if TRUE
  pub fn short_circuit_enabled(&self) -> bool {
    if self.short_circuit_enabled != 0 { true } else { false } 
  }
}

#[link(name="hdfs")]
extern "C" {
  /// Create a NativeMiniDfsCluster
  ///
  /// #### Params
  /// * ```conf``` - (inout) The cluster configuration
  ///
  /// #### Return
  /// * Return a ```NativeMiniDfsCluster````, or a NULL pointer on error.
  pub fn nmdCreate(conf: *const MiniDfsConf) -> *mut NativeMiniDfsCluster;

  /// Wait until a MiniDFSCluster comes out of safe mode.
  ///
  /// #### Params
  /// * ```cl``` - The cluster
  ///
  /// #### Return
  /// * 0 on success; a non-zero error code if the cluster fails to
  /// come out of safe mode.
  pub fn nmdWaitClusterUp(cl: *mut NativeMiniDfsCluster) -> c_int;

  /// Shut down a NativeMiniDFS cluster
  ///
  /// #### Params
  /// * ```cl``` - The cluster
  /// 
  /// #### Return
  /// * 0 on success; a non-zero error code if an exception is thrown.
  pub fn nmdShutdown(cl: *mut NativeMiniDfsCluster) -> c_int;

  /// Destroy a Native MiniDFSCluster
  ///
  /// #### Params
  /// * ```cl``` - The cluster to destroy
  pub fn nmdFree(cl: *mut NativeMiniDfsCluster) -> c_void;

  /// Get the port that's in use by the given (non-HA) nativeMiniDfs
  ///
  /// #### Params
  /// * ```cl``` - The initialized NativeMiniDfsCluster
  ///
  /// #### Return
  /// the port, or a negative error code
  pub fn nmdGetNameNodePort(cl: *const NativeMiniDfsCluster) -> c_int;

  /// Get the http address that's in use by the given (non-HA) nativeMiniDfs
  ///
  /// #### Params
  /// * ```cl``` - The initialized NativeMiniDfsCluster
  /// * ```port``` - Used to capture the http port of the NameNode 
  /// of the NativeMiniDfsCluster
  /// * hostName  Used to capture the http hostname of the NameNode
  /// of the NativeMiniDfsCluster
  pub fn nmdGetNameNodeHttpAddress(cl: *const NativeMiniDfsCluster,
    port: *mut c_int, hostName: *mut *mut c_char) -> c_int;

  /// Configure the HDFS builder appropriately to connect to this cluster.
  ///
  /// #### Params
  /// * ```bld``` - The hdfs builder
  /// 
  /// #### Return
  /// the port, or a negative error code
  pub fn nmdConfigureHdfsBuilder(cl: *mut NativeMiniDfsCluster, 
    bld: *mut hdfsBuilder) -> c_int;
}