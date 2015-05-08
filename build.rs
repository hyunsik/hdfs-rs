extern crate gcc;

use std::env;

fn main() {

  // for libhdfs.a
  match env::var("HADOOP_HOME") {
    Ok(val) => { 
      println!("cargo:rustc-link-search=native={}/lib/native", val); 
    },
    Err(e) => { panic!("HADOOP_HOME shell environment must be set: {}", e); }
  }

  // for jvm.h and linking to jni libraries
  let mut minidfs_config = gcc::Config::new();  
  minidfs_config.file("src/native/native_mini_dfs.c").include("src/native");

  match env::var("JAVA_HOME") {
    Ok(val) => { 
      minidfs_config
        .include(format!("{}/include/", val))
        // TODO - to be changed to consider a dependent platform.
        .include(format!("{}/include/linux", val));
    },
    Err(e) => { panic!("JAVA_HOME shell environment must be set: {}", e); }
  }

  minidfs_config.compile("libminidfs.a");
}