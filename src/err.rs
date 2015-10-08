/// Errors which can occur during accessing Hdfs cluster 
pub enum HdfsErr {
  Unknown,
  /// file path
  FileNotFound(String),
  /// file path           
  FileAlreadyExists(String),
  /// namenode address      
  CannotConnectToNameNode(String),
  /// URL 
  InvalidUrl(String) 
}