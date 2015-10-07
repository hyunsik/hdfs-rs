pub enum HdfsErr {
  FileNotFound(String),
  FileAlreadyExists(String),
  UNKNOWN
}