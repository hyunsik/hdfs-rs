# hdfs-rs

libhdfs binding library and rust APIs which safely wraps libhdfs binding APIs

# Current Status
 * Alpha Status (Rust wrapping APIs can be changed)
 * libhdfs C APIs all are ported.
 * Some of Rust wrapping APIs are implemented.

## Documentation
* [API documentation] (http://hyunsik.github.io/hdfs-rs/)

## Requirements
* Hadoop compiled with native library (i.e., maven profile ``-Pnative``)
  * Please refer to https://github.com/apache/hadoop/blob/trunk/BUILDING.txt if you need more description.

## Usage
Add this to your Cargo.toml:

```toml
[dependencies]
hdfs = "0.0.2"
```

and this to your crate root:
```rust
extern crate hdfs;
```

hdfs-rs uses libhdfs, which is JNI native implementation. JNI native implementation requires the proper ``CLASSPATH``. ``exec.sh`` included in the source code root plays a role to execute your program with the proper ``CLASSPATH``. ``exec.sh`` requires ``HADOOP_HOME``. So, you firstly set ``HADOOP_HOME`` shell environment variable as follows:

````sh
export HADOOP_HOME=<hadoop install dir>
```

Then, you can execute your program as follows:

```bash
./exec your_program arg1 arg2
```
