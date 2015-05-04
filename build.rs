extern crate gcc;

use std::env;

fn main() {
  println!("Hadoop!!!!!!");
  match env::var("HADOOP_HOME") {
    Ok(val) => { 
      println!("cargo:rustc-link-search=native={}/lib/native", val); 
    },
    Err(e) => { panic!("ERROR: {}", e); }
  }

  let mut minidfs_config = gcc::Config::new();  
  minidfs_config.file("src/native/native_mini_dfs.c").include("src/native");

  match env::var("JAVA_HOME") {
    Ok(val) => { 
      minidfs_config
        .include(format!("{}/include/", val))
        // TODO - to be changed to consider a dependent platform.
        .include(format!("{}/include/linux", val));
    },
    Err(e) => { panic!("ERROR: {}", e); }
  }
  minidfs_config.compile("libminidfs.a");
}