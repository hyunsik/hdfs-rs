# hdfs-rs

libhdfs binding library and rust APIs which safely wraps libhdfs binding APIs

## Documentation
* [API documentation] (http://hyunsik.github.io/hdfs-rs/)

## Requirements
* Hadoop which is compiled with ``-Pnative``
  * Please fefer to https://github.com/apache/hadoop/blob/trunk/BUILDING.txt

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
