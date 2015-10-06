#!/bin/bash

FILES=("exception.h" "hdfs.h" "hdfs_test.h" "jni_helper.h" "native_mini_dfs.c" "native_mini_dfs.h")
PLATFORM=("platform.h")

for x in ${FILES[@]}; do
  diff -u src/native/${x} ${1}/hadoop-hdfs-project/hadoop-hdfs/src/main/native/libhdfs/$x > /dev/null
  result=(`echo $?`)

  if [ $result -eq 1 ]
  then
    echo "$x needs update"
  fi
done

diff -u src/native/platform.h ${1}/hadoop-hdfs-project/hadoop-hdfs/src/main/native/libhdfs/os/posix/platform.h > /dev/null

if [ $result -eq 1 ]
then
    echo "platform.h needs update"
fi

